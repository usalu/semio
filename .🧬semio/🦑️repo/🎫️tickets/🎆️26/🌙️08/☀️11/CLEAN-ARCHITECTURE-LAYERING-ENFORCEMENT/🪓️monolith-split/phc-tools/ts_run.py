#!/usr/bin/env python3
"""🧪️ Runs every plugin-host TypeScript test file with `bun test` and records pass/fail counts and the tail of the output, in the shape of the pre-carve baseline."""
import os, sys, json, re, subprocess
REPO = '/Users/ueli/Documents/semio'
H = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host'
files = []
for root, dirs, fs in os.walk(H):
    dirs[:] = [d for d in dirs if d not in ('node_modules', 'target', 'dist', '🗑️generated', '🧪️testing')]
    for f in fs:
        if f == '🟦️.ts' and os.path.basename(root) != '📦️packages' and '🧪️tests' in os.path.relpath(root, H).split('/'):
            files.append(os.path.relpath(os.path.join(root, f), H))
files.sort()
out = {}
for rel in files:
    p = subprocess.run(['bun', 'test', './' + os.path.relpath(os.path.join(H, rel), REPO)], capture_output=True, text=True, cwd=REPO, errors='replace', timeout=300)
    txt = p.stdout + p.stderr
    m1, m2 = re.search(r'(\d+) pass', txt), re.search(r'(\d+) fail', txt)
    out[rel] = dict(rc=p.returncode, passed=int(m1.group(1)) if m1 else 0, failed=int(m2.group(1)) if m2 else 0, crash=not (m1 or m2), tail=' | '.join(txt.strip().split('\n')[-12:])[-900:])
json.dump(out, open(sys.argv[1], 'w'), ensure_ascii=False, indent=1)
for k, v in out.items():
    print('OK  ' if v['rc'] == 0 else 'FAIL', v['passed'], v['failed'], k)
