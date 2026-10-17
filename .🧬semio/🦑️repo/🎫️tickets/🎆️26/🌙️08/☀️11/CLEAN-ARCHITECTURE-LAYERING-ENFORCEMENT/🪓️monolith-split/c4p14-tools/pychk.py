"""🔎️ Static, lossless and load checks of the five split Python adapters against their pre-edit snapshots.

usage: pychk.py
For each case: (1) every new file compiles and has no undefined global (symtable); (2) the multiset of top-level statement texts
(dict assignments by row) of the snapshot minus the new files is empty, and what the new files add is listed; (3) the same for
the stripped non-blank line multiset; (4) the region markers; (5) for the energy module, the merged tables load equal in
content and key order to the snapshot's. The wiring a split adds (imports, loader, docstrings, `Imports` regions, `**gains`
rows) is the only expected difference.
"""
import ast, builtins, collections, glob, importlib.util, json, os, re, symtable, sys, types

R = "/Users/ueli/Documents/semio/"
T = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/"
HOST = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py"
files = json.load(open(T + "c4p14-tools/files.json"))
sys.dont_write_bytecode = True
CASES = ["energy_fab", "semio_model", "semio_graph", "semio_drawing", "semio_brep"]
TABLES = ("VECTOR_ROOTS", "VOCABULARY", "EXTRA_INVERT")


def new_files(key):
    live = files[key]
    d = os.path.dirname(live)
    if key == "energy_fab":
        return [live, d + "/👥️gain-kinds/🐍️.py"]
    return [live] + sorted(glob.glob(d + "/*/🐍️.py"))


def undefined(path):
    source = open(path, encoding="utf-8").read()
    compile(source, path, "exec")
    table = symtable.symtable(source, path, "exec")
    bound = {s.get_name() for s in table.get_symbols() if s.is_assigned() or s.is_imported() or s.is_namespace()}
    missing = set()

    def walk(t):
        for s in t.get_symbols():
            if t.get_type() != "module" and s.is_global() and s.is_referenced() and s.get_name() not in bound and not hasattr(builtins, s.get_name()) and s.get_name() not in ("__file__", "__name__"):
                missing.add((t.get_name(), s.get_name()))
            if t.get_type() == "module" and s.is_referenced() and not s.is_assigned() and not s.is_imported() and not s.is_namespace() and not hasattr(builtins, s.get_name()) and s.get_name() not in ("__file__", "__name__"):
                missing.add(("<module>", s.get_name()))
        for c in t.get_children():
            walk(c)

    walk(table)
    return missing


def segments(path):
    source = open(path, encoding="utf-8").read()
    lines = source.split("\n")
    out = collections.Counter()
    for node in ast.parse(source).body:
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            continue
        if isinstance(node, ast.Assign) and isinstance(node.targets[0], ast.Name) and node.targets[0].id in TABLES and isinstance(node.value, ast.Dict):
            for row in lines[node.lineno : node.end_lineno - 1]:
                out["ROW " + row.strip()] += 1
            out["TABLE " + node.targets[0].id] += 1
            continue
        out["\n".join(lines[node.lineno - 1 : node.end_lineno]).strip()] += 1
    for line in lines:
        if re.match(r"#\s?(end)?region ", line):
            out["REGION " + line.strip()] += 1
    return out


def line_multiset(path):
    return collections.Counter(l.strip() for l in open(path, encoding="utf-8").read().split("\n") if l.strip())


def load(name, path, host):
    sys.modules["semio_repo_test"] = host
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


host_spec = importlib.util.spec_from_file_location("c4p14_host", HOST)
host = importlib.util.module_from_spec(host_spec)
sys.modules["c4p14_host"] = host
host_spec.loader.exec_module(host)

total_problems = 0
for key in CASES:
    old = T + "🗑️generated/c4p14-snap/%s.py" % key
    fs = new_files(key)
    print("=== %s: %d files, %s lines (old %d)" % (key, len(fs), [len(open(p, encoding="utf-8").read().split("\n")) - 1 for p in fs], len(open(old, encoding="utf-8").read().split("\n")) - 1))
    for path in [old] + fs:
        m = undefined(path)
        if path != old and m:
            total_problems += 1
        print("  %-5s %s %s" % ("OK" if not m else "FAIL", "snapshot" if path == old else os.path.relpath(path, os.path.dirname(files[key])), "" if not m else sorted(m)[:8]))
    before = segments(old)
    after = collections.Counter()
    for p in fs:
        after.update(segments(p))
    lost, extra = before - after, after - before
    print("  statements: lost %d, added %d" % (sum(lost.values()), sum(extra.values())))
    for text, n in lost.items():
        print("    LOST", n, text[:150].replace("\n", " | "))
    for text, n in extra.items():
        print("    ADDED", n, text[:110].replace("\n", " | "))
    lb = line_multiset(old)
    la = collections.Counter()
    for p in fs:
        la.update(line_multiset(p))
    llost, lextra = lb - la, la - lb
    print("  lines (stripped, non-blank): lost %d, added %d" % (sum(llost.values()), sum(lextra.values())))
    for text, n in llost.items():
        print("    LOST-LINE", n, text[:150])
    if key == "energy_fab":
        load("mutate_energy_model_1_base", os.path.dirname(os.path.dirname(files[key])) + "/🧱️base/🐍️.py", host)
        o = load("c4p14_old_fab", old, host)
        n = load("c4p14_new_fab", files[key], host)
        for t in TABLES:
            ov, nv = getattr(o, t), getattr(n, t)
            same_order = list(ov) == list(nv)
            same_vals = all((ov[k] == nv[k]) if t == "VECTOR_ROOTS" else (ov[k].__name__ == nv[k].__name__) for k in ov)
            print("  table %s: %d vs %d keys, same order %s, same values %s" % (t, len(ov), len(nv), same_order, same_vals))
            if not (same_order and same_vals and len(ov) == len(nv)):
                total_problems += 1
print("PROBLEMS", total_problems)
sys.exit(1 if total_problems else 0)
