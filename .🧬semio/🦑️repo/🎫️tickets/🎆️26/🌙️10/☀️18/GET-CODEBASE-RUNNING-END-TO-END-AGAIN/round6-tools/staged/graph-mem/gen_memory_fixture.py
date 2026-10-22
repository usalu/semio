#!/usr/bin/env python3
"""Writes the language-agnostic fixture of the graph memory laws (staged/new/memory/tests/fixture.json): named admission cases checked against hand-computed verdicts, 300 seeded admission states, budget/language/message tables and the governor scenarios. Expectations of the tables come from the independent model in oracle.py; the named admission cases additionally carry hand-computed verdicts that the generator asserts."""
import importlib.util, json, pathlib, random
HERE = pathlib.Path(__file__).resolve().parent
NEW = HERE / "staged/new/memory"
spec = importlib.util.spec_from_file_location("oracle", NEW / "tests/oracle.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
schema = json.loads((NEW / "module/🧬️schema/🔣️.json").read_text(encoding="utf8"))
policy, messages = schema["definitions"]["Policy"]["const"], schema["definitions"]["Messages"]["const"]
MB = 1048576
state = lambda used, budget, running, workers, blocked=False, collected=False: {"used": used, "budget": budget, "running": running, "workers": workers, "blocked": blocked, "collected": collected}
named = [
    ("workers-full-ignores-memory", state(1, 1000, 4, 4), {"admit": False, "collect": False, "reason": "workers"}),
    ("nothing-running-and-within-the-budget", state(900, 1000, 0, 4), {"admit": True, "collect": False, "reason": "within"}),
    ("nothing-running-above-the-budget-collects-first", state(9000, 1000, 0, 4), {"admit": False, "collect": True, "reason": "collect"}),
    ("nothing-running-above-after-a-collection-is-forced", state(9000, 1000, 0, 4, collected=True), {"admit": True, "collect": False, "reason": "forced"}),
    ("within-the-budget", state(900, 1000, 2, 4), {"admit": True, "collect": False, "reason": "within"}),
    ("exactly-the-budget-is-within", state(1000, 1000, 2, 4), {"admit": True, "collect": False, "reason": "within"}),
    ("above-collects-first", state(1100, 1000, 2, 4), {"admit": False, "collect": True, "reason": "collect"}),
    ("above-after-a-collection-waits", state(1100, 1000, 2, 4, collected=True), {"admit": False, "collect": False, "reason": "over"}),
    ("blocked-needs-the-resume-ratio", state(900, 1000, 2, 4, blocked=True, collected=True), {"admit": False, "collect": False, "reason": "over"}),
    ("blocked-resumes-below-the-ratio", state(800, 1000, 2, 4, blocked=True), {"admit": True, "collect": False, "reason": "within"}),
    ("blocked-exactly-at-the-ratio-resumes", state(850, 1000, 2, 4, blocked=True), {"admit": True, "collect": False, "reason": "within"}),
]
for name, row, expected in named:
    assert oracle.decide(policy, row) == expected, name
admission = [{"name": name, "state": row, "expect": expected} for name, row, expected in named]
rng = random.Random(1020)
for index in range(300):
    workers = rng.randint(1, 8)
    row = state(rng.randint(0, 4000), rng.choice([500, 1000, 2000]), rng.randint(0, workers + 1), workers, rng.random() < 0.4, rng.random() < 0.5)
    admission.append({"name": "seeded-%03d" % index, "state": row, "expect": oracle.decide(policy, row)})
budgets = [
    ("default-without-environment", {}, {"bytes": 3072 * MB}),
    ("explicit-megabytes", {"SEMIO_GRAPH_MEMORY_MB": "2048"}, {"bytes": 2048 * MB}),
    ("explicit-minimum", {"SEMIO_GRAPH_MEMORY_MB": "1024"}, {"bytes": 1024 * MB}),
    ("explicit-below-minimum-refused", {"SEMIO_GRAPH_MEMORY_MB": "512"}, {"error": True}),
    ("explicit-not-a-number-refused", {"SEMIO_GRAPH_MEMORY_MB": "plenty"}, {"error": True}),
    ("explicit-empty-refused", {"SEMIO_GRAPH_MEMORY_MB": ""}, {"error": True}),
    ("grant-share-of-a-small-grant", {"SEMIO_HOST_CAPACITY_PLAN": json.dumps({"jobs": 1, "memoryMegabytes": 3072})}, {"bytes": 1536 * MB}),
    ("grant-share-of-the-show-grant", {"SEMIO_HOST_CAPACITY_PLAN": json.dumps({"jobs": 1, "memoryMegabytes": 5120})}, {"bytes": 2560 * MB}),
    ("grant-share-never-below-the-minimum", {"SEMIO_HOST_CAPACITY_PLAN": json.dumps({"memoryMegabytes": 1024})}, {"bytes": 1024 * MB}),
    ("grant-share-never-above-the-default", {"SEMIO_HOST_CAPACITY_PLAN": json.dumps({"memoryMegabytes": 100000})}, {"bytes": 3072 * MB}),
    ("grant-without-memory-is-the-default", {"SEMIO_HOST_CAPACITY_PLAN": json.dumps({"jobs": 3})}, {"bytes": 3072 * MB}),
    ("grant-not-json-is-the-default", {"SEMIO_HOST_CAPACITY_PLAN": "{"}, {"bytes": 3072 * MB}),
    ("explicit-beats-the-grant", {"SEMIO_GRAPH_MEMORY_MB": "4096", "SEMIO_HOST_CAPACITY_PLAN": json.dumps({"memoryMegabytes": 2048})}, {"bytes": 4096 * MB}),
]
for name, environment, expected in budgets:
    assert oracle.budget(policy, environment) == expected, name
languages = [
    ("explicit-language-wins", {"SEMIO_LANGUAGE": "de", "SEMIO_DASHBOARD_LOCALE": "en", "LANG": "en_US.UTF-8"}, ["de"]),
    ("dashboard-locale-before-system-locale", {"SEMIO_DASHBOARD_LOCALE": "de", "LANG": "en_US.UTF-8"}, ["de"]),
    ("system-locale-german", {"LANG": "de_DE.UTF-8"}, ["de"]),
    ("system-locale-english", {"LANG": "en_GB.UTF-8"}, ["en"]),
    ("unsupported-locale-falls-through", {"LC_ALL": "fr_FR.UTF-8", "LANG": "de_AT"}, ["de"]),
    ("lc-all-beats-lang", {"LC_ALL": "en_GB", "LANG": "de_DE"}, ["en"]),
    ("nothing-set-prints-both-languages", {}, ["en", "de"]),
    ("unsupported-only-prints-both-languages", {"LANG": "fr_FR"}, ["en", "de"]),
]
for name, environment, expected in languages:
    assert oracle.languages(policy, environment) == expected, name
rendered = [(key, {"used": 1234, "budget": 3072, "running": 5}, environment) for key in ("waiting", "serial", "resumed") for environment in ({"LANG": "en_US"}, {"LANG": "de_DE"}, {})]
governor = [
    {"name": "unconstrained", "workers": 3, "budgetMegabytes": 3000, "baseMegabytes": 100, "garbageShare": 0.5, "expectParallel": 3, "expectWaits": False, "tasks": [{"memoryMegabytes": 100, "durationMilliseconds": 1000}] * 6},
    {"name": "waits-for-memory", "workers": 4, "budgetMegabytes": 500, "baseMegabytes": 100, "garbageShare": 0.5, "expectWaits": True, "tasks": [{"memoryMegabytes": 200, "durationMilliseconds": 600}] * 8},
    {"name": "collection-frees-garbage", "workers": 4, "budgetMegabytes": 400, "baseMegabytes": 100, "garbageShare": 1.0, "tasks": [{"memoryMegabytes": 50, "durationMilliseconds": 300}] * 12},
    {"name": "retained-memory-runs-one-project-at-a-time", "workers": 4, "budgetMegabytes": 300, "baseMegabytes": 400, "garbageShare": 0.0, "expectSerial": True, "expectWaits": True, "tasks": [{"memoryMegabytes": 10, "durationMilliseconds": 200}] * 5},
    {"name": "abort-rejects-waiting-projects", "workers": 2, "budgetMegabytes": 300, "baseMegabytes": 100, "garbageShare": 0.0, "abortAtMilliseconds": 1000, "tasks": [{"memoryMegabytes": 150, "durationMilliseconds": 5000}] * 6},
    {"name": "arrival-order-under-pressure", "workers": 3, "budgetMegabytes": 250, "baseMegabytes": 100, "garbageShare": 0.0, "expectWaits": True, "tasks": [{"memoryMegabytes": m, "durationMilliseconds": d} for m, d in [(120, 400), (30, 900), (200, 300), (10, 100), (90, 700), (60, 250), (140, 500)]]},
    {"name": "progress-lines-are-spaced", "workers": 3, "budgetMegabytes": 300, "baseMegabytes": 100, "garbageShare": 0.0, "expectWaits": True, "language": "de", "tasks": [{"memoryMegabytes": 150, "durationMilliseconds": 20000}] * 4},
    {"name": "unresolved-language-prints-both", "workers": 3, "budgetMegabytes": 300, "baseMegabytes": 100, "garbageShare": 0.0, "expectWaits": True, "language": "none", "tasks": [{"memoryMegabytes": 150, "durationMilliseconds": 8000}] * 4},
]
fixture = {
    "version": 1, "policy": policy, "messages": messages,
    "admission": admission,
    "budgets": [{"name": name, "environment": environment, "expect": expected} for name, environment, expected in budgets],
    "languages": [{"name": name, "environment": environment, "expect": expected} for name, environment, expected in languages],
    "messages_cases": [],
    "governor": governor,
}
fixture["messages"] = messages
fixture["messageCases"] = [{"key": key, "fields": fields, "environment": environment, "expect": oracle.render(policy, messages, key, fields, environment)} for key, fields, environment in rendered]
del fixture["messages_cases"]
(NEW / "tests/fixture.json").write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf8")
print("fixture", len(admission), "admission states,", len(governor), "scenarios")
