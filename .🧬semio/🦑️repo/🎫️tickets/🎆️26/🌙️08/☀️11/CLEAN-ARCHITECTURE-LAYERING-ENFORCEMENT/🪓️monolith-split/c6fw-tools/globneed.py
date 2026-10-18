"""🔎 Lists identifiers of a child file that are imported or defined in the parent root (root `use` names, pub(super)/pub items of sibling children) so a `use super::*;` can be proven needed or unneeded. usage: globneed.py <root.rs> <child.rs>"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from modsplit import ident_set
root, child = sys.argv[1], sys.argv[2]
rt = open(root, encoding='utf-8').read()
ct = open(child, encoding='utf-8').read()
names = set()
for m in re.finditer(r'(?m)^\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]*);', rt):
    body = m.group(1)
    for w in re.findall(r'(?:\bas\s+|::\s*|\{\s*|,\s*|^)([A-Za-z_]\w*)\s*(?=[,}]|$)', body):
        names.add(w)
    if body.strip().endswith('*'):
        names.add('<glob:' + body.strip() + '>')
for m in re.finditer(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:fn|struct|enum|trait|type|const|static|mod|macro_rules!)\s+(\w+)', rt):
    names.add(m.group(1))
base = os.path.dirname(root)
for d in sorted(os.listdir(base)):
    f = os.path.join(base, d, '🦀️.rs')
    if os.path.isfile(f) and os.path.abspath(f) != os.path.abspath(child):
        for m in re.finditer(r'(?m)^\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:fn|struct|enum|trait|type|const|static|mod|macro_rules!)\s+(\w+)', open(f, encoding='utf-8').read()):
            names.add(m.group(1))
ids = ident_set(ct)
hit = sorted(n for n in names if not n.startswith('<') and n in ids)
print('root-provided names used by the child:', hit)
print('root globs:', sorted(n for n in names if n.startswith('<')))
