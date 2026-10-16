"""🧭 Generator of the store test prelude: the names the former kernel store root offered to its test modules, as `pub use` of their owning crates."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
from okrs import tokenize
import resolver as RS, ns, usetree
NSR = RS.NS
KERNEL_ONLY_PREFIX = ('retained_codec_catalog',)
STD_LIKE = {'std', 'core', 'alloc', 'serde', 'serde_json', 'tokio', 'futures'}
def tokens_of(text):
    return {t[1] for t in tokenize(text) if t[0] == 'id'}
def explicit_entries():
    """name -> (statement text-ish parts) from the store root, with attributes (cfg) and the canonical path."""
    out = {}
    unders = []
    for r in NSR['items']:
        if r['kind'] != 'use': continue
        cfg = [a for a in r['attrs'] if re.match(r'#\s*\[\s*cfg', a) and not re.match(r'#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]', a)]
        for segs, alias, glob in usetree.parse_use(r['text']):
            if glob: continue
            segs = list(segs)
            if segs[-1] == 'self': segs = segs[:-1]
            name = alias or segs[-1]
            canon = RS.canon_path(segs)
            ent = dict(path=canon, alias=alias, cfg=cfg, line=r['a'], raw=segs)
            if name == '_': unders.append(ent)
            else: out.setdefault(name, ent)
    return out, unders
def trait_kind(path):
    from_ns = None
    try:
        w = ns.ext_world(path[0])
        if w is None: return None
        cur = ns.Res('mod', mod=w.root)
        for i, s in enumerate(path[1:]):
            if cur.kind != 'mod': return None
            nxt = ns.lookup(cur.mod, s, pub_only=True)
            if nxt is None: return None
            if nxt.kind == ns.EXTERN_GLOB:
                for c in nxt.extern:
                    r = trait_kind(c + list(path[i + 2:]))
                    if r: return r
                return None
            if nxt.kind == 'extern': return trait_kind(nxt.extern + list(path[i + 2:]))
            cur = nxt
        return cur.item['kind'] if cur.kind == 'def' else cur.kind
    except Exception:
        return None
def build(used_tokens, extra_names=()):
    ents, unders = explicit_entries()
    lines = []; missing = []; skipped_kernel = []
    names = set(used_tokens) | set(extra_names)
    for name, e in sorted(ents.items()):
        path = e['path']
        is_trait = trait_kind(path) == 'trait'
        if name not in names and not is_trait: continue
        if path and path[0] in KERNEL_ONLY_PREFIX:
            skipped_kernel.append(name); continue
        r = RS.exists(path)
        if r is False and (path[0] in ('semio_framework_dsl_record_derive', 'semio_framework_value_derive') or path in (['semio_framework_value', 'ToValue'], ['semio_framework_value', 'FromValue'])): r = None
        if r is False:
            missing.append((name, path)); continue
        use_name = name
        alias = e['alias']
        s = 'pub use ' + '::'.join(path) + ((' as ' + alias) if alias else '') + ';'
        if name not in names and is_trait: s = 'pub use ' + '::'.join(path) + ' as _;'
        lines.append(('\n'.join(e['cfg']) + '\n' if e['cfg'] else '') + s)
    # names that the store root took from its glob re-export of the ephemeral crate
    for nm in sorted(names):
        if nm in ents or nm in NSR['hubs'] or nm in NSR['defs']: continue
        if not nm or not (nm[0].isupper()): continue
        if RS.exists(['semio_framework_os_store_ephemeral', nm]) is True:
            lines.append('pub use semio_framework_os_store_ephemeral::' + nm + ';')
    for e in unders:
        r = RS.exists(e['path'])
        if r is False: missing.append(('_', e['path'])); continue
        lines.append(('\n'.join(e['cfg']) + '\n' if e['cfg'] else '') + 'pub use ' + '::'.join(e['path']) + ' as _;')
    # hub modules used as path heads
    hubs = []
    heads = set()
    for l in lines:
        m = re.search(r'pub use ([A-Za-z0-9_]+)::', l)
        if m: heads.add(m.group(1))
    for h, info in NSR['hubs'].items():
        if h not in names and h not in heads: continue
        body = []
        for segs, alias, glob in info['uses']:
            p = RS.canon_path(list(segs))
            if p and p[0] in KERNEL_ONLY_PREFIX: continue
            body.append('    pub use ' + '::'.join(p) + ('::*' if glob else '') + ((' as ' + alias) if alias else '') + ';')
        hubs.append('pub mod ' + h + ' {\n' + '\n'.join(body) + '\n}')
    return lines, hubs, missing, skipped_kernel
if __name__ == '__main__':
    d = json.load(open(os.path.dirname(os.path.abspath(__file__)) + '/../🗑️generated/okwp8-closure-before.json'))
    R = '/Users/ueli/Documents/semio/'
    toks = set()
    for r in d['test native']:
        if r['kind'] == 'embedded' or not r['path'].endswith('.rs'): continue
        if '🏪️store' not in r['path'] and '🚪️io' not in r['path'] and '🪐️space' not in r['path']: continue
        toks |= tokens_of(open(R + r['path'], encoding='utf8').read())
    lines, hubs, missing, sk = build(toks)
    print(len(lines), len(hubs), 'missing', len(missing), 'kernel-only', sk)
    for m in missing[:60]: print('MISSING', m)
    print(sum(len(l) for l in lines) + sum(len(h) for h in hubs))

def group_lines(lines):
    """Merge `pub use a::b::X;` / `pub use a::b::Y as Z;` statements without attributes into brace lists per module path."""
    plain = collections.OrderedDict(); rest = []
    for l in lines:
        m = re.match(r'^pub use ((?:[A-Za-z0-9_]+::)+)([A-Za-z0-9_]+)( as [A-Za-z0-9_]+)?;$', l)
        if m and '\n' not in l:
            plain.setdefault(m.group(1), []).append(m.group(2) + (m.group(3) or ''))
        else: rest.append(l)
    out = []
    for prefix, leaves in plain.items():
        leaves = sorted(set(leaves), key=lambda x: (x.lower(), x))
        if len(leaves) == 1: out.append('pub use ' + prefix + leaves[0] + ';')
        else: out.append('pub use ' + prefix + '{' + ', '.join(leaves) + '};')
    return out + rest
