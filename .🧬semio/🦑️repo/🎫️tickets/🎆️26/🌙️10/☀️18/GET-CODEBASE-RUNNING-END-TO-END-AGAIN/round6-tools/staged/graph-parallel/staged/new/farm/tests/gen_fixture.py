#!/usr/bin/env python3
"""Writes the farm law fixture: the declared policy and texts, the inputs of every table, and the answers of the independent model."""
import json, pathlib, sys
HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import oracle

schema = json.loads((HERE.parent / "schema.json").read_text(encoding="utf8"))
policy, messages = schema["$defs"]["Policy"]["const"], schema["$defs"]["Messages"]["const"]
host = lambda cores, total, **more: {"cores": cores, "totalMegabytes": total, **more}
plans = [
    ("workstation: memory binds", {"projects": 3738, "host": host(10, 32768), "parentMegabytes": 2500}),
    ("workstation: cpu binds", {"projects": 3738, "host": host(6, 65536), "parentMegabytes": 2500}),
    ("small host: single", {"projects": 3738, "host": host(2, 8192), "parentMegabytes": 2000}),
    ("limit binds", {"projects": 100000, "host": host(64, 262144), "parentMegabytes": 4000}),
    ("work binds", {"projects": 30, "host": host(16, 65536), "parentMegabytes": 1000}),
    ("below the minimum", {"projects": 12, "host": host(16, 65536)}),
    ("parent larger than its reserve", {"projects": 3738, "host": host(16, 32768), "parentMegabytes": 6000}),
    ("available memory binds", {"projects": 3738, "host": host(16, 65536, availableMegabytes=9000), "parentMegabytes": 2500}),
    ("switched off", {"projects": 3738, "host": host(16, 65536), "environment": {"SEMIO_GRAPH_WORKERS": "0"}}),
    ("switched off by word", {"projects": 3738, "host": host(16, 65536), "environment": {"SEMIO_GRAPH_WORKERS": " Off "}}),
    ("forced", {"projects": 5, "host": host(2, 4096), "environment": {"SEMIO_GRAPH_WORKERS": "3"}}),
    ("forced above the limit", {"projects": 3738, "host": host(16, 65536), "environment": {"SEMIO_GRAPH_WORKERS": "13"}}),
    ("forced garbage", {"projects": 3738, "host": host(16, 65536), "environment": {"SEMIO_GRAPH_WORKERS": "many"}}),
    ("worker memory named", {"projects": 3738, "host": host(10, 32768), "parentMegabytes": 2500, "environment": {"SEMIO_GRAPH_WORKER_MB": "768"}}),
    ("worker memory below the minimum", {"projects": 3738, "host": host(10, 32768), "environment": {"SEMIO_GRAPH_WORKER_MB": "100"}}),
    ("worker memory garbage", {"projects": 3738, "host": host(10, 32768), "environment": {"SEMIO_GRAPH_WORKER_MB": "lots"}}),
    ("auto", {"projects": 3738, "host": host(10, 32768), "environment": {"SEMIO_GRAPH_WORKERS": "auto"}, "parentMegabytes": 2500}),
]
languages = [
    ("nothing named prints both", {}), ("english named", {"SEMIO_LANGUAGE": "en"}), ("german from locale", {"LANG": "de_DE.UTF-8"}),
    ("first variable wins", {"SEMIO_LANGUAGE": "de", "LANG": "en_US.UTF-8"}), ("unsupported only prints both", {"LANG": "fr_FR.UTF-8"}), ("unsupported then supported", {"SEMIO_LANGUAGE": "fr", "LC_ALL": "de"}),
]
messages_cases = [
    ("starting both", "starting", {"projects": 3738, "workers": 8, "megabytes": 1536}, {}),
    ("fallback german", "fallback", {"count": 3, "reason": "exited"}, {"LANG": "de_AT"}),
    ("recycled english", "recycled", {"worker": 2, "megabytes": 1700}, {"SEMIO_LANGUAGE": "en"}),
    ("finished both", "finished", {"projects": 3738, "workers": 8, "seconds": 600, "jobs": 3738, "fallbacks": 0, "recycled": 1}, {}),
    ("serial english", "serial", {"projects": 20, "reason": "single"}, {"LC_MESSAGES": "en_GB"}),
]
fixture = {
    "version": 1, "policy": policy, "messages": messages,
    "plan": [{"name": n, "input": i} for n, i in plans],
    "languages": [{"name": n, "environment": e} for n, e in languages],
    "messageCases": [{"name": n, "key": k, "fields": f, "environment": e} for n, k, f, e in messages_cases],
    "runtime": [{"name": "bun 1536", "runtime": "bun", "megabytes": 1536}, {"name": "node 1024", "runtime": "node", "megabytes": 1024}, {"name": "unknown runtime", "runtime": "deno", "megabytes": 1024}],
    "tables": [
        {"name": "repeats within and across batches", "batches": [["a", "b", "a"], ["b", "c"], ["a", "c", "d", "d"]]},
        {"name": "empty batches", "batches": [[], ["x"], [], ["x", "y"]]},
        {"name": "one long batch", "batches": [[f"row-{i % 7}" for i in range(40)]]},
    ],
}
fixture["expect"] = oracle.expect(fixture)
(HERE / "🔣️.json").write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf8")
print("plan", len(plans), "tables", len(fixture["tables"]))
