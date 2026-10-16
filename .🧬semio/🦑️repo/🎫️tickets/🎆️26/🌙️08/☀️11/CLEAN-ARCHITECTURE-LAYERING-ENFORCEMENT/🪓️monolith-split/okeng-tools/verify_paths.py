# 🧭️ WP5/WP6 variant (reads all three touched logs). Path-literal resolver + rustfmt parse for every Rust file this task touched or created: `#[path]` and `include!`/`include_str!`/`include_bytes!` targets exist (inline-module nesting honoured), regions balanced, longest path.
import os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
_P23 = os.path.dirname(os.path.abspath(__file__)) + '/../okwp23-tools/touched.txt'
from okrs import tokenize
import okcrates as C
R = C.R
HERE = os.path.dirname(os.path.abspath(__file__))
files = set(l.strip() for log in (HERE + '/../okwp0-tools/touched.txt', _P23, HERE + '/../okwp56-tools/touched.txt', HERE + '/touched.txt') for l in open(log, encoding='utf-8') if l.strip().endswith('.rs'))
model = C.build()[1]
for k in C.ALL_KEYS:
    rf = C.node_abs(k, model) + '/🦀️.rs'
    if os.path.exists(rf): files.add(os.path.relpath(rf, R))
    tf = C.node_abs(k, model) + '/🧪️tests/🔬️unit/🦀️.rs'
    if os.path.exists(tf): files.add(os.path.relpath(tf, R))
hub = C.K + '/📥️mounts'
for dp, dn, fn in os.walk(hub):
    for f in fn: files.add(os.path.relpath(os.path.join(dp, f), R))
files = sorted(f for f in files if os.path.exists(os.path.join(R, f)))
bad = 0; longest = 0; checked = 0
for rel in files:
    p = os.path.join(R, rel)
    longest = max(longest, len(rel.encode()))
    r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--emit', 'stdout', '--config', 'skip_children=true', p], capture_output=True, text=True)
    if r.returncode != 0 and ('error' in r.stderr[:600]):
        bad += 1; print('RUSTFMT', rel[-70:], r.stderr[:140].replace('\n', ' '))
    src = open(p, encoding='utf-8').read()
    toks = [t for t in tokenize(src) if t[0] not in ('ws',)]
    stack = []; depth = 0; pending_path = None; i = 0
    mods = []   # (inline module name, depth-at-open, path-attr)
    d = 0
    depth_stack = []
    while i < len(toks):
        kd, tx, a, b = toks[i]
        if kd == 'punct' and tx == '{': d += 1
        if kd == 'punct' and tx == '}':
            d -= 1
            while depth_stack and depth_stack[-1][1] >= d + 1 and depth_stack[-1][1] > d: depth_stack.pop()
        if kd == 'id' and tx == 'mod' and i + 2 < len(toks) and toks[i + 1][0] == 'id' and toks[i + 2][1] == '{':
            attr = None
            j = i - 1
            # scan back over attributes for path
            k = i - 1
            text_before = src[max(0, a - 400):a]
            m = list(re.finditer(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]\s*(?:pub(?:\([^)]*\))?\s+)?$', text_before))
            attr = m[-1].group(1) if m else None
            depth_stack.append((toks[i + 1][1], d + 1, attr))
        kd2 = toks[i][0]
        if kd2 == 'str' and i >= 2:
            pass
        i += 1
    # literals
    for m in re.finditer(r'(#\[\s*path\s*=\s*"([^"]*)"\s*\]|include(?:_str|_bytes)?!\(\s*"([^"]*)"\s*\))', re.sub(r'(?m)^\s*//[^\n]*', '', src)):
        lit = m.group(2) or m.group(3)
        base = os.path.dirname(p)
        found = False
        # candidate bases: file dir, and file dir + enclosing inline module dirs (naive: try all subsets of module names declared inline in the file)
        names = re.findall(r'\bmod\s+([A-Za-z_0-9]+)\s*\{', src)
        cands = [base, base + '/.']
        for nm in names: cands.append(base + '/' + nm)
        for c in cands:
            t = os.path.normpath(os.path.join(c, lit))
            if os.path.exists(t): found = True; break
        checked += 1
        if not found and not lit.startswith('.') or (not found and lit.startswith('.') and lit != '.'):
            bad += 1; print('PATH', rel[-70:], lit[:90])
    reg = 0
    for l in src.split('\n'):
        s = l.strip()
        if s.startswith('//#region'): reg += 1
        elif s.startswith('//#endregion'): reg -= 1
        if reg < 0: break
    if reg != 0: bad += 1; print('REGION', rel[-70:], reg)
print('files', len(files), 'literals checked', checked, 'problems', bad, 'longest relative path bytes', longest)
sys.exit(1 if bad else 0)
