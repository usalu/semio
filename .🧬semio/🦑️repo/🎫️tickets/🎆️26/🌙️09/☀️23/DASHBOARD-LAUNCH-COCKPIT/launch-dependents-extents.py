#!/usr/bin/env python3
import json
import re
import sys

R = "/Users/ueli/Documents/semio/"
G = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/🗑️generated/launch-dependents/"
ANCHOR = re.compile(r"launch\.seed|launch\.json|launchSeed|launchName|launchCommand|launchOrder|launchGroup|launchPath|launchCatalogs|node-terminal|devLaunchers")


def statement_extent(lines, start):
    depth = 0
    seen = False
    quote = None
    for idx in range(start, min(len(lines), start + 80)):
        line = lines[idx]
        i = 0
        while i < len(line):
            c = line[i]
            if quote:
                if c == "\\":
                    i += 2
                    continue
                if c == quote:
                    quote = None
            else:
                if c in "\"'`":
                    quote = c
                elif c == "/" and line[i:i + 2] == "//":
                    break
                elif c in "([{":
                    depth += 1
                    seen = True
                elif c in ")]}":
                    depth -= 1
            i += 1
        if quote in ("\"", "'"):
            quote = None
        if seen and depth <= 0:
            return idx + 1
    return start + 1


def main():
    scan = json.load(open(G + "scan.json", encoding="utf-8"))["results"]
    out = []
    for f in sorted(scan):
        if f.startswith(".vscode/") or f.startswith("📜️script.ts"):
            continue
        if not f.endswith((".ts", ".tsx", ".rs", ".py", ".go")):
            continue
        lines = open(R + f, encoding="utf-8", errors="replace").read().split("\n")
        hits = [i for i, l in enumerate(lines) if ANCHOR.search(l[:3000])]
        if not hits:
            continue
        clusters = []
        for h in hits:
            if clusters and h <= clusters[-1][1] + 6:
                clusters[-1][1] = max(clusters[-1][1], h)
            else:
                clusters.append([h, h])
        out.append((f, clusters, lines))
    res = {}
    for f, clusters, lines in out:
        rows = []
        for a, b in clusters:
            # walk upward to the statement start: nearest line above with lower indent that starts a for/const/it/test/describe
            start = a
            ind = len(lines[a]) - len(lines[a].lstrip())
            for k in range(a, max(-1, a - 12), -1):
                s = lines[k].lstrip()
                kind = re.match(r"(for\s*\(|const |let |it\(|test\(|describe\(|expect\(|if \(|\}\s*else|function |export )", s)
                kind_ind = len(lines[k]) - len(s)
                if kind and kind_ind <= ind:
                    start = k
                    if s.startswith(("for", "const", "let", "expect", "if")):
                        break
            end = max(statement_extent(lines, start), b + 1)
            rows.append({"from": start + 1, "to": end, "head": lines[start].strip()[:170]})
        res[f] = rows
    json.dump(res, open(G + "extents.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(len(res))


main()
