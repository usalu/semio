"""✂️ Splits the energy fabric-and-gains Python module: the people, lighting and equipment gain kinds move into `👥️gain-kinds`.

usage: fabsplit.py [--apply]
Reads the pre-edit snapshot. The gain kinds are the contiguous tail of the module (functions, the 48 VECTOR_ROOTS rows and the 24
VOCABULARY and EXTRA_INVERT rows); the parent keeps materials and constructions and merges the child's tables in catalog order.
"""
import ast, os, re, sys, symtable

F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
LIVE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🪨️fabric-and-gains/🐍️.py"
CHILD_DIR = "👥️gain-kinds"
ALIAS = "mutate_energy_model_1_gain_kinds"
APPLY = "--apply" in sys.argv
src = open(F + "/🗑️generated/c4p14-snap/energy_fab.py", encoding="utf-8").read()
L = src.split("\n")
assert L[-1] == ""
L = L[:-1]
tree = ast.parse(src)
GAIN = re.compile(r"(people|lighting|equipment)_gain")
funcs = [n for n in tree.body if isinstance(n, ast.FunctionDef)]
first_gain = next(n for n in funcs if GAIN.search(n.name))
assert all(GAIN.search(n.name) for n in funcs[funcs.index(first_gain):]), "gain functions are not the contiguous tail"
assert not any(GAIN.search(n.name) for n in funcs[: funcs.index(first_gain)])
fn_start = first_gain.lineno
last_fn_end = funcs[-1].end_lineno

def node(name):
    return next(n for n in tree.body if isinstance(n, ast.Assign) and n.targets[0].id == name)

vr, voc, extra = node("VECTOR_ROOTS"), node("VOCABULARY"), node("EXTRA_INVERT")
def rows(n):
    return [(re.match(r'\s+"([a-z0-9-]+)"', l).group(1), l) for l in L[n.lineno : n.end_lineno - 1]]
vr_rows, voc_rows, extra_rows = rows(vr), rows(voc), rows(extra)
for r in (vr_rows, voc_rows, extra_rows):
    flags = [bool(GAIN.search(k.replace("-", "_"))) for k, _ in r]
    assert flags == sorted(flags), "gain rows are not the contiguous tail"
assert len(vr_rows) == 80 and len(voc_rows) == 40 and len(extra_rows) == 40, (len(vr_rows), len(voc_rows), len(extra_rows))

def split(r):
    return [l for k, l in r if not GAIN.search(k.replace("-", "_"))], [l for k, l in r if GAIN.search(k.replace("-", "_"))]

vr_keep, vr_move = split(vr_rows)
voc_keep, voc_move = split(voc_rows)
extra_keep, extra_move = split(extra_rows)
vr_comment = L[vr.lineno - 2]
assert vr_comment.startswith("#: ")
voc_comment = [l for l in L[voc.lineno - 2 : voc.lineno - 1] if l.startswith("#")]
extra_comment = L[extra.lineno - 2]
assert extra_comment.startswith("#: ")
funcs_keep = "\n".join(L[98 - 1 : fn_start - 1]).rstrip("\n")
funcs_move = "\n".join(L[fn_start - 1 : last_fn_end])

def used(text):
    t = symtable.symtable(text, "<m>", "exec")
    names = set()
    def walk(tb):
        for s in tb.get_symbols():
            if tb.get_type() == "module":
                if s.is_referenced() and not s.is_assigned() and not s.is_imported():
                    names.add(s.get_name())
            elif s.is_global() and s.is_referenced():
                names.add(s.get_name())
        for c in tb.get_children():
            walk(c)
    walk(t)
    return names

base_names = ["applied", "no_op", "rejected", "unchanged"]
parent_used = used(funcs_keep)
child_used = used(funcs_move)
doc_parent = L[0]
V = "️"
doc_child = '"""👥%s Gain kinds: the people, lighting and equipment gains."""' % V

def base_import(u):
    return "from mutate_energy_model_1_base import " + ", ".join(n for n in base_names if n in u)

child = "\n".join(
    [
        doc_child,
        "",
        "from __future__ import annotations",
        "",
        *(["import copy", ""] if "copy" in child_used else []),
        base_import(child_used),
        "",
        "",
        "# region 🔖️Fixtures",
        vr_comment,
        "VECTOR_ROOTS = {",
        *vr_move,
        "}",
        "# endregion 🔖️Fixtures",
        "",
        "",
        "# region 🔖️Vocabulary",
        funcs_move,
        "",
        *voc_comment,
        "VOCABULARY = {",
        *voc_move,
        "}",
        "",
        extra_comment,
        "EXTRA_INVERT = {",
        *extra_move,
        "}",
        "# endregion 🔖️Vocabulary",
        "",
    ]
)
loader = '''def _load(alias, folder):
    """🧩️ Loads one child module of this module from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module
'''
std = sorted(set(["importlib.util", "os", "sys"] + (["copy"] if "copy" in parent_used else [])))
parent = "\n".join(
    [
        doc_parent,
        "",
        "from __future__ import annotations",
        "",
        *["import " + m for m in std],
        "",
        base_import(parent_used),
        "",
        "",
        loader,
        "",
        'gains = _load("%s", "%s")' % (ALIAS, CHILD_DIR),
        "",
        "",
        "# region 🔖️Fixtures",
        vr_comment,
        "VECTOR_ROOTS = {",
        *vr_keep,
        "    **gains.VECTOR_ROOTS,",
        "}",
        "# endregion 🔖️Fixtures",
        "",
        "",
        "# region 🔖️Vocabulary",
        funcs_keep,
        "",
        "",
        *voc_comment,
        "VOCABULARY = {",
        *voc_keep,
        "    **gains.VOCABULARY,",
        "}",
        "",
        extra_comment,
        "EXTRA_INVERT = {",
        *extra_keep,
        "    **gains.EXTRA_INVERT,",
        "}",
        "# endregion 🔖️Vocabulary",
        "",
    ]
)
print("parent", len(parent.split("\n")) - 1, "child", len(child.split("\n")) - 1, "rows moved", len(vr_move), len(voc_move), len(extra_move))
print("parent std", std, "base", [n for n in base_names if n in parent_used], "child base", [n for n in base_names if n in child_used], "child copy", "copy" in child_used)
if APPLY:
    assert open(LIVE, encoding="utf-8").read() == src, "live file changed since the snapshot"
    target = os.path.join(os.path.dirname(LIVE), CHILD_DIR, "🐍️.py")
    assert not os.path.exists(target)
    os.makedirs(os.path.dirname(target))
    open(target, "w", encoding="utf-8").write(child)
    open(LIVE, "w", encoding="utf-8").write(parent)
    print("applied")
