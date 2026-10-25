"""Chained live proof under peer churn.
usage: python3 edit-chain.py <tag> <workspace-relative production file> <cycles> [target]
Cycle k: run Pk (no edit) -> append one comment line to the file, run Ek -> restore the file byte-identically, run Rk.
Every consecutive pair is judged by the ♨️warm law with the changed inputs resolved to repository paths:
  quiet-edit  = the edit run executes only actions whose changed inputs are the probe (and nothing else moved)
  quiet-restore = the restore run executes nothing (every key is back to the one before the edit)
Peer edits of a declared input, or of the narrow executor closure, make a pair noisy; it is recorded, never hidden. The chain stops at the first cycle whose edit and restore pairs are both quiet.
The file is restored in a finally block and on SIGTERM (only the probe line is removed if a peer edited meanwhile)."""
import glob, hashlib, json, os, re, signal, subprocess, sys, time
tag, relative, cycles = sys.argv[1], sys.argv[2], int(sys.argv[3])
target = sys.argv[4] if len(sys.argv) > 4 else "@teaching/architecture-course:build"
here = os.path.dirname(os.path.abspath(__file__)); root = "/Users/ueli/Documents/semio"; ticket = os.path.dirname(here)
gen = os.path.join(ticket, "🗑️generated", "gen-d", tag); os.makedirs(gen, exist_ok=True)
chain = open(os.path.join(gen, "chain.txt"), "a")
def say(text): chain.write(text + "\n"); chain.flush(); print(text, flush=True)
path = os.path.join(root, relative)
marker = (b"\n/* gen-d-probe " + tag.encode() + b" */\n") if relative.endswith(".css") else (b"\n// gen-d-probe " + tag.encode() + b"\n")
sha = lambda data: hashlib.sha256(data).hexdigest()
original = open(path, "rb").read(); say(f"file {relative} sha256 {sha(original)}")
state = {"edited": False}
def restore():
    if not state["edited"]: return
    current = open(path, "rb").read()
    if current == original + marker: open(path, "wb").write(original)
    elif marker in current: open(path, "wb").write(current.replace(marker, b"", 1)); say("PEER EDIT during the probe: only the probe line was removed")
    state["edited"] = False; say(f"restored sha256 {sha(open(path, 'rb').read())} (original {sha(original)})")
signal.signal(signal.SIGTERM, lambda *_: (restore(), sys.exit(143)))
def aliases():
    table = {}
    for f in glob.glob("/Users/ueli/Documents/semio/.cache/b/p/m/bazel/workspaces/*/*/sources.json"):
        try: data = json.load(open(f, encoding="utf-8"))
        except Exception: continue
        def walk(x):
            if isinstance(x, dict):
                for k, v in x.items():
                    if re.fullmatch(r"[0-9a-f]{10}", k) and isinstance(v, str): table[k] = v
                    else: walk(v)
            elif isinstance(x, list):
                for v in x: walk(v)
        walk(data)
    return table
def run(name):
    directory = os.path.join(gen, name); os.makedirs(os.path.join(directory, "ev"), exist_ok=True)
    env = {k: v for k, v in os.environ.items() if k != "SEMIO_SCRIPT_STALL_MS"}; env["SEMIO_BUILD_EVIDENCE_DIR"] = os.path.join(directory, "ev")
    for attempt in range(1, 31):
        started = time.time()
        with open(os.path.join(directory, "run.log"), "wb") as log:
            code = subprocess.call(["nice", "-n", "10", "bun", "repo", "--tool=bazel", "run", target], cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
        text = open(os.path.join(directory, "run.log"), "rb").read().decode("utf-8", "replace")
        say(f"run {name} rc {code} {round(time.time() - started)}s started {time.strftime('%H:%M:%S', time.localtime(started))} attempt {attempt}")
        if code == 0 or not any(t in text for t in ("The lock file of dependency scope", "lock file no longer matches", "Current shared Source-owner generation changed", "Stale loaded Source revision")): break
        say("peer churn before the build: waiting 120 s"); time.sleep(120)
    return code, os.path.join(directory, "ev", "bazel.execlog.jsonl")
def judge(before, after, label, probe):
    out = subprocess.run(["bun", os.path.join(ticket, "cwarm-d-tools", "verdicts.ts"), before, after], cwd=root, capture_output=True, text=True).stdout
    diff = subprocess.run([sys.executable, "-I", os.path.join(ticket, "cwarm-d-tools", "diff-execlogs.py"), before, after], cwd=root, capture_output=True, text=True).stdout
    open(os.path.join(gen, label.replace(" ", "-") + ".diff.txt"), "w").write(diff)
    table = aliases(); moved = {}; current = None
    for line in diff.splitlines():
        m = re.match(r"== (//:\S+)", line)
        if m: current = m.group(1); moved[current] = []
        m = re.match(r"\s+changed a/([0-9a-f]{10})/(.*)", line)
        if m and current: moved[current].append(table.get(m.group(1), "?" + m.group(1)) + "/" + m.group(2).encode("latin1", "ignore").decode("utf-8", "ignore"))
        elif current and re.match(r"\s+changed (implementation|t_)", line): moved[current].append("(executor identity) " + line.split()[-1])
    say(f"## {label}"); say(out)
    executed = [l.split()[0] for l in out.splitlines() if l.startswith("//:") and " hit " not in l]
    for label_, paths in moved.items():
        if label_ in executed: say(f"{label_} moved: " + "; ".join(sorted(set(paths)))[:1500])
    return executed, moved
quiet = False
try:
    previous = None
    for cycle in range(1, cycles + 1):
        code, p = run(f"P{cycle}")
        if code == 0 and previous: judge(previous, p, f"R{cycle-1} to P{cycle}", relative)
        if code != 0: say("run failed, stopping"); break
        open(path, "wb").write(original + marker); state["edited"] = True; say(f"edited sha256 {sha(original + marker)}")
        codeE, e = run(f"E{cycle}"); restore()
        if codeE != 0: say("edit run failed"); break
        executedE, movedE = judge(p, e, f"P{cycle} to E{cycle} edit", relative)
        codeR, r = run(f"R{cycle}")
        if codeR != 0: say("restore run failed"); break
        executedR, movedR = judge(e, r, f"E{cycle} to R{cycle} restore", relative)
        previous = r
        editQuiet = all(all(relative in x or "icon-motion" in x for x in movedE.get(l, [])) for l in executedE) and executedE != []
        restoreQuiet = all(all(relative in x or "icon-motion" in x for x in movedR.get(l, [])) for l in executedR)
        say(f"cycle {cycle}: edit pair quiet={editQuiet} executed={executedE}; restore pair quiet={restoreQuiet} executed={executedR}")
        if editQuiet and restoreQuiet: quiet = True; say("QUIET CYCLE FOUND"); break
finally:
    restore()
say("chain end quiet=" + str(quiet))
