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
pw = Path(subprocess.check_output(
    ["repo", "tools", "exec", "--light", "node", "--", "-p", "require.resolve('@playwright/test/package.json')"],
    cwd=demo, encoding="utf-8",
).strip()).parent
roster = json.loads(gate["CASES"].read_bytes())
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
    if (title === 'bearbeiten undo-redo' && process.env.FAULT === 'edit') throw Error('fixture')
    if (title === 'aggregator fill' && process.env.FAULT === 'behaviour') throw Error('fixture')
  })
}
""")
    output = scratch / "result.json"
    discovery_file = scratch / "list.json"
    command = ["repo", "tools", "exec", "--light", "node", "--", str(pw / "cli.js"),
               "test", "--reporter=json", "--config=playwright.config.cjs", "--output", str(scratch / "pw")]
    env = dict(os.environ, PROOF_PW=str(pw), PROOF_TITLES=json.dumps(roster["titles"]), PLAYWRIGHT_JSON_OUTPUT_FILE=str(discovery_file))
    subprocess.run(command + ["--list"], cwd=scratch, env=env, check=True)
    discovery = json.loads(discovery_file.read_bytes())
    for fault in ("skip", "expected-failure", "pass"):
        env = dict(os.environ, PROOF_PW=str(pw), PROOF_TITLES=json.dumps(roster["titles"]), FAULT=fault, PLAYWRIGHT_JSON_OUTPUT_FILE=str(output))
        subprocess.run(command + ["--grep", " boot$"], cwd=scratch, env=env, check=True)
        report = json.loads(output.read_bytes())
        check(report, origin, fault == "pass")
    for mode in ("missing", "duplicate", "flaky"):
        altered = deepcopy(report)
        specs = altered["suites"][0]["specs"]
        if mode == "missing":
            specs.pop()
        elif mode == "duplicate":
            specs.append(specs[0])
        else:
            specs[0]["tests"][0]["results"].insert(0, {"status": "failed"})
        check(altered, origin, False)
    check(report, origin + '\n127.0.0.1 - - [now] "GET /%5f_semio/agent-bridge?q=1 HTTP/1.1" 404 -', False)
    check(report, origin + '\n127.0.0.1 - - [now] "GET /vt/3/2/1.pbf HTTP/1.1" 304 -', True)
    for path, status in (("/assets/guest.core.wasm.gz.bin", 404), ("/vt/3/2/1.pbf", 503)):
        check(report, origin + f'\n127.0.0.1 - - [now] "GET {path} HTTP/1.1" {status} -', False)
    behaviour_titles = json.loads(gate["BEHAVIOUR_CASES"].read_bytes())["titles"]
    behaviour_env = dict(env, PROOF_TITLES=json.dumps(behaviour_titles))
    subprocess.run(command + ["--list"], cwd=scratch,
                   env=dict(behaviour_env, PLAYWRIGHT_JSON_OUTPUT_FILE=str(discovery_file)), check=True)
    behaviour_discovery = json.loads(discovery_file.read_bytes())
    rosters = gate["release_rosters"](discovery, behaviour_discovery)
    assert tuple(map(len, rosters)) == (52, 44)
    subprocess.run(command, cwd=scratch, env=behaviour_env, check=True)
    behaviours = json.loads(output.read_bytes())
    def full_gate(baseline_report, behaviour_report=behaviours):
        gate["verify_release"](rosters, baseline_report, behaviour_report, origin)

    baseline = None
    for subset in ([], ["--grep", " boot$"]):
        subprocess.run(command + subset, cwd=scratch, env=env, check=True)
        results = json.loads(output.read_bytes())
        try:
            full_gate(results)
        except ValueError:
            assert subset
        else:
            assert not subset
            baseline = results
    subprocess.run(command, cwd=scratch, env=dict(env, FAULT="edit"), check=False)
    try:
        full_gate(json.loads(output.read_bytes()))
    except ValueError:
        pass
    else:
        raise AssertionError("full release gate accepted failed editing")
    subprocess.run(command, cwd=scratch, env=dict(behaviour_env, FAULT="behaviour"), check=False)
    failed_behaviour = json.loads(output.read_bytes())
    behaviours["suites"][0]["specs"].pop()
    for invalid in (behaviours, failed_behaviour):
        try:
            full_gate(baseline, invalid)
        except ValueError:
            pass
        else:
            raise AssertionError("release gate accepted missing or failing behavior")
    behaviour_discovery["suites"][0]["specs"].pop()
    try:
        gate["release_rosters"](discovery, behaviour_discovery)
    except ValueError:
        pass
    else:
        raise AssertionError("release gate accepted partial behavior discovery")
