"""🔎️ Static and lossless checks of the split BIM IFC oracle against its snapshot.

usage: pychk.py <snapshot> <root 🐍️.py> [child 🐍️.py ...]
(1) every file compiles and has no undefined global (symtable); (2) the multiset of top-level statement texts of the snapshot minus the new
files is empty (imports excluded); what the new files add is listed; (3) the same for stripped non-blank lines; (4) region markers balance;
(5) with a stub `semio_repo_test`, the root loads and `adapter()` registers the ten oracle scenarios.
"""
import ast, builtins, collections, importlib.util, os, re, symtable, sys, types

snapshot, *new = sys.argv[1:]


def undefined(path):
    source = open(path, encoding="utf-8").read()
    compile(source, path, "exec")
    table = symtable.symtable(source, path, "exec")
    bound = {s.get_name() for s in table.get_symbols() if s.is_assigned() or s.is_imported() or s.is_namespace()}
    missing = set()

    def walk(t):
        for s in t.get_symbols():
            name = s.get_name()
            if hasattr(builtins, name) or name in ("__file__", "__name__"):
                continue
            if t.get_type() != "module" and s.is_global() and s.is_referenced() and name not in bound:
                missing.add((t.get_name(), name))
            if t.get_type() == "module" and s.is_referenced() and not s.is_assigned() and not s.is_imported() and not s.is_namespace():
                missing.add(("<module>", name))
        for c in t.get_children():
            walk(c)

    walk(table)
    return missing


def statements(path):
    source = open(path, encoding="utf-8").read()
    out = collections.Counter()
    for node in ast.parse(source).body:
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            continue
        out[ast.get_source_segment(source, node)] += 1
    return out


def stripped_lines(path):
    return collections.Counter(l.strip() for l in open(path, encoding="utf-8").read().split("\n") if l.strip())


problems = 0
for p in new:
    miss = undefined(p)
    print("undefined globals %-60s %s" % (os.path.basename(os.path.dirname(p))[-30:], sorted(miss) or "none"))
    problems += len(miss)
old_s, new_s = statements(snapshot), collections.Counter()
for p in new:
    new_s += statements(p)
lost, added = old_s - new_s, new_s - old_s
print("statements: snapshot %d, new %d, lost %d, added %d" % (sum(old_s.values()), sum(new_s.values()), sum(lost.values()), sum(added.values())))
for k in lost:
    print("  LOST ", k[:100].replace("\n", " "))
for k in added:
    print("  ADDED", k[:100].replace("\n", " "))
problems += sum(lost.values())
old_l, new_l = stripped_lines(snapshot), collections.Counter()
for p in new:
    new_l += stripped_lines(p)
lost_l, added_l = old_l - new_l, new_l - old_l
print("lines: snapshot %d, lost %d, added %d" % (sum(old_l.values()), sum(lost_l.values()), sum(added_l.values())))
for k in lost_l:
    print("  LOST ", k[:110])
for p in new:
    stack = []
    for l in open(p, encoding="utf-8").read().split("\n"):
        m = re.match(r"# (end)?region (.*)$", l)
        if m:
            if m.group(1):
                assert stack and stack[-1] == m.group(2), (p, l)
                stack.pop()
            else:
                stack.append(m.group(2))
    assert not stack, (p, stack)
print("regions balanced in all files")
stub = types.ModuleType("semio_repo_test")


class Outcome:
    def __init__(self, table, raw=None):
        self.table, self.raw = table, raw


class Adapter:
    def __init__(self, name):
        self.rows = []

    def oracle(self, scenario, handler):
        self.rows.append((scenario, handler.__name__))
        return self


stub.Outcome, stub.Adapter = Outcome, Adapter
sys.modules["semio_repo_test"] = stub
spec = importlib.util.spec_from_file_location("oracle_root", new[0])
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
rows = module.adapter().rows
print("adapter rows:", len(rows), [r[0] for r in rows])
problems += 0 if len(rows) == 10 else 1
print("PROBLEMS", problems)
