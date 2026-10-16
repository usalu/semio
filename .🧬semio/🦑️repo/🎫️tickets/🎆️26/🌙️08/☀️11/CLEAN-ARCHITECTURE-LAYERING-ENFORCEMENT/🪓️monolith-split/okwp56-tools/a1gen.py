# 🏭 A1 generator: rewrites the store root so `ArtifactStore` is a thin composer of six component structs with their own impls and delegates (PLAN-OS-KERNEL-CRATES.md 7). Pure: text in, text out.
import re, collections, json, sys
import em, rs, a1lib, a1c1
from a1lib import *
from okrs import tokenize

MARK = re.compile(r'^\s*//#(end)?region\b\s*(.*)$')
STRUCT_WHERE = "where\n    P: Clone + ToValue + FromValue,\n    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,\n"
MARKER_FIELD = "    marker: std::marker::PhantomData<fn() -> (P, Mutation)>,\n"
NEEDS_MARKER = {'cursor', 'live'}
COMP_DOC = {
    'log': "📜️ The event-sourced history of one store: the envelope and its causal DAG, the applied and redo edit-id stacks, the checkpoint, actor, clock and merge policy.",
    'owners': "♻️ The retirement discipline of one store: the installed retirement factories, the snapshot-read leases, the displaced-owner buffer and the owned disposer every value swap pays into.",
    'wire': "📡️ The transport of one store: the attached backbone, the member inbox and the durable-group root.",
    'cursor': "🧭️ The revision mirror of one store: the content revision, its accumulator and mirror dirt, the generation and the last projection cause.",
    'live': "⚡️ The incrementally maintained fold of one store: the live snapshot, the tail undo cache, the supersessions, the prefix ring and the fold frontier.",
    'replay': "🐢️ The deferred reprojection of one store: the remote and local replay budgets, the pending reprojection and the pending command report.",
}

def region_events(lines, lo, hi):
    """marker lines in [lo, hi) as ('beg'|'end', name)"""
    ev = []
    for i in range(lo, hi):
        m = MARK.match(lines[i])
        if m: ev.append(('end' if m.group(1) else 'beg', m.group(2).strip(), i))
    return ev

def stack_before(lines, start_line, upto_line):
    """region stack formed by markers in lines[start_line:upto_line] (0-based, exclusive)"""
    st = []
    for i in range(start_line, upto_line):
        m = MARK.match(lines[i])
        if m:
            if m.group(1):
                if st: st.pop()
            else: st.append(m.group(2).strip())
    return st

def strip_leading_markers(mtext):
    out = []; lead = True
    for l in mtext.split('\n'):
        if lead and MARK.match(l): continue
        if lead and l.strip() and not l.strip().startswith('//') and not l.strip().startswith('#['):
            lead = False
        out.append(l)
    return '\n'.join(out)

def transition(cur, tgt):
    k = 0
    while k < len(cur) and k < len(tgt) and cur[k] == tgt[k]: k += 1
    out = ['    //#endregion ' + n for n in reversed(cur[k:])] + ['    //#region ' + n for n in tgt[k:]]
    return out

def field_block(lines, field):
    """doc lines + declaration line of a struct field, from the original struct text"""
    return None

def gen_structs(text, st, fields):
    L = text.split('\n')
    decl = {}
    for f in fields:
        i = f['line'] - 1
        docs = []
        j = i - 1
        while j > st['h'] - 1 and L[j].strip().startswith('///'):
            docs.insert(0, L[j]); j -= 1
        decl[f['name']] = '\n'.join(docs + [L[i]])
    out = []
    for c in COMPS:
        fs = [f for f in fields if FIELD_COMP[f['name']] == c]
        body = '\n'.join(decl[f['name']] for f in fs)
        marker = MARKER_FIELD if c in NEEDS_MARKER else ''
        out.append(f"/// {COMP_DOC[c]}\npub struct {STRUCT[c]}<P, Mutation>\n{STRUCT_WHERE}{{\n{body}\n{marker}}}")
    comp = '\n'.join(f"    pub(crate) {c}: {STRUCT[c]}<P, Mutation>," for c in COMPS)
    out.append(f"/// 🗃️ The composer of one store: the six components of [`StoreLog`], [`StoreOwners`], [`StoreWire`], [`StoreCursor`], [`StoreLive`] and [`StoreReplay`], the use cases that need several of them and the delegates of the public surface.\npub struct ArtifactStore<P, Mutation>\n{STRUCT_WHERE}{{\n{comp}\n}}")
    return '\n\n'.join(out)

def gen_parts():
    mut = '\n'.join(f"    pub {c}: &'a mut {STRUCT[c]}<P, Mutation>," for c in COMPS)
    ref = '\n'.join(f"    pub {c}: &'a {STRUCT[c]}<P, Mutation>," for c in COMPS)
    mk = ', '.join(f'{c}: &mut self.{c}' for c in COMPS)
    mr = ', '.join(f'{c}: &self.{c}' for c in COMPS)
    ms = ', '.join(f'{c}: &*self.{c}' for c in COMPS)
    return (f"/// 🧩️ The six components of one store borrowed at once: what staging code below the composer takes instead of `&mut ArtifactStore`.\npub struct StoreParts<'a, P, Mutation>\n{STRUCT_WHERE}{{\n{mut}\n}}\n\n"
            f"/// 🧩️ The shared counterpart of [`StoreParts`] for code that only reads the store.\npub struct StorePartsRef<'a, P, Mutation>\n{STRUCT_WHERE}{{\n{ref}\n}}\n\n"
            f"impl<'a, P, Mutation> StoreParts<'a, P, Mutation>\n{STRUCT_WHERE}{{\n    /// 🧩️ Reborrows the components for read-only helpers.\n    #[inline]\n    pub fn shared(&self) -> StorePartsRef<'_, P, Mutation> {{\n        StorePartsRef {{ {ms} }}\n    }}\n}}\n\n"
            f"impl<P, Mutation> ArtifactStore<P, Mutation>\n{STRUCT_WHERE}{{\n    /// 🧩️ Splits the composer into its component borrows.\n    #[inline]\n    pub fn parts(&mut self) -> StoreParts<'_, P, Mutation> {{\n        StoreParts {{ {mk} }}\n    }}\n\n    /// 🧩️ Shares the composer's components.\n    #[inline]\n    pub fn parts_ref(&self) -> StorePartsRef<'_, P, Mutation> {{\n        StorePartsRef {{ {mr} }}\n    }}\n}}")

def block_where(text, b):
    """clause text of an impl block header: from `where` to the opening brace"""
    hdr = '\n'.join(text.split('\n')[b['h'] - 1: b['h'] + 12])
    st = rs.strip(hdr)
    i = st.index('where'); j = st.index('{', i)
    return hdr[i:j].rstrip() + '\n'

def first_doc(mtext):
    for l in mtext.split('\n'):
        if l.strip().startswith('///') : return l.strip()
        if l.strip() and not l.strip().startswith(('//', '#[')): break
    return None

def leading_attrs(mtext):
    toks = [t for t in tokenize(mtext) if t[0] not in ('ws', 'comment')]
    out = []; k = 0
    while k < len(toks) and toks[k][1] == '#':
        s0 = toks[k][2]; k += 1
        while toks[k][1] != '[': k += 1
        d = 0
        while True:
            if toks[k][1] == '[': d += 1
            elif toks[k][1] == ']':
                d -= 1
                if d == 0: break
            k += 1
        out.append(mtext[s0:toks[k][3]]); k += 1
    return out

def lead_docs(mtext):
    out = []
    for l in mtext.split('\n'):
        t = l.strip()
        if t.startswith('///'):
            if t == '///' and out: break
            out.append(t)
            if len(out) >= 4: break
        elif t.startswith('//') or t == '': continue
        else: break
    return out

def header_text(mtext, sig):
    """attrs + docs + signature (up to the body opener) of a method"""
    return mtext[:sig['body_start']].rstrip()

def delegate_text(r, ms, vis_override=None):
    sig = r.sig
    L = r.text
    # signature without docs: from first token of fn prefix
    toks = sig['toks']
    # find start of visibility/fn prefix: first token after attrs
    k = 0
    while k < len(toks) and toks[k][1] == '#':
        k += 1
        while toks[k][1] != '[': k += 1
        d = 0
        while True:
            if toks[k][1] == '[': d += 1
            elif toks[k][1] == ']':
                d -= 1
                if d == 0: k += 1; break
            k += 1
    sig_start = toks[k][2]
    signature = L[sig_start:sig['body_start']].rstrip()
    names = param_names(sig)
    args = []
    for pat in names:
        assert len(pat) == 1 and re.match(r'[A-Za-z_]\w*$', pat[0]), ('delegate param pattern', r.name, pat)
        args.append(pat[0])
    mutable = mut_of(sig['recv'])
    needs = ', '.join(f'&{"mut " if mutable else ""}self.{c}' for c in r.need)
    parts = ([needs] if needs else []) + args
    gen = ''
    if sig['gen']:
        # method generics: forward as turbofish only when type parameters (not lifetimes) exist
        g = L[sig['gen'][0]:sig['gen'][1]]
        tys = [x.strip() for x in g[1:-1].split(',') if x.strip() and not x.strip().startswith("'")]
        if tys: gen = '::<' + ', '.join(re.split(r'[:=]', t)[0].strip() for t in tys) + '>'
    awaits = '.await' if sig['is_async'] else ''
    if sig['recv'] is None:
        call = f"{STRUCT[r.home]}::<P, Mutation>::{r.name}{gen}({', '.join(parts)}){awaits}"
    else:
        call = f"self.{r.home}.{r.name}{gen}({', '.join(parts)}){awaits}"
    attrs = ' '.join(leading_attrs(r.text))
    one = ' '.join(signature.split())
    return f"    /// 🪞️ Delegates to [`{STRUCT[r.home]}::{r.name}`].\n    {attrs + ' ' if attrs else ''}#[inline] {one} {{ {call} }}"

def gen_blocks(text, ms, blocks, delegate_names, stats):
    """-> {block index: replacement text} for the four impl blocks"""
    L = text.split('\n')
    out = {}
    for bi, b in enumerate(blocks):
        where = block_where(text, b)
        mlist = [r for r in ms.values() if r.block == bi]
        pieces = []
        # constants of the block
        for comp in COMPS + ['ops']:
            sub = [r for r in mlist if r.home == comp]
            consts = [(k, m) for k, m in CONST_ITEMS.items() if m['block'] == bi and CONSTS[k] == comp]
            if not sub and not consts: continue
            items = [(r.raw['s'], r.raw['h'], 'm', r) for r in sub] + [(m['s'], m['h'], 'c', (k, m)) for k, m in consts]
            items.sort(key=lambda x: x[0])
            cur = []; body = []
            for s, h, kind, obj in items:
                tgt = stack_before(L, b['h'], h - 1)
                body += transition(cur, tgt); cur = tgt
                if kind == 'm':
                    new, warns = rewrite_body(obj, ms)
                    stats['warns'] += warns
                    body.append(strip_leading_markers(new))
                else:
                    body.append(strip_leading_markers(obj[1]['text']))
                body.append('')
            body += transition(cur, [])
            ty = f'{STRUCT[comp]}<P, Mutation>' if comp != 'ops' else 'ArtifactStore<P, Mutation>'
            pieces.append(f"impl<P, Mutation> {ty}\n{where}{{\n" + '\n'.join(body).rstrip('\n') + "\n}")
        # delegates of this block
        dl = [r for r in mlist if r.home != 'ops' and r.name in delegate_names]
        if dl:
            body = []
            for r in dl:
                body.append(delegate_text(r, ms)); body.append('')
            stats['delegates'] += len(dl)
            pieces.append(f"impl<P, Mutation> ArtifactStore<P, Mutation>\n{where}{{\n" + '\n'.join(body).rstrip('\n') + "\n}")
        out[bi] = '\n\n'.join(pieces)
    return out

CONST_ITEMS = {}

def split_top(text, lo, hi):
    """split text[lo:hi] at depth-0 commas (brackets only; closures with `|a, b|` are not present in field initialisers of these literals)"""
    toks = [t for t in tokenize(text[lo:hi])]
    parts = []; cur_s = 0; d = 0
    for t in toks:
        if t[0] == 'punct':
            if t[1] in '([{': d += 1
            elif t[1] in ')]}': d -= 1
            elif t[1] == ',' and d == 0:
                parts.append((cur_s, t[2])); cur_s = t[3]
    parts.append((cur_s, hi - lo))
    return [(lo + a, lo + b) for a, b in parts if text[lo + a: lo + b].strip()]

def regroup_literal(text, open_idx, indent):
    """text[open_idx] == '{' of an `ArtifactStore { .. }` literal; returns (new_literal_text, end_idx_after_close)"""
    d = 0; i = open_idx
    toks = tokenize(text[open_idx:])
    end = None
    for t in toks:
        if t[0] == 'punct':
            if t[1] in '([{': d += 1
            elif t[1] in ')]}':
                d -= 1
                if d == 0: end = open_idx + t[3]; break
    inner_lo = open_idx + 1; inner_hi = end - 1
    entries = split_top(text, inner_lo, inner_hi)
    by = collections.defaultdict(list)
    for a, b in entries:
        s = text[a:b].strip()
        m = re.match(r'^([a-z_][a-z_0-9]*)\b\s*(:|$)', s)
        assert m, ('literal entry', s[:60])
        name = m.group(1)
        assert name in FIELD_COMP, ('unknown field in literal', name)
        by[FIELD_COMP[name]].append(s)
    names = [re.match(r'^([a-z_0-9]+)', text[a:b].strip()).group(1) for a, b in entries]
    assert sorted(names) == sorted(FIELD_COMP), ('literal field set', set(FIELD_COMP) ^ set(names))
    pad = ' ' * (indent + 4); pad2 = ' ' * (indent + 8)
    rows = []
    for c in COMPS:
        inner = ''.join(f'{pad2}{e},\n' for e in by[c])
        if c in NEEDS_MARKER: inner += f'{pad2}marker: std::marker::PhantomData,\n'
        rows.append(f'{pad}{c}: {STRUCT[c]} {{\n{inner}{pad}}},\n')
    return '{\n' + ''.join(rows) + ' ' * indent + '}', end

def rewrite_fields_only(text):
    """composer-side code: `self.field` (not a call) gains the component hop; everything else stays"""
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    edits = []
    n = len(toks)
    for i, t in enumerate(toks):
        if t[0] == 'id' and t[1] == 'self' and i + 2 < n and toks[i + 1][1] == '.' and toks[i + 2][0] == 'id' and toks[i + 2][1] in FIELD_COMP:
            nx = toks[i + 3][1] if i + 3 < n else ''
            if nx == '(' or (nx == ':' and i + 4 < n and toks[i + 4][1] == ':'): continue
            edits.append((t[3], t[3], '.' + FIELD_COMP[toks[i + 2][1]]))
    return apply_edits(text, edits)

def self_calls(text):
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    out = set()
    for i in range(len(toks) - 3):
        if toks[i][1] == 'self' and toks[i + 1][1] == '.' and toks[i + 2][0] == 'id' and toks[i + 3][1] in ('(', ':'):
            out.add(toks[i + 2][1])
    return out

def build_a1(text, ext_use=None):
    stats = collections.Counter(); stats['warns'] = []; stats['delegates'] = 0
    L = text.split('\n')
    its = rs.outline(text)
    st, fields = em.struct_fields(text)
    blocks = em.impl_blocks(text)
    tab = em.method_table(text)
    ms = load_methods(text)
    # constants
    CONSTS.clear(); CONST_ITEMS.clear()
    for k, m in tab.items():
        if not k.startswith('<const'): continue
        nm = k.split()[1].rstrip('>')
        refs = [r for r in ms.values() if re.search(r'\bSelf::' + nm + r'\b', r.text)]
        homes_ = {r.home for r in refs}
        assert homes_, ('const unused', nm)
        CONSTS[k] = min(homes_, key=lambda c: RANK[c])
        CONST_ITEMS[k] = m
    ext_use = ext_use or {}
    dnames = set()
    for r in ms.values():
        if r.home == 'ops':
            dnames |= {c for c in r.callees if ms[c].home != 'ops'}
    drop = next(i for i in its if i['kind'] == 'impl' and 'Drop for ArtifactStore<P, Mutation>' in i['name'])
    smem = next(i for i in its if i['kind'] == 'impl' and 'SpaceMember for ArtifactStore<P, Mutation>' in i['name'])
    cfg = next(i for i in its if i['kind'] == 'fn' and i['name'] == 'config_store_from_initialized_runtime_with_owners')
    for it in (drop, smem):
        dnames |= {c for c in self_calls('\n'.join(L[it['s'] - 1: it['e']])) if c in ms and ms[c].home != 'ops'}
    for r in ms.values():
        if r.home != 'ops' and (r.vis_pub or r.name in ext_use):
            dnames.add(r.name)
    dnames |= set(a1c1.HOST_DELEGATES)
    STATIC_EXT = {'empty_batch_stage', 'empty_batch_edit', 'replay_suffix_partitioned'}
    dnames |= {n for n in STATIC_EXT if n in ms}
    dnames = {n for n in dnames if n in ms and ms[n].home != 'ops' and (ms[n].recv is not None or n in STATIC_EXT)}
    stats['delegate_names'] = len(dnames)
    gb = gen_blocks(text, ms, blocks, dnames, stats)
    # construct literal regroup
    for bi in gb:
        txt = gb[bi]
    edits = []   # (lo_line, hi_line, text) 1-based inclusive
    edits.append((st['s'], st['e'], gen_structs(text, st, fields) + '\n\n' + gen_parts()))
    for bi, b in enumerate(blocks):
        edits.append((b['s'], b['e'], gb[bi] + ('\n\n' + a1c1.HOST_IMPL.rstrip('\n') if bi == 0 else '')))
    dt = '\n'.join(L[drop['s'] - 1: drop['e']])
    edits.append((drop['s'], drop['e'], rewrite_fields_only(dt)))
    sm = '\n'.join(L[smem['s'] - 1: smem['e']])
    edits.append((smem['s'], smem['e'], rewrite_fields_only(sm)))
    ct = '\n'.join(L[cfg['s'] - 1: cfg['e']])
    k = ct.index('ArtifactStore {\n')
    ob = ct.index('{', k)
    lit, endi = regroup_literal(ct, ob, 4)
    edits.append((cfg['s'], cfg['e'], ct[:ob] + lit + ct[endi:]))
    out = list(L)
    for lo, hi, newt in sorted(edits, key=lambda e: -e[0]):
        out[lo - 1: hi] = newt.split('\n')
    new = '\n'.join(out)
    return new, ms, stats

def post_construct(new):
    """regroup the `Self { .. }` literal of `construct`"""
    c0 = new.index('async fn construct<C:ArtifactStorePublication<P>>')
    k = new.index('        Ok(Self {\n', c0)
    ob = new.index('{', k + 8)
    lit, endi = regroup_literal(new, ob, 8)
    return new[:ob] + lit + new[endi:]

if __name__ == '__main__':
    t = em.read()
    ext = json.load(open(em.HERE + '/../🗑️generated/okwp56-ext-method-use.json'))
    new, ms, stats = build_a1(t, ext)
    new = post_construct(new)
    new = a1c1.patch_c1(new)
    assert new.count('store.envelope.') == 2 or new.count('store.envelope.active') == 2, new.count('store.envelope.')
    new = new.replace('store.envelope.active_alternative_id = saved_line.clone();', 'store.log.envelope.active_alternative_id = saved_line.clone();').replace('store.envelope.viewer_checkpoint_id = saved_checkpoint.clone();', 'store.log.envelope.viewer_checkpoint_id = saved_checkpoint.clone();')
    print('delegates', stats['delegates'], 'names', stats['delegate_names'])
    from collections import Counter
    print(Counter(w[0] for w in stats['warns']))
    open(em.HERE + '/../🗑️generated/okwp56-a1-root.rs', 'w', encoding='utf8').write(new)
    print(len(t.split('\n')), '->', len(new.split('\n')))
