"""🔬️ Runs every puzzle3d test in its own process, so neither cross-test pollution nor a destructor that
aborts the binary can hide a verdict — the measurement behind 📓️2026-09-09-wave-X-test-suite.md §4.

Build the binary first:
  RUSTC_WRAPPER="" CARGO_TARGET_DIR=<dir> cargo test -p semio-s-artifact-puzzle-3d \
      --features component-app-assembly -j 4 --no-run
then point SC at <dir>'s parent and run this. Writes p3d-isolated.txt beside it.
"""
import os, subprocess, pathlib, re, sys, glob

SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad"
env = dict(os.environ, RUST_MIN_STACK="134217728")
binaries = sorted(glob.glob(f"{SC}/target-p3d/debug/deps/semio_s_artifact_puzzle_3d-*"), key=os.path.getmtime)
binaries = [b for b in binaries if os.access(b, os.X_OK) and not b.endswith(".d")]
BIN = binaries[-1]
print("binary", BIN, flush=True)
names = subprocess.run([BIN, "--list"], capture_output=True, text=True, env=env).stdout.splitlines()
names = [l[:-len(": test")] for l in names if l.endswith(": test")]
print("tests", len(names), flush=True)

results, reasons = {}, {}
for i, name in enumerate(names):
    p = subprocess.run([BIN, "--exact", "--test-threads=1", "--nocapture", name], capture_output=True, text=True, env=env)
    text = p.stdout + p.stderr
    status = "ABORTED"
    if re.search(r"^test result: ok\.", text, re.M):
        status = "ok"
    elif re.search(r"^test result: FAILED", text, re.M):
        status = "FAILED"
    results[name] = status
    if status != "ok":
        lines = text.splitlines()
        why = []
        for j, line in enumerate(lines):
            if "panicked at" in line:
                why = [l.strip() for l in lines[j + 1:j + 4]]
                break
        reasons[name] = " | ".join(why)[:300] or "<no panic captured>"
    if (i + 1) % 100 == 0:
        print(f"[{i+1}/{len(names)}] failed={sum(1 for v in results.values() if v != 'ok')}", flush=True)

failed = {k: v for k, v in results.items() if v != "ok"}
pathlib.Path(f"{SC}/p3d-isolated.txt").write_text(
    f"total {len(results)} ok {len(results)-len(failed)} failed {len(failed)}\n\n"
    + "\n".join(f"{v}\t{k}\n    {reasons.get(k,'')}" for k, v in sorted(failed.items())) + "\n"
)
print("total", len(results), "ok", len(results) - len(failed), "failed", len(failed))
