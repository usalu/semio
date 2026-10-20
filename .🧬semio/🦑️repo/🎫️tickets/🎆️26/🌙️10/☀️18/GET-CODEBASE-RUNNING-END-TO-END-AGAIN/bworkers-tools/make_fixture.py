# 🧪 Writes the neutral worker fixture: Bazel captures recorded from the provisioned binary, plus plan, classification and protocol cases whose expected values come from an independent Python implementation of the same rules.
import json, os, re, sys
import lib

CAPTURES = "/private/tmp/bworkers/captures"
policy = json.loads(lib.staged("workers-policy.json"))
KINDS = ["test", "check", "generate", "build", "script", "service"]


def classify(target, continuous):
    for rule in policy["classification"]:
        if "continuous" in rule:
            if rule["continuous"] == continuous:
                return rule["kind"]
        elif re.search(rule["pattern"], target):
            return rule["kind"]
    return policy["defaultKind"]


def plan(host, overrides):
    enabled = overrides.get("workers", policy["enabled"])
    jobs = overrides.get("jobs", max(1, min(host["cores"], policy["localJobs"]["maximum"])))
    total = max(64, int(host["memoryBytes"] / 1048576 * policy["memory"]["hostShareBasisPoints"] / 10000))
    instances = max(1, min(1024, total // policy["memory"]["perWorkerMegabytes"]))
    wanted = [k for k in KINDS if enabled and policy["kinds"][k]["workers"]]
    counts = {k: 1 for k in wanted[:instances]}
    remaining = instances - len(counts)
    for k in list(counts):
        add = min(max(0, min(policy["kinds"][k]["maxInstances"], jobs) - 1), remaining)
        counts[k] += add
        remaining -= add
    kinds = {}
    for k in KINDS:
        row = policy["kinds"][k]
        kinds[k] = {"workers": True, "maxInstances": counts[k], "multiplex": row["multiplex"], "inProcess": row["inProcess"]} if k in counts else {"workers": False, "maxInstances": 1, "multiplex": False, "inProcess": False}
    return {"version": 1, "enabled": enabled, "protocol": policy["protocol"], "localJobs": jobs, "memory": {"totalMegabytes": total, "perWorkerMegabytes": policy["memory"]["perWorkerMegabytes"], "instances": instances}, "sandboxing": policy["sandboxing"], "cancellation": policy["cancellation"], "resetAfterRequests": policy["resetAfterRequests"], "kinds": kinds}


def capture(name):
    if not os.path.exists(os.path.join(CAPTURES, name + ".log")):
        return json.loads(lib.staged("workers-fixture.json"))["bazel"]["captures"][name]
    chunks, responses = [], []
    for line in open(os.path.join(CAPTURES, name + ".log"), encoding="utf8"):
        if line.startswith("STDIN["):
            chunks.extend(json.loads(line[len("STDIN"):]))
        elif line.startswith("RESP ") or line.startswith("CANCELRESP "):
            responses.append(json.loads(line.split(" ", 1)[1]))
    return {"stdin": chunks, "responses": responses}


targets = [
    ("test", False), ("test-snapshot-sqlite", False), ("test-original-receiving-source", False), ("smoke-battery", False), ("mutation-run", False), ("bench-hot", False), ("e2e-live", False),
    ("typecheck", False), ("check", False), ("lint", False), ("verify-dependencies", False), ("format-check", False), ("schema-sync", False), ("stdio-contract", False), ("graph-wire-check", False), ("source-check", False),
    ("generate", False), ("generate-browser-boot", False), ("graph-generate", False), ("\U0001F3ED️generator", False), ("\U0001F3ED️generator-inputs", False),
    ("build", False), ("build-release", False), ("prepare-prototype", False), ("activate-wgpu-react-prototype", False), ("wasm", False), ("native-build", False), ("package", False), ("publish-docs", False), ("deps-browsers", False), ("preview-generated", False),
    ("run", False), ("session-play", False), ("canonical-architecture", False), ("watch-types", False), ("home", False), ("regenerated", False),
    ("serve", True), ("dev", True), ("watch", True), ("test", True),
]
classification = [{"target": t, "continuous": c, "kind": classify(t, c)} for t, c in targets]

M = 1024 ** 3
hosts = [
    {"name": "ten cores 32 GiB", "host": {"cores": 10, "memoryBytes": 32 * M}, "overrides": {}},
    {"name": "two cores 4 GiB", "host": {"cores": 2, "memoryBytes": 4 * M}, "overrides": {}},
    {"name": "sixty-four cores 256 GiB", "host": {"cores": 64, "memoryBytes": 256 * M}, "overrides": {}},
    {"name": "one core 512 MiB", "host": {"cores": 1, "memoryBytes": 512 * 1048576}, "overrides": {}},
    {"name": "explicit jobs", "host": {"cores": 10, "memoryBytes": 32 * M}, "overrides": {"jobs": 3}},
    {"name": "workers off", "host": {"cores": 10, "memoryBytes": 32 * M}, "overrides": {"workers": False}},
    {"name": "memory fits three instances", "host": {"cores": 10, "memoryBytes": 5 * M}, "overrides": {}},
]
plans = [{**row, "expected": plan(row["host"], row["overrides"])} for row in hosts]

invalid = [
    {"name": "unknown protocol", "set": {"protocol": "proto"}},
    {"name": "no classification rule", "set": {"classification": []}},
    {"name": "rule without a condition", "set": {"classification": [{"kind": "test"}]}},
    {"name": "unknown kind", "set": {"defaultKind": "service"}},
    {"name": "zero instances", "setIn": {"kinds.test.maxInstances": 0}},
    {"name": "multiplex and in-process together", "setIn": {"kinds.test.multiplex": True}},
    {"name": "extra top-level field", "set": {"unexpected": True}},
    {"name": "memory share beyond the host", "setIn": {"memory.hostShareBasisPoints": 9500}},
]

frames = {
    "valid": [
        {"name": "empty object", "line": "{}", "expected": {"arguments": [], "inputs": [], "requestId": 0, "cancel": False, "verbosity": 0}},
        {"name": "unknown fields are ignored", "line": '{"arguments":["a"],"future":{"x":1},"requestId":7}', "expected": {"arguments": ["a"], "inputs": [], "requestId": 7, "cancel": False, "verbosity": 0}},
        {"name": "cancel without id", "line": '{"cancel":true}', "expected": {"arguments": [], "inputs": [], "requestId": 0, "cancel": True, "verbosity": 0}},
        {"name": "sandbox directory", "line": '{"arguments":["x"],"sandboxDir":"sandbox/3","verbosity":10}', "expected": {"arguments": ["x"], "inputs": [], "requestId": 0, "cancel": False, "verbosity": 10, "sandboxDir": "sandbox/3"}},
        {"name": "non-ascii argument and input", "line": json.dumps({"arguments": ["s/\U0001F9EC️/ü.txt"], "inputs": [{"path": "dir-\U0001F9EC/in-\U0001F991.txt", "digest": "ZGlnZXN0"}]}, ensure_ascii=False), "expected": {"arguments": ["s/\U0001F9EC️/ü.txt"], "inputs": [{"path": "dir-\U0001F9EC/in-\U0001F991.txt", "digest": "ZGlnZXN0"}], "requestId": 0, "cancel": False, "verbosity": 0}},
    ],
    "invalid": [
        {"name": "not json", "line": "{arguments"},
        {"name": "array", "line": "[]"},
        {"name": "arguments not strings", "line": '{"arguments":[1]}'},
        {"name": "negative request id", "line": '{"requestId":-1}'},
        {"name": "input without digest", "line": '{"inputs":[{"path":"a"}]}'},
        {"name": "cancel not boolean", "line": '{"cancel":"yes"}'},
    ],
    "responses": [
        {"name": "plain", "response": {"exitCode": 0, "output": "ok", "requestId": 0}, "line": '{"exitCode":0,"output":"ok","requestId":0}\n'},
        {"name": "cancelled", "response": {"exitCode": 0, "output": "", "requestId": 4, "wasCancelled": True}, "line": '{"exitCode":0,"output":"","requestId":4,"wasCancelled":true}\n'},
        {"name": "failure with newline and non-ascii", "response": {"exitCode": 1, "output": "bad\nü\U0001F9EC", "requestId": 2}, "line": json.dumps({"exitCode": 1, "output": "bad\nü\U0001F9EC", "requestId": 2}, ensure_ascii=False, separators=(",", ":")) + "\n"},
    ],
    "truncation": {"maximumOutputBytes": 64, "output": "\U0001F9EC" * 40, "marker": "\n[worker] output truncated\n"},
}

fixture = {
    "version": 1,
    "bazel": {"version": "8.4.2", "note": "Recorded with the provisioned binary: a rule with requires-worker-protocol=json, supports-workers, supports-worker-cancellation and, for multiplex, supports-multiplex-workers; the worker logged every chunk it read from standard input.", "captures": {"singleplex": capture("singleplex"), "multiplex": capture("multiplex"), "cancel": capture("cancel")}},
    "classification": classification,
    "plans": plans,
    "invalidPolicies": invalid,
    "frames": frames,
    "invocation": [
        {"args": ["run", "a:b"], "environment": {}, "workers": None},
        {"args": ["run", "a:b", "--workers", "off"], "environment": {}, "workers": "off", "rest": ["run", "a:b"]},
        {"args": ["--workers=on", "run", "a:b"], "environment": {}, "workers": "on", "rest": ["run", "a:b"]},
        {"args": ["run", "a:b"], "environment": {"SEMIO_BUILD_WORKERS": "off"}, "workers": "off", "rest": ["run", "a:b"]},
        {"args": ["run", "a:b", "--workers=off"], "environment": {"SEMIO_BUILD_WORKERS": "on"}, "workers": "off", "rest": ["run", "a:b"]},
        {"args": ["run", "a:b", "--", "--workers", "off"], "environment": {}, "workers": None, "rest": ["run", "a:b", "--", "--workers", "off"]},
    ],
    "invalidSwitches": ["yes", "1", "ON", "true"],
}
lib.write_staged("workers-fixture.json", json.dumps(fixture, indent=1, ensure_ascii=False) + "\n")
print("fixture", len(json.dumps(fixture)), "bytes;", len(classification), "classification rows;", len(plans), "plans")
for row in plans:
    print(row["name"], {k: v["maxInstances"] for k, v in row["expected"]["kinds"].items() if v["workers"]}, row["expected"]["memory"])
