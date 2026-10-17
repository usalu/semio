"""✂️ Splits one Python oracle module into child modules (one folder per child) by top-level statement groups, and rewrites the root.

usage: pysplit.py <spec.json> [--apply]
spec: {file, snapshot, alias, children: [{key, dir, doc, nodes: [index | [first, last], ..]}]}
`nodes` are indices of the top-level statements (see pynodes.py). Statements that no child lists stay in the root, in file order.
Region markers (`# region X` / `# endregion X`, any nesting) are not copied as lines: every statement remembers the stack of regions it
sat in, and each output file re-opens and closes exactly those regions around its statements. The `Imports` region and the imports
are rebuilt per file from the names it uses. The root loads the children by path (the case folders are not packages), in dependency order.
A dry run prints sizes and the import edges; `--apply` writes the children and the root (refusing when the root changed since the snapshot).
"""
import ast, collections, json, os, re, symtable, sys

spec = json.load(open(sys.argv[1], encoding="utf-8"))
APPLY = "--apply" in sys.argv
src = open(spec["snapshot"], encoding="utf-8").read()
assert src.endswith("\n")
lines = src.split("\n")[:-1]
tree = ast.parse(src)
ALIAS = spec["alias"]

markers = []
for number, line in enumerate(lines, 1):
    m = re.match(r"# (end)?region (.*)$", line)
    if m:
        markers.append((number, bool(m.group(1)), m.group(2)))
marker_lines = {n for n, _, _ in markers}

nodes = []
for node in tree.body:
    first = node.decorator_list[0].lineno if getattr(node, "decorator_list", None) else node.lineno
    nodes.append(dict(node=node, first=first, last=node.end_lineno))
for i, n in enumerate(nodes):
    start = n["first"]
    while start - 1 >= 1 and lines[start - 2].startswith("#") and (start - 1) not in marker_lines:
        start -= 1
    n["start"] = start
    stack = []
    for number, is_end, name in markers:
        if number >= n["start"]:
            break
        if is_end:
            assert stack and stack[-1] == name, "unbalanced region %s at %d" % (name, number)
            stack.pop()
        else:
            stack.append(name)
    n["stack"] = list(stack)
    n["text"] = "\n".join(lines[n["start"] - 1 : n["last"]])

header_last = nodes[0]["last"]
import_idx = [i for i, n in enumerate(nodes) if isinstance(n["node"], (ast.Import, ast.ImportFrom))]
body_idx = [i for i in range(1, len(nodes)) if i not in import_idx]


def expand(items):
    out = []
    for it in items:
        if isinstance(it, list):
            out += list(range(it[0], it[1] + 1))
        else:
            out.append(it)
    return out


assign = {}
for child in spec["children"]:
    for i in expand(child["nodes"]):
        assert i in body_idx, "node %d is not a body statement" % i
        assert i not in assign, "node %d listed twice (%s, %s)" % (i, assign[i], child["key"])
        assign[i] = child["key"]
for i in body_idx:
    assign.setdefault(i, "root")


def defined_names(node):
    names = []
    if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
        names.append(node.name)
    elif isinstance(node, ast.Assign):
        for t in node.targets:
            names += [n.id for n in ast.walk(t) if isinstance(n, ast.Name) and isinstance(n.ctx, ast.Store)]
    elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
        names.append(node.target.id)
    return names


def used_names(text):
    found = set()
    table = symtable.symtable(text, "<block>", "exec")

    def walk(t):
        for s in t.get_symbols():
            if t.get_type() == "module":
                if s.is_referenced():
                    found.add(s.get_name())
            elif s.is_global() and s.is_referenced():
                found.add(s.get_name())
        for c in t.get_children():
            walk(c)

    walk(table)
    for n in ast.walk(ast.parse(text)):
        if isinstance(n, ast.Name):
            found.add(n.id)
    return found


files = collections.OrderedDict()
for key in [c["key"] for c in spec["children"]] + ["root"]:
    idx = sorted(i for i in body_idx if assign[i] == key)
    files[key] = dict(idx=idx)
owner = {}
for key, f in files.items():
    f["defined"] = []
    for i in f["idx"]:
        for name in defined_names(nodes[i]["node"]):
            f["defined"].append(name)
            if name in owner and owner[name] != key:
                raise SystemExit("name %s defined in %s and %s (statement %d)" % (name, owner[name], key, i))
            owner[name] = key
for key, f in files.items():
    body_text = "\n".join(nodes[i]["text"] for i in f["idx"])
    f["uses"] = used_names(body_text)
    f["text_all"] = body_text

needs, edges = {}, {}
for key, f in files.items():
    need = collections.defaultdict(set)
    for n in f["uses"]:
        o = owner.get(n)
        if o and o != key and n not in f["defined"]:
            need[o].add(n)
    needs[key] = need
    edges[key] = set(need)
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
assert not [k for k in files if k != "root" and "root" in edges[k]], "a child depends on the root"
module_name = {k: ALIAS + k for k in files if k != "root"}

owner_order = {}
for key, f in files.items():
    for name in f["defined"]:
        owner_order.setdefault(name, len(owner_order))


def fmt_import(module, names):
    names = sorted(names, key=lambda n: owner_order[n])
    one = "from %s import %s" % (module, ", ".join(names))
    if len(one) <= 130:
        return one
    return "from %s import (\n%s\n)" % (module, "\n".join("    %s," % n for n in names))


std_nodes = [nodes[i] for i in import_idx]


def std_imports(f, extra=()):
    out, tops = [], set()
    for n in std_nodes:
        node = n["node"]
        if isinstance(node, ast.Import):
            for a in node.names:
                parts = a.name.split(".")
                if len(parts) > 1:
                    if re.search(r"\b" + re.escape(a.name) + r"\b", f["text_all"]) or a.name in extra:
                        out.append("import " + a.name)
                        tops.add(parts[0])
        elif isinstance(node, ast.ImportFrom):
            wanted = [a.name for a in node.names if a.name in f["uses"]]
            if wanted:
                out.append("from %s import %s" % (node.module, ", ".join(wanted)))
    for n in std_nodes:
        node = n["node"]
        if isinstance(node, ast.Import):
            for a in node.names:
                parts = a.name.split(".")
                if len(parts) == 1 and (a.name in f["uses"] or a.name in extra) and a.name not in tops:
                    out.append("import " + a.name)
    plain = [x for x in out if x.startswith("import ") and "." not in x]
    rest = [x for x in out if x not in plain]
    return sorted(plain) + sorted(x for x in rest if x.startswith("import ")) + sorted(x for x in rest if x.startswith("from "))


def render(key, f, head):
    out = head
    stack_now = []
    prev = None
    after_marker = False

    def emit(text, blank):
        nonlocal out
        out = out.rstrip("\n") + "\n" + "\n" * blank + text + "\n"

    def to_stack(target):
        nonlocal stack_now, after_marker, prev
        k = 0
        while k < len(stack_now) and k < len(target) and stack_now[k] == target[k]:
            k += 1
        for x in range(len(stack_now) - 1, k - 1, -1):
            emit("# endregion " + stack_now[x], 0 if after_marker else 2)
            after_marker = True
        for x in range(k, len(target)):
            emit("# region " + target[x], 0 if after_marker else 2)
            after_marker = True
        if len(target) != len(stack_now) or k != len(stack_now):
            prev = None
        stack_now = list(target)

    for i in f["idx"]:
        n = nodes[i]
        to_stack(n["stack"])
        simple = lambda m: not isinstance(m["node"], (ast.FunctionDef, ast.ClassDef))
        if after_marker:
            gap = 0
        elif prev is not None and prev == i - 1:
            gap = min(max(n["start"] - nodes[prev]["last"] - 1, 0), 2)
        else:
            gap = 1 if (prev is not None and simple(n) and simple(nodes[prev])) else 2
        emit(n["text"], gap)
        after_marker = False
        prev = i
    to_stack([])
    return out


outputs = {}
for key in order:
    if key == "root":
        continue
    f = files[key]
    child = next(c for c in spec["children"] if c["key"] == key)
    emoji = child["dir"][: next(i for i, ch in enumerate(child["dir"]) if ch.isascii() and ch.isalpha())]
    doc = '"""%s %s"""\n\nfrom __future__ import annotations\n\n' % (emoji, child["doc"])
    imp = []
    std = std_imports(f)
    if std:
        imp.append("\n".join(std))
    internal = [fmt_import(module_name[d], names) for d, names in sorted(needs[key].items(), key=lambda kv: order.index(kv[0]))]
    if internal:
        imp.append("\n".join(internal))
    head = doc + ("# region 🔖️Imports\n" + "\n\n".join(imp) + "\n# endregion 🔖️Imports\n\n\n" if imp else "")
    text = render(key, f, head)
    text = re.sub(r"\n{4,}", "\n\n\n", text)
    outputs[key] = (child["dir"], text)

root = files["root"]
root_std = std_imports(root, extra=("importlib.util", "sys"))
if "import os" not in root_std:
    root_std = ["import os"] + root_std
loader = '''def _load(alias, folder):
    """🧩️ Loads one child module of this oracle from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module
'''
loads = ['_load("%s", "%s")' % (module_name[k], outputs[k][0]) for k in order if k != "root"]
root_internal = [fmt_import(module_name[d], names) for d, names in sorted(needs["root"].items(), key=lambda kv: order.index(kv[0]))]
imports_region = "# region 🔖️Imports\n" + "\n".join(root_std) + "\n\n\n" + loader + "\n" + "\n".join(loads) + "\n\n" + "\n".join(root_internal) + "\n# endregion 🔖️Imports\n\n\n"
head = "\n".join(lines[:header_last]).rstrip("\n") + "\n\n" + imports_region
root_text = re.sub(r"\n{4,}", "\n\n\n", render("root", root, head))
if not root_text.endswith("\n"):
    root_text += "\n"

print("order:", order)
for k in files:
    print("  %-12s needs %s" % (k, {d: len(n) for d, n in needs[k].items()}))
for key, (folder, text) in outputs.items():
    print("%-12s %-26s %5d lines" % (key, folder, len(text.split("\n")) - 1))
print("root %-34s %5d lines" % ("", len(root_text.split("\n")) - 1))

if APPLY:
    base = os.path.dirname(spec["file"])
    assert open(spec["file"], encoding="utf-8").read() == src, "root changed since the snapshot"
    for key, (folder, text) in outputs.items():
        target = os.path.join(base, folder, "🐍️.py")
        assert not os.path.exists(target), "child exists: " + target
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8").write(text)
    open(spec["file"], "w", encoding="utf-8").write(root_text)
    print("applied")
