#!/usr/bin/env python3
"""Summarizes a Bazel --execution_log_json_file: one row per spawn, labelled by task id.

usage: exec-summary.py <log.json> <graph.json>   prints  task  mnemonic  runner  cacheHit  exit"""
import hashlib, json, sys


def load(path):
    text = open(path, encoding="utf-8", errors="surrogateescape").read()
    dec, i, rows = json.JSONDecoder(), 0, []
    while True:
        while i < len(text) and text[i].isspace():
            i += 1
        if i >= len(text):
            break
        obj, i = dec.raw_decode(text, i)
        rows.append(obj)
    return rows


def main():
    log, graph = sys.argv[1], sys.argv[2]
    g = json.load(open(graph))
    names = {}
    for p in g["projects"].values():
        for t in p["targets"]:
            ident = p["name"] + ":" + t
            names["t_" + hashlib.sha256(ident.encode()).hexdigest()[:16]] = ident
    rows = load(log)
    counts = {}
    for r in rows:
        label = (r.get("targetLabel") or "?").removeprefix("//:")
        task = names.get(label, label)
        runner = r.get("runner", "?")
        hit = r.get("cacheHit", False)
        print(f'{task}\t{r.get("mnemonic")}\trunner={runner}\tcacheHit={hit}\texit={r.get("exitCode", 0)}')
        counts[runner] = counts.get(runner, 0) + 1
    print("spawns:", json.dumps(counts, sort_keys=True), "total", len(rows))


main()
