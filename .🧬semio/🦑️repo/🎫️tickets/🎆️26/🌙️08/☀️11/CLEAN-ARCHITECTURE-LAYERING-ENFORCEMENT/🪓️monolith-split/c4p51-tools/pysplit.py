"""✂️ Splits a region-structured Python mutation adapter into child modules, one folder per child, and rewrites the root.

usage: pysplit.py <spec.json> [--apply]
spec: {file, snapshot, alias, children: [{key, dir, doc, regions: [..], openMissing?: [..]}]}
The snapshot is the pre-edit text; a re-run after a restore is reproducible. Dry run prints sizes and import edges.
Regions that stay in the root are every region not listed by a child (except Imports, which is rebuilt). A region whose start
marker is missing (an `# endregion X` with no `# region X`) is read as starting after the previous region; `openMissing` names the
children that get the missing start marker added.
"""
import ast, builtins, collections, json, os, re, symtable, sys

spec = json.load(open(sys.argv[1], encoding="utf-8"))
APPLY = "--apply" in sys.argv
V = "️"
src = open(spec["snapshot"], encoding="utf-8").read()
lines = src.split("\n")
assert lines[-1] == "", "snapshot must end with a newline"
lines = lines[:-1]
ALIAS = spec["alias"]

blocks = []
open_at = {}
last_end = None
for number, line in enumerate(lines, 1):
    m = re.match(r"# region (.*)$", line)
    if m:
        assert m.group(1) not in open_at, "nested region " + m.group(1)
        open_at[m.group(1)] = number
        continue
    m = re.match(r"# endregion (.*)$", line)
    if m:
        name = m.group(1)
        if name in open_at:
            start, missing = open_at.pop(name), False
        else:
            start = (last_end or 0) + 1
            while lines[start - 1].strip() == "":
                start += 1
            missing = True
        assert not open_at, "region left open across %s: %s" % (name, list(open_at))
        blocks.append(dict(name=name, start=start, end=number, missing=missing))
        last_end = number
assert not open_at, open_at
blocks.sort(key=lambda b: b["start"])
header = lines[: blocks[0]["start"] - 1]
by_name = {b["name"]: b for b in blocks}
for b in blocks:
    b["lines"] = lines[b["start"] - 1 : b["end"]]
    b["text"] = "\n".join(b["lines"])
covered = set()
for b in blocks:
    covered.update(range(b["start"], b["end"] + 1))
outside = [n for n in range(len(header) + 1, len(lines) + 1) if n not in covered and lines[n - 1].strip() != ""]
assert not outside, "code outside regions at lines %s" % outside[:10]

imports_block = by_name["🔖️Imports"]
import_tree = ast.parse(imports_block["text"].split("\n", 1)[1].rsplit("\n", 1)[0])
std_imports = []
other_from = []
host_names = []
for node in import_tree.body:
    if isinstance(node, ast.Import):
        std_imports += [a.name for a in node.names]
    elif isinstance(node, ast.ImportFrom):
        if node.module == "semio_repo_test":
            host_names += [a.name for a in node.names]
        else:
            other_from += [(node.module, a.name) for a in node.names]

child_regions = {r for c in spec["children"] for r in c["regions"]}
root_regions = [b["name"] for b in blocks if b["name"] != "🔖️Imports" and b["name"] not in child_regions]
for r in child_regions:
    assert r in by_name, r


def block_text(b, open_marker=False):
    if b["missing"] and open_marker:
        return "# region " + b["name"] + "\n" + b["text"]
    return b["text"]


def defined_names(text):
    names = []
    for node in ast.parse(text).body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            names.append(node.name)
        elif isinstance(node, ast.Assign):
            for t in node.targets:
                names += [n.id for n in ast.walk(t) if isinstance(n, ast.Name)]
        elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
            names.append(node.target.id)
    return names


def referenced_globals(text):
    table = symtable.symtable(text, "<block>", "exec")
    found = set()

    def walk(t):
        for s in t.get_symbols():
            if t.get_type() == "module":
                if s.is_referenced() and not s.is_assigned() and not s.is_imported():
                    found.add(s.get_name())
            elif s.is_global() and s.is_referenced():
                found.add(s.get_name())
        for c in t.get_children():
            walk(c)

    walk(table)
    return found


def used_by_text(text):
    table_names = referenced_globals(text)
    return set(table_names)


files = {}
for child in spec["children"]:
    parts = [block_text(by_name[r], r in child.get("openMissing", [])) for r in sorted(child["regions"], key=lambda r: by_name[r]["start"])]
    files[child["key"]] = dict(kind="child", child=child, body="\n\n\n".join(parts))
files["root"] = dict(kind="root", body="\n\n\n".join(by_name[r]["text"] for r in root_regions))

owner = {}
for key, f in files.items():
    f["defined"] = defined_names(f["body"])
    for n in f["defined"]:
        assert owner.get(n, key) == key, "name defined in two files: %s (%s, %s)" % (n, owner[n], key)
        owner[n] = key
    f["uses"] = used_by_text(f["body"])

edges = {}
needs = {}
for key, f in files.items():
    need = collections.defaultdict(set)
    for n in f["uses"]:
        o = owner.get(n)
        if o and o != key:
            need[o].add(n)
    needs[key] = need
    edges[key] = set(need)


def toposort():
    order, state = [], {}

    def visit(k, path):
        if state.get(k) == 2:
            return
        assert state.get(k) != 1, "import cycle through " + " -> ".join(path + [k])
        state[k] = 1
        for d in sorted(edges[k]):
            visit(d, path + [k])
        state[k] = 2
        order.append(k)

    for k in files:
        visit(k, [])
    return order


if "--edges" in sys.argv:
    for k in files:
        for d, ns in needs[k].items():
            print("EDGE %s -> %s: %s" % (k, d, sorted(ns)))
    sys.exit(0)
order = toposort()
assert "root" not in [d for k in files if k != "root" for d in edges[k]], "a child depends on the root"

module_name = {k: ALIAS + k for k in files if k != "root"}


def fmt_import(module, names):
    names = sorted(names, key=lambda n: owner_order[n])
    one = "from %s import %s" % (module, ", ".join(names))
    if len(one) <= 110:
        return one
    return "from %s import (\n%s\n)" % (module, "\n".join("    %s," % n for n in names))


owner_order = {}
position = 0
for key, f in files.items():
    for n in f["defined"]:
        owner_order[n] = position
        position += 1


def std_used(f):
    return [m for m in std_imports if m.split(".")[0] in f["uses"]]


def from_used(f):
    grouped = {}
    for module, name in other_from:
        if name in f["uses"]:
            grouped.setdefault(module, []).append(name)
    return ["from %s import %s" % (module, ", ".join(names)) for module, names in grouped.items()]


def host_used(f):
    return [n for n in host_names if n in f["uses"]]


out = {}
for key in order:
    f = files[key]
    if key == "root":
        continue
    child = f["child"]
    emoji = child["dir"][: next(i for i, ch in enumerate(child["dir"]) if ch.isascii() and ch.isalpha())]
    doc = '"""%s %s"""' % (emoji, child["doc"])
    imp = []
    std = std_used(f)
    if std or from_used(f):
        imp.append("\n".join(["import " + m for m in std] + from_used(f)))
    host = host_used(f)
    if host:
        imp.append("from semio_repo_test import " + ", ".join(host))
    internal = [fmt_import(module_name[d], names) for d, names in sorted(needs[key].items(), key=lambda kv: order.index(kv[0]))]
    if internal:
        imp.append("\n".join(internal))
    imports = "# region 🔖️Imports\n" + "\n\n".join(imp) + ("\n" if imp else "") + "# endregion 🔖️Imports" if imp else ""
    text = doc + "\n\nfrom __future__ import annotations\n\n" + (imports + "\n\n\n" if imports else "\n") + f["body"] + "\n"
    text = re.sub(r"\n{4,}", "\n\n\n", text)
    out[key] = (child["dir"], text)

root = files["root"]
root_std = sorted(set(std_used(root) + ["importlib.util", "os", "sys"]))
root_host = host_used(root)
loader = '''def _load(alias, folder):
    """🧩️ Loads one child module of this adapter from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module
'''
loads = ["_load(\"%s\", \"%s\")" % (module_name[k], out[k][0]) for k in order if k != "root"]
root_internal = [fmt_import(module_name[d], names) for d, names in sorted(needs["root"].items(), key=lambda kv: order.index(kv[0]))]
imports_region = "# region 🔖️Imports\n" + "\n".join(["import " + m for m in root_std] + from_used(root)) + "\n\n" + "from semio_repo_test import " + ", ".join(root_host) + "\n\n\n" + loader + "\n\n" + "\n".join(loads) + "\n\n" + "\n".join(root_internal) + "\n# endregion 🔖️Imports"
head = "\n".join(header).rstrip("\n")
root_text = head + "\n\n" + imports_region + "\n\n\n" + root["body"] + "\n"
root_text = re.sub(r"\n{4,}", "\n\n\n", root_text)

print("order:", order)
for k, v in edges.items():
    print("  %s needs %s" % (k, {d: len(n) for d, n in needs[k].items()}))
for key, (folder, text) in out.items():
    print("%-10s %-24s %5d lines" % (key, folder, len(text.split("\n")) - 1))
print("root       %-24s %5d lines" % ("", len(root_text.split("\n")) - 1))
unused = [n for n in root_host if False]

if APPLY:
    base = os.path.dirname(spec["file"])
    assert open(spec["file"], encoding="utf-8").read() == open(spec.get("origin", spec["snapshot"]), encoding="utf-8").read(), "root changed since the snapshot"
    for key, (folder, text) in out.items():
        target = os.path.join(base, folder, "🐍️.py")
        assert not os.path.exists(target), "child exists: " + target
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8").write(text)
    open(spec["file"], "w", encoding="utf-8").write(root_text)
    print("applied")
