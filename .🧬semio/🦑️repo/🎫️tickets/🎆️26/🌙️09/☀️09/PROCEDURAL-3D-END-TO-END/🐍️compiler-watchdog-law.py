"""🕸️ Runs focused component driver laws under a declared test process."""
from pathlib import Path
import subprocess
import sys

workspace = Path.cwd()
ticket = Path(__file__).resolve().parent
label = sys.argv[1]
args = ["repo", "tools", "exec", "--light", "bun", "--", "test", "./🟦️.ts"]
cwd = workspace / "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/⏳️job-wait/🧪️tests"
with (ticket / "🗑️generated" / ("compiler-planning-watchdog-" + label + ".log")).open("w") as log:
    result = subprocess.run(args, cwd=cwd, stdout=log, stderr=subprocess.STDOUT)
raise SystemExit(result.returncode)
