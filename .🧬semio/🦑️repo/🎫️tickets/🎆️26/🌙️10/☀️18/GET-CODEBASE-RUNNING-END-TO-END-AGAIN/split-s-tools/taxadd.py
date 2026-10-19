"""📚 c7w taxonomy list updater: taxadd.py <leaf-rel> — every `"<leaf>"` line of 🔣️taxonomy.json (sourceModulePaths and input lists) gains the child module files found on disk, bytewise sorted with the leaf."""
import sys, os
REPO = '/Users/ueli/Documents/semio'
TAX = REPO + '/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json'
leaf = sys.argv[1]
d = os.path.dirname(leaf)
kids = sorted(f'{d}/{n}/🟦️.ts' for n in os.listdir(os.path.join(REPO, d)) if os.path.isfile(os.path.join(REPO, d, n, '🟦️.ts')) and n not in sys.argv[2:])
t = open(TAX, encoding='utf-8').read().split('\n')
idx = [i for i, l in enumerate(t) if l.strip().rstrip(',') == f'"{leaf}"']
print('lists', idx, 'children', len(kids))
for i in reversed(idx):
    ind = t[i][:len(t[i]) - len(t[i].lstrip())]
    trail = t[i].rstrip().endswith(',')
    # drop children that already exist adjacent
    existing = {l.strip().rstrip(',').strip('"') for l in t[max(0, i - 80):i + 80]}
    new = [k for k in kids if k not in existing]
    entries = sorted(new + [leaf], key=lambda s: s.encode())
    lines = [f'{ind}"{e}",' for e in entries]
    if not trail: lines[-1] = lines[-1][:-1]
    t[i:i + 1] = lines
open(TAX, 'w', encoding='utf-8').write('\n'.join(t))
