#!/usr/bin/env python3
"""⏲️ Rebuilds batch `clock` (lane cbudget) from the CURRENT live files: new files come from src/, the three modified files (execution runner, derived schema index, build guide) are re-edited at anchors and re-hashed, base/ gets the live bytes, manifest.json the hashes.
usage: python3 -I rebuild.py [--root DIR]   (default root: walk up to the directory holding 🧰️framework)
Refuses (exit 2) when an anchor no longer matches exactly once, so a peer's edit of the same lines is never overwritten silently."""
import hashlib, json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
PATHS = json.load(open(os.path.join(HERE, "paths.json"), encoding="utf8"))
def sha(data): return hashlib.sha256(data).hexdigest()
def once(text, old, new):
    if text.count(old) != 1: print("ANCHOR", repr(old[:70]), text.count(old)); sys.exit(2)
    return text.replace(old, new)
def execution(text):
    text = once(text, 'import { bunTestCeilingV1 } from "../🟨️bun/🧯️ceiling/🟦️.ts";\n', 'import { describeTestBudgetV1, sharedHostContentionV1, watchTestBudgetV1, type ContentionSourceV1, type TestBudgetSnapshotV1 } from "../🎚️budget/⏲️contention/🟦️.ts";\nimport { bunTestCeilingV1 } from "../🟨️bun/🧯️ceiling/🟦️.ts";\n')
    text = once(text, '  budgetMs: number;\n', '  budgetMs: number;\n  contention?: ContentionSourceV1;\n')
    text = once(text, '/** ⏱️ Executes one explicitly bounded test process without selecting a tool or repository policy. */', "/** ⏱️ Executes one explicitly bounded test process without selecting a tool or repository policy; the budget is work, not wall time: it counts at the speed the host gives (`contention`, the load of this host by default), so a slow shared host does not stop a run that fits its budget on an idle one, while a hung process is still stopped after at most the policy's stretch times its budget. */")
    text = once(text, '  const timer = options.budgetMs > 0 ? setTimeout(() => stop("timeout"), options.budgetMs) : undefined;\n', '  let expired: TestBudgetSnapshotV1 | undefined;\n  const watch = options.budgetMs > 0 ? watchTestBudgetV1(options.budgetMs, { source: options.contention ?? sharedHostContentionV1(), report: console.error, expire: snapshot => { expired = snapshot; stop("timeout"); } }) : undefined;\n')
    text = once(text, '    if (reason === "timeout") console.error(`[budget] ${command} ${args.join(" ")} exceeded ${options.budgetMs}ms — killed. ${options.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive)."}`);\n', '    if (reason === "timeout") console.error(`[budget] ${command} ${args.join(" ")} exceeded ${options.budgetMs}ms — killed. ${options.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive)."}${expired ? "\\n" + describeTestBudgetV1("expired", expired) : ""}`);\n')
    return once(text, '    if (timer) clearTimeout(timer);\n', '    watch?.stop();\n')
def index(text):
    marker = '      "https://json.schemas.assets.semio-tech.com/framework/process/testing/bun/operands.json"'
    new = '      "https://json.schemas.assets.semio-tech.com/framework/process/testing/budget/contention.json": "' + PATHS["new"]["02.json"][0] + '",\n'
    return once(text, marker, new + marker)
EN = "   - **Test budgets** are work, not wall time (`🧪️testing/🎚️budget/⏲️contention`, run by `🧪️testing/🎛️execution`): every wall second is worth 1/stretch seconds of the budget (`quick` 300 s, `long` 900 s, `exhaustive` 1800 s), the stretch being the one-minute load average per core clamped to 1..8 (Windows keeps no load average: a smoothed measurement of the wall time a thread needs per CPU millisecond replaces it). A run that fits its budget on an idle host is never stopped on a busy one (measured 10-27: the course suite needed 50-260 s of wall time at load 40-70 and the old wall clock cut it at 300 s); a hung test is still stopped after at most 8 times its budget; the run says `[budget] Busy host: load …` in English and German while it counts slower than the wall, and a stop names the wall time, the counted time and the stretch. The policy is `🎚️budget/⏲️contention/🔣️.json` with its schema beside it.\n"
DE = "   - **Testbudgets** sind Arbeit, keine Wanduhrzeit (`🧪️testing/🎚️budget/⏲️contention`, ausgeführt von `🧪️testing/🎛️execution`): jede Wandsekunde zählt 1/Dehnung Sekunden des Budgets (`quick` 300 s, `long` 900 s, `exhaustive` 1800 s); die Dehnung ist der Ein-Minuten-Lastmittelwert je Kern, begrenzt auf 1..8 (Windows führt keinen Lastmittelwert: eine geglättete Messung der Wandzeit, die ein Thread je CPU-Millisekunde braucht, ersetzt ihn). Ein Lauf, der sein Budget auf einem freien Rechner einhält, wird auf einem ausgelasteten nie angehalten (gemessen 27.10.: die Kurs-Suite brauchte bei Last 40-70 zwischen 50 und 260 s Wandzeit, die alte Wanduhr schnitt sie bei 300 s ab); ein hängender Test wird weiterhin nach höchstens dem Achtfachen seines Budgets angehalten; der Lauf meldet `[budget] Ausgelasteter Rechner: Last …` auf Englisch und Deutsch, solange er langsamer als die Wanduhr zählt, und ein Halt nennt Wandzeit, gezählte Zeit und Dehnung. Die Richtlinie ist `🎚️budget/⏲️contention/🔣️.json` mit ihrem Schema daneben.\n"
def guide(text):
    lines = text.split("\n"); out = []; done = [False, False]
    for line in lines:
        out.append(line)
        if line.startswith("   - **Test runners** start the workers"): out.append(EN.rstrip("\n")); done[0] = True
        if line.startswith("   - **Testläufer** starten so viele Worker"): out.append(DE.rstrip("\n")); done[1] = True
    if done != [True, True]: print("ANCHOR guide", done); sys.exit(2)
    return "\n".join(out)
EDITS = {"06.ts": execution, "07.json": index, "08.md": guide}
rows = {}
os.makedirs(os.path.join(HERE, "files"), exist_ok=True); os.makedirs(os.path.join(HERE, "base"), exist_ok=True)
for name, (rel, src) in PATHS["new"].items():
    data = open(os.path.join(HERE, src), "rb").read()
    live = os.path.join(ROOT, rel)
    if os.path.exists(live) and open(live, "rb").read() != data: print("NEW FILE EXISTS WITH OTHER BYTES", rel[-60:]); sys.exit(2)
    open(os.path.join(HERE, "files", name), "wb").write(data)
    rows[rel] = {"file": name, "base": None, "new": sha(data)}
for name, rel in PATHS["modified"].items():
    base = open(os.path.join(ROOT, rel), "rb").read()
    text = base.decode("utf8")
    if name == "06.ts" and "⏲️contention" in text: print("already edited", rel[-50:]); sys.exit(2)
    new = EDITS[name](text).encode("utf8")
    open(os.path.join(HERE, "base", name), "wb").write(base); open(os.path.join(HERE, "files", name), "wb").write(new)
    rows[rel] = {"file": name, "base": sha(base), "new": sha(new)}
json.dump(rows, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("rebuilt", len(rows), "files against", ROOT)
