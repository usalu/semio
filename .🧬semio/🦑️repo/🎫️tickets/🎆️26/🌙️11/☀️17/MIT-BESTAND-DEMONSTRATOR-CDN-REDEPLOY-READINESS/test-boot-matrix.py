"""🎭 Check the release gate against real Playwright outcomes and origin requests."""
from copy import deepcopy
import json
import os
from pathlib import Path
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket = Path(__file__).resolve().parent
gate = runpy.run_path(str(ticket / "verify-boot-matrix.py"))
demo = gate["ROOT"] / "♻️mit-bestand/🧺️demonstrator"
pw = Path(subprocess.check_output(["node", "-p", "require.resolve('@playwright/test/package.json')"], cwd=demo, text=True).strip()).parent
roster = json.loads(gate["CASES"].read_bytes())
titles = [title for title in roster["titles"] if title.endswith(" boot")]
origin = '127.0.0.1 - - [now] "GET / HTTP/1.1" 200 -'


def check(report, log, accepted):
    try:
        gate["verify"](report, log)
    except ValueError:
        assert not accepted
    else:
        assert accepted


with TemporaryDirectory(dir=ticket / "🗑️generated", prefix="matrix-") as directory:
    scratch = Path(directory)
    (scratch / "playwright.config.cjs").write_text("module.exports={workers:1,projects:['en','de'].map(name=>({name}))}")
    (scratch / "boot.spec.cjs").write_text("""
const {test} = require(process.env.PROOF_PW)
for (const title of JSON.parse(process.env.PROOF_TITLES)) {
  test(title, async ({}, info) => {
    if (title === 'statik boot' && info.project.name === 'de') {
      test.skip(process.env.FAULT === 'skip')
      test.fail(process.env.FAULT === 'expected-failure')
      if (process.env.FAULT === 'expected-failure') throw Error('fixture')
    }
  })
}
""")
    output = scratch / "result.json"
    for fault in ("skip", "expected-failure", "pass"):
        env = dict(os.environ, PROOF_PW=str(pw), PROOF_TITLES=json.dumps(titles), FAULT=fault, PLAYWRIGHT_JSON_OUTPUT_FILE=str(output))
        subprocess.run(["node", str(pw / "cli.js"), "test", "--reporter=json", "--config=playwright.config.cjs"], cwd=scratch, env=env, check=True)
        report = json.loads(output.read_bytes())
        check(report, origin, fault == "pass")
    for mode in ("missing", "duplicate"):
        altered = deepcopy(report)
        specs = altered["suites"][0]["specs"]
        if mode == "missing":
            specs.pop()
        else:
            specs.append(specs[0])
        check(altered, origin, False)
    check(report, origin + '\n127.0.0.1 - - [now] "GET /%5f_semio/agent-bridge?q=1 HTTP/1.1" 404 -', False)
