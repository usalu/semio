# ✅️ Static gates on the repo state of every Rust file WP4 wrote or edited: rustfmt parse, path/include resolution, region balance, docstring emoji of new files.
import os, re, subprocess, sys
from common import *
touched = [l.strip() for l in open(GEN + '/pwp4-touched.txt', encoding='utf-8') if l.strip()]
extra = ['⚛️reactor/🚪️lifetime/🦀️.rs', '⚛️reactor/🚪️lifetime/🧪️tests/🚪️lifetime/🦀️.rs', '⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime/🦀️.rs', '⚛️reactor/🔄️turn/🦀️.rs', '🚪️lifetime/🦀️.rs', '🧷️companions/🍇️tests-07/🦀️.rs', '🌐host/🛬️lifted/💼️jobs/🦀️.rs', '👷️job/🛬️lifted/🏛️reserved/🦀️.rs', '🎮️command/🛬️lifted/📦️completed/🦀️.rs', '⚛️reactor/📥️cold-pair/🛬️lifted/🪟️surfaces/🦀️.rs']
files = sorted(set(touched + extra))
RX = re.compile(r'(?:#\[\s*path\s*=\s*"([^"]*)"|\binclude(?:_str|_bytes)?!\s*\(\s*"([^"]*)")')
bad = 0; fmt = 0; emoji_missing = []
new_files = set(touched)
for rel in files:
    p = PLUG + '/' + rel
    if not rel.endswith('.rs'): continue
    t = open(p, encoding='utf-8').read()
    r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--emit', 'stdout', '--config', 'skip_children=true', p], capture_output=True, text=True)
    if r.returncode != 0 and 'error' in r.stderr.lower(): fmt += 1; print('RUSTFMT', rel, r.stderr.strip().split('\n')[0][:160])
    for m in RX.finditer(t):
        lit = m.group(1) or m.group(2)
        tgt = os.path.normpath(os.path.join(os.path.dirname(p), lit))
        if not os.path.exists(tgt): bad += 1; print('PATH', rel, lit)
    st = []
    for l in t.split('\n'):
        mm = re.match(r'^\s*//#(region|endregion)\b(.*)$', l)
        if mm:
            if mm.group(1) == 'region': st.append(mm.group(2).strip())
            elif st: st.pop()
            else: bad += 1; print('REGION', rel)
    if st: bad += 1; print('REGION OPEN', rel, st)
    if rel in new_files and not re.match(r'^//! \S', t.split('\n')[0] or ''): emoji_missing.append(rel)
print('files', len(files), 'rustfmt failures', fmt, 'problems', bad, 'new files without a docstring first line', emoji_missing)
