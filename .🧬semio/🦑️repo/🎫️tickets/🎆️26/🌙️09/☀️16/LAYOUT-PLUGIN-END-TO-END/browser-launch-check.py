import json
import runpy
import subprocess
import sys
from pathlib import Path
from unittest.mock import patch
ticket=Path(__file__).resolve().parent
contract=json.loads((ticket/"browser-journey.json").read_text())["renderer"]
workspace=ticket.parents[6]
cases=[("activate",None,["repo","run",contract["activationTarget"],"--output=stream"]),("serve",None,["repo","run",contract["serveTarget"],"--output=stream"]),("probe",None,["repo","tools","exec","bun","--",str(ticket/"browser-journey.mjs")]),("activate","bazel",["repo","--tool=bazel","run",contract["activationTarget"],"--output=stream"])]
for operation,tool,expected in cases:
 argv=[str(ticket/"launch-layout.py"),operation]+(["--tool",tool] if tool else [])
 with patch.object(sys,"argv",argv),patch.object(subprocess,"run",return_value=subprocess.CompletedProcess(expected,0)) as execute:
  try:runpy.run_path(str(ticket/"launch-layout.py"),run_name="__main__")
  except SystemExit as result:assert result.code==0
  assert execute.call_count==1
  assert execute.call_args.args[0]==expected,(operation,execute.call_args.args[0],expected)
  assert execute.call_args.kwargs["cwd"]==workspace
  assert execute.call_args.kwargs["env"]["SEMIO_PLUGIN_CATALOG"]=="✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"
print("[DEBUG] Launch contract commands passed: 4; no actual target launched")
