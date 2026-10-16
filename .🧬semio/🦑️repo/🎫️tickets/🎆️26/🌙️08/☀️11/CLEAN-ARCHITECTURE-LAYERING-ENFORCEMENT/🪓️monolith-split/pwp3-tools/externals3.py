"""📦️ External crates named by the post-lift files of each WP3 crate (comment- and string-stripped token evidence) against the external dependencies its Cargo.toml declares, and the externals each lift destination (other work packages' crates) must add. usage: CLOSURE=<closure.json> python3 externals3.py"""
import collections, json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import graph3 as g
pt = g.pt
ROOT = '/Users/ueli/Documents/semio/'
PL = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
MAN = json.load(open(HERE + '/../🗑️generated/pwp3-manifests.json'))
facade = MAN[PL[:-1] + '/📦️packages/🦀️rust']
ALIAS = {'dsl': 'semio_framework_os_kernel', 'protocol': 'semio_framework_os_kernel', 'store': 'semio_framework_os_kernel', 'vcs': 'semio_framework_os_kernel'}
def libnames(tbl):
    out = {}
    for k, v in tbl.items():
        if isinstance(v, dict) and v.get('package'): out[k.replace('-', '_')] = v['package'].replace('-', '_')
        else: out[k.replace('-', '_')] = k.replace('-', '_')
    return out
fac = libnames(facade.get('dependencies', {}))
for tgt in facade.get('target', {}).values(): fac.update(libnames(tgt.get('dependencies', {})))
inp = json.load(open(HERE + '/../🗑️generated/pwp3-census-input.json'))
ROOTS = {'child': '🪆️child', 'composition-emission': '🧩️composition', 'composition-publication': '🧩️composition/📬️publication', 'publication-mounted': '🧩️composition/📬️publication/🤝️group/🪟️mounted', 'document-media': '🗃️document', 'job': '👷️job', 'command': '🎮️command', 'peer': '👥️peer', 'retained-command': '🧵️retained-command', 'extension': '🧩️extension'}
def manifest(crate):
    key = [k for k in MAN if k == PL + ROOTS[crate] + '/📦️packages/🦀️rust'][0]
    return MAN[key]
from rsitems import split_items
CFGTEST = re.compile(r'#\[cfg\((?:test|any\(test[^\]]*)\)\]')
def stripped(path):
    raw = open(ROOT + path, encoding='utf8').read()
    keep = []
    for it in split_items(raw):
        if CFGTEST.search('\n'.join(l for l in it.text.split('\n')[:6] if l.lstrip().startswith('#['))): continue
        keep.append(it.text)
    return pt.strip('\n'.join(keep))
def used_externals(files):
    ids = set()
    for p in files:
        t = stripped(p)
        for m in re.finditer(r'(?<![\w:])(?:::)?([A-Za-z_]\w*)\s*::', t): ids.add(m.group(1))
        for m in re.finditer(r'\buse\s+(?:::)?([A-Za-z_]\w*)', t): ids.add(m.group(1))
        for m in re.finditer(r'\bextern\s+crate\s+(\w+)', t): ids.add(m.group(1))
        for m in re.finditer(r'#\[derive\(([^)]*)\)\]', t):
            for d in re.findall(r'([A-Za-z_]\w*)::', m.group(1)): ids.add(d)
    out = set()
    for i in ids:
        if i in ALIAS: out.add(ALIAS[i])
        elif i in fac: out.add(fac[i])
    return out
if __name__ == '__main__':
    for crate in ROOTS:
        files = inp['lib'].get(crate, []) + inp['test'].get(crate, [])
        lib_used = used_externals(inp['lib'].get(crate, []))
        all_used = used_externals(files)
        m = manifest(crate)
        decl = libnames({k: v for k, v in m.get('dependencies', {}).items() if not k.startswith('semio-framework-plugin-')})
        decl_l = set(decl.values()); dev = set(libnames(m.get('dev-dependencies', {})).values())
        print(f'{crate}: lib uses {len(lib_used)}; declared {len(decl_l)}')
        print(f'    used-not-declared (lib) {sorted(lib_used - decl_l)}')
        print(f'    declared-not-used  (lib+test) {sorted(decl_l - all_used - {"semio_framework_os_kernel"})}')
