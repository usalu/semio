"""Runs the reader test files with `bun test ./<path>` from the repo root and records pass/fail per file. usage: readers_tests.py <label>"""
import os, re, subprocess, sys, time
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
M2 = "🧰️framework/🛍️products/💻️os/🔨️modules"
FILES = [
    "🧰️framework/🔨️modules/🚪️io/📦️codec/🧪️tests/🟦️.ts",
    f"{M2}/🚪️io/📸️typed-snapshot/🧪️tests/🟦️.ts",
    f"{M2}/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement/🟦️.ts",
    f"{M2}/🏃️run/🧪️tests/🎟️original-call/🟦️.ts",
    f"{M2}/🔌️plugin/🖥️host/🔔️wake/📃️policy/🧪️tests/🟦️.ts",
    f"{M2}/🔌️plugin/🧠️interpreter/🏃️runtime/🏗️startup/🧪️tests/🟦️.ts",
]
extra = sys.argv[2:]
label = sys.argv[1]
for i, f in enumerate(FILES + extra, 1):
    log = f"{F}/🗑️generated/orv-{label}-reader-{i}.log"
    t0 = time.time()
    p = subprocess.run(["perl", "-e", "alarm 280; exec @ARGV", "--", "bun", "test", "./" + f], cwd=ROOT, capture_output=True, text=True)
    out = p.stdout + p.stderr
    write(log, out)
    tail = [l for l in out.split("\n") if re.match(r"^\s*(\d+ (pass|fail)|Ran \d+ tests)", l) or "panic" in l or "error:" in l.lower()][:6]
    print(f"[{i}] rc={p.returncode} {time.time()-t0:.0f}s {f.split('/')[-4]}/{f.split('/')[-3]}: {' | '.join(t.strip() for t in tail)}", flush=True)
