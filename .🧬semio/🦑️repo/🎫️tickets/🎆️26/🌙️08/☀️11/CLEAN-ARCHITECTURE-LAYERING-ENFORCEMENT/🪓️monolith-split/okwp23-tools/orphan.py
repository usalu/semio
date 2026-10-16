# 🧱 Orphan-rule scan of the live tree: every trait impl (and every `artifact_retire_struct!` expansion) of the kernel and of the carved crates must have a local trait or a local self type (E0117 is invisible to the union differential).
# usage: orphan.py [crate-key ...]   (default: the kernel and every crate of the b* batches)
import os, re, sys, collections, glob
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, registry
from okrs import tokenize, items

MACROS = {'artifact_retire_struct': 'RetireOwned'}
FUNDAMENTAL = {'Box', 'Pin', 'Arc', 'Rc'}   # Box/Pin are fundamental; Arc/Rc are not, kept out of the transparent set below
TRANSPARENT = {'Box', 'Pin'}

def world_crate(w):
    return w.ident or 'kernel'

def defining_world(res):
    """-> crate world id of a resolved Def / Mod (None for extern / unresolved)"""
    if res is None or res.kind not in ('def', 'mod'): return None
    return res.mod.cw.ident if res.mod.cw is not None else None

def head_ident(ty):
    toks = [t for t in tokenize(ty) if t[0] not in ('ws', 'comment')]
    i = 0
    # peel references, dyn, and fundamental wrappers
    while i < len(toks):
        tx = toks[i][1]
        if tx in ('&', 'mut', 'dyn', "'", 'impl') or toks[i][0] == 'life': i += 1; continue
        if tx in ('(', '[') : return None
        if toks[i][0] == 'id':
            segs = [tx]; j = i + 1
            while j + 2 < len(toks) + 1 and j + 1 < len(toks) and toks[j][1] == ':' and toks[j + 1][1] == ':' and j + 2 < len(toks) and toks[j + 2][0] == 'id':
                segs.append(toks[j + 2][1]); j += 3
            if segs[-1] in TRANSPARENT and j < len(toks) and toks[j][1] == '<':
                i = j + 1; continue
            return segs
        i += 1
    return None

IMPL_RX = re.compile(r'^\s*impl\b\s*(?:<[^{]*?>)?\s*(?:(?P<neg>!)?\s*(?P<trait>[A-Za-z_][\w:<>,\s\'&\[\]()+?*-]*?)\s+for\s+)?(?P<ty>[^{]*?)\s*(?:where\b[^{]*)?$', re.S)

def impl_parts(header):
    h = header
    # strip leading generics `<...>` with nesting
    h = h.strip()
    assert h.startswith('impl')
    rest = h[4:].lstrip()
    if rest.startswith('<'):
        d = 0; i = 0
        while i < len(rest):
            if rest[i] == '<': d += 1
            elif rest[i] == '>':
                d -= 1
                if d == 0: i += 1; break
            i += 1
        rest = rest[i:].lstrip()
    rest = re.sub(r'\bwhere\b.*$', '', rest, flags=re.S).strip()
    m = re.search(r'(?<![\w])for\s+', rest)
    # `for` of a higher-ranked bound `for<'a>` is not an impl separator
    cand = [mm for mm in re.finditer(r'(?<![\w])for\s+(?!<)', rest)]
    if cand:
        k = cand[0]
        return rest[:k.start()].strip().lstrip('!'), rest[k.end():].strip()
    return None, rest

def scan_world(w, label, out):
    for m in w.mods:
        if m.key != m.file and m.parent is None: continue
        for it in m.impls:
            header = it['text']
            hd = header[:header.index('{')] if '{' in header else header
            hd = '\n'.join(l for l in hd.split('\n') if not l.strip().startswith('//') and not l.strip().startswith('#['))
            try: tr, ty = impl_parts(hd)
            except Exception: continue
            check(m, label, tr, ty, it, out, inherent=(tr is None))
        # macro expansions
    for f, m in w.files.items():
        try: src = open(f, encoding='utf8').read()
        except Exception: continue
        for name, trait in MACROS.items():
            for mm in re.finditer(name + r'!\s*\(\s*([A-Za-z_][\w:]*)', src):
                ty = mm.group(1)
                class It(dict): pass
                it = {'start': mm.start(), 'text': mm.group(0)}
                check(m, label, trait, ty, it, out, macro=name)

def local_to(m, label, segs):
    """does the path (as written in module m) name something defined in the crate world `label`?"""
    if not segs: return False
    r = ns.resolve(m, segs)
    if r.kind in ('extern', ns.EXTERN_GLOB, 'unresolved'):
        return False
    w = defining_world(r)
    return w == (m.cw.ident if m.cw else None)

def check(m, label, tr, ty, it, out, macro=None, inherent=False):
    trsegs = head_ident(tr) if isinstance(tr, str) else None
    tysegs = head_ident(ty)
    tl = local_to(m, label, trsegs) if trsegs else False
    yl = local_to(m, label, tysegs) if tysegs else False
    if tl or yl: return
    if inherent and not tysegs: return
    # primitive / std types as self type with a foreign trait are always orphan; unresolved names are reported as unknown
    out.append((label, m.file, (macro or 'impl') + ' ' + (tr or '')[:50] + ' for ' + (ty or '')[:60], tysegs, trsegs))

def main():
    registry.load()
    keys = sys.argv[1:]
    out = []
    w = ns.world()
    scan_world(w, 'kernel', out)
    B = {'child','ledger','replay','read','backbone','canonical_json','one_item_publication','operation_wire','snapshot_clone','one_item_batch','space_schema','retirement','owned_schema_fields','owned_schema_decode'}
    idents = [ 'semio_framework_' + k.replace('-', '_') for k in keys ] if keys else ['semio_framework_os_store_' + b for b in sorted(B)]
    for ident in idents:
        ew = ns.ext_world(ident)
        if ew is None: continue
        scan_world(ew, ident, out)
    by = collections.Counter(o[0] for o in out)
    print('orphan-suspect impls:', dict(by))
    for o in out[:int(os.environ.get('LIMIT', '80'))]:
        print(' ', o[0], o[1].replace(ns.R + '/', '')[-60:], '|', o[2], '|', o[3], o[4])

if __name__ == '__main__': main()
