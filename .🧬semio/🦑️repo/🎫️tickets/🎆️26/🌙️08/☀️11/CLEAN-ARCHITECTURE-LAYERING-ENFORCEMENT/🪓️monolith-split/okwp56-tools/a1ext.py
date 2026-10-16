# 🔧 A1 outside the store root: receiver field hops, the staging functions of the durable group over `StoreParts`, the member-group impl and the codec glue (PLAN-OS-KERNEL-CRATES.md 5.3, 7.6).
import re, collections
import em, rs
from okrs import tokenize
FIELD_COMP = em.FIELD_COMP
STRUCT = em.STRUCT

def hop(text, receivers, tolerate=()):
    """`recv.field` (not a call) gains the component hop: `recv.comp.field`; a receiver that follows `.` is a field path and is left alone"""
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    edits = []; n = len(toks)
    for i in range(1, n - 2):
        if toks[i][0] == 'id' and toks[i][1] in receivers and toks[i + 1][1] == '.' and toks[i + 2][0] == 'id' and toks[i + 2][1] in FIELD_COMP:
            if toks[i - 1][1] in ('.', ':') and toks[i - 1][1] == '.': continue
            nx = toks[i + 3][1] if i + 3 < n else ''
            if nx == '(' or (nx == ':' and i + 4 < n and toks[i + 4][1] == ':'): continue
            edits.append((toks[i][3], toks[i][3], '.' + FIELD_COMP[toks[i + 2][1]]))
    out = text
    for s, e, r in sorted(edits, key=lambda x: -x[0]): out = out[:s] + r + out[e:]
    return out, len(edits)

STAGE_FNS_MUT = ['stage_store_member', 'stage_prebuilt_batch_root', 'abort_staged_store_member', 'adopt_staged_store_member', 'clear_adopted_store_member', 'retire_unstaged_store_member']
STAGE_FNS_REF = ['validate_adopted_store_member', 'store_post_revision', 'recovery_frontier', 'store_member_identity', 'prepared_base_matches_store']
STAGE_IMPLS = {'bind': 'impl<P, Mutation> DurableStorePreparedOwnerV1<P, Mutation>', 'recover': 'impl<P, Mutation> DurableStoreBoundOutcomeV1<P, Mutation>'}

def to_parts_sig(text):
    text = re.sub(r'store: &mut ArtifactStore<(\w+), (\w+)>', r"store: &mut StoreParts<'_, \1, \2>", text)
    text = re.sub(r'store: &ArtifactStore<(\w+), (\w+)>', r"store: &StorePartsRef<'_, \1, \2>", text)
    return text

def convert_durable_group(src):
    its = rs.outline(src)
    L = src.split('\n')
    edits = []
    for it in its:
        name = it['name']
        body = '\n'.join(L[it['s'] - 1: it['e']])
        if it['kind'] == 'fn' and name in STAGE_FNS_MUT + STAGE_FNS_REF:
            new = to_parts_sig(body)
            new, n = hop(new, {'store'})
            if name == 'clear_adopted_store_member':
                new = new.replace('validate_adopted_store_member(store, visibility)?;', 'validate_adopted_store_member(&store.shared(), visibility)?;')
            edits.append((it['s'], it['e'], new))
        elif it['kind'] == 'impl' and name in STAGE_IMPLS.values():
            new = to_parts_sig(body)
            new, n = hop(new, {'store'})
            edits.append((it['s'], it['e'], new))
        elif it['kind'] == 'fn' and name == 'captured_store_snapshot':
            new, n = hop(body, {'store'})
            edits.append((it['s'], it['e'], new))
        elif it['kind'] in ('fn', 'impl') and name != 'validate_adopted_store_member':
            new, n = hop(body, {'parent_store', 'drawing_store', 'value_store'})
            # call sites of the stage set with a store argument
            def mut_call(m): return f"{m.group(1)}(&mut {m.group(2)}.parts()"
            def ref_call(m): return f"{m.group(1)}(&{m.group(2)}.parts_ref()"
            for fn in STAGE_FNS_MUT:
                new = re.sub(r'(?<![\w])(' + fn + r')\((parent_store|drawing_store|value_store)(?=[,)])', mut_call, new)
            for fn in STAGE_FNS_REF:
                new = re.sub(r'(?<![\w])(' + fn + r')\((parent_store|drawing_store|value_store)(?=[,)])', ref_call, new)
            new = re.sub(r'(DurableStoreBoundOutcomeV1::recover)\((parent_store|drawing_store|value_store)(?=,)', lambda m: f"{m.group(1)}(&{m.group(2)}.parts_ref()", new)
            new = re.sub(r'(\.bind)\((parent_store|drawing_store|value_store)(?=,)', lambda m: f"{m.group(1)}(&{m.group(2)}.parts_ref()", new)
            if new != body: edits.append((it['s'], it['e'], new))
    out = list(L)
    for lo, hi, newt in sorted(edits, key=lambda e: -e[0]):
        out[lo - 1: hi] = newt.split('\n')
    res = '\n'.join(out)
    anchor = 'use crate::os_store::{ArtifactStore, '
    assert res.count(anchor) == 1
    return res.replace(anchor, 'use crate::os_store::{ArtifactStore, StoreParts, StorePartsRef, ')

def convert_member_group(src):
    new = rewrite_self_fields(src)
    for fn in ('stage_prebuilt_batch_root', 'adopt_staged_store_member', 'abort_staged_store_member'):
        new, k = re.subn(r'durable_group::' + fn + r'\(self(?=,)', f'durable_group::{fn}(&mut self.parts()', new)
        assert k == 1, (fn, k)
    return new

def rewrite_self_fields(text):
    import a1gen
    return a1gen.rewrite_fields_only(text)
