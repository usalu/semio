"""usage: final-table.py <census-dir>... > table-route-g.md ; merges the census summaries (later dirs override), the route table, the causes and the audit's unreached-before list."""
import json, os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import causes
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G = C + "/🗑️generated/route-g"
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
table = json.load(open(B + "🗂️suites/🔣️.json", encoding="utf8"))
files = [l.strip() for l in open(G + "/build-files.txt", encoding="utf8")]
unreached = set(l.strip() for l in open(G + "/unreached-before.txt", encoding="utf8"))
suite_of = {}
for f in files:
    best = (-1, "?")
    for s in table["suites"]:
        for t in s.get("trees", []) + s.get("files", []):
            if f == t or f.startswith(t.rstrip("/") + "/"):
                if len(t) > best[0]: best = (len(t), s["id"])
    suite_of[f] = best[1]
rows = {}
for d in sys.argv[1:]:
    p = os.path.join(d, "summary.txt")
    if not os.path.exists(p): continue
    for l in open(p, encoding="utf8"):
        parts = l.rstrip("\n").split("\t")
        if len(parts) < 5: continue
        f, rc, cnt, t, log = parts[:5]; rss = parts[5].replace("rss=", "") if len(parts) > 5 else ""
        first = ""
        with open(os.path.join(d, log.split("=")[1]), encoding="utf8", errors="replace") as h:
            for x in h:
                if x.startswith("(fail)"): first = x[7:100].strip(); break
        rows[f] = (rc.replace("rc=", ""), cnt, t, rss, first)
bysuite = collections.defaultdict(lambda: [0, 0, 0])
for f, s in suite_of.items():
    r = rows.get(f)
    bysuite[s][0] += 1
    if r is None: bysuite[s][2] += 1
    elif r[0] == "0": bysuite[s][1] += 1
budget = {s["id"]: s["budgetMs"] for s in table["suites"]}
green = sum(1 for f in files if f in rows and rows[f][0] == "0")
print("# table-route-g: build library tests, closed route table and census\n")
print(f"Route table `{B}🗂️suites/🔣️.json` ({len(table['suites'])} suites, {len(files)} test files, each routed exactly once; law + Python oracle `🗂️suites/🧪️tests`). Census: official-default capability mode, one file per process, graph-free, 240 s wall, 3 GB RSS ceiling, tools `C/route-g-tools/census-run.ts`. `before` = reached by no official route before this wave (audit-build-infra-tests-1026g). Verdicts: 0 = green, 1 = red, WALL = timeout, 75 = declared restart after a mid-run edit, `-` = created after the census.\n")
print(f"**{green} green / {sum(1 for f in files if f in rows and rows[f][0] not in ('0',))} not green / {sum(1 for f in files if f not in rows)} not run, of {len(files)} files.**\n")
print("## Suites\n\n| suite | budget | files | green | not run |\n|---|---:|---:|---:|---:|")
for s in table["suites"]:
    a = bysuite[s["id"]]; print(f"| `{s['id']}` | {budget[s['id']]//1000} s | {a[0]} | {a[1]} | {a[2]} |")
print("\n## Red files by cause\n")
cls = collections.defaultdict(list)
for f in files:
    r = rows.get(f)
    if r and r[0] != "0": cls[causes.classify(f)[0]].append(f)
print("| class | files |\n|---|---:|")
for k, v in sorted(cls.items(), key=lambda kv: -len(kv[1])): print(f"| {k} | {len(v)} |")
print("\n| file | suite | verdict | counts | wall | peak RSS | class | owner | cause / first failing test |\n|---|---|---|---|---|---|---|---|---|")
for f in sorted(files, key=lambda f: (causes.classify(f)[0] if f in rows and rows[f][0] != "0" else "~", suite_of[f], f)):
    r = rows.get(f)
    if r is None: print(f"| `{f}` | {suite_of[f]} | - | | | | | | created after the census |"); continue
    if r[0] == "0": continue
    c = causes.classify(f)
    print(f"| `{f}` | {suite_of[f]} | {r[0]} | {r[1][:28]} | {r[2]} | {r[3]} | {c[0]} | {c[1]} | {c[2].replace('|', '/')}{' — first red: ' + r[4].replace('|', '/') if r[4] else ''} |")
print("\n## Green files (all suites)\n\n| file | suite | counts | wall | peak RSS | before |\n|---|---|---|---|---|---|")
for f in sorted(files, key=lambda f: (suite_of[f], f)):
    r = rows.get(f)
    if r and r[0] == "0": print(f"| `{f}` | {suite_of[f]} | {r[1][:28]} | {r[2]} | {r[3]} | {'unreached' if f in unreached else 'reached'} |")
