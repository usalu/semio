#!/usr/bin/env python3
"""Usage: anchors.py <test.ts> <corpus-root> ; for every readFileSync of the facade '../../🦀️.rs' list the string literals used after it and the corpus files (tracked + untracked, non-test .rs) that contain each literal."""
import re, sys, subprocess, os
test = sys.argv[1]; root = sys.argv[2]
files = subprocess.run(['git','-c','core.fsmonitor=false','ls-files','--cached','--others','--exclude-standard','--','*.rs'],cwd=root,capture_output=True,text=True).stdout.split('\n')
files = [f for f in files if f and '🧪️tests' not in f]
corpus = {f: open(os.path.join(root,f),errors='replace').read() for f in files if os.path.isfile(os.path.join(root,f))}
src = open(test).read()
pat = re.compile(r'readFileSync\(new URL\("\.\./\.\./🦀️\.rs",import\.meta\.url\),"utf8"\)')
sites = [m.start() for m in pat.finditer(src)]
for n, s in enumerate(sites):
    e = src.find('\n', s)
    nxt = [x for x in sites if x > s]
    end = min([e] + ([nxt[0]] if nxt and nxt[0] < e else []))
    seg = src[s:end]
    lits = re.findall(r'"((?:\\.|[^"\\]){3,})"|`((?:\\.|[^`\\]){3,})`', seg)
    lits = [a or b for a, b in lits]
    lits = [l for l in lits if not l.startswith('../') and not l.endswith('.json') and l not in ('utf8',)]
    line = src.count('\n', 0, s) + 1
    print(f'--- site {n} line {line}')
    for l in dict.fromkeys(lits):
        needle = l.replace('${name}', '')
        hits = [f for f, t in corpus.items() if needle in t]
        short = [h.split('🔌️plugin/')[-1] for h in hits]
        print(f'  {needle[:90]!r}: {len(hits)} {short[:4]}')

if len(sys.argv) > 3 and sys.argv[3] == 'roots':
    import json
    roots = json.load(open(sys.argv[4]))
    rootdirs = sorted({os.path.dirname(p): p for n, p in roots}.items(), key=lambda kv: -len(kv[0]))
    def owner(f):
        for d, p in rootdirs:
            if f.startswith(d + '/') or f == p:
                return p
        return '?'
    print('=== owner sets')
    for n, s in enumerate(sites):
        e = src.find('\n', s)
        nxt = [x for x in sites if x > s]
        end = min([e] + ([nxt[0]] if nxt and nxt[0] < e else []))
        seg = src[s:end]
        lits = re.findall(r'"((?:\\.|[^"\\]){3,})"|`((?:\\.|[^`\\]){3,})`', seg)
        lits = [a or b for a, b in lits]
        lits = [l for l in lits if not l.startswith('../') and not l.endswith('.json') and not l.startswith('[DEBUG]')]
        owners = {}
        for l in dict.fromkeys(lits):
            if len(l) < 8: continue
            hits = [f for f, t in corpus.items() if l in t]
            if 0 < len(hits) <= 3:
                for h in hits:
                    owners.setdefault(owner(h), []).append(l[:30])
        print(n, 'line', src.count('\n', 0, s) + 1, sorted(owners))
