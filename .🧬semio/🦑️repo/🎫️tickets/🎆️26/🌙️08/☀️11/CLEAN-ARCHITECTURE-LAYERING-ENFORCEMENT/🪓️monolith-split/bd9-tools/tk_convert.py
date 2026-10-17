#!/usr/bin/env python3
"""bd9: canonical testkit idiom. usage: tk_convert.py HITS.json SELFDEV.json [--apply]  (cwd = repo root)"""
import json, re, sys, os
apply = '--apply' in sys.argv
hits = json.load(open(sys.argv[1], encoding='utf-8'))
selfdev = json.load(open(sys.argv[2], encoding='utf-8'))
stat = {'rs_files': 0, 'rs_sites': 0, 'toml': 0, 'targets': 0}
PAT = re.compile(r'(?<!any\(test, )feature\s*=\s*"testkit"')
for f in hits:
    t = open(f, encoding='utf-8').read()
    n, k = PAT.subn('any(test, feature = "testkit")', t)
    if k:
        stat['rs_files'] += 1
        stat['rs_sites'] += k
        if apply:
            open(f + '.bd9tmp', 'w', encoding='utf-8').write(n)
            os.replace(f + '.bd9tmp', f)
for f, name, val in selfdev:
    t = open(f, encoding='utf-8').read()
    line = re.compile(r'^[ \t]*"?' + re.escape(name) + r'"?[ \t]*=[ \t]*\{[^\n]*path\s*=\s*"\."[^\n]*\n', re.M)
    n, k = line.subn('', t)
    assert k == 1, (f, k)
    def add(m):
        block = m.group(0)
        if 'required-features' in block:
            return block
        stat['targets'] += 1
        return block.rstrip('\n') + '\nrequired-features = ["testkit"]\n' + ('\n' if block.endswith('\n\n') else '')
    n = re.sub(r'^\[\[test\]\]\n(?:(?!^\[).*\n?)*', add, n, flags=re.M)
    n = re.sub(r'\n+\[dev-dependencies\]\n(?=\n*(\[|\Z))', '\n', n)
    stat['toml'] += 1
    if apply:
        open(f + '.bd9tmp', 'w', encoding='utf-8').write(n)
        os.replace(f + '.bd9tmp', f)
print(json.dumps(stat))
