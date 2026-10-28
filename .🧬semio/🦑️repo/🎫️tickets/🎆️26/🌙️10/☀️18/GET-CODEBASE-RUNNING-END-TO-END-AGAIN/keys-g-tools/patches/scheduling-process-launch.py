"""python3 scheduling-process-launch.py [--check] : the process-launch scheduling domain (coordinator request 09:10) and the scopes law repairs.
Domain `process-launch`: the build door with its restart, the invocation graph load and the kernel entry; each only decides which process starts, with which graph slice and under which capacity.
Fail closed (NOT scheduling, stay keyed): `📥️source/🔏️closure/🍞️bun/` (the loader transpiles and serves the code every action executes), `📥️source/🔏️closure/🚪️command/` (executes captured build source) and `🏃️process/🎛️owned-execution/` (its capture decides the exit status and output an action reports).
Repairs: the graph input storage (`📥️inputs/💾️storage/`: writer, ledger, staging) joins `store-custody` (it decides where graph bytes live, never what they are); two keyed imports of scheduling modules that no declared port covers get their ports; the 20 modules the runtime trace opened but the current closure no longer reaches become byte-only leaves of the executor scope (fail closed: more is keyed, never less) until a fresh trace replaces them.
Anchored, idempotent, all-or-nothing."""
import json, os, sys
ROOT = "/Users/ueli/Documents/semio/"
B = ROOT + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
SCHED = B + "📥️source/🔏️closure/🗓️scheduling/🔣️.json"
SCOPES = B + "🔧️tools/🏗️bazel/🎬️scopes/🔣️.json"
POLICY = B + "🔣️policy.json"
NX = ROOT + "nx.json"
HERE = os.path.dirname(os.path.abspath(__file__))
sched = json.load(open(SCHED, encoding="utf-8"))
scopes = json.load(open(SCOPES, encoding="utf-8"))
b = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
domains = {d["id"]: d for d in sched["domains"]}
def add(lst, item):
    if item not in lst:
        lst.append(item)
add(domains["retry-observation"]["ports"], "🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🌊️races/🟦️.ts")
add(domains["leases"]["ports"], "🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts")
add(domains["store-custody"]["prefixes"], b + "📥️inputs/💾️storage/")
if "process-launch" not in domains:
    sched["domains"].append({
        "id": "process-launch",
        "contract": {
            "en": "Launching a run: the build door with its restart, the graph slice an invocation loads and the kernel entry under the inherited grant. They decide which process starts, with which slice and grant, never what an action computes; the loader, captured-source execution and child output capture stay keyed.",
            "de": "Start eines Laufs: Build-Tuer samt Neustart, der vom Aufruf geladene Graph-Ausschnitt und der Kern-Eintritt unter der geerbten Zusage. Sie entscheiden, welcher Prozess mit welchem Ausschnitt startet, nie was eine Aktion berechnet; Lader, Ausfuehrung erfassten Quelltexts und Erfassung der Kindausgabe bleiben im Schluessel.",
        },
        "prefixes": [b + "🚪️door/", b + "🧭️invocation/🕸️graph/", b + "⌨️command/📥️entry/"],
        "ports": [b + "🚪️door/🟦️.ts", b + "🚪️door/🔄️restart/🟦️.ts", b + "🧭️invocation/🕸️graph/🟦️.ts", b + "⌨️command/📥️entry/🟦️.ts"],
    })
row = scopes["scopes"][0]
assert row["id"] == "executor-command"
add(row["scheduling"], "process-launch")
known = set(row["roots"]) | set(row["leaves"])
for path in json.load(open(os.path.join(HERE, "stale-trace-leaves.json"), encoding="utf-8")):
    if path not in known and os.path.isfile(ROOT + path):
        row["leaves"].append(path)
        known.add(path)
prefixes = sorted({p for d in sched["domains"] for p in d["prefixes"]}, key=lambda s: s.encode("utf-16-be"))
exclusions = ["!{workspaceRoot}/" + p + "**/*" for p in prefixes]
files = {SCHED: json.dumps(sched, ensure_ascii=False, indent=2) + "\n", SCOPES: json.dumps(scopes, ensure_ascii=False, indent=2) + "\n"}
for path in (POLICY, NX):
    doc = json.load(open(path, encoding="utf-8"))
    doc["namedInputs"]["schedulingSourceExclusions"] = exclusions
    files[path] = json.dumps(doc, ensure_ascii=False, indent=2) + "\n"
if "--check" in sys.argv:
    print("ok", len(exclusions), "exclusions,", len(row["leaves"]), "leaves")
    sys.exit(0)
for path, text in files.items():
    open(path, "w", encoding="utf-8").write(text)
print("applied", len(files), "files")
