#!/usr/bin/env python3
"""Usage: suggest.py <diag-output.txt> <test.ts> <plugin-dir> : for every failing L:C site, extract the literal (includes/indexOf/slice) and print corpus lines that hold its longest identifiers."""
import re, sys, os, subprocess
diag = open(sys.argv[1]).read().split('\n'); test = open(sys.argv[2]).read().split('\n'); root = sys.argv[3]
files = subprocess.run(['git','-c','core.fsmonitor=false','ls-files','--cached','--others','--exclude-standard','--','*.rs'],cwd=root,capture_output=True,text=True).stdout.split('\n')
corp = {}
for f in files:
    p = os.path.join(root, f)
    if f and '🧪️tests' not in f and not f.startswith('🧷️companions') and os.path.isfile(p):
        corp[f] = open(p, errors='replace').read()
for d in diag:
    m = re.match(r'L(\d+):C(\d+)', d)
    if not m: continue
    ln, col = int(m.group(1)), int(m.group(2))
    seg = test[ln - 1][max(0, col - 260):col + 5]
    lits = re.findall(r'(?:includes|indexOf|test)\((?:"((?:\\.|[^"\\])*)"|`((?:\\.|[^`\\])*)`)', seg)
    if not lits:
        print(f'L{ln}:C{col}  (no literal) {seg[-140:]}'); continue
    lit = [a or b for a, b in lits][-1]
    idents = sorted(set(re.findall(r'[A-Za-z_][A-Za-z0-9_]{5,}', lit)), key=len, reverse=True)[:3]
    print(f'L{ln}:C{col}  LIT={lit[:140]!r}')
    shown = 0
    for ident in idents[:2]:
        for f, t in corp.items():
            for i, line in enumerate(t.split('\n')):
                if ident in line and shown < 4:
                    print(f'      [{ident}] {f.split("🔌️plugin/")[-1][:70]}:{i+1}: {line.strip()[:170]}'); shown += 1
