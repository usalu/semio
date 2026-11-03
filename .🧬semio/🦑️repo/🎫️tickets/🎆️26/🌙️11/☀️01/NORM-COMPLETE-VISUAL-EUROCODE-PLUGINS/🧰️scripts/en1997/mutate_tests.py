import sys, json
sys.path.insert(0, ".")
from spec import *
from rows import *
T = S + "/🧪️tests/🌍️mutate-en1997-1"
NK = newkinds()
INS = [(r, v) for r, v in NK if v == "insert"]

# rust
p = T + "/🦀️.rs"
s = open(p, encoding="utf8").read()
if "insert-anchor" not in s:
    s = s.replace('    "remove-pile",\n];', '    "remove-pile",\n' + "".join(f'    "{kind(r, v)}",\n' for r, v in NK) + "];", 1)
    s = s.replace('    "remove-pile-middle",\n];', '    "remove-pile-middle",\n' + "".join(f'    "{kind(r, v)}-dupe",\n' for r, v in INS) + "];", 1)
    def arm(key, d, fx):
        base = f"../🧫️fixtures/🧬️mutations/{d}/{fx}"
        return f'        "{key}" => (\n            include_str!("{base}/🔣️.before.json"),\n            include_str!("{base}/🔣️.mutation.json"),\n            include_str!("{base}/🔣️.after.json"),\n            include_str!("{base}/🔣️.outcome.json"),\n        ),\n'
    arms = ""
    for r, v in NK:
        arms += arm(kind(r, v), dirname(r, v), "✅apply")
        if v == "insert": arms += arm(kind(r, v) + "-dupe", dirname(r, v), "⛔dupe")
    s = s.replace('        other => panic!("mutate-en1997-1: no committed fixture', arms + '        other => panic!("mutate-en1997-1: no committed fixture', 1)
    s = s.replace("current 20-kind", "current 47-kind")
    open(p, "w", encoding="utf8").write(s)

# python
p = T + "/🐍️.py"
s = open(p, encoding="utf8").read()
if "insert-anchor" not in s:
    s = s.replace('    "remove-pile",\n]', '    "remove-pile",\n' + "".join(f'    "{kind(r, v)}",\n' for r, v in NK) + "]", 1)
    vec = ""
    for r, v in NK:
        vec += f'    "{kind(r, v)}": ("{dirname(r, v)}", "✅apply"),\n'
        if v == "insert": vec += f'    "{kind(r, v)}-dupe": ("{kind(r, v)}", "{dirname(r, v)}", "⛔dupe"),\n'
    s = s.replace('    "remove-pile-middle": ("remove-pile", "📤remove-pile", "🧭️middle"),\n', '    "remove-pile-middle": ("remove-pile", "📤remove-pile", "🧭️middle"),\n' + vec, 1)
    open(p, "w", encoding="utf8").write(s)

# feature
p = T + "/🥒️.feature"
s = open(p, encoding="utf8").read()
if "insert-anchor" not in s:
    def row(i, d, fx): return f"      | {i:<31} | {d:<33} | {fx:<7} |\n"
    apply_rows = ""; inv_rows = ""
    for r, v in NK:
        apply_rows += row(kind(r, v), dirname(r, v), "✅apply")
        inv_rows += row(kind(r, v), dirname(r, v), "✅apply")
        if v == "insert": apply_rows += row(kind(r, v) + "-dupe", dirname(r, v), "⛔dupe")
    a = "      | remove-pile-middle              | 📤remove-pile                      | 🧭️middle |\n"
    assert a in s
    s = s.replace(a, a + apply_rows, 1)
    b = "      | remove-pile                     | 📤remove-pile                      | ✅apply  |\n"
    i = s.rindex(b)
    s = s[:i + len(b)] + inv_rows + s[i + len(b):]
    s = s.replace("The 20 `✅apply` vectors\n  cover every kind of the current vocabulary (14 `change`, 3 `insert`, 3 `remove`)", "The 47 `✅apply` vectors\n  cover every kind of the current vocabulary (14 `change`, 11 `insert`, 11 `remove`, 11 `update`)")
    open(p, "w", encoding="utf8").write(s)

# oracle json
p = S + "/🔮️oracles/🔣️.json"
d = json.load(open(p, encoding="utf8"))
def walk(o):
    if isinstance(o, dict):
        yield o
        for v in o.values(): yield from walk(v)
    elif isinstance(o, list):
        for v in o: yield from walk(v)
done = False
for o in walk(d):
    if o.get("kinds") and "remove-pile" in o["kinds"] and "insert-anchor" not in o["kinds"]:
        o["kinds"] += [kind(r, v) for r, v in NK]
        done = True
    if "mutationId" in o and o["mutationId"] == "remove-pile" and False:
        pass
def find_list_with(key_val):
    for o in walk(d):
        for k, v in o.items():
            if isinstance(v, list) and v and isinstance(v[0], dict) and v[0].get(key_val[0]) is not None:
                if any(x.get(key_val[0]) == key_val[1] for x in v): return o, k, v
scen = find_list_with(("mutationId", "remove-pile"))
if scen and not any(x["mutationId"] == "insert-anchor" for x in scen[2]):
    for r, v in NK:
        sc = [{"id": "apply", "directoryName": "✅apply"}]
        if v == "insert": sc.append({"id": kind(r, v) + "-dupe", "directoryName": "⛔dupe"})
        scen[2].append({"mutationId": kind(r, v), "sourceMutationDirectoryName": dirname(r, v), "mutationDirectoryName": dirname(r, v), "scenarios": sc})
cat = find_list_with(("id", "remove-pile"))
if cat and not any(x["id"] == "insert-anchor" for x in cat[2]):
    for r, v in NK:
        cat[2].append({"id": kind(r, v), "capability": "en1997-1-mutate", "payloadSchema": "🧬️.schema.json", "outcomes": ["applied", "no-op", "rejected"], "productionDispatch": {"operation": kind(r, v), "bridgeVersion": 1, "variant": struct(r, v)}, "oracleRequirements": [{"capability": "en1997-1-mutate", "qualifyingKind": "verified-native-second-implementation"}]})
open(p, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
print("done", done)
