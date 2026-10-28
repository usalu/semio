"""usage: table.py <census-out-dir>... -> markdown table (file, suite, verdict, counts, wall, rss, first failing test / error) from census summaries; later dirs override earlier ones per file. Streams every log (head only)."""
import json, os, sys
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
table = json.load(open(B + "🗂️suites/🔣️.json", encoding="utf8"))
files = [l.strip() for l in open(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/route-g/build-files.txt", encoding="utf8")]
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
        f, rc, cnt, t, log = parts[:5]; rss = parts[5] if len(parts) > 5 else ""
        first_fail = first_err = ""
        with open(os.path.join(d, log.split("=")[1]), encoding="utf8", errors="replace") as h:
            for x in h:
                if not first_fail and x.startswith("(fail)"): first_fail = x[7:110].strip()
                if not first_err and x.startswith(("error:", "SyntaxError", "TypeError", "ReferenceError")) and "admission closed" not in x: first_err = x[:150].strip()
                if first_fail and first_err: break
        rows[f] = (rc, cnt, t, rss.replace("rss=", ""), first_fail, first_err, d)
green = sum(1 for r in rows.values() if r[0] == "rc=0")
print("| file | suite | verdict | counts | wall | peak RSS | first failing test | first error |\n|---|---|---|---|---|---|---|---|")
for f, (rc, cnt, t, rss, ff, fe, d) in sorted(rows.items(), key=lambda kv: (kv[1][0] == "rc=0", suite_of.get(kv[0], "?"), kv[0])):
    print(f"| `{f}` | {suite_of.get(f, '?')} | {rc} | {cnt[:30]} | {t} | {rss} | {ff.replace('|', '/')} | {fe.replace('|', '/')} |")
print(f"\nfiles {len(rows)}, green {green}", file=sys.stderr)
