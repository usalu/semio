import os
from pathlib import Path
import subprocess
ticket=Path(__file__).resolve().parent
workspace=ticket.parents[6]
output=ticket/"🗑️generated"/"child-cleanup-bazel.log"
output.parent.mkdir(parents=True,exist_ok=True)
with output.open("w") as log:
 result=subprocess.run(["repo","--tool=bazel","run","@semio-tech/framework-process:test-native-child-cleanup","--output=stream"],cwd=workspace,stdout=log,stderr=subprocess.STDOUT)
print("[DEBUG] native-child-cleanup exit",result.returncode,output)
raise SystemExit(result.returncode)
