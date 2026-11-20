"""🔎 Run catalog validation for each equipment asset and retain per-asset diagnostics during review."""
import os
import subprocess
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
catalog = next((ticket.parents[6] / "🎓️teaching").glob("*architecture/*staffage/*catalog"))
output = ticket / "🗑️generated" / "equipment-check"
output.mkdir(parents=True, exist_ok=True)
specs = [f"{kind}/{entry.name}" for kind in ("machinery","vehicles","components") for entry in sorted(next(catalog.glob("*"+kind)).iterdir()) if entry.is_dir()]
filters = sys.argv[1:]
if filters:
    specs = [s for s in specs if s.split("/")[-1] in filters]

def check(spec):
    with (output/(spec.split("/")[-1]+".log")).open("w") as sink:
        result = subprocess.run(["repo","tools","exec","bun","--",str(ticket/"r3-validate.ts"),spec,"--catalog-only","--volumes"],stdout=sink,stderr=sink)
    print(spec,result.returncode,flush=True)

with ThreadPoolExecutor(max_workers=2) as workers:
    list(workers.map(check,specs))

