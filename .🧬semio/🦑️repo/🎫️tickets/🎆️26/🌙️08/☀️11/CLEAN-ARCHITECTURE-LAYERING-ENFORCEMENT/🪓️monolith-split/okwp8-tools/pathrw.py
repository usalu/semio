"""🛣️ Token-level path rewriter: kernel-root/store-root relative paths -> canonical crate paths (or fixtures-crate paths)."""
import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
from okrs import tokenize
import usetree
import resolver as RS
FIX = 'semio_framework_os_store_testing_fixtures'
FIXB = 'semio_framework_os_store_testing_doubles'
OWNER = {}
KEYWORDS = {'impl', 'for', 'as', 'in', 'return', 'where', 'dyn', 'mut', 'ref', 'use', 'pub', 'let', 'match', 'if', 'else', 'while', 'move', 'const', 'static', 'unsafe', 'async', 'await', 'loop', 'break', 'continue', 'trait', 'type', 'fn', 'struct', 'enum', 'mod', 'extern'}
HEADS = {'crate', 'super', 'store', 'dsl', 'spr', 'vcs', 'semio_format', 'dsl_grammar', 'semio_framework_os_kernel'}
FIXTURE_MODS = {'demo', 'lossy', 'severity', 'timestamped', 'validated'}
class Rewriter:
    def __init__(self, mode, depth=1, fixtures_prefix=None, local_super=None, log=None, super_ok=True):
        """mode: 'f' (inside the fixtures crate: own items are `crate::X`) or 'bin' (fixtures crate is an external crate).
        depth: module depth of the file below the store root (1 = unit file / direct child of the store root is depth 1 as `super::` = store root)."""
        self.mode = mode; self.depth = depth; self.log = log if log is not None else []; self.super_ok = super_ok
        self.fix = 'crate' if mode in ('a', 'b') else FIX
        self.unresolved = []
    def fixtures_path(self, segs):
        own = OWNER.get(segs[0], 'A') if segs else 'A'
        if self.mode == 'a': return ['crate'] + segs
        if self.mode == 'b': return (['crate'] if own == 'B' else [FIX]) + segs
        return [FIX if own == 'A' else FIXB] + segs
    def resolve(self, head, segs, depth):
        """segs: simple segments after head (list). returns (canonical_segs, consumed_count) or None"""
        n = len(segs)
        for take in range(n, 0, -1):
            part = segs[:take]
            r = self.resolve_exact(head, part, depth)
            if r is not None: return r, take
        return None
    KERNEL_MACROS = {'space_members', 'field_set_mutations', 'field_set_mutations_binary', 'field_set_mutations_text', 'config_record', 'config_diff'}
    def resolve_exact(self, head, part, depth):
        if head == 'crate' and len(part) == 1 and part[0] in self.KERNEL_MACROS and self.mode == 'bin':
            return ['semio_framework_os_kernel', part[0]]
        if head == 'crate':
            if part[0] == 'os_store' and len(part) >= 2 and part[1] == 'tests':
                return self.fixtures_path(part[2:]) if len(part) > 2 else None
            if part[0] == 'os_store' and len(part) >= 2 and part[1] == 'fixture_mutations':
                return self.fixtures_path(part[2:]) if len(part) > 2 else None
            return RS.kroot(part)
        if head in RS.KERNEL_ALIASES or head == 'semio_framework_os_kernel':
            return RS.kroot(part)
        if head == 'super':
            return None
        return None
    def resolve_super(self, k, segs):
        """`super^k::segs` from a module `depth` levels below the store root."""
        target = self.depth - k
        if target < 0: return None
        if target == 0:
            # store root namespace
            for take in range(len(segs), 0, -1):
                part = segs[:take]
                if part[0] == 'tests' and take >= 2: return self.fixtures_path(part[1:]), take
                if part[0] == 'fixture_mutations' and take >= 2: return self.fixtures_path(part[1:]), take
                r = RS.sroot_name(part[0], tuple(part[1:]))
                if r: return r, take
            return None
        if target == 1:
            # the unit-file namespace: fixtures items
            return ('UNIT', segs), 1
        return None
    def leaf_rewrite(self, segs):
        segs = list(segs)
        if segs and segs[0] == '::': segs = segs[1:]
        if not segs: return None
        head = segs[0]
        if head == 'super':
            k = 0
            while k < len(segs) and segs[k] == 'super': k += 1
            r = self.resolve_super(k, segs[k:]) if (self.super_ok and segs[k:]) else None
            if r is None or (isinstance(r[0], tuple)): return None
            canon, take = r
            return canon + segs[k:][take:]
        if head in HEADS:
            if head == 'crate' and len(segs) == 1: return None
            r = self.resolve(head, segs[1:], self.depth)
            if r is None: return None
            canon, take = r
            return canon + segs[1:][take:]
        return None
    def rewrite_uses(self, text):
        toks = tokenize(text)
        sig = [i for i, t in enumerate(toks) if t[0] not in ('ws', 'comment')]
        pos = {i: k for k, i in enumerate(sig)}
        edits = []
        for k, i in enumerate(sig):
            t = toks[i]
            if t[0] != 'id' or t[1] != 'use': continue
            prev = toks[sig[k - 1]][1] if k > 0 else ';'
            if prev not in (';', '}', '{', ']', ')', 'pub', ''): continue
            # find end ';' at brace depth 0
            d = 0; e = None
            for kk in range(k + 1, len(sig)):
                x = toks[sig[kk]][1]
                if x == '{': d += 1
                elif x == '}': d -= 1
                elif x == ';' and d == 0: e = kk; break
            if e is None: continue
            stmt = text[toks[i][2]:toks[sig[e]][3]]
            try: leaves = usetree.parse_use(stmt)
            except Exception: continue
            if not any(l[0] and (l[0][0] in HEADS or l[0][0] == 'super' or (l[0][0] == '::' and len(l[0]) > 1 and l[0][1] in HEADS)) for l in leaves): continue
            if len(leaves) == 1 and leaves[0][0] in (['crate'], ['semio_framework_os_kernel']) and leaves[0][1] == 'store':
                edits.append((toks[i][2], toks[sig[e]][3], '')); continue
            outs = []; changed = False
            for segs, alias, glob in leaves:
                nsegs = self.leaf_rewrite(segs)
                if nsegs is None:
                    nsegs = list(segs)
                    if segs and ((segs[0] in HEADS and segs[0] != 'super') or (segs[0] == '::' and len(segs) > 1 and segs[1] in HEADS)):
                        self.unresolved.append(('use', list(segs)))
                else: changed = True
                if nsegs and nsegs[-1] == 'self': nsegs = nsegs[:-1]
                outs.append('use ' + '::'.join(nsegs) + ('::*' if glob else '') + ((' as ' + alias) if alias else '') + ';')
            if not changed: continue
            edits.append((toks[i][2], toks[sig[e]][3], ' '.join(outs)))
        for a, b, rep in sorted(edits, key=lambda x: -x[0]):
            text = text[:a] + rep + text[b:]
        return text
    def rewrite(self, text):
        text = self.rewrite_uses(text)
        toks = tokenize(text)
        out = []; i = 0; n = len(toks)
        def is_ws(t): return t[0] in ('ws', 'comment')
        def prev_sig(idx):
            j = idx - 1
            while j >= 0 and toks[j][0] in ('ws', 'comment'): j -= 1
            return j
        while i < n:
            k, tx, a, b = toks[i]
            lead_colon = False
            if k == 'punct' and tx == ':' and i + 1 < n and toks[i + 1][1] == ':' and (i == 0 or prev_sig(i) < 0 or toks[prev_sig(i)][1] not in (':',)) and i + 2 < n and toks[i + 2][0] == 'id' and toks[i + 2][1] in HEADS:
                # leading `::head`
                j0 = prev_sig(i)
                if (j0 >= 0 and toks[j0][0] == 'id' and toks[j0][1] not in KEYWORDS) or (j0 >= 0 and toks[j0][1] in ('>',)): out.append(text[a:b]); i += 1; continue
                lead_colon = True; start = i; i += 2; k, tx, a, b = toks[i]
            if k == 'id' and tx in HEADS:
                j0 = prev_sig(i) if not lead_colon else prev_sig(start)
                if j0 >= 0 and toks[j0][1] in ('.',) : out.append(text[a:b]); i += 1; continue
                if j0 >= 0 and toks[j0][1] == ':' and j0 - 1 >= 0 and toks[j0 - 1][1] == ':' and not lead_colon: out.append(text[a:b]); i += 1; continue
                # gather the simple run
                segs = []; j = i; head = tx; kk = 0
                if head == 'super':
                    while j < n and toks[j][1] == 'super':
                        kk += 1; j += 1
                        if j + 1 < n and toks[j][1] == ':' and toks[j + 1][1] == ':': j += 2
                        else: break
                    # now segs follow
                else:
                    j = i + 1
                    if j + 1 < n and toks[j][1] == ':' and toks[j + 1][1] == ':': j += 2
                    else:
                        # a bare head without path
                        out.append(text[a:b] if not lead_colon else '::' + text[a:b]); i += 1; continue
                end_idx = j
                simple = []
                while j < n and toks[j][0] == 'id':
                    simple.append(toks[j][1]); end_idx = j + 1
                    if j + 2 < n and toks[j + 1][1] == ':' and toks[j + 2][1] == ':' and j + 3 < n and toks[j + 3][0] == 'id': j += 3
                    else: break
                if head == 'super':
                    r = self.resolve_super(kk, simple) if (simple and self.super_ok) else None
                    if r is None:
                        out.append(text[toks[i][2]:toks[end_idx - 1][3]] if end_idx > i else text[a:b]); i = max(end_idx, i + 1); continue
                    canon, take = r
                    if isinstance(canon, tuple) and canon[0] == 'UNIT':
                        out.append(text[toks[i][2]:toks[end_idx - 1][3]]); i = end_idx; continue
                else:
                    r = self.resolve(head, simple, self.depth)
                    if r is None:
                        self.unresolved.append((head, simple))
                        out.append(text[toks[i][2] if not lead_colon else toks[start][2]:toks[end_idx - 1][3]] if simple else text[a:b]); i = max(end_idx, i + 1); continue
                    canon, take = r
                # emit canonical + remainder of simple run
                rem = simple[take:]
                piece = '::'.join(canon + rem)
                out.append(piece)
                i = end_idx
                continue
            out.append(text[a:b]); i += 1
        return ''.join(out)
if __name__ == '__main__':
    rw = Rewriter('f', 1)
    t = "let x = crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1, &mut o); impl crate::os_spr::Mutation<DemoSnapshot> for X {} fn f(){ ::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer; crate::os_store::tests::DemoMemberStoreProducer::<M>::new(); super::Mutation<P>; store::os_spr::ActorId(1) }"
    print(rw.rewrite(t)); print(rw.unresolved)
