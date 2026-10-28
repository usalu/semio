"""python3 pair-driver.py <tag> <target> <probe-file> : the one live warm pair of exec-f.
Run A of `bun repo --tool=bazel run <target>` (candidate scope table with the scheduling domains, execution log on), then an unrelated SCHEDULING-file probe (one `// [DEBUG] exec-f probe` line appended to <probe-file>), then run B, then the probe line is removed again. Verdict: the ♨️warm law of cwarm-d (verdicts.ts) between A and B and the digests of the keyed snapshot and the overlay before and after (the keyed digest must not move while the overlay does). Output under C/🗑️generated/exec-f/<tag>/."""
import glob, hashlib, json, os, subprocess, sys, time
tag, target, probe, apid = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
root = "/Users/ueli/Documents/semio"
C = os.path.join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN")
out = os.path.join(C, "🗑️generated/exec-f", tag); os.makedirs(out, exist_ok=True)
candidate = os.path.join(C, "🗑️generated/exec-f/scopes-scheduling.json")
log = open(os.path.join(out, "driver.log"), "a", buffering=1)
def say(text): log.write(time.strftime("%H:%M:%S ") + text + "\n")
def run(name):
    d = os.path.join(out, name); os.makedirs(os.path.join(d, "ev"), exist_ok=True)
    env = {k: v for k, v in os.environ.items() if k != "SEMIO_SCRIPT_STALL_MS"}
    env["SEMIO_BUILD_EVIDENCE_DIR"] = os.path.join(d, "ev"); env["SEMIO_BUILD_IMPLEMENTATION_SCOPES"] = candidate; (env.update(SEMIO_BUILD_GRAPH_POLICY=os.environ["GRAPH_POLICY"]) if os.environ.get("GRAPH_POLICY") else env.pop("SEMIO_BUILD_GRAPH_POLICY", None))
    started = time.time(); say(f"{name} start")
    with open(os.path.join(d, "run.log"), "wb") as handle:
        code = subprocess.call(["nice", "-n", "10", "bun", "repo", "--tool=bazel", "run", target], cwd=root, env=env, stdout=handle, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
    open(os.path.join(d, "rc"), "w").write(f"{code} {round(time.time() - started)}s\n"); say(f"{name} rc {code} {round(time.time() - started)}s")
    return code
def digests():
    rows = []
    for path in glob.glob(os.path.join(root, ".cache/b/p/m/bazel/workspaces/*/*/implementation.executor-command.json")):
        rows.append((os.path.getmtime(path), path))
    if not rows: return {}
    path = max(rows)[1]; folder = os.path.dirname(path)
    h = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest() if os.path.exists(p) else None
    return {"workspace": folder, "keyed": h(path), "overlay": h(path[:-5] + ".scheduling.json")}
def alive(pid):
    try: os.kill(pid, 0); return True
    except OSError: return False
say("waiting for A pid %d" % apid)
while alive(apid): time.sleep(10)
ok = os.path.exists(os.path.join(out, "A", "run.log")) and "Build completed successfully" in open(os.path.join(out, "A", "run.log"), errors="replace").read()
open(os.path.join(out, "A", "rc"), "w").write("0 external\n" if ok else "1 external\n")
if not ok: say("A did not complete successfully, stopping"); sys.exit(1)
say("A ok")
before = digests(); say("digests A " + json.dumps(before))
path = os.path.join(root, probe); marker = "// [DEBUG] exec-f probe " + tag + "\n"
original = open(path, encoding="utf8").read(); open(path, "w", encoding="utf8").write(original + ("" if original.endswith("\n") else "\n") + marker); say("probe written " + probe)
try:
    code = run("B")
finally:
    text = open(path, encoding="utf8").read()
    if marker in text: open(path, "w", encoding="utf8").write(text.replace(marker, "")); say("probe removed")
after = digests(); say("digests B " + json.dumps(after))
verdict = []
if code == 0:
    a, b = (os.path.join(out, n, "ev", "bazel.execlog.jsonl") for n in "AB")
    r = subprocess.run(["bun", os.path.join(C, "cwarm-d-tools", "verdicts.ts"), a, b], cwd=root, capture_output=True, text=True); verdict += [r.stdout, r.stderr]
    r = subprocess.run([sys.executable, "-I", os.path.join(C, "cwarm-d-tools", "diff-execlogs.py"), a, b], cwd=root, capture_output=True, text=True); verdict += ["--- diff", r.stdout, r.stderr]
verdict.append(json.dumps({"before": before, "after": after, "keyedMoved": before.get("keyed") != after.get("keyed"), "overlayMoved": before.get("overlay") != after.get("overlay")}))
open(os.path.join(out, "verdict.txt"), "w").write("\n".join(verdict)); say("done")
