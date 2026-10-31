import argparse
import json
import os
from pathlib import Path
import subprocess

parser=argparse.ArgumentParser(description="Run the current layout owner targets through repo")
parser.add_argument("operation",choices=("activate","serve","probe"))
parser.add_argument("--tool",choices=("nx","bazel"),help="Explicit repo backend; omit to use repository policy")
args=parser.parse_args()
ticket=Path(__file__).resolve().parent
renderer=json.loads((ticket/"browser-journey.json").read_text())["renderer"]
workspace=ticket.parents[6]
catalog="✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"
environment=dict(os.environ,SEMIO_PLUGIN_CATALOG=catalog)
commands={
 "activate":["repo","run",renderer["activationTarget"],"--output=stream"],
 "serve":["repo","run",renderer["serveTarget"],"--output=stream"],
 "probe":["repo","tools","exec","bun","--",str(ticket/"browser-journey.mjs")],
}
command=commands[args.operation]
if args.tool:
 command.insert(1,"--tool="+args.tool)
raise SystemExit(subprocess.run(command,cwd=workspace,env=environment).returncode)
