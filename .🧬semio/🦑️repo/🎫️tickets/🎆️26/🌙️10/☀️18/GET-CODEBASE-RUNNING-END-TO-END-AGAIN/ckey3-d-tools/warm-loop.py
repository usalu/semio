"""Chained warm proof: python3 warm-loop.py <prefix> <previous-run-dir-or-> <maxRuns> <target> [extra repo args...]
Runs the user's command (default config, execution log on) again and again, every run compared with the one before it by the ♨️warm law (cwarm-d-tools/verdicts.ts); stops at the first run that executes nothing (no key-changed, new, uncached or executed-with-stable-key action) and writes <prefix>-chain.txt with one verdict block per link. Chaining makes every link as short as one build, so a peer edit of a declared input or of the narrow closure only costs the link it lands in."""
import os, subprocess, sys, time
prefix, previous, limit, target, extra = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[4], sys.argv[5:]
here = os.path.dirname(os.path.abspath(__file__)); root = "/Users/ueli/Documents/semio"
gen = os.path.join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/ckey3-d")
chain = open(os.path.join(gen, prefix + "-chain.txt"), "a")
env = dict(os.environ, TRACE_WORKERS="default")
for index in range(1, limit + 1):
    tag = f"{prefix}{index}"; started = time.strftime("%H:%M:%S")
    subprocess.call([sys.executable, os.path.join(here, "trace-run.py"), tag, target, *extra], env=env)
    current = os.path.join(gen, tag); rc = open(os.path.join(current, "rc")).read().strip()
    chain.write(f"## {tag} started {started} rc {rc} vs {previous}\n"); chain.flush()
    verdict = None
    if previous != "-" and rc.startswith("0 "):
        a, b = (os.path.join(gen, previous if previous != '-' else '', "ev", "bazel.execlog.jsonl"), os.path.join(current, "ev", "bazel.execlog.jsonl"))
        run = subprocess.run(["bun", os.path.join(os.path.dirname(here), "cwarm-d-tools", "verdicts.ts"), a, b], cwd=root, capture_output=True, text=True)
        chain.write(run.stdout + run.stderr); chain.flush()
        bad = [line for line in run.stdout.splitlines() if line.startswith("//:") and " hit " not in line]
        verdict = not bad and run.returncode == 0
        if verdict: chain.write("ZERO re-executions\n"); chain.flush(); break
        run2 = subprocess.run([sys.executable, "-I", os.path.join(os.path.dirname(here), "cwarm-d-tools", "diff-execlogs.py"), a, b], cwd=root, capture_output=True, text=True)
        open(os.path.join(current, "diff.txt"), "w").write(run2.stdout + run2.stderr)
    previous = tag
    time.sleep(5)
