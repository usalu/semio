"""✂️ Splits the energy mutation Python adapter into the same domain folders the Rust vector tables use.

usage: pysplit.py [--apply]
Dry run prints module sizes, helper placement and import edges; --apply writes the children and the new root. The original
text is the snapshot under 🗑️generated/pfb-orig, so a re-run after a restore is reproducible.
"""
import ast, builtins, collections, os, re, sys

F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
CASE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1"
O = F + "/🗑️generated/pfb-orig/energy-test/"
APPLY = "--apply" in sys.argv
V = "️"
src = open(O + "🐍️.py", encoding="utf-8").read()
lines = src.split("\n")
rust = open(O + "🦀️.rs", encoding="utf-8").read().split("\n")
tree = ast.parse(src)

ALIAS = "mutate_energy_model_1_"
GROUPS = [
    ("building", "🏠" + V + "building", "Building kinds: model, site, run period, zones, spaces, surfaces and surface boundaries."),
    ("openings", "🪟" + V + "openings", "Opening kinds: fenestration, glazing, shading surfaces, shading devices and surface connections."),
    ("fabric", "🪨" + V + "fabric-and-gains", "Fabric and gain kinds: materials, finishes, constructions and the people, lighting and equipment gains."),
    ("airflow", "💨" + V + "airflow-and-controls", "Airflow and control kinds: infiltration, leakage, ventilation, thermostats, humidistats and ideal loads."),
    ("zone_systems", "🔧" + V + "zone-systems", "Zone system kinds: zone equipment, daylight, sizing, room air, setpoints, air loops, plant loops and outdoor air."),
    ("power", "🔌" + V + "power-and-plant", "Power and plant kinds: load centers, PV, batteries, hot water, solar thermal, refrigeration, water systems and faults."),
    ("schedules", "📆" + V + "schedules-and-layers", "Schedule and layer kinds: space lists, enclosures, schedules, time series, window layers and finishes."),
]
BASE = ("base", "🧱" + V + "base", "Shared vocabulary of the mutation kinds: the committed wire form, outcomes, artifact links and the helpers more than one group uses.")
FORCE = {"FIELD_LEAVES": "g0"}
CUTS = [0, 45, 70, 110, 154, 202, 252, 297]

order, id_kind = [], {}
for index, line in enumerate(rust):
    match = re.match(r'\s+id: "([a-z0-9-]+)",', line)
    if match:
        kind = re.match(r'\s+kind: "([a-z0-9-]+)",', rust[index + 1]).group(1)
        id_kind[match.group(1)] = kind
        if kind not in order:
            order.append(kind)
assert len(order) == 297
def group_of_kind(kind):
    i = order.index(kind)
    return max(g for g in range(len(GROUPS)) if CUTS[g] <= i)
snake = {k.replace("-", "_"): k for k in order}

regions = {}
stack = []
for number, line in enumerate(lines, 1):
    m = re.match(r"# region (.*)$", line)
    if m:
        stack.append((m.group(1).strip(), number))
    m = re.match(r"# endregion (.*)$", line)
    if m:
        name, start = stack.pop()
        regions[name] = (start, number)
def region_of(number):
    for name, (a, b) in regions.items():
        if a <= number <= b:
            return name
    return None

nodes = []
for node in tree.body:
    nodes.append(dict(node=node, start=node.lineno, end=node.end_lineno))
for entry in nodes:
    s = entry["start"]
    while s > 1 and lines[s - 2].startswith("#") and not re.match(r"#\s?(end)?region", lines[s - 2]):
        s -= 1
    entry["start"] = s
merged = []
for entry in nodes:
    n = entry["node"]
    if isinstance(n, ast.Expr) and isinstance(getattr(n, "value", None), ast.Constant) and isinstance(n.value.value, str) and merged and n.lineno > 1:
        merged[-1]["end"] = entry["end"]
        continue
    merged.append(entry)
nodes = merged

def names_of(n):
    if isinstance(n, (ast.FunctionDef, ast.ClassDef)):
        return [n.name]
    if isinstance(n, ast.Assign):
        return [ast.unparse(t) for t in n.targets]
    return []

for entry in nodes:
    entry["names"] = names_of(entry["node"])
    entry["region"] = region_of(entry["node"].lineno)
    entry["text"] = "\n".join(lines[entry["start"] - 1:entry["end"]])

defined = {}
for entry in nodes:
    for name in entry["names"]:
        defined[name] = entry

def refs(entry):
    found = set()
    for sub in ast.walk(entry["node"]):
        if isinstance(sub, ast.Name) and sub.id in defined:
            found.add(sub.id)
    return found - set(entry["names"])

STD = ["copy", "json", "math", "re"]
def std_used(texts):
    used = set()
    for text in texts:
        for sub in ast.walk(ast.parse(text)):
            if isinstance(sub, ast.Name) and sub.id in STD:
                used.add(sub.id)
    return [m for m in STD if m in used]

# --- ownership
for entry in nodes:
    entry["owner"] = None
    n = entry["node"]
    region = entry["region"]
    if region in ("🔖️Wire", "🔖️Outcomes", "🔖️Links"):
        entry["owner"] = "base"
    elif region in ("🔖️Imports",):
        entry["owner"] = "imports"
    elif region == "🔖️Fixtures":
        entry["owner"] = "fixtures"
    elif region == "🔖️Vocabulary":
        if isinstance(n, (ast.FunctionDef, ast.Assign)) and entry["names"] not in (["VOCABULARY"], ["EXTRA_INVERT"]):
            name = entry["names"][0]
            base = name[len("_invert_"):] if name.startswith("_invert_") else name
            if base in snake:
                entry["owner"] = "g%d" % group_of_kind(snake[base])
            else:
                entry["owner"] = "helper"
        elif entry["names"] in (["VOCABULARY"], ["EXTRA_INVERT"]):
            entry["owner"] = "dict"
        else:
            entry["owner"] = "helper"
    elif region in ("🔖️Inverse", "🔖️TextCarrier", "🔖️Oracle", "🔖️Registration"):
        entry["owner"] = "root"
    else:
        entry["owner"] = "docstring" if isinstance(n, ast.Expr) or isinstance(n, ast.ImportFrom) else "?"
unknown = [e for e in nodes if e["owner"] in (None, "?")]
assert not unknown, [(e["names"], e["start"]) for e in unknown]

# --- helper placement
helpers = [e for e in nodes if e["owner"] == "helper"]
for e in helpers:
    e["refs"] = refs(e)
def users(name):
    out = set()
    for e in nodes:
        if name in refs(e) if "refs" not in e else name in e["refs"]:
            out.add(e)
    return out
placement = {}
changed = True
for e in nodes:
    e.setdefault("refs", refs(e))
while changed:
    changed = False
    for h in helpers:
        if h["names"][0] in placement:
            continue
        owners = set()
        pending = False
        for e in nodes:
            if any(n in e["refs"] for n in h["names"]):
                o = e["owner"]
                if o in ("dict", "fixtures"):
                    continue
                if o == "helper":
                    p = placement.get(e["names"][0])
                    if p is None:
                        pending = True
                        continue
                    o = p
                owners.add(o)
        if pending and not owners:
            continue
        gs = {o for o in owners if o.startswith("g")}
        if h["names"][0] in FORCE:
            placement[h["names"][0]] = FORCE[h["names"][0]]
        elif len(owners) == 0:
            placement[h["names"][0]] = "root"
        elif len(owners) == 1:
            placement[h["names"][0]] = next(iter(owners))
        elif owners <= {"root"} | gs and len(gs) == 1:
            placement[h["names"][0]] = next(iter(gs))
        else:
            placement[h["names"][0]] = "base"
        changed = True
print("helper placement:", placement)
for h in helpers:
    h["owner"] = placement[h["names"][0]]

# --- gather module contents
GROUP_KEYS = ["g%d" % i for i in range(len(GROUPS))]
def rows(start, end):
    return lines[start:end - 1]
roots_rows = {k: [] for k in GROUP_KEYS}
vocab_rows = {k: [] for k in GROUP_KEYS}
extra_rows = {k: [] for k in GROUP_KEYS}
vr = defined["VECTOR_ROOTS"]["node"]
for line in lines[vr.lineno:vr.end_lineno - 1]:
    m = re.match(r'^    "([a-z0-9-]+)": "', line)
    assert m, line
    roots_rows["g%d" % group_of_kind(id_kind[m.group(1)])].append(line)
for key, target in (("VOCABULARY", vocab_rows), ("EXTRA_INVERT", extra_rows)):
    node = defined[key]["node"]
    for line in lines[node.lineno:node.end_lineno - 1]:
        m = re.match(r'^    "([a-z0-9-]+)": ', line)
        assert m, line
        target["g%d" % group_of_kind(m.group(1))].append(line)
roots_comment = [l for l in lines[defined["VECTOR_ROOTS"]["start"] - 1:defined["VECTOR_ROOTS"]["node"].lineno - 1]]
vocab_comment = [l for l in lines[defined["VOCABULARY"]["start"] - 1:defined["VOCABULARY"]["node"].lineno - 1]]
extra_comment = [l for l in lines[defined["EXTRA_INVERT"]["start"] - 1:defined["EXTRA_INVERT"]["node"].lineno - 1]]

module_names = {"base": ALIAS + "base"}
for i, g in enumerate(GROUPS):
    module_names["g%d" % i] = ALIAS + g[0]
module_of = {}
for e in nodes:
    for n in e["names"]:
        if e["owner"] in module_names:
            module_of[n] = e["owner"]
# imports computed per module
def needed_from_others(owner_key, entries):
    need = collections.defaultdict(set)
    for e in entries:
        for r in e["refs"]:
            m = module_of.get(r)
            if m and m != owner_key:
                need[m].add(r)
    return need

def docstring(folder, text):
    return '"""%s %s"""\n' % (folder[0:2] if folder[1] == V else folder[0], text) if False else '"""%s%s %s"""\n' % (folder[0], V, text)

out_files = {}
module_entries = {k: [e for e in nodes if e["owner"] == k] for k in list(module_names)}
edges = {}
for key in module_names:
    entries = module_entries[key]
    need = needed_from_others(key, entries)
    edges[key] = set(need)
    parts = []
    folder, doc = (BASE[1], BASE[2]) if key == "base" else (GROUPS[int(key[1:])][1], GROUPS[int(key[1:])][2])
    parts.append(docstring(folder, doc))
    parts.append("from __future__ import annotations\n")
    texts = [e["text"] for e in entries] + roots_rows.get(key, []) + vocab_rows.get(key, [])
    std = std_used([e["text"] for e in entries])
    if std:
        parts.append("\n".join("import " + m for m in std) + "\n")
    imports = []
    for other in sorted(need, key=lambda k: (k != "base", k)):
        names = sorted(need[other])
        imports.append("from %s import %s" % (module_names[other], ", ".join(names) if len(", ".join(names)) < 100 else "(\n    " + ",\n    ".join(names) + ",\n)"))
    if imports:
        parts.append("\n".join(imports) + "\n")
    body = []
    if key == "base":
        # original Wire, Outcomes, Links regions verbatim, then helpers in a Vocabulary region
        for region in ("🔖️Wire", "🔖️Outcomes", "🔖️Links"):
            a, b = regions[region]
            body.append("\n".join(lines[a - 1:b]))
        shared = [e for e in entries if e["region"] == "🔖️Vocabulary"]
        if shared:
            body.append("# region 🔖️Vocabulary\n" + "\n\n\n".join(e["text"] for e in shared) + "\n# endregion 🔖️Vocabulary")
    else:
        body.append("# region 🔖️Fixtures\n" + "\n".join(roots_comment) + "\nVECTOR_ROOTS = {\n" + "\n".join(roots_rows[key]) + "\n}\n# endregion 🔖️Fixtures")
        vocab = "\n\n\n".join(e["text"] for e in entries)
        vocab += "\n\n\n" + "\n".join(vocab_comment) + "\nVOCABULARY = {\n" + "\n".join(vocab_rows[key]) + "\n}"
        if extra_rows[key]:
            vocab += "\n\n" + "\n".join(extra_comment) + "\nEXTRA_INVERT = {\n" + "\n".join(extra_rows[key]) + "\n}"
        body.append("# region 🔖️Vocabulary\n" + vocab + "\n# endregion 🔖️Vocabulary")
    text = "\n".join(parts) + "\n\n" + "\n\n\n".join(body) + "\n"
    text = re.sub(r"\n{4,}", "\n\n\n", text)
    out_files[key] = (folder, text)

# --- cycles
def has_cycle():
    seen, stack = set(), set()
    def visit(k):
        if k in stack:
            return True
        if k in seen:
            return False
        seen.add(k); stack.add(k)
        if any(visit(n) for n in edges.get(k, ())):
            return True
        stack.discard(k)
        return False
    return any(visit(k) for k in edges)
print("import edges:", {k: sorted(v) for k, v in edges.items()})
assert not has_cycle(), "import cycle"

# --- root
root_need = collections.defaultdict(set)
root_entries = [e for e in nodes if e["owner"] in ("root", "fixtures")]
for e in root_entries + [defined["VOCABULARY"], defined["EXTRA_INVERT"], defined["VECTOR_ROOTS"]]:
    pass
for e in [x for x in nodes if x["owner"] in ("root", "fixtures")]:
    for r in e["refs"]:
        m = module_of.get(r)
        if m:
            root_need[m].add(r)
root_std = std_used([e["text"] for e in root_entries])
print("root needs:", {k: sorted(v) for k, v in root_need.items()})
loader = '''def _load(alias, folder):
    """🧩️ Loads one child module of this adapter from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module
'''
loads = ["%s = _load(\"%s\", \"%s\")" % (g[0] if False else ("base" if k == "base" else GROUPS[int(k[1:])][0]), module_names[k], out_files[k][0]) for k in ["base"] + GROUP_KEYS]
root_imports = []
for other in sorted(root_need, key=lambda k: (k != "base", k)):
    names = sorted(root_need[other])
    root_imports.append("from %s import %s" % (module_names[other], ", ".join(names) if len(", ".join(names)) < 100 else "(\n    " + ",\n    ".join(names) + ",\n)"))
std_lines = "\n".join("import " + m for m in sorted(set(root_std + ["importlib.util", "os", "sys"]), key=lambda m: m))
imports_region = "# region 🔖️Imports\n" + std_lines + "\n\nfrom semio_repo_test import Adapter, Context, Outcome\n\n\n" + loader + "\n\n" + "\n".join(loads) + "\n\n" + "\n".join(root_imports) + "\n# endregion 🔖️Imports"
groups_mod = [GROUPS[i][0] for i in range(len(GROUPS))]
fixtures_region = "# region 🔖️Fixtures\n" + "\n".join(roots_comment) + "\nVECTOR_ROOTS = {\n" + "".join("    **%s.VECTOR_ROOTS,\n" % g for g in groups_mod) + "}\n\n\n" + "\n\n\n".join(e["text"] for e in nodes if e["owner"] == "fixtures" and e["names"] != ["VECTOR_ROOTS"]) + "\n# endregion 🔖️Fixtures"
vocab_region = "# region 🔖️Vocabulary\n" + "\n".join(vocab_comment) + "\nVOCABULARY = {\n" + "".join("    **%s.VOCABULARY,\n" % g for g in groups_mod) + "}\n\n" + "\n".join(extra_comment) + "\nEXTRA_INVERT = {\n" + "".join("    **%s.EXTRA_INVERT,\n" % g for g in groups_mod) + "}\n# endregion 🔖️Vocabulary"
tail = "\n".join(lines[regions["🔖️Inverse"][0] - 1:])
head = "\n".join(lines[:regions["🔖️Imports"][0] - 1])
root_text = head + "\n" + imports_region + "\n\n\n" + fixtures_region + "\n\n\n" + vocab_region + "\n\n\n" + tail
root_text = re.sub(r"\n{4,}", "\n\n\n", root_text)

for key, (folder, text) in out_files.items():
    print("%-14s %-28s %5d lines" % (key, folder, len(text.split("\n"))))
print("root", len(root_text.split("\n")), "lines")
if APPLY:
    for key, (folder, text) in out_files.items():
        target = os.path.join(CASE, folder, "🐍️.py")
        os.makedirs(os.path.dirname(target), exist_ok=True)
        existing = os.path.exists(target)
        open(target, "w", encoding="utf-8").write(text)
    open(os.path.join(CASE, "🐍️.py"), "w", encoding="utf-8").write(root_text)
    print("applied")
