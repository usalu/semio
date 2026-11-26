"""🪟 Run the repository's React tests against the local browser and isolated cache."""
import json
import os
import subprocess
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
output = ticket / "🗑️generated/web2/diagram"
output.mkdir(parents=True, exist_ok=True)
tool = root / "node_modules/vitest/vitest.mjs"
policy = dict(version=1, cwd=str(root), toolPath=str(tool), runtime="node", coverageRuntime="node",
              cacheRoot=str(output / "cache"), coverageDirectory=str(output / "coverage"), budgetMs=300000)
args = ["node", str(tool), "run", "--config", str(ticket / "react-native-browser.config.mjs"), "--maxWorkers=4", *sys.argv[1:]]
with (output / "diagram.log").open("w") as log:
    result = subprocess.run(args, cwd=root, env={**os.environ, "SEMIO_VITEST_POLICY": json.dumps(policy)}, stdout=log, stderr=subprocess.STDOUT)
print((output / "diagram.log").read_text()[-5000:])
sys.exit(result.returncode)
