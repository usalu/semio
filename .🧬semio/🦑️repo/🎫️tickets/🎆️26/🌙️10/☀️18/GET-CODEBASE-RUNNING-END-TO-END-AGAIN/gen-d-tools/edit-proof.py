"""Live proof of the generator key narrowing.
usage: python3 edit-proof.py <tag> <workspace-relative production file> [target]
Runs the user's command `bun repo --tool=bazel run <target>` (default config, execution log on) as A, appends one comment line to the file (B), restores the file byte-identically and runs again (C).
Writes <tag>-chain.txt next to the runs: the ♨️warm verdict of every action A->B and B->C, the changed inputs with their aliases resolved, and the sha256 of the file before, during and after.
The file is restored in a finally block and on SIGTERM; a peer edit during the window is reported, never overwritten (only the probe line is removed)."""
import hashlib, os, signal, subprocess, sys, time
tag, relative = sys.argv[1], sys.argv[2]
target = sys.argv[3] if len(sys.argv) > 3 else "@teaching/architecture-course:build"
here = os.path.dirname(os.path.abspath(__file__)); root = "/Users/ueli/Documents/semio"
ticket = os.path.dirname(here)
gen = os.path.join(ticket, "🗑️generated", "gen-d", tag); os.makedirs(gen, exist_ok=True)
chain = open(os.path.join(gen, "chain.txt"), "a")
def say(text): chain.write(text + "\n"); chain.flush(); print(text, flush=True)
path = os.path.join(root, relative)
marker = (b"\n/* gen-d-probe " + tag.encode() + b" */\n") if relative.endswith(".css") else (b"\n// gen-d-probe " + tag.encode() + b"\n")
sha = lambda data: hashlib.sha256(data).hexdigest()
original = open(path, "rb").read(); say(f"file {relative} sha256 {sha(original)} bytes {len(original)}")
state = {"edited": False}
def restore():
    if not state["edited"]: return
    current = open(path, "rb").read()
    if current == original + marker: open(path, "wb").write(original)
    elif marker in current: open(path, "wb").write(current.replace(marker, b"", 1)); say("PEER EDIT during the probe: only the probe line was removed")
    else: say("probe line already gone")
    state["edited"] = False
    say(f"restored sha256 {sha(open(path, 'rb').read())} (original {sha(original)})")
signal.signal(signal.SIGTERM, lambda *_: (restore(), sys.exit(143)))
def run(name):
    directory = os.path.join(gen, name); os.makedirs(os.path.join(directory, "ev"), exist_ok=True)
    env = {k: v for k, v in os.environ.items() if k != "SEMIO_SCRIPT_STALL_MS"}; env["SEMIO_BUILD_EVIDENCE_DIR"] = os.path.join(directory, "ev")
    started = time.time()
    with open(os.path.join(directory, "run.log"), "wb") as log:
        code = subprocess.call(["nice", "-n", "10", "bun", "repo", "--tool=bazel", "run", target], cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
    say(f"run {name} rc {code} {round(time.time() - started)}s started {time.strftime('%H:%M:%S', time.localtime(started))}")
    return code, os.path.join(directory, "ev", "bazel.execlog.jsonl")
def compare(before, after, label):
    say(f"## {label}")
    for script in ("verdicts.ts", "diff-execlogs.py"):
        tool = os.path.join(ticket, "cwarm-d-tools", script)
        out = subprocess.run((["bun", tool] if script.endswith(".ts") else [sys.executable, "-I", tool]) + [before, after], cwd=root, capture_output=True, text=True)
        say(f"### {script}"); say(out.stdout + out.stderr)
        if script == "diff-execlogs.py":
            open(os.path.join(gen, label.replace(" ", "-") + ".diff.txt"), "w").write(out.stdout)
            moved = subprocess.run([sys.executable, os.path.join(ticket, "cinp-d-tools", "moved.py"), os.path.join(gen, label.replace(" ", "-") + ".diff.txt")], capture_output=True, text=True)
            say("### moved inputs (aliases resolved)"); say(moved.stdout + moved.stderr)
def churn(name):
    log = open(os.path.join(gen, name, "run.log"), "rb").read().decode("utf-8", "replace")
    return any(text in log for text in ("lock file no longer matches its manifests", "The lock file of dependency scope", "Current shared Source-owner generation changed", "Stale loaded Source revision"))
def stable(name):
    for attempt in range(1, 31):
        label = name if attempt == 1 else f"{name}{attempt}"
        code, execlog = run(label)
        if code == 0 and os.path.exists(execlog): return code, execlog
        if not churn(label): break
        say(f"peer churn (dependency lock or Source generation) in run {label}, attempt {attempt}: waiting 120 s and trying again"); time.sleep(120)
    return code, execlog
try:
    codeA, a = stable("A")
    if codeA != 0: say("A failed: no edit applied, stopping"); sys.exit(2)
    open(path, "wb").write(original + marker); state["edited"] = True; say(f"edited sha256 {sha(original + marker)}")
    codeB, b = run("B")
    restore()
    if codeB != 0: say("B failed after the edit")
    codeC, c = run("C")
    for before, after, label in ((a, b, "A to B edit"), (b, c, "B to C restore"), (a, c, "A to C")) if codeA == 0 else ():
        if os.path.exists(before) and os.path.exists(after): compare(before, after, label)
finally:
    restore()
