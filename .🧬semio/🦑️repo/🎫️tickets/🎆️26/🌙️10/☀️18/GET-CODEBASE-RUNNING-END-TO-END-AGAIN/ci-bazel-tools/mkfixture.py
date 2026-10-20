#!/usr/bin/env python3
"""Generates both language-agnostic fixtures with oracles that do not share code with the TypeScript modules.

Oracles: Python (json, hashlib, xml.etree, re) for every expected value, then cross-checked with the third-party tools
`jq` (selections, deltas, BEP rows) and `xmllint` (JUnit counts) when installed; a disagreement aborts generation.
Real inputs: the Bazel 8.4.2 build event streams, console logs and test.xml files under fixtures-src/ (scratch workspace
with a three-line test rule, provisioned Bazel, one run fresh and one rerun).

  python3 mkfixture.py            writes gen/ci-report-fixture.json and gen/test-results-fixture.json
"""
import hashlib, json, os, re, shutil, subprocess, sys
import xml.etree.ElementTree as ET

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "fixtures-src")
OUT = os.path.join(HERE, "gen")
ROOT_TOKEN = "@ROOT@"
REAL_ROOT = "/private/tmp/ci-bazel/ob/execroot/_main/bazel-out/darwin_arm64-fastbuild"


def read(name):
    return open(os.path.join(SRC, name), encoding="utf8").read()


def sh(*args, input=None):
    return subprocess.run(args, input=input, capture_output=True, text=True, check=True).stdout


def pretty(value):
    return json.dumps(value, indent=2, ensure_ascii=False)


def status(tool, entries, nbytes, activity=None, available=True):
    value = {"version": 1, "tool": tool, "available": available,
             "directory": "/ws/.cache/b/p/m/bazel/actions" if tool == "bazel" else "/ws/.cache/b/n/artifacts",
             "entries": entries, "bytes": nbytes}
    if activity is not None:
        value["activity"] = dict(zip(("builds", "actions", "executed", "cacheHits"), activity))
    return value


def decorate(value, before="[script] counting cache entries\n", after="[script] done in 0.4 s\n"):
    return before + pretty(value) + "\n" + after


def delta_oracle(a, b):
    out = {"tool": b["tool"], "entries": b["entries"] - a["entries"], "bytes": b["bytes"] - a["bytes"]}
    fa, fb = a.get("activity"), b.get("activity")
    keys = ("builds", "actions", "executed", "cacheHits")
    for key in keys:
        out[key] = 0 if fb is None else fb[key] - (fa[key] if fa else 0)
    worked = out["cacheHits"] + out["executed"]
    out["hitRate"] = None if fb is None or worked <= 0 else out["cacheHits"] / worked
    return out


NX_HIT = re.compile(r"read the output from the cache|\[local cache\]|\[existing outputs match the cache\]")
BZ_HIT = re.compile(r"(disk|remote) cache hit")
SPAWN = re.compile(r"[0-9]+ (linux-sandbox|darwin-sandbox|processwrapper-sandbox|windows-sandbox|local|worker)")


def proof_oracle(tool, s, logs):
    problems = []
    if not (s[1]["entries"] > s[0]["entries"] and s[1]["bytes"] > s[0]["bytes"]):
        problems.append("proofNoGrowth")
    if s[2]["entries"] != s[1]["entries"]:
        problems.append("proofGrowth")
    rule = "none"
    acts = [x.get("activity") for x in s]
    if tool == "nx":
        if NX_HIT.search(logs[1]):
            rule = "log-marker"
    elif BZ_HIT.search(logs[1]):
        rule = "log-marker"
    elif all(acts) and acts[1]["executed"] - acts[0]["executed"] > 0 and acts[2]["executed"] == acts[1]["executed"] and acts[2]["builds"] > acts[1]["builds"]:
        rule = "activity"
    elif SPAWN.search(logs[0]) and not SPAWN.search(logs[1]) and "Build completed successfully" in logs[1]:
        rule = "spawn-marker"
    if rule == "none":
        problems.append("proofNoHit")
    return {"tool": tool, "ok": not problems, "rule": rule,
            "entries": [x["entries"] for x in s], "bytes": [x["bytes"] for x in s], "problems": problems}


def task(project, target, configuration=None):
    t = {"project": project, "target": target}
    if configuration:
        t["configuration"] = configuration
    return {"id": f"{project}:{target}" + (f":{configuration}" if configuration else ""), "target": t}


def plan(tasks):
    entries = {t["id"]: t for t in tasks}
    return {"tasks": {"roots": [t["id"] for t in tasks][:1], "tasks": entries, "dependencies": {k: [] for k in entries}}}


def matches(target, patterns):
    rx = [re.compile("^" + ".*".join(re.escape(p) for p in item.strip().split("*")) + "$") for item in patterns.split(",") if item.strip()]
    return any(r.match(target) for r in rx)


def selection_oracle(p, targets):
    return sorted({f"{t['target']['project']}:{t['target']['target']}" + (f":{t['target']['configuration']}" if "configuration" in t["target"] else "")
                   for t in p["tasks"]["tasks"].values() if matches(t["target"]["target"], targets)})


def junit_oracle(xml):
    root = ET.fromstring(xml)
    count = lambda name: len(root.findall(f".//{name}")) + (1 if root.tag == name else 0)
    return {"tests": count("testcase"), "failures": count("failure"), "errors": count("error"), "skipped": count("skipped")}


def jq_selection(p, targets):
    regex = "^(" + "|".join(re.escape(x).replace("\\*", ".*") for x in targets.split(",")) + ")$"
    program = f'[.tasks.tasks[] | select(.target.target|test("{regex}")) | "\\(.target.project):\\(.target.target)" + (if .target.configuration then ":" + .target.configuration else "" end)] | unique | sort'
    return json.loads(sh("jq", "-c", program, input=json.dumps(p)))


def xmllint_counts(xml):
    def n(expression):
        return int(float(sh("xmllint", "--xpath", f"count({expression})", "-", input=xml)))
    return {"tests": n("//testcase"), "failures": n("//failure"), "errors": n("//error"), "skipped": n("//skipped")}


have_jq = shutil.which("jq") is not None
have_xmllint = shutil.which("xmllint") is not None


def ci_report_fixture():
    f = {"version": 1}
    base = status("bazel", 0, 0, (0, 0, 0, 0))
    grown = status("bazel", 14, 52113, (1, 9, 4, 0))
    again = status("bazel", 14, 52113, (2, 10, 4, 0))
    cached = status("bazel", 20, 90000, (3, 30, 4, 21))
    f["extraction"] = [
        {"name": "pretty-with-log-lines", "text": decorate(grown), "expectedCount": 1},
        {"name": "crlf-and-trailing-brace-in-log", "text": decorate(grown).replace("\n", "\r\n") + "{not json}\r\n", "expectedCount": 1},
        {"name": "two-documents-last-wins", "text": decorate(base) + decorate(grown), "expectedCount": 2},
        {"name": "braces-inside-strings", "text": '{"version":1,"tool":"nx","available":true,"directory":"/a}b{c","entries":1,"bytes":2}\n', "expectedCount": 1},
        {"name": "no-json", "text": "[script] nothing here\n", "expectedCount": 0},
    ]
    f["statuses"] = [
        {"name": "bazel-decorated", "text": decorate(grown), "expected": grown},
        {"name": "nx-plain", "text": pretty(status("nx", 3, 9000)) + "\n", "expected": status("nx", 3, 9000)},
        {"name": "bazel-unavailable", "text": decorate(status("bazel", 0, 0, (0, 0, 0, 0), available=False)), "expected": status("bazel", 0, 0, (0, 0, 0, 0), available=False)},
    ]
    pairs = [("cold-run", base, grown), ("warm-run", grown, cached), ("no-work", cached, cached), ("nx-no-activity", status("nx", 1, 10), status("nx", 4, 80)),
             ("gc-shrank", cached, status("bazel", 5, 1000, (3, 30, 4, 21)))]
    f["deltas"] = []
    for name, a, b in pairs:
        row = {"name": name, "before": decorate(a), "after": decorate(b), "expected": delta_oracle(a, b), "hasActivity": "activity" in b}
        if have_jq:
            jq = json.loads(sh("jq", "-c", ".entries", input=pretty(b)))
            assert jq == b["entries"]
        f["deltas"].append(row)
    real1, real2 = read("proof-run1.log"), read("proof-run2.log")
    nx1 = "> nx run @teaching/architecture-course:test\n\nNx: executed 3 tasks\n"
    nx2 = "> nx run @teaching/architecture-course:test  [local cache]\n\nNx read the output from the cache instead of running the command for 3 out of 3 tasks.\n"
    ns0, ns1, ns2 = status("nx", 0, 0), status("nx", 5, 4096), status("nx", 5, 4096)
    f["proofs"] = []

    def proof(name, tool, statuses, logs):
        f["proofs"].append({"name": name, "tool": tool, "statuses": [decorate(s) for s in statuses], "logs": list(logs), "expected": proof_oracle(tool, statuses, logs)})

    proof("bazel-activity-rule-real-logs", "bazel", [base, grown, again], (real1, real2))
    nogrow = lambda s: {k: v for k, v in s.items() if k != "activity"}
    proof("bazel-spawn-marker-real-logs", "bazel", [nogrow(base), nogrow(grown), nogrow(again)], (real1, real2))
    proof("bazel-remote-or-disk-marker", "bazel", [nogrow(base), nogrow(grown), nogrow(again)], (real1, "INFO: 1 process: 3 disk cache hit, 1 internal.\nINFO: Build completed successfully, 4 total actions\n"))
    proof("bazel-action-cache-hit-alone-is-not-a-marker", "bazel", [nogrow(base), nogrow(grown), nogrow(again)], (real1, "INFO: 1 process: 2 action cache hit, 1 darwin-sandbox.\nINFO: Build completed successfully, 3 total actions\n"))
    proof("bazel-second-run-executed", "bazel", [base, grown, status("bazel", 14, 52113, (2, 18, 12, 0))], (real1, real1))
    proof("bazel-nothing-stored", "bazel", [base, base, base], (real1, real2))
    proof("bazel-second-run-grew-the-store", "bazel", [base, grown, status("bazel", 16, 60000, (2, 10, 4, 0))], (real1, real2))
    proof("nx-hit", "nx", [ns0, ns1, ns2], (nx1, nx2))
    proof("nx-miss", "nx", [ns0, ns1, ns2], (nx1, nx1))
    proof("nx-bazel-markers-do-not-count", "nx", [ns0, ns1, ns2], (nx1, "INFO: 1 process: 3 disk cache hit\n"))
    plans = {
        "first": plan([task("@teaching/architecture-course", "build"), task("@teaching/architecture-course", "test"), task("@semio-tech/framework", "build"), task("@semio-tech/framework", "lint")]),
        "reordered": plan([task("@semio-tech/framework", "lint"), task("@semio-tech/framework", "build"), task("@teaching/architecture-course", "test"), task("@teaching/architecture-course", "build")]),
        "missing-test": plan([task("@teaching/architecture-course", "build"), task("@semio-tech/framework", "build")]),
        "extra-project": plan([task("@teaching/architecture-course", "build"), task("@teaching/architecture-course", "test"), task("@semio-tech/framework", "build"), task("@semio-tech/play", "test")]),
        "configured": plan([task("@a/b", "build", "release"), task("@a/b", "build", "prototype"), task("@a/b", "typecheck")]),
        "empty": plan([]),
    }
    f["plans"] = {k: decorate(v, "[repo] planning\n", "[repo] planned\n") for k, v in plans.items()}
    f["selections"] = []
    for name, targets in [("first", "build,test"), ("first", "b*"), ("first", "*"), ("configured", "build"), ("empty", "build,test"), ("missing-test", "test")]:
        expected = selection_oracle(plans[name], targets)
        if have_jq and targets in ("build,test", "build", "test"):
            assert jq_selection(plans[name], targets) == expected, (name, targets)
        f["selections"].append({"plan": name, "targets": targets, "expected": expected,
                                "summary": {"tasks": len(plans[name]["tasks"]["tasks"]), "selected": len(expected), "projects": len({x.split(":")[0] for x in expected}),
                                            "targets": [t.strip() for t in targets.split(",")]}})
    f["parities"] = []
    for a, b, targets in [("first", "reordered", "build,test"), ("first", "missing-test", "build,test"), ("first", "extra-project", "build,test"), ("empty", "empty", "build,test"), ("first", "extra-project", "build")]:
        sa, sb = selection_oracle(plans[a], targets), selection_oracle(plans[b], targets)
        f["parities"].append({"first": a, "second": b, "targets": targets,
                              "expected": {"equal": sa == sb, "first": len(sa), "second": len(sb), "onlyFirst": [x for x in sa if x not in sb], "onlySecond": [x for x in sb if x not in sa]}})
    pass_xml, fail_xml = read("pass_one.test.xml"), FAIL_XML
    junit = [
        ("real-pass", pass_xml), ("real-fail-error", fail_xml),
        ("nested-suites-not-double-counted", '<testsuites tests="3" failures="1"><testsuite name="outer" tests="3"><testsuite name="inner"><testcase name="a"/><testcase name="b"><failure message="x">trace</failure></testcase></testsuite><testcase name="c"><skipped/></testcase></testsuite></testsuites>'),
        ("cdata-and-comments-ignored", '<testsuites><!-- <testcase name="ghost"><failure/></testcase> --><testsuite><testcase name="a"><system-out><![CDATA[<testcase name="x"><failure/></testcase><error/>]]></system-out></testcase></testsuite></testsuites>'),
        ("lookalike-elements", '<testsuites><testsuite><testcases/><testcase name="a"><errors/><failures/><skippedx/></testcase></testsuite></testsuites>'),
        ("self-closed-and-attributes", '<testsuite><testcase name="a" time="0.1"/><testcase name="b"><failure type="AssertionError" message="m"/></testcase><testcase name="c"><error message="boom"/></testcase></testsuite>'),
        ("empty", "<testsuites/>"),
    ]
    f["junit"] = []
    for name, xml in junit:
        expected = junit_oracle(xml)
        if have_xmllint and name not in ("cdata-and-comments-ignored",):
            assert xmllint_counts(xml) == expected, (name, xmllint_counts(xml), expected)
        f["junit"].append({"name": name, "xml": xml, "expected": expected})
    f["tests"] = [
        {"name": "mixed-with-index", "files": [{"file": "pass_one-aaaa1111.xml", "xml": pass_xml}, {"file": "fail_one-bbbb2222.xml", "xml": fail_xml}],
         "index": {"version": 1, "tool": "bazel", "results": [
             {"label": "//:pass_one", "status": "passed", "cached": True, "durationMillis": 579, "file": "pass_one-aaaa1111.xml"},
             {"label": "//:fail_one", "status": "failed", "cached": False, "durationMillis": 750, "file": "fail_one-bbbb2222.xml"}]},
         "expected": {"files": 2, "tests": 2, "failures": 0, "errors": 1, "skipped": 0, "cached": 1, "failed": ["//:fail_one"]}},
        {"name": "no-index-counts-files", "files": [{"file": "a.xml", "xml": pass_xml}], "index": None,
         "expected": {"files": 1, "tests": 1, "failures": 0, "errors": 0, "skipped": 0, "cached": 0, "failed": []}},
        {"name": "timeout-without-xml-failure", "files": [{"file": "t-cccc3333.xml", "xml": pass_xml}],
         "index": {"version": 1, "tool": "bazel", "results": [{"label": "//:slow", "status": "timeout", "cached": False, "durationMillis": 90000, "file": "t-cccc3333.xml"}]},
         "expected": {"files": 1, "tests": 1, "failures": 0, "errors": 0, "skipped": 0, "cached": 0, "failed": ["//:slow"]}},
        {"name": "none", "files": [], "index": None, "expected": {"files": 0, "tests": 0, "failures": 0, "errors": 0, "skipped": 0, "cached": 0, "failed": []}},
    ]
    f["languages"] = [
        {"flag": "de", "environment": {"SEMIO_LANGUAGE": "en"}, "expected": "de"},
        {"flag": None, "environment": {"SEMIO_LANGUAGE": "de_DE"}, "expected": "de"},
        {"flag": None, "environment": {"LC_ALL": "de_CH.UTF-8", "LANG": "en_US.UTF-8"}, "expected": "de"},
        {"flag": None, "environment": {"LANG": "en_US.UTF-8"}, "expected": "en"},
        {"flag": "fr", "environment": {"LANG": "de_DE"}, "expected": "de"},
        {"flag": None, "environment": {}, "expected": None},
        {"flag": "fr", "environment": {"LANG": "C.UTF-8"}, "expected": None},
    ]
    f["pins"] = {"bun": "1.4.2", "assets": {"bun-linux-x64": "36368faef7527875d5ffa52e53cd48021741f2a83eb6208a8dd64068d422a913",
                                           "bun-darwin-aarch64": "90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f",
                                           "bun-windows-x64": "ce4c17497b2f29712a99d3d53f028de28cd42e3bacb8589599e7f000e49b6405"}}
    ZERO = "0" * 40
    A40, B40 = "a" * 40, "b" * 40
    both = {"SEMIO_REMOTE_CACHE_READ_TOKEN": "r", "SEMIO_REMOTE_CACHE_WRITE_TOKEN": "w"}
    endpoint = {"SEMIO_REMOTE_CACHE_ENDPOINT": "https://cache.example.test"}
    repo = {"default_branch": "main"}

    def scenario(name, event_name, ref, event, variables, secrets, inputs, env, rng, group, cancel, steps):
        return {"name": name, "github": {"event_name": event_name, "ref": ref, "sha": "c" * 40, "event": {"repository": repo, **event}},
                "vars": variables, "secrets": secrets, "inputs": inputs,
                "expected": {"env": env, "range": rng, "group": group, "cancel": cancel, "steps": steps}}

    SAVE_BUN, RESTORE_DISK, SAVE_DISK = "smoke: Save the Bun store (default branch)", "affected: Restore the local cache (no remote cache configured)", "affected: Save the local cache (default branch, no remote cache configured)"
    f["workflow"] = [
        scenario("pull-request-same-repository", "pull_request", "refs/pull/7/merge", {"pull_request": {"number": 7, "base": {"sha": A40}}}, endpoint, both, {},
                 {"SEMIO_REMOTE_CACHE_ENDPOINT": endpoint["SEMIO_REMOTE_CACHE_ENDPOINT"], "SEMIO_REMOTE_CACHE_ACCESS": "read-only", "SEMIO_REMOTE_CACHE_UPLOAD": "false", "SEMIO_REMOTE_CACHE_TOKEN": "r", "BUILD_TOOL": "bazel", "TARGETS": "build,test"},
                 f"--base={A40} --head=HEAD", "build-system-pull_request-7", "true", {SAVE_BUN: False, RESTORE_DISK: False, SAVE_DISK: False}),
        scenario("pull-request-from-fork-has-no-secrets-or-endpoint", "pull_request", "refs/pull/9/merge", {"pull_request": {"number": 9, "base": {"sha": A40}}}, {}, {}, {},
                 {"SEMIO_REMOTE_CACHE_ENDPOINT": "", "SEMIO_REMOTE_CACHE_ACCESS": "read-only", "SEMIO_REMOTE_CACHE_UPLOAD": "false", "SEMIO_REMOTE_CACHE_TOKEN": "", "BUILD_TOOL": "bazel", "TARGETS": "build,test"},
                 f"--base={A40} --head=HEAD", "build-system-pull_request-9", "true", {SAVE_BUN: False, RESTORE_DISK: True, SAVE_DISK: False}),
        scenario("push-to-main-writes-the-remote-cache", "push", "refs/heads/main", {"before": B40}, endpoint, both, {},
                 {"SEMIO_REMOTE_CACHE_ENDPOINT": endpoint["SEMIO_REMOTE_CACHE_ENDPOINT"], "SEMIO_REMOTE_CACHE_ACCESS": "read-write", "SEMIO_REMOTE_CACHE_UPLOAD": "true", "SEMIO_REMOTE_CACHE_TOKEN": "w", "BUILD_TOOL": "bazel", "TARGETS": "build,test"},
                 f"--base={B40} --head=HEAD", "build-system-push-refs/heads/main", "false", {SAVE_BUN: True, RESTORE_DISK: False, SAVE_DISK: False}),
        scenario("push-to-main-without-endpoint-uses-the-github-cache", "push", "refs/heads/main", {"before": B40}, {}, both, {},
                 {"SEMIO_REMOTE_CACHE_ENDPOINT": "", "SEMIO_REMOTE_CACHE_ACCESS": "read-write", "SEMIO_REMOTE_CACHE_UPLOAD": "true", "SEMIO_REMOTE_CACHE_TOKEN": "w", "BUILD_TOOL": "bazel", "TARGETS": "build,test"},
                 f"--base={B40} --head=HEAD", "build-system-push-refs/heads/main", "false", {SAVE_BUN: True, RESTORE_DISK: True, SAVE_DISK: True}),
        scenario("push-of-a-new-branch-selects-everything", "push", "refs/heads/main", {"before": ZERO}, {}, {}, {},
                 {"SEMIO_REMOTE_CACHE_TOKEN": "", "SEMIO_REMOTE_CACHE_ACCESS": "read-write"}, "--all", "build-system-push-refs/heads/main", "false", {SAVE_DISK: True}),
        scenario("dispatch-nx-all-never-writes-and-skips-the-github-cache-when-an-endpoint-is-set", "workflow_dispatch", "refs/heads/feature", {}, endpoint, both,
                 {"tool": "nx", "scope": "all", "base": "origin/main", "targets": "test"},
                 {"SEMIO_REMOTE_CACHE_ACCESS": "read-only", "SEMIO_REMOTE_CACHE_UPLOAD": "false", "SEMIO_REMOTE_CACHE_TOKEN": "r", "BUILD_TOOL": "nx", "TARGETS": "test"},
                 "--all", "build-system-workflow_dispatch-refs/heads/feature", "false", {SAVE_BUN: False, RESTORE_DISK: False, SAVE_DISK: False}),
        scenario("dispatch-affected-compares-against-origin-main-by-default", "workflow_dispatch", "refs/heads/feature", {}, {}, {},
                 {"tool": "bazel", "scope": "affected", "base": "", "targets": ""},
                 {"BUILD_TOOL": "bazel", "TARGETS": "build,test", "SEMIO_REMOTE_CACHE_ACCESS": "read-only"},
                 "--base=origin/main --head=HEAD", "build-system-workflow_dispatch-refs/heads/feature", "false", {RESTORE_DISK: True, SAVE_DISK: False}),
    ]
    return f


FAIL_XML = '''<?xml version="1.0" encoding="UTF-8"?>
<testsuites>
  <testsuite name="fail_one.sh" tests="1" failures="0" errors="1">
    <testcase name="fail_one.sh" status="run" duration="0" time="0"><error message="exited with error code 3"></error></testcase>
      <system-out>
Generated test.log (if the file is not UTF-8, then this may be unreadable):
<![CDATA[exec ${PAGER:-/usr/bin/less} "$0" || exit 1
Executing tests from //:fail_one
-----------------------------------------------------------------------------
no]]>
      </system-out>
    </testsuite>
</testsuites>
'''
SAMPLE_XML_TEMPLATE = read("pass_one.test.xml")


def trim_bep(name):
    lines = []
    for line in read(name).split("\n"):
        if not line:
            continue
        d = json.loads(line)
        key = next(iter(d["id"]))
        if key in ("started", "pattern", "testResult", "testSummary", "buildFinished"):
            lines.append(line.replace(REAL_ROOT, "file-root"))
    text = "\n".join(lines) + "\n"
    return text.replace("file://file-root", "file://" + ROOT_TOKEN)


def bep_oracle(text, with_files):
    latest = {}
    for line in text.split("\n"):
        if not line.strip():
            continue
        try:
            d = json.loads(line)
        except ValueError:
            continue
        ident = d["id"].get("testResult")
        if ident is None:
            continue
        r = d["testResult"]
        uri = next((o["uri"] for o in r.get("testActionOutput", []) if o["name"] == "test.xml"), None)
        row = {"label": ident["label"], "run": ident.get("run", 1), "shard": ident.get("shard", 1), "attempt": ident.get("attempt", 1),
               "status": {"PASSED": "passed", "FAILED": "failed", "FLAKY": "flaky", "TIMEOUT": "timeout"}.get(r.get("status"), "error"),
               "cached": bool(r.get("cachedLocally")) or bool(r.get("executionInfo", {}).get("cachedRemotely")),
               "durationMillis": int(r.get("testAttemptDurationMillis", 0)), "uri": uri}
        key = (row["label"], row["run"], row["shard"])
        if key not in latest or latest[key]["attempt"] <= row["attempt"]:
            latest[key] = row
    per = {}
    for (label, _, _) in latest:
        per[label] = per.get(label, 0) + 1
    rows = []
    for (label, run, shard), row in latest.items():
        stem = re.sub(r"[^A-Za-z0-9._-]+", "_", re.sub(r"^//", "", label))
        stem = stem.strip("_")[:90] or "test"
        h = hashlib.sha256(label.encode()).hexdigest()[:8]
        file = f"{stem}-{h}" + (f".run{run}.shard{shard}" if per[label] > 1 else "") + ".xml"
        uri = row["uri"]
        prefix = "file://" + ROOT_TOKEN + "/"
        rows.append({"label": label, "run": run, "shard": shard, "attempt": row["attempt"], "status": row["status"], "cached": row["cached"],
                     "durationMillis": row["durationMillis"], "file": file, "xml": uri[len(prefix):] if uri and uri.startswith(prefix) else None})
    return rows


def test_results_fixture():
    f = {"version": 1, "rootToken": ROOT_TOKEN}
    f["xml"] = {"pass": SAMPLE_XML_TEMPLATE, "fail": FAIL_XML}
    fresh, rerun = trim_bep("bep-mixed-fresh.json"), trim_bep("bep-mixed-rerun.json")
    synthetic = "\n".join(json.dumps(x) for x in [
        {"id": {"started": {}}, "started": {"uuid": "x"}},
        {"id": {"testResult": {"label": "//pkg:sharded", "run": 1, "shard": 1, "attempt": 1}}, "testResult": {"status": "FAILED", "testAttemptDurationMillis": "100", "testActionOutput": [{"name": "test.xml", "uri": f"file://{ROOT_TOKEN}/pkg/sharded/shard_1_of_2/test_attempts/attempt_1.xml"}]}},
        {"id": {"testResult": {"label": "//pkg:sharded", "run": 1, "shard": 1, "attempt": 2}}, "testResult": {"status": "PASSED", "testAttemptDurationMillis": "120", "testActionOutput": [{"name": "test.xml", "uri": f"file://{ROOT_TOKEN}/pkg/sharded/shard_1_of_2/test.xml"}]}},
        {"id": {"testResult": {"label": "//pkg:sharded", "run": 1, "shard": 2, "attempt": 1}}, "testResult": {"status": "FLAKY", "testAttemptDurationMillis": "90", "cachedLocally": True, "testActionOutput": [{"name": "test.xml", "uri": f"file://{ROOT_TOKEN}/pkg/sharded/shard_2_of_2/test.xml"}]}},
        {"id": {"testResult": {"label": "//remote:kept", "run": 1, "shard": 1, "attempt": 1}}, "testResult": {"status": "PASSED", "testAttemptDurationMillis": "7", "testActionOutput": [{"name": "test.xml", "uri": "bytestream://cache.example/blobs/abc/12"}]}},
        {"id": {"testResult": {"label": "//remote:hit", "run": 1, "shard": 1, "attempt": 1}}, "testResult": {"status": "PASSED", "testAttemptDurationMillis": "3", "executionInfo": {"cachedRemotely": True}, "testActionOutput": [{"name": "test.xml", "uri": f"file://{ROOT_TOKEN}/remote/hit/test.xml"}]}},
        {"id": {"testResult": {"label": "//:vanished", "run": 1, "shard": 1, "attempt": 1}}, "testResult": {"status": "TIMEOUT", "testAttemptDurationMillis": "9000", "testActionOutput": [{"name": "test.xml", "uri": f"file://{ROOT_TOKEN}/vanished/test.xml"}]}},
        {"id": {"testResult": {"label": "//:nostatus", "run": 1, "shard": 1, "attempt": 1}}, "testResult": {"status": "NO_STATUS", "testActionOutput": [{"name": "test.log", "uri": f"file://{ROOT_TOKEN}/nostatus/test.log"}]}},
        {"id": {"testResult": {"label": "//:émoji/测试", "run": 1, "shard": 1, "attempt": 1}}, "testResult": {"status": "PASSED", "testAttemptDurationMillis": "1", "testActionOutput": [{"name": "test.xml", "uri": f"file://{ROOT_TOKEN}/uni/test.xml"}]}},
    ]) + "\nnot json at all\n"
    f["streams"] = []
    for name, text, files in [
        ("mixed-fresh-real", fresh, {"testlogs/pass_one/test.xml": "pass", "testlogs/course/test/test.xml": "pass", "testlogs/fail_one/test.xml": "fail"}),
        ("mixed-rerun-real-cached", rerun, {"testlogs/pass_one/test.xml": "pass", "testlogs/course/test/test.xml": "pass", "testlogs/fail_one/test.xml": "fail"}),
        ("pass-cached-real", trim_bep("bep-pass-cached.json"), {"testlogs/pass_one/test.xml": "pass", "testlogs/course/test/test.xml": "pass"}),
        ("synthetic-shards-remote-missing-unicode", synthetic, {"pkg/sharded/shard_1_of_2/test.xml": "pass", "pkg/sharded/shard_2_of_2/test.xml": "pass", "remote/hit/test.xml": "pass", "uni/test.xml": "pass"}),
    ]:
        rows = bep_oracle(text, True)
        f["streams"].append({"name": name, "events": text, "files": files, "expected": rows})
    return f


def main():
    os.makedirs(OUT, exist_ok=True)
    a = ci_report_fixture()
    b = test_results_fixture()
    # jq cross-check of the BEP rows: labels and statuses of the final attempts
    if have_jq:
        for s in b["streams"]:
            out = sh("jq", "-r", 'select(.id.testResult) | "\\(.id.testResult.label) \\(.id.testResult.attempt // 1) \\(.testResult.status)"', input=s["events"].split("not json at all")[0])
            seen = {}
            for line in out.strip().split("\n"):
                if line:
                    label, attempt, st = line.split(" ", 2)
                    seen.setdefault(label, []).append(st)
            expected_labels = {r["label"] for r in s["expected"]}
            assert expected_labels <= set(seen), (s["name"], expected_labels, set(seen))
    json.dump(a, open(os.path.join(OUT, "ci-report-fixture.json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)
    json.dump(b, open(os.path.join(OUT, "test-results-fixture.json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)
    print("fixtures written; jq:", have_jq, "xmllint:", have_xmllint)


if __name__ == "__main__":
    sys.exit(main())
