"""usage: rs-tests.py <original.txt> <file>...   multiset of #[test] fn names (with the enclosing inline module names) before and after"""
import sys, re, collections
def names(text, label=""):
    out = []
    ls = text.split("\n")
    depth_mods = []
    for i, l in enumerate(ls):
        m = re.match(r"^(\s*)mod\s+(\w+)\s*\{", l)
        if m: depth_mods.append((len(m.group(1)), m.group(2)))
        s = l.strip()
        if s == "}" and depth_mods and len(l) - len(l.lstrip()) == depth_mods[-1][0]: depth_mods.pop()
        if s.startswith("#[test]"):
            j = i + 1
            while not re.match(r"\s*(?:async\s+)?fn\s+(\w+)", ls[j]): j += 1
            out.append("::".join([n for _, n in depth_mods] + [re.match(r"\s*(?:async\s+)?fn\s+(\w+)", ls[j]).group(1)]))
    return out
orig, *rest = sys.argv[1:]
a = collections.Counter(names(open(orig, encoding="utf-8").read()))
b = collections.Counter()
for f in rest: b.update(names(open(f, encoding="utf-8").read()))
print("tests before", sum(a.values()), "after", sum(b.values()), "missing", sum((a - b).values()), "extra", sum((b - a).values()), "IDENTICAL (fn names incl. inline mods)" if a == b else "DIFFERENT")
