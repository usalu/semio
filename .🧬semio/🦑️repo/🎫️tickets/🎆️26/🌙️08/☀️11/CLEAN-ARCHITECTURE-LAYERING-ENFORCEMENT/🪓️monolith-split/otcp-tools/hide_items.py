"""Add #[doc(hidden)] to every widened item line (`pub fn|struct|enum|const|type ...` produced by the widening) that lacks it. usage: hide_items.py [--apply]"""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import edit as E
rows = [l.rstrip("\n").split("\t") for l in open(E.F + "/🗑️generated/otcp-ledger.tsv", encoding="utf-8")]
apply_ = "--apply" in sys.argv
n = 0
for rel, kind, line in rows:
    if kind != "WIDEN" or not re.match(r"pub (?:async |const |unsafe )*(?:fn|struct|enum|const|type|static|trait)\b", line):
        continue
    text = E.read(rel)
    full = [l for l in text.split("\n") if l.strip().startswith(line[:60]) and l.strip().startswith("pub ")]
    if len(full) != 1:
        print("SKIP", rel[-60:], len(full), line[:60])
        continue
    old = full[0]
    new = re.sub(r"^(\s*)pub ", r"\1#[doc(hidden)] pub ", old, count=1)
    if text.count(old) != 1:
        print("SKIP-DUP", rel[-60:], line[:60])
        continue
    n += 1
    if apply_:
        E.edit(rel, old, new)
print("items to hide:", n)
