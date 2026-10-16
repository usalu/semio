"""🔒 Hand-derives the root Cargo.lock package blocks of the WP8 crates (no cargo run is possible): dependency names from the manifests."""
import sys, os, re, json, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, common as C
FE = '️'
A_PKG = L.R + '/' + L.S_REL + '/🧪' + FE + 'testing/📦' + FE + 'packages/🦀' + FE + 'rust'
B_PKG = L.R + '/' + L.S_REL + '/🧪' + FE + 'testing/🎭' + FE + 'doubles/📦' + FE + 'packages/🦀' + FE + 'rust'
CRATES = [('os-store-testing-fixtures', A_PKG), ('os-store-testing-doubles', B_PKG)] + [(name, C.pkg_abs(bk)) for bk, (name, node) in C.BINS.items()]
def manifests(pairs):
    script = 'import {readFileSync} from "node:fs"; const out={}; for (const [k,p] of Object.entries(JSON.parse(process.argv[1]))) out[k]=Bun.TOML.parse(readFileSync(p,"utf8")); console.log(JSON.stringify(out))'
    return json.loads(subprocess.run(['bun', '-e', script, json.dumps({n: p + '/Cargo.toml' for n, p in pairs})], capture_output=True, text=True).stdout)
def dep_names(m):
    names = set()
    tables = [m.get('dependencies', {}), m.get('dev-dependencies', {})] + [t.get(k, {}) for t in m.get('target', {}).values() for k in ('dependencies', 'dev-dependencies')]
    for tb in tables:
        for k, spec in tb.items():
            names.add(spec.get('package', k) if isinstance(spec, dict) else k)
    return sorted('tokio-tungstenite 0.26.2' if n == 'tokio-tungstenite' else n for n in names)
def block(name, deps):
    return '[[package]]\nname = "%s"\nversion = "0.1.0"\ndependencies = [\n%s]\n' % (name, ''.join(' "%s",\n' % d for d in deps))
def main(apply):
    pairs = list(CRATES) + [('os-kernel', L.R + '/' + L.K_REL)]
    man = manifests(pairs)
    text = open(L.R + '/Cargo.lock', encoding='utf8').read()
    orig = text
    for n, p in pairs:
        full = 'semio-framework-' + n
        deps = dep_names(man[n])
        rx = re.compile(r'\[\[package\]\]\nname = "' + re.escape(full) + r'"\nversion = "[^"]*"\n(?:dependencies = \[\n(?: "[^"\n]+",\n)*\]\n)?')
        m = rx.search(text)
        new = block(full, deps) if deps else '[[package]]\nname = "%s"\nversion = "0.1.0"\n' % full
        if m:
            old = m.group(0)
            ver = re.search(r'version = "([^"]*)"', old).group(1)
            new = new.replace('version = "0.1.0"', 'version = "%s"' % ver)
            text = text[:m.start()] + new + text[m.end():]
        else:
            # insert in name order
            pos = None
            for mm in re.finditer(r'\[\[package\]\]\nname = "([^"]+)"', text):
                if mm.group(1) > full: pos = mm.start(); break
            text = text[:pos] + new + '\n' + text[pos:]
    print('lock changed', text != orig, len(text) - len(orig))
    if apply and text != orig:
        cur = open(L.R + '/Cargo.lock', encoding='utf8').read()
        if cur != orig: raise SystemExit('Cargo.lock changed concurrently')
        L.snap(L.R + '/Cargo.lock'); open(L.R + '/Cargo.lock', 'w', encoding='utf8').write(text); L.log_touch(L.R + '/Cargo.lock', 'M')
if __name__ == '__main__':
    main(len(sys.argv) > 1 and sys.argv[1] == '--apply')
