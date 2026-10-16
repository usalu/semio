"""🔎 Finds exact-law selectors that still address the kernel lib but whose test now lives in a companion crate."""
import sys, os, re, json, subprocess, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, testnames
census = json.load(open(L.GEN + '/okwp8-census.json'))
name2crate = collections.defaultdict(set)
seen = set()
for crate in census:
    if crate == 'os-kernel': continue
    d = json.load(open(L.GEN + '/okwp8-census-%s.json' % crate))
    for r in d['test native']:
        if r['kind'] != 'embedded' and r['path'].endswith('.rs'):
            p = L.R + '/' + r['path']
            if p in seen: continue
            seen.add(p)
            for n in testnames.names(open(p, encoding='utf8').read()): name2crate[n].add(crate)
before = {l.split('\t')[0] for l in open(L.GEN + '/okwp8-tests-before.tsv', encoding='utf8') if l.strip()}
files = subprocess.run(['rg', '-l', '-F', 'semio-framework-os-kernel', '--glob', '*.ts', '--glob', '!node_modules', '--glob', '!.🧬semio', '--glob', '!target', L.R], capture_output=True, text=True).stdout.split('\n')
lit = re.compile(r'"([A-Za-z_][A-Za-z0-9_:]*)"')
for f in files:
    if not f or not os.path.isfile(f): continue
    t = open(f, encoding='utf8').read()
    if 'ExactCargoLaws' not in t and 'cargoArgs' not in t: continue
    for i, line in enumerate(t.split('\n'), 1):
        for m in lit.finditer(line):
            s = m.group(1); last = s.split('::')[-1]
            if last in name2crate and len(last) > 12:
                print(f.replace(L.R + '/', '')[-90:], i, s[:90], sorted(name2crate[last]), 'MOVED' if last in before else '')
