"""🔎️ Static lossless and load check of the split Python reference against its pre-edit snapshot (c4p51).

usage: pychk.py   -> compile and symtable undefined-global check of every file, top-level statement multiset (snapshot minus new) and stripped line multiset.
"""
import ast, builtins, collections, glob, os, re, symtable, sys

R = "/Users/ueli/Documents/semio/"
F = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/"
LIVE = R + "🧰️framework/🛍️products/❓️quiz/🧪️tests/🧾️learner-lifecycle/🐍️.py"
OLD = F + "🗑️generated/c4p51-snap/f5.py"


def undefined(path):
    source = open(path, encoding="utf-8").read()
    compile(source, path, "exec")
    table = symtable.symtable(source, path, "exec")
    bound = {s.get_name() for s in table.get_symbols() if s.is_assigned() or s.is_imported() or s.is_namespace()}
    missing = set()

    def walk(t):
        for s in t.get_symbols():
            skip = hasattr(builtins, s.get_name()) or s.get_name() in ("__file__", "__name__")
            if t.get_type() != "module" and s.is_global() and s.is_referenced() and s.get_name() not in bound and not skip:
                missing.add((t.get_name(), s.get_name()))
            if t.get_type() == "module" and s.is_referenced() and not s.is_assigned() and not s.is_imported() and not s.is_namespace() and not skip:
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
        out["\n".join(lines[node.lineno - 1 : node.end_lineno]).strip()] += 1
    for line in lines:
        if re.match(r"#\s?(end)?region ", line):
            out["REGION " + line.strip()] += 1
    return out


def line_multiset(path):
    return collections.Counter(l.strip() for l in open(path, encoding="utf-8").read().split("\n") if l.strip())


files = [LIVE] + sorted(glob.glob(os.path.dirname(LIVE) + "/*/🐍️.py"))
problems = 0
print("files", len(files), [len(open(p, encoding="utf-8").read().split("\n")) - 1 for p in files], "old", len(open(OLD, encoding="utf-8").read().split("\n")) - 1)
for path in [OLD] + files:
    m = undefined(path)
    problems += bool(m and path != OLD)
    print("  %-5s %s %s" % ("OK" if not m else "FAIL", "snapshot" if path == OLD else os.path.relpath(path, os.path.dirname(LIVE)), "" if not m else sorted(m)[:8]))
before, after = segments(OLD), collections.Counter()
for p in files:
    after.update(segments(p))
lost, extra = before - after, after - before
print("statements: lost %d, added %d" % (sum(lost.values()), sum(extra.values())))
for text, n in lost.items():
    print("  LOST", n, text[:150].replace("\n", " | "))
for text, n in extra.items():
    print("  ADDED", n, text[:110].replace("\n", " | "))
lb, la = line_multiset(OLD), collections.Counter()
for p in files:
    la.update(line_multiset(p))
llost, lextra = lb - la, la - lb
print("lines (stripped, non-blank): lost %d, added %d" % (sum(llost.values()), sum(lextra.values())))
for text, n in llost.items():
    print("  LOST-LINE", n, text[:150])
print("PROBLEMS", problems + sum(lost.values()))
