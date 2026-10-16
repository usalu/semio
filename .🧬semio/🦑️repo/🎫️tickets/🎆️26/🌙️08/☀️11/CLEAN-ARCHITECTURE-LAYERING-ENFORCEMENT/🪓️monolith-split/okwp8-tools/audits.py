"""🔍 Disk audits of the WP8 crates: unbound capitalized/lowercase names, orphan rule, path existence, private paths, definition scan."""
import sys, os, re, json, collections, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, audit as A, resolver as RS, pathscan as PS, defscan as DS
FE = '️'
census = json.load(open(L.GEN + '/okwp8-census.json'))
def files_of(crate):
    d = json.load(open(L.GEN + '/okwp8-census-%s.json' % crate))
    return [r['path'] for r in d['test native'] if r['kind'] != 'embedded' and r['path'].endswith('.rs')]
def texts_of(crate, skip=()):
    out = {}
    for f in files_of(crate):
        if any(s in f for s in skip): continue
        out[L.R + '/' + f] = open(L.R + '/' + f, encoding='utf8').read()
    return out
def glob_names(crate_text_map):
    return set()
if __name__ == '__main__':
    fa = texts_of('os-store-testing-fixtures'); fb = texts_of('os-store-testing-doubles')
    # A exports
    def exported(m): 
        names = set()
        for t in m.values(): names |= A.defined_names(t) | set(re.findall(r'\bfn\s+([a-z_][A-Za-z0-9_]*)', t)) | set(re.findall(r'\bmod\s+([a-z_]\w*)', t))
        return names
    ea, eb = exported(fa), exported(fb)
    prel = [t for p, t in fa.items() if 'prelude' in p][0]
    pn = A.use_leaves(prel) | A.defined_names(prel) | set(re.findall(r'pub mod (\w+)', prel))
    print('== caps audit')
    for nm, files, bound in (('A', fa, pn), ('B', fb, pn | ea)):
        res = A.audit_texts(files, bound, RS.exists)
        for p, m in res.items(): print(nm, L.rel(p)[-50:], dict(list(m.items())[:10]))
    for crate in census:
        if crate in ('os-store-testing-fixtures', 'os-store-testing-doubles', 'os-kernel'): continue
        files = texts_of(crate)
        res = A.audit_texts(files, ea | eb | pn, RS.exists)
        for p, m in res.items(): print(crate, L.rel(p)[-50:], dict(list(m.items())[:10]))
    print('== orphan audit')
    for crate in census:
        if crate in ('os-kernel',): continue
        files = texts_of(crate, skip=('prelude',))
        bad, inh = A.orphan_audit(files)
        for b in bad: print(crate, L.rel(b[0])[-45:], b[1:3], b[3][:90])
        for b in inh: print(crate, 'INHERENT', L.rel(b[0])[-45:], b[1:3])
