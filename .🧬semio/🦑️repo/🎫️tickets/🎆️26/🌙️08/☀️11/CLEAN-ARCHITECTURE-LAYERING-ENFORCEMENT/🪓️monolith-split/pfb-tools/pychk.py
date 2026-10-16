"""🔎️ Static and load checks of the split energy Python adapter against the pre-edit snapshot.

1. every file compiles and has no undefined global (symtable: a global that is read but bound neither in the module nor in builtins);
2. lossless: the multiset of top-level statement texts (functions, assignments, docstrings), dict rows and region markers
   of the original equals the union over the new files, modulo the wiring that the split adds (listed);
3. load-and-compare: the original and the new adapter, imported side by side, expose equal VECTOR_ROOTS, VOCABULARY (same key order, same
   function names), EXTRA_INVERT, FIELD_LEAVES keys, and equal adapter registrations.
"""
import ast, builtins, collections, importlib.util, os, re, symtable, sys

F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
CASE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1"
OLD = F + "/🗑️generated/pfb-orig/energy-test/🐍️.py"
HOST = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py"
sys.dont_write_bytecode = True

files = [CASE + "/🐍️.py"] + sorted(os.path.join(CASE, d, "🐍️.py") for d in os.listdir(CASE) if os.path.isfile(os.path.join(CASE, d, "🐍️.py")))
print("files:", len(files))

def undefined(path):
    source = open(path, encoding="utf-8").read()
    compile(source, path, "exec")
    table = symtable.symtable(source, path, "exec")
    module_bound = {s.get_name() for s in table.get_symbols() if s.is_assigned() or s.is_imported() or s.is_namespace()}
    missing = set()
    def walk(t):
        for s in t.get_symbols():
            if t.get_type() != "module" and s.is_global() and s.is_referenced() and s.get_name() not in module_bound and not hasattr(builtins, s.get_name()) and s.get_name() not in ("__file__", "__name__"):
                missing.add((t.get_name(), s.get_name()))
        for child in t.get_children():
            walk(child)
    walk(table)
    return missing

total = 0
for path in [OLD] + files:
    missing = undefined(path)
    total += len(missing)
    print(("OK   " if not missing else "FAIL ") + os.path.relpath(path, CASE) if path != OLD else "BASE " + ("clean" if not missing else str(sorted(missing))[:200]), "" if not missing else sorted(missing)[:10])
assert total == 0

def segments(path):
    source = open(path, encoding="utf-8").read()
    lines = source.split("\n")
    out = collections.Counter()
    for node in ast.parse(source).body:
        text = "\n".join(lines[node.lineno - 1:node.end_lineno]).strip()
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            continue
        if isinstance(node, ast.Assign) and isinstance(node.targets[0], ast.Name) and node.targets[0].id in ("VECTOR_ROOTS", "VOCABULARY", "EXTRA_INVERT"):
            for row in lines[node.lineno:node.end_lineno - 1]:
                out["ROW " + row.strip()] += 1
            continue
        out[text] += 1
    for line in lines:
        if re.match(r"#\s?(end)?region ", line):
            out["REGION " + line.strip()] += 1
    return out

before = segments(OLD)
after = collections.Counter()
for path in files:
    after.update(segments(path))
lost = before - after
extra = after - before
print("lost statements:", sum(lost.values()), "added statements:", sum(extra.values()))
for text, n in list(lost.items())[:12]:
    print("  LOST", n, text[:140].replace("\n", " | "))
for text, n in list(extra.items())[:40]:
    print("  ADDED", n, text[:140].replace("\n", " | "))

def load(name, path, host):
    sys.modules["semio_repo_test"] = host
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module
host_spec = importlib.util.spec_from_file_location("pfb_host", HOST)
host = importlib.util.module_from_spec(host_spec)
sys.modules["pfb_host"] = host
host_spec.loader.exec_module(host)
old = load("pfb_old", OLD, host)
new = load("pfb_new", CASE + "/🐍️.py", host)
assert list(old.VECTOR_ROOTS.items()) == list(new.VECTOR_ROOTS.items()), "VECTOR_ROOTS"
assert list(old.VOCABULARY) == list(new.VOCABULARY), "VOCABULARY order"
assert all(old.VOCABULARY[k].__name__ == new.VOCABULARY[k].__name__ for k in old.VOCABULARY), "VOCABULARY names"
assert list(old.EXTRA_INVERT) == list(new.EXTRA_INVERT), "EXTRA_INVERT order"
assert all(old.EXTRA_INVERT[k].__name__ == new.EXTRA_INVERT[k].__name__ for k in old.EXTRA_INVERT), "EXTRA_INVERT names"
assert list(old.FIELD_LEAVES) == list(new.FIELD_LEAVES), "FIELD_LEAVES"
a, b = old.adapter(), new.adapter()
assert sorted(a._handlers) == sorted(b._handlers), "registrations"
print("LOAD-AND-COMPARE: VECTOR_ROOTS %d, VOCABULARY %d, EXTRA_INVERT %d, FIELD_LEAVES %d, registrations %d: identical, same order" % (len(new.VECTOR_ROOTS), len(new.VOCABULARY), len(new.EXTRA_INVERT), len(new.FIELD_LEAVES), len(b._handlers)))
