"""📑 Concise listing of the NEW diagnostics of an oracle result: code, message, label, module and file:line (shortened).
usage: rep.py <result.json> [cfg] [grep-substring]"""
import json, re, sys, collections
r = json.load(open(sys.argv[1], encoding="utf-8"))
want = sys.argv[2] if len(sys.argv) > 2 and sys.argv[2] != "-" else None
sub = sys.argv[3] if len(sys.argv) > 3 else None
for cfg, e in r["summary"].items():
    if want and cfg != want:
        continue
    print("==", cfg, "cur", e["cur_total"], "base", e.get("base_total"), "NEW", e.get("new"), "GONE", e.get("gone"))
    rows = collections.Counter()
    for d in e["new_list"]:
        m = re.search(r"__m_([A-Za-z0-9_]+)/(.*)$", d["file"])
        mod, rel = (m.group(1), m.group(2)) if m else ("", d["file"])
        rel = rel.replace("🧰️framework/🔨️modules/", "M/")
        line = f'{d["code"]:6} {mod[-18:]:18} {rel[-70:]}:{d["line"]} {d["msg"][:120]} | {d["label"][:60]}'
        if sub and sub not in line:
            continue
        rows[line] += 1
    for k, n in sorted(rows.items()):
        print(f"  {n:3} {k}")
