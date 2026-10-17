"""✂️ Splits an energy mutation-group Python module into contiguous function groups; the first group stays in the parent.

usage: esplit.py <spec.json> [--apply]
spec: {file, snapshot, alias, pieces: [{key, dir, doc, first, last}]}  (piece 0 is the parent, `dir`/`doc` unused for it)
`first`/`last` name the first and last function of the piece in source order; helper functions between them belong to it.
Rows of VECTOR_ROOTS follow the piece that owns the longest matching kind (a VOCABULARY or EXTRA_INVERT key); rows of
VOCABULARY and EXTRA_INVERT follow the piece that owns the function they name. The parent merges the children's tables
after its own rows, in piece order. A child may not need a name from a later piece or from the parent.
"""
import ast, os, re, symtable, sys, json

spec = json.load(open(sys.argv[1], encoding="utf-8"))
APPLY = "--apply" in sys.argv
src = open(spec["snapshot"], encoding="utf-8").read()
L = src.split("\n")
assert L[-1] == ""
L = L[:-1]
tree = ast.parse(src)
funcs = [n for n in tree.body if isinstance(n, ast.FunctionDef)]
names = [f.name for f in funcs]
pieces = spec["pieces"]
piece_of_fn = {}
cursor = 0
for p, piece in enumerate(pieces):
    a, b = names.index(piece["first"]), names.index(piece["last"])
    assert a == cursor, "piece %d starts at %d, expected %d" % (p, a, cursor)
    for n in names[a : b + 1]:
        piece_of_fn[n] = p
    cursor = b + 1
assert cursor == len(names), "functions left over: %s" % names[cursor:]

def tbl(name):
    return next(n for n in tree.body if isinstance(n, ast.Assign) and n.targets[0].id == name)

vr, voc, extra = tbl("VECTOR_ROOTS"), tbl("VOCABULARY"), tbl("EXTRA_INVERT")
ROW = re.compile(r'^    "([a-z0-9-]+)": (.*),$')

def rows(n):
    out = []
    for j in range(n.lineno + 1, n.end_lineno):
        m = ROW.match(L[j - 1])
        assert m, L[j - 1]
        out.append((m.group(1), m.group(2), L[j - 1]))
    return out

vr_rows, voc_rows, extra_rows = rows(vr), rows(voc), rows(extra)
kinds = {k for k, _, _ in voc_rows} | {k for k, _, _ in extra_rows}
piece_of_kind = {}
for k, v, _ in voc_rows + extra_rows:
    assert v in piece_of_fn, v
    piece_of_kind.setdefault(k, piece_of_fn[v])
    assert piece_of_kind[k] == piece_of_fn[v], "kind %s spans pieces" % k

def kind_of_row(key):
    best = None
    for k in kinds:
        if key == k or key.startswith(k + "-"):
            if best is None or len(k) > len(best):
                best = k
    return best

def piece_of_row(kind_key, value, which):
    if which == "vr":
        k = kind_of_row(kind_key)
        assert k is not None, "VECTOR_ROOTS row %s matches no kind" % kind_key
        return piece_of_kind[k]
    return piece_of_fn[value]

rows_by_piece = [{"vr": [], "voc": [], "extra": []} for _ in pieces]
for key, val, line in vr_rows:
    rows_by_piece[piece_of_row(key, val, "vr")]["vr"].append(line)
for key, val, line in voc_rows:
    rows_by_piece[piece_of_row(key, val, "voc")]["voc"].append(line)
for key, val, line in extra_rows:
    rows_by_piece[piece_of_row(key, val, "extra")]["extra"].append(line)

def fn_text(f):
    start = f.lineno
    return "\n".join(L[start - 1 : f.end_lineno])

fn_texts = [[] for _ in pieces]
for f in funcs:
    fn_texts[piece_of_fn[f.name]].append((f, fn_text(f)))

def used(text):
    t = symtable.symtable(text, "<m>", "exec")
    out = set()
    def walk(tb):
        for s in tb.get_symbols():
            if tb.get_type() == "module":
                if s.is_referenced() and not s.is_assigned() and not s.is_imported():
                    out.add(s.get_name())
            elif s.is_global() and s.is_referenced():
                out.add(s.get_name())
        for c in tb.get_children():
            walk(c)
    walk(t)
    return out

BASE = ["applied", "no_op", "rejected", "unchanged"]
piece_funcs_text = ["\n\n\n".join(t for _, t in fn_texts[p]) for p in range(len(pieces))]
defined = [{f.name for f, _ in fn_texts[p]} for p in range(len(pieces))]
for p in range(len(pieces)):
    u = used(piece_funcs_text[p])
    for q in range(len(pieces)):
        if q != p:
            clash = u & defined[q]
            assert not clash, "piece %d needs %s from piece %d" % (p, sorted(clash), q)

vr_comment = L[vr.lineno - 2]
assert vr_comment.startswith("#: ")
extra_comment = L[extra.lineno - 2]
assert extra_comment.startswith("#: ")
voc_comment = [l for l in L[voc.lineno - 2 : voc.lineno - 1] if l.startswith("#")]
doc_parent = L[0]
V = "️"

def base_import(u):
    return "from mutate_energy_model_1_base import " + ", ".join(n for n in BASE if n in u)

LOADER = '''def _load(alias, folder):
    """🧩️ Loads one child module of this module from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module
'''

def emit(p, doc_line, loads):
    u = used(piece_funcs_text[p])
    std = sorted(set((["copy"] if "copy" in u else []) + (["importlib.util", "os", "sys"] if loads else [])))
    out = [doc_line, "", "from __future__ import annotations", ""]
    if std:
        out += ["import " + m for m in std] + [""]
    out += [base_import(u), "", ""]
    if loads:
        out += [LOADER, ""]
        for key, alias, folder in loads:
            out.append('%s = _load("%s", "%s")' % (key, alias, folder))
        out += ["", ""]
    r = rows_by_piece[p]
    merge = lambda name: ["    **%s.%s," % (key, name) for key, _, _ in loads]
    out += ["# region 🔖️Fixtures", vr_comment, "VECTOR_ROOTS = {", *r["vr"], *merge("VECTOR_ROOTS"), "}", "# endregion 🔖️Fixtures", "", "", "# region 🔖️Vocabulary"]
    out += [piece_funcs_text[p], "", ""]
    out += [*voc_comment, "VOCABULARY = {", *r["voc"], *merge("VOCABULARY"), "}", "", extra_comment, "EXTRA_INVERT = {", *r["extra"], *merge("EXTRA_INVERT"), "}", "# endregion 🔖️Vocabulary", ""]
    return "\n".join(out), u

outputs = []
loads = []
for p in range(1, len(pieces)):
    piece = pieces[p]
    emoji = piece["dir"][: next(i for i, ch in enumerate(piece["dir"]) if ch.isascii() and ch.isalpha())]
    text, _ = emit(p, '"""%s %s"""' % (emoji, piece["doc"]), [])
    outputs.append((piece["dir"], text))
    loads.append(("c%d" % p, spec["alias"] + "_" + piece["key"], piece["dir"]))
parent_text, _ = emit(0, doc_parent, loads)
live = spec["file"]
print("parent %d lines" % (len(parent_text.split("\n")) - 1))
for d, t in outputs:
    print("child %-40s %d lines" % (d, len(t.split("\n")) - 1))
for p in range(len(pieces)):
    r = rows_by_piece[p]
    print("piece %d rows vr/voc/extra %d/%d/%d, funcs %d" % (p, len(r["vr"]), len(r["voc"]), len(r["extra"]), len(fn_texts[p])))
assert sum(len(r["vr"]) for r in rows_by_piece) == len(vr_rows)
if APPLY:
    origin = spec.get("origin", spec["snapshot"])
    assert open(live, encoding="utf-8").read() == open(origin, encoding="utf-8").read(), "live file changed since the snapshot"
    base = os.path.dirname(live)
    for d, t in outputs:
        target = os.path.join(base, d, "🐍️.py")
        assert not os.path.exists(target), target
        os.makedirs(os.path.dirname(target))
        open(target, "w", encoding="utf-8").write(t)
    open(live, "w", encoding="utf-8").write(parent_text)
    print("applied")
