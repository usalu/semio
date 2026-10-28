"""[lane diskproof] runs the laws of one staged batch against a root (a mirror or the checkout): `verify_batch.py <batch-dir> <root> [before]`.
after  (default): the batch is applied to <root> when it is not yet, then every law runs; exit 0 and `VERIFY OK <batch>` when the failing tests are exactly the batch's known-red set (artifacts of a mirror without .git/.github).
before: the batch's TEST files are applied, every other file of the batch is put back to its base bytes, the laws run and must fail (RED as expected); the batch is applied again afterwards.
Each bun law runs alone from <root>; SEMIO_TEST_ARTIFACT_DIR is <root>/.tmp-diskproof."""
import json, os, re, subprocess, sys, shutil
HERE = os.path.dirname(os.path.abspath(sys.argv[1]))
BATCH = os.path.abspath(sys.argv[1])
ROOT = os.path.abspath(sys.argv[2])
BEFORE = len(sys.argv) > 3 and sys.argv[3] == "before"
laws = json.load(open(os.path.join(BATCH, "laws.json"), encoding="utf8"))
manifest = json.load(open(os.path.join(BATCH, "manifest.json"), encoding="utf8"))
def stage(verb):
    return subprocess.run([sys.executable, "-I", os.path.join(BATCH, "stage.py"), verb, "--root", ROOT], capture_output=True, text=True)
def live_git_root():
    here = BATCH
    while here != os.path.dirname(here):
        if os.path.isdir(os.path.join(here, ".git")): return here
        here = os.path.dirname(here)
    return ROOT
def law_lists():
    base = live_git_root(); out = {}
    for name, marker in (("SEMIO_CACHE_LAW_FILES", "⚡️cache"), ("SEMIO_CACHE_NATIVE_LAW_FILES", ".cache")):
        run = subprocess.run(["git", "-c", "core.quotepath=off", "grep", "-lFz", marker, "--", ".", ":(exclude).🧬semio"], cwd=base, capture_output=True, text=True)
        path = os.path.join(ROOT, ".tmp-diskproof", name.lower() + ".txt")
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf8").write("\n".join(x for x in run.stdout.split("\0") if x) + "\n"); out[name] = path
    return out
def run_laws():
    env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=os.path.join(ROOT, ".tmp-diskproof", "art"), **law_lists())
    os.makedirs(env["SEMIO_TEST_ARTIFACT_DIR"], exist_ok=True)
    failing = []; total = [0, 0]
    for law in laws["laws"]:
        if law["kind"] == "bun":
            cmd = ["bun", "test", "./" + law["file"]] + (["-t", law["filter"]] if law.get("filter") else [])
            run = subprocess.run(cmd, cwd=ROOT, env=env, capture_output=True, text=True)
            text = run.stdout + run.stderr
            failing += [(law["file"][-50:], re.sub(r" \[[0-9.]+m?s\]$", "", m)) for m in re.findall(r"^\(fail\) (.*)$", text, re.M)]
            for kind, index in (("pass", 0), ("fail", 1)):
                m = re.findall(r"^\s*(\d+) %s$" % kind, text, re.M)
                total[index] += int(m[-1]) if m else 0
            if not re.findall(r"^\s*\d+ pass$", text, re.M): failing.append((law["file"][-50:], "no result: " + text[-200:].replace("\n", " ")))
            elif run.returncode != 0 and not re.findall(r"^\(fail\) ", text, re.M): failing.append((law["file"][-50:], "exit %d without a failing test (load error): %s" % (run.returncode, re.sub(r"\s+", " ", " ".join(re.findall(r"^error: .*$", text, re.M)))[:160])))
        else:
            run = subprocess.run([sys.executable, "-I", os.path.join(ROOT, law["script"])] + [os.path.join(ROOT, a) for a in law["args"]], cwd=ROOT, capture_output=True, text=True)
            if run.returncode != 0: failing.append((law["script"][-50:], "python law failed: " + run.stderr[-300:].replace("\n", " ")))
            else: total[0] += 1
    return failing, total
known = set(laws.get("knownRed", []))
if BEFORE:
    tests = {rel for rel, row in manifest.items() if rel in set(laws["tests"])}
    stage("apply")
    for rel, row in manifest.items():
        if rel in tests: continue
        target = os.path.join(ROOT, rel)
        if row["base"] is None:
            if os.path.exists(target): os.remove(target)
        else: shutil.copyfile(os.path.join(BATCH, "base", row["file"]), target)
    failing, total = run_laws()
    stage("apply")
    mine = [f for f in failing if f[1] not in known]
    print("before: %d pass, %d fail (unexpected-for-base: %d)" % (total[0], total[1], len(mine)))
    for f in mine: print("  RED", f[0], "|", f[1][:140])
    print("RED AS EXPECTED" if mine else "NOT RED: the laws do not fail on base")
    sys.exit(0 if mine else 1)
if stage("check").returncode != 0 and stage("apply").returncode != 0:
    print("DRIFT: the batch does not apply to", ROOT); sys.exit(2)
stage("apply")
failing, total = run_laws()
unexpected = [f for f in failing if f[1] not in known]
print("after: %d pass, %d fail, %d unexpected" % (total[0], total[1], len(unexpected)))
for f in unexpected: print("  FAIL", f[0], "|", f[1][:200])
print(("VERIFY OK " + os.path.basename(BATCH)) if not unexpected else "VERIFY FAILED " + os.path.basename(BATCH))
sys.exit(0 if not unexpected else 1)
