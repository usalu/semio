"""📦️ Crate catalog of the OS kernel family: member dirs of the root workspace, package names, lib roots, dependency tables (regex TOML reader, python 3.9 has no tomllib)."""
import re, os, json
R = '/Users/ueli/Documents/semio'
M_REL = '🧰️framework/🛍️products/💻️os/🔨️modules'
K_REL = '🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust'
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = HERE + '/../🗑️generated'
def members():
    s = open(R + '/Cargo.toml', encoding='utf8').read()
    m = re.search(r'\[workspace\]\s.*?members\s*=\s*\[(.*?)\]', s, re.S)
    return re.findall(r'"([^"]+)"', m.group(1))
def read_manifest(rel):
    p = R + '/' + rel + '/Cargo.toml'
    s = open(p, encoding='utf8').read()
    name = re.search(r'^\s*name\s*=\s*"([^"]+)"', s, re.M)
    pkg = re.search(r'\[package\](.*?)(?=^\[)', s, re.S | re.M)
    pn = re.search(r'^\s*name\s*=\s*"([^"]+)"', pkg.group(1), re.M).group(1) if pkg else None
    lib = re.search(r'^\[lib\](.*?)(?=^\[|\Z)', s, re.S | re.M)
    lp = re.search(r'path\s*=\s*"([^"]+)"', lib.group(1)).group(1) if lib and re.search(r'path\s*=\s*"([^"]+)"', lib.group(1)) else None
    ln = re.search(r'name\s*=\s*"([^"]+)"', lib.group(1)).group(1) if lib and re.search(r'name\s*=\s*"([^"]+)"', lib.group(1)) else None
    return dict(dir=rel, name=pn, lib_path=lp, lib_name=ln or (pn.replace('-', '_') if pn else None), text=s)
def catalog():
    out = {}
    for m in members():
        if m.startswith(M_REL) or m.startswith(K_REL):
            try:
                c = read_manifest(m)
            except Exception as e:
                continue
            if c['name']:
                lp = c['lib_path']
                c['root'] = os.path.normpath(R + '/' + m + '/' + (lp or 'src/lib.rs'))[len(R) + 1:]
                out[c['name']] = c
    return out
if __name__ == '__main__':
    cat = catalog()
    print(len(cat))
    for k, v in sorted(cat.items()):
        if 'os-' in k: print(k, v['root'].replace(M_REL, 'M'))
