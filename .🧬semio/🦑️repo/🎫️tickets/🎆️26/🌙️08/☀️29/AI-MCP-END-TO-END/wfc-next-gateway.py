"""🧭 Verify the first relinked native gateway through the installed SDK."""
from pathlib import Path
import argparse
import os
import re
import subprocess
import time

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--label", required=True)
parser.add_argument("--models-root", type=Path, required=True)
options = parser.parse_args()
if not re.fullmatch("[a-z0-9-]+", options.label):
    parser.error("label must contain lowercase letters, digits or hyphens")
if not options.models_root.is_dir():
    parser.error("the verified models root must exist")
binary = generated / "native-mcp" / "debug" / ("semio-os-mcp.exe" if os.name=="nt" else "semio-os-mcp")
baseline = binary.stat().st_mtime_ns
deadline = time.monotonic() + 2400
while binary.stat().st_mtime_ns == baseline:
    if time.monotonic() >= deadline:
        raise TimeoutError("the current native build has not linked a gateway")
    time.sleep(1)
args = ["repo","tools","exec","--light","bun","--", str(ticket/"wfc-seeded-sdk.ts"),"solve","--pinned-pair","--schema-probe","--gateway="+str(binary),"--models-root="+str(options.models_root.resolve())]
environment = dict(os.environ, SEMIO_DEBUG_GUEST_RELAY_REFUSAL="1",SEMIO_DEBUG_COLD_ACTION="1")
runs = [(options.label+"-short", ["--solve-timeout-ms=90000"]), (options.label+"-long", ["--large-request","--solve-timeout-ms=340000"])]
processes=[]
for label, flags in runs:
    with (generated/(label+"-solver.log")).open("wb") as log:
        processes.append(subprocess.Popen([*args,*flags,"--label="+label],env=environment,stdout=log,stderr=subprocess.STDOUT))
statuses = [process.wait() for process in processes]
print("observed gateway client exits", statuses, flush=True)
raise SystemExit(int(any(statuses)))
