"""Run a ticket script with repo bun. argv: script-name [args...]"""
import os
import subprocess
import sys

here = os.path.dirname(os.path.abspath(__file__))
ticket = os.path.dirname(os.path.dirname(here))
gen = next(n for n in os.listdir(ticket) if "generated" in n)
outdir = os.path.join(ticket, gen, "NW-FIX")
os.makedirs(outdir, exist_ok=True)
base = "/home/deck/semio"
teach = next(x for x in os.listdir(base) if "teaching" in x)
staff = None
for dp, ds, fs in os.walk(os.path.join(base, teach)):
    if dp.endswith("staffage") and any("modules" in d for d in ds):
        staff = dp
        break
script = os.path.join(here, sys.argv[1])
out = os.path.join(outdir, sys.argv[2] if len(sys.argv) > 2 else "out.txt")
cmd = ["repo", "tools", "exec", "bun", "--", script, out, *sys.argv[3:]]
print("[DEBUG] cwd", staff)
print("[DEBUG] out", out)
raise SystemExit(subprocess.run(cmd, cwd=staff).returncode)
