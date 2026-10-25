#!/usr/bin/env python3
"""Independent Python model of the test-runner module: the seven runner report dialects folded into one outcome, plus the
scope selector arithmetic.  Writes (or with --check verifies) 🧪️tests/🧫️fixtures/🔬️unit-vectors.json of the module."""
import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
MODULE = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🏃️test-runner"
FIXTURES = MODULE / "🧪️tests/🧫️fixtures"
ORACLE_SCRIPT = str(Path(__file__).resolve().relative_to(ROOT))
TARGET = FIXTURES / "🔬️unit-vectors.json"


def lines_of(text):
    return [line[:-1] if line.endswith("\r") else line for line in text.split("\n")]


def finish(runner, tests, output):
    totals = {"passed": sum(t["status"] == "passed" for t in tests), "failed": sum(t["status"] == "failed" for t in tests), "skipped": sum(t["status"] == "skipped" for t in tests), "total": len(tests)}
    status = "failed" if totals["failed"] > 0 or output["status"] != 0 else "passed"
    return {"runner": runner, "status": status, "tests": tests, "totals": totals, "exit_status": output["status"]}


def case(name, suite, status, duration=None, message=""):
    row = {"name": name, "status": status}
    if suite:
        row["suite"] = suite
    if duration is not None:
        row["duration_ms"] = duration
    if message:
        row["message"] = message
    return row


def parse_go(runner, output):
    messages, tests = {}, []
    for line in lines_of(output["stdout"]):
        try:
            event = json.loads(line.strip())
        except ValueError:
            continue
        if not isinstance(event, dict) or not isinstance(event.get("Test"), str):
            continue
        action, package, name = event.get("Action", ""), event.get("Package", ""), event["Test"]
        key = package + "::" + name
        if action == "output":
            messages[key] = messages.get(key, "") + event.get("Output", "")
            continue
        status = {"pass": "passed", "fail": "failed", "skip": "skipped"}.get(action)
        if status is None:
            continue
        elapsed = event.get("Elapsed")
        duration = float(elapsed) * 1000.0 if isinstance(elapsed, (int, float)) else None
        message = messages.get(key, "").rstrip() if status == "failed" else ""
        tests.append(case(name, package, status, duration, message))
    return finish(runner, tests, output)


def json_body(text):
    start = text.find("{")
    start = 0 if start < 0 else start
    end = text.rfind("}")
    end = len(text) if end < 0 else end + 1
    return text[start:end] if start < end else text


def parse_vitest(runner, output):
    tests = []
    try:
        report = json.loads(json_body(output["stdout"]))
    except ValueError:
        report = None
    if isinstance(report, dict):
        for file in report.get("testResults") or []:
            suite = file.get("name", "")
            for assertion in file.get("assertionResults") or []:
                name = assertion.get("fullName") if isinstance(assertion.get("fullName"), str) else assertion.get("title", "")
                status = {"passed": "passed", "failed": "failed"}.get(assertion.get("status", ""), "skipped")
                message = "\n".join(m for m in assertion.get("failureMessages") or [] if isinstance(m, str))
                duration = assertion.get("duration")
                tests.append(case(name, suite, status, float(duration) if isinstance(duration, (int, float)) else None, message))
    return finish(runner, tests, output)


def parse_pytest(runner, output):
    tests = []
    for line in lines_of(output["stdout"]):
        trimmed = line.rstrip()
        if " " not in trimmed:
            continue
        node, tail = trimmed.split(" ", 1)
        if "::" not in node:
            continue
        words = tail.split()
        verdict = words[0] if words else ""
        status = {"PASSED": "passed", "XPASS": "passed", "FAILED": "failed", "ERROR": "failed", "SKIPPED": "skipped", "XFAIL": "skipped"}.get(verdict)
        if status is None:
            continue
        suite, name = node.split("::", 1)
        tests.append(case(name, suite, status))
    return finish(runner, tests, output)


def cargo_body(stdout, name):
    header = f"---- {name} stdout ----"
    index = stdout.find(header)
    if index < 0:
        return ""
    rest = stdout[index + len(header):]
    ends = [i for i in (rest.find("\n----"), rest.find("\nfailures:\n")) if i >= 0]
    return (rest[:min(ends)] if ends else rest).strip()


def parse_cargo(runner, output):
    tests = []
    for line in lines_of(output["stdout"]):
        trimmed = line.strip()
        if not trimmed.startswith("test "):
            continue
        rest = trimmed[5:]
        if " ... " not in rest:
            continue
        name, verdict = rest.split(" ... ", 1)
        verdict = verdict.strip()
        status = "passed" if verdict == "ok" else "failed" if verdict == "FAILED" else "skipped" if verdict.startswith("ignored") else None
        if status is None:
            continue
        tests.append(case(name.strip(), "", status, None, cargo_body(output["stdout"], name.strip()) if status == "failed" else ""))
    return finish(runner, tests, output)


def parse_nextest(runner, output):
    tests = []
    for line in lines_of(output["stdout"]) + lines_of(output["stderr"]):
        trimmed = line.strip()
        if " " not in trimmed:
            continue
        verdict, rest = trimmed.split(" ", 1)
        status = {"PASS": "passed", "FAIL": "failed", "TRY": "failed", "SKIP": "skipped"}.get(verdict)
        if status is None:
            continue
        rest = rest.strip()
        close = rest.find("]")
        if close < 0:
            continue
        raw = rest[:close].lstrip("[").strip().rstrip("s").strip()
        try:
            duration = float(raw) * 1000.0
        except ValueError:
            duration = None
        identifier = rest[close + 1:].strip()
        suite, name = identifier.split(" ", 1) if " " in identifier else ("", identifier)
        tests.append(case(name.strip(), suite.strip(), status, duration))
    return finish(runner, tests, output)


def dotnet_duration(value):
    value = value.strip()
    if " " not in value:
        return None
    number, unit = value.split(" ", 1)
    try:
        number = float(number)
    except ValueError:
        return None
    return {"ms": number, "s": number * 1000.0, "us": number / 1000.0}.get(unit)


def parse_dotnet(runner, output):
    tests = []
    for line in lines_of(output["stdout"]):
        trimmed = line.strip()
        if " " not in trimmed:
            continue
        verdict, rest = trimmed.split(" ", 1)
        status = {"Passed": "passed", "Failed": "failed", "Skipped": "skipped"}.get(verdict)
        if status is None:
            continue
        rest = rest.strip()
        if rest.startswith("!") or rest.startswith("-") or not rest:
            continue
        if " [" in rest:
            name, tail = rest.rsplit(" [", 1)
            tests.append(case(name.strip(), "", status, dotnet_duration(tail.rstrip("]"))))
        else:
            tests.append(case(rest, "", status))
    return finish(runner, tests, output)


def parse_rspec(runner, output):
    tests = []
    try:
        report = json.loads(json_body(output["stdout"]))
    except ValueError:
        report = None
    if isinstance(report, dict):
        for example in report.get("examples") or []:
            name = example.get("full_description") if isinstance(example.get("full_description"), str) else example.get("description", "")
            status = {"passed": "passed", "failed": "failed"}.get(example.get("status", ""), "skipped")
            exception = example.get("exception")
            message = exception.get("message", "") if isinstance(exception, dict) and isinstance(exception.get("message"), str) else ""
            run_time = example.get("run_time")
            tests.append(case(name, example.get("file_path", ""), status, float(run_time) * 1000.0 if isinstance(run_time, (int, float)) else None, message))
    return finish(runner, tests, output)


PARSERS = {"go": parse_go, "cargo": parse_cargo, "cargo-nextest": parse_nextest, "dotnet": parse_dotnet, "npx": parse_vitest, "npm": parse_vitest, "uv": parse_pytest, "pytest": parse_pytest, "rspec": parse_rspec}

EXTRA = [
    {"id": "npm-runs-the-vitest-dialect", "runner": "npm", "output": {"status": 0, "stdout": "> app@1.0.0 test\n> vitest run --reporter=json\n{\"testResults\":[{\"name\":\"/r/a.test.ts\",\"assertionResults\":[{\"title\":\"only a title\",\"status\":\"passed\",\"duration\":2,\"failureMessages\":[]}]}]}\nchatter after\n", "stderr": ""}},
    {"id": "uv-runs-the-pytest-dialect", "runner": "uv", "output": {"status": 1, "stdout": "tests/test_a.py::test_one XPASS [ 10%]\ntests/test_a.py::test_two ERROR [ 20%]\ntests/test_a.py::test_three XFAIL (reason) [ 30%]\nnot a test line\nsingle\ntests/test_b.py::test_four WARNING\n", "stderr": ""}},
    {"id": "pytest-with-no-tests-collected", "runner": "pytest", "output": {"status": 5, "stdout": "collected 0 items\n", "stderr": ""}},
    {"id": "go-with-skips-package-events-and-crlf", "runner": "go", "output": {"status": 0, "stdout": "{\"Action\":\"start\",\"Package\":\"p\"}\r\n{\"Action\":\"output\",\"Package\":\"p\",\"Test\":\"TestA\",\"Output\":\"line one\\n\"}\r\n{\"Action\":\"skip\",\"Package\":\"p\",\"Test\":\"TestA\",\"Elapsed\":0}\r\n{\"Action\":\"pass\",\"Package\":\"p\",\"Elapsed\":0.5}\r\nnot json\r\n[1,2]\r\n{\"Action\":\"fail\",\"Package\":\"p\",\"Test\":\"TestB\"}\r\n{\"Action\":\"output\",\"Package\":\"p\",\"Test\":\"TestB\",\"Output\":\"late\\n\"}\r\n{\"Action\":\"pass\",\"Package\":\"q\",\"Test\":\"TestA\",\"Elapsed\":1}\r\n", "stderr": ""}},
    {"id": "go-failure-keeps-its-output-with-the-trailing-whitespace-trimmed", "runner": "go", "output": {"status": 1, "stdout": "{\"Action\":\"output\",\"Package\":\"p\",\"Test\":\"TestA\",\"Output\":\"=== RUN\\n\"}\n{\"Action\":\"output\",\"Package\":\"p\",\"Test\":\"TestA\",\"Output\":\"    a_test.go:1: boom\\n\\n\"}\n{\"Action\":\"fail\",\"Package\":\"p\",\"Test\":\"TestA\",\"Elapsed\":0.25}\n", "stderr": ""}},
    {"id": "cargo-with-two-failures-and-ignored-reasons", "runner": "cargo", "output": {"status": 101, "stdout": "running 4 tests\ntest a::one ... ok\ntest a::two ... FAILED\ntest a::three ... ignored, needs network\ntest b::four ... FAILED\ntest weird line without verdict\ntest c::five ... bench: 12 ns/iter\n\nfailures:\n\n---- a::two stdout ----\nthread 'a::two' panicked\nleft: 1\n\n---- b::four stdout ----\nboom\n\nfailures:\n    a::two\n    b::four\n\ntest result: FAILED. 1 passed; 2 failed; 1 ignored\n", "stderr": ""}},
    {"id": "cargo-failure-without-a-body", "runner": "cargo", "output": {"status": 101, "stdout": "test only::case ... FAILED\n", "stderr": ""}},
    {"id": "nextest-with-retries-and-stderr", "runner": "cargo-nextest", "output": {"status": 100, "stdout": "        PASS [   1.500s] crate-a a::fast\n        TRY 1 FAIL [   0.200s] crate-a a::flaky\n", "stderr": "        FAIL [   0.004s] crate-b b::slow\n        SKIP [   0.000s] crate-b b::skipped\n        PASS [0.5s] crate-c lone_identifier\n        PASS [ nonsense ] crate-c c::no_duration\n        PASS no bracket here\n"}},
    {"id": "dotnet-with-units-and-summary-lines", "runner": "dotnet", "output": {"status": 1, "stdout": "  Passed A.B.One [3 ms]\n  Passed A.B.Two [1.5 s]\n  Failed A.B.Three [250 us]\n  Failed A.B.Four [7 parsecs]\n  Skipped A.B.Five\n  Passed! - Failed: 0, Passed: 2\n  Failed! - Failed: 2\n  Passed ! odd\n  Passed - dash\n  Passed\n", "stderr": ""}},
    {"id": "vitest-with-chatter-around-the-json-and-a-missing-full-name", "runner": "npx", "output": {"status": 1, "stdout": "RUN  v3.0.0\n{\"testResults\":[{\"name\":\"/r/a.test.ts\",\"assertionResults\":[{\"fullName\":\"suite does\",\"title\":\"does\",\"status\":\"failed\",\"duration\":1,\"failureMessages\":[\"first\",\"second\"]},{\"title\":\"plain title\",\"status\":\"todo\"}]},{\"name\":\"/r/empty.test.ts\",\"assertionResults\":[]}]}\nDuration 1s done\n", "stderr": ""}},
    {"id": "vitest-with-braces-in-the-trailing-chatter-finds-no-json", "runner": "npx", "output": {"status": 0, "stdout": "{\"testResults\":[]}\ntrailing {noise}\n", "stderr": ""}},
    {"id": "vitest-with-broken-json", "runner": "npx", "output": {"status": 0, "stdout": "{ \"testResults\": [ not json }", "stderr": ""}},
    {"id": "rspec-with-pending-and-exception", "runner": "rspec", "output": {"status": 1, "stdout": "{\"examples\":[{\"description\":\"only description\",\"status\":\"pending\",\"file_path\":\"./a_spec.rb\",\"run_time\":1},{\"full_description\":\"Thing breaks\",\"status\":\"failed\",\"file_path\":\"./b_spec.rb\",\"run_time\":0.25,\"exception\":{\"message\":\"expected 1 to eq 2\"}},{\"full_description\":\"No exception message\",\"status\":\"failed\",\"exception\":{}}]}", "stderr": ""}},
    {"id": "green-run-with-a-nonzero-exit-is-failed", "runner": "go", "output": {"status": 2, "stdout": "{\"Action\":\"pass\",\"Package\":\"p\",\"Test\":\"TestA\",\"Elapsed\":0.001}\n", "stderr": "panic: boom"}},
    {"id": "empty-output-and-zero-exit-is-passed-with-no-tests", "runner": "pytest", "output": {"status": 0, "stdout": "", "stderr": ""}},
]

SELECTORS = [
    "", "notavalidid", "repo://p/🧰/semio", "A B_c-D.e", "ÄÖÜ äöü ß", "Name 123 !?", "🧬️alpha.ts", "x%20y/z%20w", "%20", "100%", "a%2520b", "a/b%20/c", "%20%20", "plain", "dir%20name/file%20name.go", "UPPER lower", "tab\there", "ǅ ǆ Ǆ", "İstanbul", "ΣΑΣ",
]


def simple_lower(text):
    return "".join(c.lower() if len(c.lower()) == 1 else c for c in text)


def flat(text):
    return simple_lower("".join(c for c in text if (c.isascii() and c.isalnum()) or ord(c) > 0x7F))


def path_from_uri_path(text):
    return "/".join(segment.replace("%20", " ") for segment in text.split("/"))


def build():
    transcripts = json.loads((FIXTURES / "📜️runner-transcripts.json").read_text(encoding="utf8"))["vectors"]
    vectors = transcripts + EXTRA
    selectors = json.loads((FIXTURES / "🧬️scope-selectors.json").read_text(encoding="utf8"))["selectors"] + SELECTORS
    return {
        "schema": "semio.repo.test-runner.unit-vectors/1",
        "$comment": "🔬️ Expected answers for the cargo unit tests, computed by an independent Python model (json, re and string methods; no code shared with the Rust or Go implementation) of the seven runner report dialects and the scope selector arithmetic. `extra` are additional recorded-style transcripts that stress every parser branch. Regenerate with the model; never edit by hand.",
        "oracle": {"script": ORACLE_SCRIPT, "tool": "python3", "libraries": ["json", "re"], "model": "go test -json, vitest/jest json, pytest, libtest, nextest, dotnet, rspec parsers; flat; path_from_uri_path"},
        "extra": EXTRA,
        "parsed": [{"id": v["id"], "outcome": PARSERS[v["runner"]](v["runner"], v["output"])} for v in vectors],
        "selectors": [{"selector": s, "flat": flat(s), "path": path_from_uri_path(s)} for s in selectors],
    }


def main():
    text = json.dumps(build(), ensure_ascii=False, indent=1) + "\n"
    if "--check" in sys.argv:
        same = TARGET.exists() and TARGET.read_text(encoding="utf8") == text
        print("SAME" if same else "DIFFERENT")
        sys.exit(0 if same else 1)
    TARGET.write_text(text, encoding="utf8")
    print("written", TARGET, len(text))


main()
