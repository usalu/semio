"""usage: lossless-rs.py <original.txt> <hub> <child>...   line multiset original vs hub+children, ignoring blank lines, `pub(super) ` widenings, the leading `//!` doc block of children, `use super::*;`, #[path]/mod/use-glob mount lines and the hub doc lines that the plan inserted."""
import sys, re, collections
orig, hub, *children = sys.argv[1:]
def lines(p): return open(p, encoding="utf-8").read().split("\n")
def norm(l): return l.strip().replace("pub(super) ", "")
def body_of_child(ls):
    i = 0
    while i < len(ls) and (ls[i].startswith("//!") or not ls[i].strip()): i += 1
    return ls[i:]
MOUNT = re.compile(r'^(#\[path = ".*"\]|(pub(\(crate\))? )?mod [a-z_0-9]+;|(pub(\(crate\))? )?use [a-z_0-9]+::\*;|use super::\*;)$')
def keep(l): return l.strip() and not MOUNT.match(l.strip())
o = collections.Counter(norm(l) for l in lines(orig) if keep(l))
n = collections.Counter()
for c in children:
    for l in body_of_child(lines(c)):
        if keep(l): n[norm(l)] += 1
for l in lines(hub):
    if keep(l) and not l.startswith("//! The ") and not l.startswith("//! `") : n[norm(l)] += 1
missing = o - n
extra = n - o
print("original lines", sum(o.values()), "new lines", sum(n.values()), "missing", sum(missing.values()), "extra", sum(extra.values()))
for k, v in list(missing.items())[:20]: print("  -", v, k[:160])
for k, v in list(extra.items())[:20]: print("  +", v, k[:160])
