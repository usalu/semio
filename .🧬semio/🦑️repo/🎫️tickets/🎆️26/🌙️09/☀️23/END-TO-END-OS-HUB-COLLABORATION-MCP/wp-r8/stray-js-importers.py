import os, re, subprocess, sys
root = '/Users/ueli/Documents/semio'
stray = [line.split('\t')[2].rstrip('\n') for line in open(os.path.join(root, '.tmp-ticket/wp-r8/generated/stray-js-present.tsv'), encoding='utf-8')]
stray_set = {os.path.normpath(p) for p in stray}
names = sorted({os.path.basename(p) for p in stray})
files = subprocess.run(['git', '-c', 'core.quotepath=false', 'ls-files', '-z'], cwd=root, capture_output=True).stdout.decode('utf-8').split('\0')
exts = ('.ts', '.tsx', '.mts', '.cts', '.js', '.mjs', '.cjs', '.json', '.jsonc', '.html', '.toml', '.md', '.rs', '.go', '.yaml', '.yml')
lit = re.compile(r'["\'`]([^"\'`\n]*?(?:' + '|'.join(re.escape(n) for n in names) + r'))["\'`]')
hits = []
for f in files:
    if not f.endswith(exts) or os.path.normpath(f) in stray_set: continue
    try: s = open(os.path.join(root, f), encoding='utf-8').read()
    except Exception: continue
    if not any(n in s for n in names): continue
    for m in lit.finditer(s):
        spec = m.group(1)
        cands = []
        if spec.startswith('.'): cands.append(os.path.normpath(os.path.join(os.path.dirname(f), spec)))
        cands.append(os.path.normpath(spec.lstrip('/')))
        for c in cands:
            if c in stray_set or any(c.endswith('/' + p) or p.endswith('/' + c) and '/' in c for p in stray_set):
                hits.append((f, spec)); break
for f, spec in hits: print(f, '->', spec)
print('hits', len(hits))
