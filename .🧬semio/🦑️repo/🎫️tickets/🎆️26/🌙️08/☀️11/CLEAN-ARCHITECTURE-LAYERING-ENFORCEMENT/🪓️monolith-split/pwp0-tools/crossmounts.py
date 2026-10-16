# 🔗️ Mount edges (parent file -> child file) of the plugin lib+test closure whose two ends belong to different work packages: the places where a carve edits a foreign mount line.
# usage: python3 crossmounts.py [out.json]
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_tools as pt, crates as C
MOUNT = re.compile(r'((?:^[ \t]*#\[[^\n]*\]\n)*)^[ \t]*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', re.M)
PATHA = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCL = re.compile(r'include!\(\s*"([^"]+)"\s*\)')
def main():
    crates, order = C.build()
    wp = {k: c['wp'] for k, c in crates.items()}
    clo = pt.CLOSURE
    files = sorted({x['path'] for k in clo for x in clo[k] if x['path'].endswith('.rs')})
    import ownership as ow
    def wp_of(f):
        w = ow.owner_wp(f.replace(pt.P, ''))
        return w, pt.file_crate(f.replace(pt.P, ''))
    edges = []
    for f in files:
        text = open(pt.ROOT + f, encoding='utf-8').read()
        base = os.path.dirname(pt.ROOT + f)
        for m in MOUNT.finditer(text):
            pm = PATHA.search(m.group(1))
            if not pm: continue
            tgt = os.path.normpath(os.path.join(base, pm.group(1)))
            tf = tgt[len(pt.ROOT):] if tgt.startswith(pt.ROOT) else tgt
            edges.append((f, tf, m.group(2)))
        for m in INCL.finditer(text):
            tgt = os.path.normpath(os.path.join(base, m.group(1)))
            if tgt.endswith('.rs'): edges.append((f, tgt[len(pt.ROOT):], 'include!'))
    out = []
    for p, ch, name in edges:
        if not p.startswith(pt.P) or not ch.startswith(pt.P): continue
        pf, pc = wp_of(p); cf, cc = wp_of(ch)
        if '/🧪️tests/' in ch or '/🧪️tests/' in p: continue
        if pf is not None and cf is not None and pf != cf:
            out.append(dict(parent=p.replace(pt.P, ''), parentCrate=pc, parentWp=pf, child=ch.replace(pt.P, ''), childCrate=cc, childWp=cf, mod=name))
    if len(sys.argv) > 1: json.dump(out, open(sys.argv[1], 'w'), ensure_ascii=False, indent=1)
    print(len(edges), 'mount edges;', len(out), 'cross-WP (non-test)')
    for o in out: print(f"WP{o['parentWp']} {o['parent']} ({o['parentCrate']}) mounts WP{o['childWp']} {o['child']} ({o['childCrate']}) as {o['mod']}")
if __name__ == '__main__': main()
