#!/usr/bin/env python3
"""🧪 Static and functional selftest of the e2e orchestrator: language-agnostic vectors, an independent awk implementation of the Bazel summary parser, Ajv (third-party) on the plan schema, and a fake-plan run.

Usage: python3 selftest.py            (light: no bun repo, no Bazel, no cargo; the fake plan runs `echo` and `sleep`)
"""
import importlib.util
import json
import os
import shutil
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parent
spec = importlib.util.spec_from_file_location("orch", str(HERE / "orch.py"))
orch = importlib.util.module_from_spec(spec)
sys.modules["orch"] = orch
spec.loader.exec_module(orch)

failures = []
checks = 0


def expect(condition, message):
    """✔️ Counts one assertion."""
    global checks
    checks += 1
    if not condition:
        failures.append(message)
        print("FAIL", message)


def vectors():
    """📚 The shared vectors."""
    return json.loads((HERE / "vectors.json").read_text(encoding="utf-8"))


def test_vectors():
    """📊 Pure functions against the vectors."""
    data = vectors()
    for row in data["summary"]:
        got = orch.parse_summary_line(row["line"])
        if row.get("none"):
            expect(got is None, "summary none: %s" % row["line"])
            continue
        expect(got is not None and all(got[key] == row[key] for key in ("processes", "internal", "cacheHits", "spawns")), "summary %s -> %s" % (row["line"], got))
    for row in data["verdict"]:
        if row["phase"] == "warm":
            got = orch.warm_verdict(row["mode"], row["allow"], row["rc"], row["spawns"], row["summaries"], row["seconds"], row["maxSeconds"], row["slack"])[0]
        else:
            got = orch.plain_verdict(row["rc"], {"tests": row["tests"]})[0]
        expect(got == row["status"], "verdict %s: %s != %s" % (row["name"], got, row["status"]))
    for row in data["slots"]:
        expect(orch.parse_log(row["line"])["slots"] == row["slots"], "slot %s" % row["line"])
    for row in data["waits"]:
        expect(bool(orch.parse_log(row["line"])["waits"]) == row["wait"], "wait %s" % row["line"])
    for row in data["slotsFor"]:
        expect(orch.lane_slots(row["memoryMb"], row["cores"]) == row["slots"], "slots for %s" % row)
    expect([list(r) for r in orch.counter_delta(data["delta"]["before"], data["delta"]["after"])] == data["delta"]["rows"], "counter delta")
    for row in data["queue"]:
        clock = orch.QueueClock(row["grace"])
        moments = sorted(set([event[0] for event in row["events"]] + list(range(0, row["end"] + 1))))
        feed = {}
        for at, text in row["events"]:
            feed.setdefault(at, []).append(text)
        for at in moments:
            clock.feed("\n".join(feed.get(at, [])), at)
            credit = clock.update(at)
        expect(abs(credit - row["credit"]) <= 0.5, "queue %s: credit %s != %s" % (row["name"], credit, row["credit"]))
    log = "INFO: 2 processes: 1 disk cache hit, 1 internal.\nINFO: 1 process: 1 internal.\n  Compiling foo v1.0.0\n  11 passed (3m)\n  1 failed\n"
    parsed = orch.parse_log(log)
    expect((parsed["summaries"], parsed["processes"], parsed["spawns"], parsed["cacheHits"], parsed["internal"]) == (2, 3, 0, 1, 2), "aggregate %s" % parsed)
    expect(parsed["tests"] == {"passed": 11, "failed": 1} and len(parsed["evidence"]) == 1, "tests/evidence %s" % parsed)


def test_awk_oracle():
    """🧮 Second implementation of the summary parser in awk must agree with the Python one on every vector line."""
    awk = shutil.which("awk")
    if not awk:
        print("awk missing: oracle skipped")
        return
    program = r'''{
      line=$0; gsub(/\033\[[0-9;]*m/, "", line);
      if (match(line, /[0-9]+ process(es)?: .*/)) {
        s=substr(line, RSTART, RLENGTH); n=split(s, a, ": "); total=a[1]+0; body=a[2]; sub(/\.[ \t]*$/, "", body);
        k=split(body, parts, ", "); internal=0; hits=0; spawns=0;
        for (i=1;i<=k;i++) { cnt=parts[i]+0; name=parts[i]; sub(/^[0-9]+ /, "", name);
          if (name=="internal") internal+=cnt; else if (name=="disk cache hit" || name=="remote cache hit") hits+=cnt; else spawns+=cnt }
        print total, internal, hits, spawns } else print "none" }'''
    for row in vectors()["summary"]:
        done = subprocess.run([awk, program], input=row["line"].encode("utf-8") + b"\n", stdout=subprocess.PIPE, timeout=30)
        text = done.stdout.decode().strip()
        want = "none" if row.get("none") else "%d %d %d %d" % (row["processes"], row["internal"], row["cacheHits"], row["spawns"])
        expect(text == want, "awk oracle %s: %r != %r" % (row["line"], text, want))


def ajv_validate(plan_path, schema_path):
    """🛡️ Validates a plan with Ajv (third-party) via bun, run from the repository root."""
    bun = shutil.which("bun")
    root = subprocess.run(["git", "-C", str(TICKET), "rev-parse", "--show-toplevel"], stdout=subprocess.PIPE).stdout.decode().strip()
    if not bun or not root or not (Path(root) / "node_modules" / "ajv" / "dist" / "2020.js").exists():
        print("bun/ajv unavailable: schema oracle skipped")
        return None
    script = ("const Ajv=require('ajv/dist/2020').default;const fs=require('fs');const a=new Ajv({strict:true,allErrors:true});"
              "const v=a.compile(JSON.parse(fs.readFileSync(process.argv[1],'utf8')));const ok=v(JSON.parse(fs.readFileSync(process.argv[2],'utf8')));"
              "console.log(ok?'valid':JSON.stringify(v.errors));process.exit(ok?0:1)")
    done = subprocess.run([bun, "-e", script, str(schema_path), str(plan_path)], cwd=root, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=120)
    return done.returncode == 0, done.stdout.decode()


def test_plan():
    """📋 Plan against schema (Ajv) and against the invariants the schema cannot say."""
    plan = json.loads((HERE / "plan.json").read_text(encoding="utf-8"))
    result = ajv_validate(HERE / "plan.json", HERE / "plan.schema.json")
    if result is not None:
        expect(result[0], "plan.json invalid: %s" % result[1][:600])
    lanes, ports = set(), set()
    for name, product in plan["products"].items():
        expect(product["cargoLane"] not in lanes, "lane unique %s" % name)
        lanes.add(product["cargoLane"])
        for port in product["ports"]:
            expect(port not in ports, "port unique %s" % port)
            ports.add(port)
        groups = {s["group"] for phase in ("cold", "e2e", "probe") for s in product["steps"][phase]}
        for phase in ("cold", "e2e", "probe"):
            for step in product["steps"][phase]:
                for need in step.get("needs", []):
                    expect(need in groups, "%s %s needs unknown group %s" % (name, step["id"], need))
                if step["kind"] == "ticket-script":
                    expect((TICKET / step["script"]).is_file(), "script exists %s" % step["script"])
                if phase == "cold":
                    expect("warm" in step, "%s cold step %s has a warm assertion" % (name, step["id"]))
        for step in product["steps"]["cold"]:
            expect(step["kind"] == "repo" and "run" in step["args"], "cold steps are bun repo runs")
    expect(set(plan["products"]) == {"play", "demo", "course"}, "three products")


def test_cli():
    """⌨️ Shell syntax, dry run and the GO refusal (no heavy command is started)."""
    expect(subprocess.run(["sh", "-n", str(HERE / "run.sh")]).returncode == 0, "run.sh syntax")
    done = subprocess.run(["sh", str(HERE / "run.sh"), "all", "--dry-run", "--phase", "all"], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=60)
    text = done.stdout.decode()
    expect(done.returncode == 0 and "bun repo --tool=bazel run @semio-tech/semio-tech-play:activate-prototype" in text and "assert spawns<=0" in text, "dry run output: %s" % text[:300])
    declared = subprocess.run(["sh", str(HERE / "run.sh"), "course", "--dry-run", "--phase", "e2e"], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=60).stdout.decode()
    narrowed = subprocess.run(["sh", str(HERE / "run.sh"), "course", "--dry-run", "--phase", "e2e", "--course-topology", "dev"], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=60).stdout.decode()
    expect("--topology" not in declared and "--release" in declared, "without --course-topology the course gate runs the topologies its target declares (release mode): %s" % declared[-300:])
    expect("--topology=dev" in narrowed and "--release" not in narrowed, "--course-topology dev narrows the gate: %s" % narrowed[-300:])
    refused = subprocess.run(["sh", str(HERE / "run.sh"), "play", "--phase", "cold"], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=60, env={k: v for k, v in os.environ.items() if k != "E2E_GO"})
    expect(refused.returncode == 2 and b"need the coordinator" in refused.stdout, "cold without --go is refused: rc %s" % refused.returncode)


def fake_plan(path, steps):
    """🧸 A one-product plan whose steps run echo/sleep."""
    real = json.loads((HERE / "plan.json").read_text(encoding="utf-8"))
    plan = {k: real[k] for k in ("version", "tool", "thresholds", "playwright", "tectonic", "phases")}
    plan["products"] = {"fake": {"label": "Fake", "package": "@fake/fake", "order": 1, "cargoLane": "efake", "ports": [], "needs": [], "steps": steps}}
    Path(path).write_text(json.dumps(plan), encoding="utf-8")


def exec_step(step_id, group, text, warm=None, needs=None, timeout=60, sleep=None):
    """🧱 An exec step printing text."""
    argv = ["sh", "-c", "printf '%s\\n' \"$0\"" if sleep is None else "sleep 58.7321", text]
    step = {"id": step_id, "group": group, "kind": "exec", "argv": argv, "timeoutSeconds": timeout}
    if warm:
        step["warm"] = warm
    if needs:
        step["needs"] = needs
    return step


def test_functional():
    """🧸 Runs the engine on a fake plan: cold, warm verdicts, skipping after a failure, report, state, GO refusal and cancellation."""
    work = Path(tempfile.mkdtemp(prefix="e2e-orch-selftest-"))
    try:
        steps = {
            "cold": [
                exec_step("a", "g1", "INFO: 2 processes: 1 disk cache hit, 1 internal.", {"mode": "zero", "maxSeconds": 60}),
                exec_step("b", "g2", "INFO: 3 processes: 2 internal, 1 darwin-sandbox.", {"mode": "allow", "allow": 1, "maxSeconds": 60}),
                exec_step("c", "g3", "INFO: 1 process: 1 internal.", {"mode": "zero"}),
            ],
            "warm": [
                exec_step("a", "g1", "INFO: 1 process: 1 internal.", {"mode": "zero", "maxSeconds": 60}),
                exec_step("b", "g2", "INFO: 4 processes: 2 internal, 2 local.", {"mode": "allow", "allow": 1, "maxSeconds": 60}),
                exec_step("c", "g3", "no summary line here", {"mode": "zero"}),
            ],
            "e2e": [exec_step("t", "e2e", "  7 passed (1.0s)", needs=["g1"])],
            "probe": [],
        }
        plan_path = work / "plan.json"
        fake_plan(plan_path, steps)
        result = ajv_validate(plan_path, HERE / "plan.schema.json")
        if result is not None:
            expect(result[0], "fake plan invalid: %s" % result[1][:300])
        common = [sys.executable, str(HERE / "orch.py"), "fake", "--plan", str(plan_path), "--ticket", str(work), "--root", str(work), "--out", str(work / "out"), "--report-dir", str(work), "--skip-preflight", "--no-snapshots", "--go", "--fast", "--no-stagger", "--run-id", "t1"]
        done = subprocess.run(common + ["--phase", "cold,warm,e2e"], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=120)
        expect(done.returncode == 1, "fake run exit 1 (one warm FAIL), got %s: %s" % (done.returncode, done.stdout.decode()[-800:]))
        state = json.loads((work / "out" / "t1" / "state.json").read_text(encoding="utf-8"))
        status = {(r["phase"], r["id"]): r["status"] for r in state["steps"]}
        expect(status == {("cold", "a"): "PASS", ("cold", "b"): "PASS", ("cold", "c"): "PASS", ("warm", "a"): "PASS", ("warm", "b"): "FAIL", ("warm", "c"): "WARN", ("e2e", "t"): "PASS"}, "statuses %s" % status)
        warm_b = next(r for r in state["steps"] if r["phase"] == "warm" and r["id"] == "b")
        expect(warm_b["bazel"]["spawns"] == 2 and warm_b["warm"]["allow"] == 1, "warm b record %s" % warm_b["bazel"])
        report = (work / "report-e2e-orch-t1.md").read_text(encoding="utf-8")
        expect("## Warm assertions" in report and "2 spawns" in "".join(r["note"] for r in state["steps"]) and "| fake | b | 2 | 1 |" in report, "report content")
        again = subprocess.run(common + ["--phase", "cold"], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=60)
        expect(b"already PASS in this run" in again.stdout, "resume skips passed steps")
        broken = dict(steps)
        broken["cold"] = [exec_step("x", "gx", "boom", {"mode": "zero"})]
        broken["cold"][0]["argv"] = ["sh", "-c", "echo boom; exit 4"]
        broken["cold"].append(exec_step("y", "gx", "INFO: 1 process: 1 internal.", {"mode": "zero"}))
        broken["cold"].append(exec_step("z", "gz", "INFO: 1 process: 1 internal.", {"mode": "zero"}))
        broken["e2e"] = [exec_step("t", "e2e", "never", needs=["gx"])]
        broken["warm"] = "cold"
        fake_plan(work / "plan2.json", broken)
        done = subprocess.run(common[:4] + [str(work / "plan2.json")] + common[5:-1] + ["t2", "--phase", "cold,warm,e2e"], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=120)
        state2 = json.loads((work / "out" / "t2" / "state.json").read_text(encoding="utf-8"))
        status2 = {(r["phase"], r["id"]): r["status"] for r in state2["steps"]}
        expect(done.returncode == 1 and status2[("cold", "x")] == "FAIL" and status2[("cold", "y")] == "SKIP-FAILED" and status2[("cold", "z")] == "PASS" and status2[("e2e", "t")] == "SKIP-FAILED" and status2[("warm", "z")] == "PASS" and status2[("warm", "x")] == "SKIP-FAILED", "group skipping %s" % status2)
        report2 = (work / "report-e2e-orch-t2.md").read_text(encoding="utf-8")
        expect("boom" in report2 and "## Failures and warnings" in report2, "failure tail in report")
        slow = dict(steps)
        slow["cold"] = [exec_step("s", "gs", "x", {"mode": "zero"}, timeout=120, sleep=60)]
        slow["warm"] = "cold"
        slow["e2e"] = []
        fake_plan(work / "plan3.json", slow)
        cmd = common[:4] + [str(work / "plan3.json")] + common[5:-1] + ["t3", "--phase", "cold"]
        process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        time.sleep(4)
        started = time.time()
        os.kill(process.pid, signal.SIGTERM)
        out, _ = process.communicate(timeout=60)
        state3 = json.loads((work / "out" / "t3" / "state.json").read_text(encoding="utf-8"))
        expect(process.returncode == 130 and state3["steps"][0]["status"] == "CANCELLED" and state3["cancelled"], "cancellation rc %s %s" % (process.returncode, [r["status"] for r in state3["steps"]]))
        expect(time.time() - started < 50, "cancellation stops the child promptly")
        expect(not (work / "out" / "ORCH.lock").exists(), "lock released")
        queued = dict(steps)
        queued["cold"] = [{"id": "q", "group": "gq", "kind": "exec", "argv": ["sh", "-c", "for i in 1 2 3 4 5 6 7 8; do echo '[bazel] all 3 Bazel server slots are in use; waiting for one'; sleep 1; done; echo ready"], "timeoutSeconds": 3, "warm": {"mode": "skip"}},
                          {"id": "r", "group": "gr", "kind": "exec", "argv": ["sh", "-c", "for i in 1 2 3 4 5 6 7 8; do echo working; sleep 1; done"], "timeoutSeconds": 3, "warm": {"mode": "skip"}}]
        queued["warm"] = "cold"
        queued["e2e"] = []
        fake_plan(work / "plan4.json", queued)
        cmd = common[:4] + [str(work / "plan4.json")] + common[5:-1] + ["t4", "--phase", "cold"]
        subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=120)
        state4 = json.loads((work / "out" / "t4" / "state.json").read_text(encoding="utf-8"))
        status4 = {r["id"]: (r["status"], r["note"]) for r in state4["steps"]}
        expect(status4["q"][0] == "PASS", "queue waits do not count against the step timeout: %s" % (status4,))
        expect(status4["r"][0] == "FAIL" and "timeout after 3s" in status4["r"][1], "work beyond the timeout still fails: %s" % (status4,))
        fixed = dict(broken)
        fixed["cold"] = [exec_step("x", "gx", "INFO: 1 process: 1 internal.", {"mode": "zero"}), exec_step("y", "gx", "INFO: 1 process: 1 internal.", {"mode": "zero"}), exec_step("z", "gz", "INFO: 1 process: 1 internal.", {"mode": "zero"})]
        fake_plan(work / "plan2.json", fixed)
        done = subprocess.run(common[:4] + [str(work / "plan2.json")] + common[5:-1] + ["t2", "--phase", "cold,warm,e2e"], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=120)
        state2 = json.loads((work / "out" / "t2" / "state.json").read_text(encoding="utf-8"))
        status2 = {(r["phase"], r["id"]): r["status"] for r in state2["steps"]}
        expect(done.returncode == 0 and status2[("cold", "x")] == "PASS" and status2[("cold", "y")] == "PASS" and status2[("cold", "z")] == "PASS" and status2[("e2e", "t")] == "PASS", "a continued run id retries failed steps: rc %s %s" % (done.returncode, status2))
        settle = dict(steps)
        script = "if [ -f \"$0\" ]; then echo 'INFO: 2 processes: 2 internal.'; else touch \"$0\"; echo 'INFO: 3 processes: 2 internal, 1 local.'; fi"
        settle["cold"] = [exec_step("w", "gw", "INFO: 1 process: 1 internal.", {"mode": "zero"}), exec_step("v", "gv", "INFO: 1 process: 1 internal.", {"mode": "zero"})]
        settle["warm"] = [{"id": "w", "group": "gw", "kind": "exec", "argv": ["sh", "-c", script, str(work / "settle.marker")], "timeoutSeconds": 30, "warm": {"mode": "zero"}},
                          {"id": "v", "group": "gv", "kind": "exec", "argv": ["sh", "-c", "echo 'INFO: 2 processes: 1 internal, 1 local.'"], "timeoutSeconds": 30, "warm": {"mode": "zero"}}]
        settle["e2e"] = []
        fake_plan(work / "plan6.json", settle)
        cmd = common[:4] + [str(work / "plan6.json")] + common[5:-1] + ["t6", "--phase", "cold,warm"]
        subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=120)
        state6 = json.loads((work / "out" / "t6" / "state.json").read_text(encoding="utf-8"))
        status6 = {(r["phase"], r["id"]): (r["status"], r["note"]) for r in state6["steps"]}
        expect(status6[("warm", "w")][0] == "PASS" and "settle run 1" in status6[("warm", "w")][1], "a warm step that rebuilt once passes on its settle run: %s" % (status6,))
        expect(status6[("warm", "v")][0] == "FAIL" and "settle run 1" in status6[("warm", "v")][1], "a warm step that always spawns still fails: %s" % (status6,))
        stale = dict(steps)
        stale["cold"] = [{"id": "s", "group": "gs", "kind": "exec", "argv": ["sh", "-c", "if [ -f \"$0\" ]; then echo fine; else touch \"$0\"; echo 'This command runs an outdated implementation: the declared format stamp moved'; exit 75; fi", str(work / "stale.marker")], "timeoutSeconds": 30, "warm": {"mode": "skip"}},
                         {"id": "u", "group": "gu", "kind": "exec", "argv": ["sh", "-c", "echo 'boom'; exit 75"], "timeoutSeconds": 30, "warm": {"mode": "skip"}}]
        stale["warm"] = "cold"
        stale["e2e"] = []
        fake_plan(work / "plan5.json", stale)
        cmd = common[:4] + [str(work / "plan5.json")] + common[5:-1] + ["t5", "--phase", "cold"]
        subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=120)
        state5 = json.loads((work / "out" / "t5" / "state.json").read_text(encoding="utf-8"))
        status5 = {r["id"]: (r["status"], r["note"]) for r in state5["steps"]}
        expect(status5["s"][0] == "PASS" and "retried 1" in status5["s"][1], "outdated-implementation exit is retried: %s" % (status5,))
        expect(status5["u"] == ("FAIL", "exit code 75"), "a plain exit 75 is not retried: %s" % (status5,))
        listing = subprocess.run(["pgrep", "-f", "sleep 58.7321"], stdout=subprocess.PIPE).stdout.decode().split()
        expect(not [p for p in listing if p != str(os.getpid())], "no orphan sleep left: %s" % listing)
    finally:
        shutil.rmtree(str(work), ignore_errors=True)


def test_paths():
    """📏 Every file of this tool stays far below the 256 character path limit."""
    longest = max(len(str(p)) for p in HERE.rglob("*"))
    expect(longest < 256, "path length %d" % longest)


def main():
    """🏁 Runs all tests."""
    for test in (test_vectors, test_awk_oracle, test_plan, test_cli, test_functional, test_paths):
        started = time.time()
        test()
        print("%-18s %5.1fs" % (test.__name__, time.time() - started))
    print("%d assertions, %d failure(s)" % (checks, len(failures)))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
