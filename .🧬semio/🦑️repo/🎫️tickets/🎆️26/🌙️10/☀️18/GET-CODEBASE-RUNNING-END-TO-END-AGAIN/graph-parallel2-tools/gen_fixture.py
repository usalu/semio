#!/usr/bin/env python3
"""🧪️ Writes the language-agnostic fixture of the project farm laws: plan, language and message inputs and a synthetic corpus of projects whose staged leaves share closures, blocks and receipts. Pure standard library; the expectations come from the independent oracle, never from this file."""
import hashlib, json, os, sys
OUT = sys.argv[1]
sha = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()
plan = []
for projects, env, cores in [
    (3741, {}, 10), (3741, {}, 2), (3741, {}, 3), (3741, {}, 64), (23, {}, 10), (24, {}, 10), (47, {}, 10), (12, {"SEMIO_GRAPH_WORKERS": "auto"}, 10),
    (3741, {"SEMIO_GRAPH_WORKERS": "0"}, 10), (3741, {"SEMIO_GRAPH_WORKERS": "off"}, 10), (3741, {"SEMIO_GRAPH_WORKERS": "False"}, 10), (5, {"SEMIO_GRAPH_WORKERS": "3"}, 1),
    (5, {"SEMIO_GRAPH_WORKERS": " 7 "}, 1), (5, {"SEMIO_GRAPH_WORKERS": "64"}, 1), (5, {"SEMIO_GRAPH_WORKERS": "65"}, 1), (5, {"SEMIO_GRAPH_WORKERS": "x"}, 1), (5, {"SEMIO_GRAPH_WORKERS": "-1"}, 1), (1000, {"SEMIO_GRAPH_WORKERS": ""}, 4),
]:
    plan.append({"projects": projects, "environment": env, "cores": cores})
languages = [{"environment": e} for e in ({}, {"LANG": "de_CH.UTF-8"}, {"LANG": "en_US.UTF-8"}, {"LANG": "fr_FR.UTF-8"}, {"LANG": "fr_FR.UTF-8", "LC_ALL": "de"}, {"SEMIO_LANGUAGE": "EN", "LANG": "de_DE"}, {"LC_MESSAGES": "de@euro"}, {"LC_ALL": "C", "LANG": "en-GB"})]
messages = [
    {"key": "waiting", "fields": {"workers": 6, "projects": 3741}}, {"key": "starting", "fields": {"workers": 4, "megabytes": 2048}},
    {"key": "serial", "fields": {"projects": 3741, "reason": "host capacity"}}, {"key": "refused", "fields": {"reason": "identity differs"}},
    {"key": "lost", "fields": {"reason": "exited"}}, {"key": "degraded", "fields": {"reason": "restarts: exited"}},
    {"key": "failed", "fields": {"project": "@semio-tech/os-command-rs", "reason": "boom"}}, {"key": "declined", "fields": {"project": "p"}},
    {"key": "recycled", "fields": {"worker": 2, "megabytes": 2101}},
    {"key": "finished", "fields": {"adopted": 3700, "projects": 3741, "workers": 6, "seconds": 412, "failed": 3, "lost": 1, "recycled": 5}},
]
runtime = [{"runtime": "bun", "megabytes": 2048}, {"runtime": "bun", "megabytes": 1000}, {"runtime": "node", "megabytes": 2048}, {"runtime": "deno", "megabytes": 2048}]
closures = [{"namespace": "build-input-closures", "text": '{"files":["a.ts","b.ts"]}'}, {"namespace": "build-input-closures", "text": '{"files":["a.ts","c.ts"]}'}, {"namespace": "build-lock-closures", "text": '{"lock":"alpha"}'}, {"namespace": "build-lock-closures", "text": '{"lock":"beta","crates":3}'}, {"namespace": "build-input-closures", "text": '{"files":[]}'}]
blocks = ['[{"kind":"file","path":"/w/shared/%d.ts","fact":{"size":%d}}]' % (i, i) for i in range(7)]
def hex24(text): return sha(text)[:24]
projects = []
for n in range(24):
    root = "crates/p%02d" % n
    projects.append({
        "root": root,
        "record": '{"name":"p%02d","value":%d}' % (n, n * n),
        "closures": sorted({n % 5, (n * 3 + 1) % 5}),
        "blocks": sorted({n % 7, (n * 2 + 1) % 7, (n + 3) % 7}),
        "receipts": [{"name": "rr/%s.json" % hex24("crate-%d" % ((n + k) % 11)), "text": '{"crate":"crate-%d"}' % ((n + k) % 11)} for k in range(3)],
        "packs": [{"name": "np/%s.json.records/%d-%d-%s.%s" % (hex24(root), n, k, sha(root + str(k))[:16], ext), "text": "pack %d %d %s" % (n, k, ext)} for k, ext in enumerate(("pack", "idx"))],
    })
jobs = [{"n": n, "delay": int(sha("delay%d" % n)[:2], 16) % 60, "value": (n * n + 7) % 1013} for n in range(48)]
doc = {"jobs": jobs, "version": 1, "plan": plan, "languages": languages, "messages": messages, "runtime": runtime, "corpus": {"closures": closures, "blocks": blocks, "projects": projects, "workers": [1, 2, 5], "seeds": [1, 7, 42]}}
with open(OUT, "w", encoding="utf-8") as handle:
    json.dump(doc, handle, ensure_ascii=False, indent=1)
    handle.write("\n")
print("fixture", OUT, len(projects), "projects")
