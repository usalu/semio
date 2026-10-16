"""🔎 Source-text assertions: for every test that include_str!s a Rust source, which carved files hold its asserted literals."""
import os, re, sys, collections
from lib import *
STG = STAGE
files = {}
for root, ds, fs in os.walk(STG):
    for f in fs:
        if f.endswith('.rs'):
            p = os.path.join(root, f)
            files[os.path.relpath(p, STG)] = open(p, encoding='utf-8').read()
def literals(body):
    out = []
    for m in re.finditer(r'"((?:[^"\\]|\\.)*)"', body):
        s = m.group(1)
        try:
            s = bytes(s, 'utf-8').decode('unicode_escape').encode('latin-1').decode('utf-8')
        except Exception:
            pass
        if len(s) > 6 and 'include_str' not in s and not s.startswith('../'):
            out.append(s)
    return out
rows = []
for rel, text in files.items():
    if 'include_str!' not in text: continue
    for m in re.finditer(r'\n(?:#\[[^\n]*\]\n)*(?:async )?fn (\w+)\([^)]*\)[^{]*\{', text):
        name = m.group(1)
        start = m.end()
        depth = 1; i = start
        while i < len(text) and depth:
            c = text[i]
            if c == '{': depth += 1
            elif c == '}': depth -= 1
            i += 1
        body = text[start:i]
        incs = re.findall(r'include_str!\("([^"]*🦀️\.rs)"\)', body)
        if not incs: continue
        lits = literals(body)
        rows.append((rel, name, incs, lits))
for rel, name, incs, lits in rows:
    print('##', rel, name, incs)
    for l in lits:
        hold = sorted(k for k, t in files.items() if l in t and '🧪️tests/' not in k)
        print('    ', repr(l[:70]), '->', [h.replace('/🦀️.rs','') for h in hold][:6])
