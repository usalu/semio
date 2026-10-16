"""📦️ Dependency check: external crate identifiers a crate's source closure names against the dependencies its manifest declares (specs are taken from the old top-crate manifest)."""
import re, sys, os, json, subprocess, collections
from model import *
import oracle2_closure as oc
PKG = T + '/📦️packages/🦀️rust'
HERE = os.path.dirname(os.path.abspath(__file__))
def toml(path): return json.loads(subprocess.check_output(['bun', HERE + '/toml.ts', path]))
def idents(manifest):
    d = toml(manifest); out = {}
    tables = [d.get('dependencies', {}), d.get('dev-dependencies', {})] + [v.get('dependencies', {}) for v in d.get('target', {}).values()]
    for tb in tables:
        for k, v in tb.items(): out[k.replace('-', '_')] = (k, v)
    return out
def check(crate):
    root = T + '/' + FOLDER[crate] + '/🦀️.rs'
    man = T + '/' + FOLDER[crate] + '/📦️packages/🦀️rust/Cargo.toml'
    have = idents(man); top = idents(PKG + '/Cargo.toml')
    local = set(); used = collections.Counter()
    files = oc.closure(root)
    for f in files:
        code = strip(open(f, encoding='utf8').read())
        for m in re.finditer(r'\bmod\s+(\w+)', code): local.add(m.group(1))
        for m in re.finditer(r'\bas\s+(\w+)', code): local.add(m.group(1))
        for m in re.finditer(r'(?<![\w:.$])([a-z_][a-z0-9_]*)::', code): used[m.group(1)] += 1
        for m in re.finditer(r'\buse\s+([a-z_][a-z0-9_]*)\b', code): used[m.group(1)] += 1
        for m in re.finditer(r'extern\s+crate\s+(\w+)', code): used[m.group(1)] += 1
    missing = sorted(n for n in used if n in top and n not in have and n not in local)
    return files, missing, top
if __name__ == '__main__':
    for c in sys.argv[1:]:
        files, missing, top = check(c)
        print(c, len(files), 'files; missing deps:', [(n, top[n][1]) for n in missing])
