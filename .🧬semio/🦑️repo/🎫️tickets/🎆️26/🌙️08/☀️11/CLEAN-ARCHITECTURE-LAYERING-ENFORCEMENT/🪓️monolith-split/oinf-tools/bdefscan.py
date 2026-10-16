"""Definition scan over the generated world crates: every non-pub definition must not be named by another crate or a test."""
import sys, os, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wgen import *
import bcrates

def idents_of(text, items):
    plain = set(); names = set()
    for it in items:
        plain |= it.plain | it.dots | it.fields
        for (a, b) in it.assoc: plain.add(a); plain.add(b)
        names |= set(re.findall(r'\b([a-z_][a-z0-9_]*)\s*:(?!:)', text[it.start:it.end]))
    return plain | names

def scan(root, folders, cids):
    files = {}
    for cid, folder in zip(cids, folders):
        main = f"{root}/{folder}/🦀️.rs"
        extra = [main]
        for sub in ('🧰️testkit', '⏯️tool-run-trace'):
            p = f"{root}/{folder}/{sub}/🦀️.rs"
            if os.path.exists(p): extra.append(p)
        files[cid] = ('lib', extra)
        tdir = f"{root}/{folder}/🧪️tests"
        tests = []
        if os.path.isdir(tdir):
            for d in sorted(os.listdir(tdir)):
                p = f"{tdir}/{d}/🦀️.rs"
                if os.path.exists(p): tests.append(p)
        files[cid + ':tests'] = ('tests', tests)
    used = {}
    defs = {}
    for key, (kind, paths) in files.items():
        u = set(); d = []
        for p in paths:
            text = open(p, encoding='utf-8').read()
            items, _ = chunk_file(text)
            u |= idents_of(text, items)
            if kind == 'lib' and not p.endswith('/🧰️testkit/🦀️.rs'):
                for it in items:
                    if it.kind in ('fn','struct','enum','union','trait','type','const','static'):
                        if it.vis != 'pub' and it.name: d.append((p, it.kind, it.name, it.vis))
                        if it.kind == 'struct':
                            kind2, fields = struct_field_spans(text, it)
                            for f in fields:
                                v = text[f['vis_a']:f['vis_b']] if f['vis_a'] is not None else ''
                                if v != 'pub' and f['name']: d.append((p, 'field', it.name + '.' + f['name'], v))
                    elif it.kind == 'impl' and it.impl_trait is None:
                        bs = text.index('{', it.start)
                        inner = text[bs + 1: it.end - 1]
                        sub, _ = chunk_file(inner)
                        for s in sub:
                            if s.kind in ('fn','const','type') and s.name and s.vis != 'pub': d.append((p, 'member', (it.impl_ty or '?') + '::' + s.name, s.vis))
        used[key] = u; defs[key] = d
    bad = []
    for cid in cids:
        others = set()
        for key, u in used.items():
            if key == cid: continue
            if key == cid + ':tests': pass
            others |= u
        for (p, kind, name, vis) in defs[cid]:
            leaf = name.split('.')[-1].split('::')[-1]
            if leaf in others: bad.append((cid, p[len(root):], kind, name, vis or 'private'))
    return bad

if __name__ == '__main__':
    root = sys.argv[1]
    bad = scan(root, [c[2] for c in bcrates.CRATES], [c[0] for c in bcrates.CRATES])
    print('definitions named by another crate but not pub:', len(bad))
    for b in bad[:80]: print(b)
