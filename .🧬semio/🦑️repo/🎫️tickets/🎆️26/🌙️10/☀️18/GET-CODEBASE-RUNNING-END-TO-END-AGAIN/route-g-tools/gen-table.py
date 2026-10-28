"""Generates the build route table from suites-def.py + the census overrides (budgets, capabilities, launchers). Usage: gen-table.py [--write]"""
import json, os, sys
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
exec(open(C + "/route-g-tools/suites-def.py", encoding="utf8").read())
overrides = json.load(open(C + "/route-g-tools/suite-overrides.json", encoding="utf8")) if os.path.exists(C + "/route-g-tools/suite-overrides.json") else {}
fixture = json.load(open(B + "/🗂️suites/🧪️tests/🧫️fixtures/🔣️.json", encoding="utf8")) if os.path.exists(B + "/🗂️suites/🧪️tests/🧫️fixtures/🔣️.json") else {"levels": {"fundamental": 15000, "quick": 300000}, "spawnTokens": ["spawn","execFile","execSync","captureOwned","runOwned","Bun.$","child_process","runRepositoryCommand"]}
files = [l.strip() for l in open(C + "/🗑️generated/route-g/build-files.txt", encoding="utf8")]
owner = {}
for f in files:
    best = (-1, None)
    for sid, trees in S.items():
        for t in trees:
            if f == t or f.startswith(t.rstrip("/") + "/"):
                if len(t) > best[0]: best = (len(t), sid)
    if best[1] is None: raise SystemExit("unrouted " + f)
    owner[f] = best[1]
suites = []
for sid, trees in S.items():
    mine = [f for f, o in owner.items() if o == sid]
    spawning = any(any(tok in open(os.path.join(B, f), encoding="utf8").read() for tok in fixture["spawnTokens"]) for f in mine)
    row = {"id": sid, "budgetMs": fixture["levels"]["quick" if spawning else "fundamental"], "typecheck": "none"}
    ft = [t for t in trees if t.endswith(".ts")]; dt = [t for t in trees if not t.endswith(".ts")]
    if dt: row["trees"] = dt
    if ft: row["files"] = ft
    row.update(overrides.get(sid, {}))
    suites.append(row)
doc = {"version": 1, "suites": suites, "codeSuites": [], "helpers": []}
text = json.dumps(doc, ensure_ascii=False, indent=2) + "\n"
if "--write" in sys.argv: open(B + "/🗂️suites/🔣️.json", "w", encoding="utf8").write(text)
import collections
print(len(suites), "suites;", collections.Counter(s["budgetMs"] for s in suites))
