# 📐️ WP4 tier-1 plan: component struct of every tier-1 method, extra component parameters in rank order with mutability, derived from the re-homed call graph.
import re, sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rehome import *

STD_MUT = set('''push push_str push_back push_front pop pop_back pop_front insert remove clear take replace extend retain drain sort sort_by sort_by_key sort_unstable
truncate resize swap swap_remove dedup dedup_by dedup_by_key append entry or_insert or_insert_with or_default get_mut iter_mut values_mut as_mut as_deref_mut get_or_insert
get_or_insert_with last_mut first_mut split_off set reverse fill rotate_left rotate_right retain_mut remove_entry insert_str pop_first pop_last
last_entry first_entry and_modify borrow_mut lock write fetch_add fetch_sub store next peek_mut as_mut_slice chunks_mut'''.split())
MUT_NAMES = STD_MUT | set(open(GEN + '/rwp4-mutnames.txt').read().split())

def match_close_paren(s, i):
    d = 0
    while i < len(s):
        if s[i] in '([{': d += 1
        elif s[i] in ')]}':
            d -= 1
            if d == 0: return i
        i += 1
    return len(s) - 1

ASSIGN = re.compile(r'\s*(?:=(?!=|>)|\+=|-=|\*=|/=|%=|\|=|&=|\^=|<<=|>>=)')

def field_mutations(text, fields, var='self'):
    """set of fields (names in `fields`) that `self.<f>` may mutate within text (blanked)."""
    out = set()
    for m in re.finditer(r'(?<![A-Za-z0-9_])(?<![\w)\]]\.)' + var + r'\s*\.\s*([A-Za-z_]\w*)', text):
        f = m.group(1)
        if f not in fields: continue
        i = m.end()
        pre = text[max(0, m.start() - 12):m.start()]
        if re.search(r'&\s*mut\s*\(?\s*$', pre) or re.search(r'&\s*raw\s+mut\s*$', pre): out.add(f); continue
        # walk the access chain
        j = i
        while True:
            mm = re.match(r'\s*\.\s*([A-Za-z_]\w*)', text[j:])
            if mm:
                k = j + mm.end()
                nxt = text[k:k + 1]
                tm = re.match(r'\s*(::<[^>]*>)?\s*\(', text[k:])
                if tm:
                    name = mm.group(1)
                    if name in MUT_NAMES: out.add(f)
                    k2 = match_close_paren(text, k + tm.end() - 1)
                    j = k2 + 1
                    continue
                j = k; continue
            mm = re.match(r'\s*\.\s*\d+', text[j:])
            if mm: j += mm.end(); continue
            if text[j:j + 1] == '[':
                j = match_close_paren(text, j) + 1; continue
            mm = re.match(r'\s*\?', text[j:])
            if mm: j += mm.end(); continue
            break
        if ASSIGN.match(text[j:]): out.add(f)
    return out

def plan(force=()):
    mem, f2s, order, s2c, comps, home, orig, touch, calls, callers, lines = rehome(force=force)
    RK = {c: k for k, c in enumerate(RANK)}
    struct_rank = {s: i for i, s in enumerate(order)}
    crate_structs = collections.defaultdict(list)
    for s in order: crate_structs[s2c[s]].append(s)
    tier1 = {n for n in home if RK[home[n]] <= RK['shell-commands']}
    # struct home
    shome = {}
    for n in tier1:
        if mem[n][0]['recv'] == 'free' or (mem[n][0]['recv'] is None and not touch[n]): shome[n] = None; continue
        cands = [s for s in crate_structs[home[n]]]
        tch = [s for s in touch[n] if s in cands]
        shome[n] = max(tch, key=lambda s: struct_rank[s]) if tch else cands[0]
    # fields per struct
    sfields = collections.defaultdict(set)
    for f, s in f2s.items(): sfields[s].add(f)
    # direct mutations per method per struct
    dmut = {}
    for n, infos in mem.items():
        ms = set()
        for i in infos:
            ms |= field_mutations(i['unit'].text, set(f2s), i.get('var', 'self'))
        dmut[n] = {f2s[f] for f in ms}
    # needs
    # process in order callee-first: iterate to fixpoint
    needs = {n: set() for n in tier1}; mutn = {n: set() for n in tier1}
    for n in tier1:
        needs[n] |= {s for s in touch[n] if s != shome[n]}
        mutn[n] |= {s for s in dmut[n] if s != shome[n]}
    recv = {n: (mem[n][0]['recv']) for n in tier1}
    ch = True
    while ch:
        ch = False
        for n in tier1:
            for c in calls[n]:
                if c not in tier1: continue
                H = shome[c]
                add = set(needs[c])
                if H is not None and H != shome[n]: add.add(H)
                add.discard(shome[n])
                if not add <= needs[n]: needs[n] |= add; ch = True
                madd = {k for k in mutn[c]}
                if H is not None and H != shome[n] and recv[c] == 'mut': madd.add(H)
                madd.discard(shome[n])
                if not madd <= mutn[n]: mutn[n] |= madd; ch = True
    return dict(mem=mem, f2s=f2s, order=order, s2c=s2c, comps=comps, home=home, orig=orig, touch=touch, calls=calls, callers=callers, lines=lines, tier1=tier1, shome=shome, needs=needs, mutn=mutn, dmut=dmut, recv=recv, struct_rank=struct_rank)

THRESHOLD = 4

def plan_fixed():
    force = set()
    while True:
        P = plan(force)
        more = {n for n in P['tier1'] if len(P['needs'][n]) >= THRESHOLD and n not in force}
        if not more: P['force'] = force; return P
        force |= more

if __name__ == '__main__':
    P = plan_fixed()
    t1 = P['tier1']; needs = P['needs']; mutn = P['mutn']
    hist = collections.Counter(len(needs[n]) for n in t1)
    print('tier-1 methods', len(t1), 'forced to use-case', len(P['force']), 'foreign components', dict(sorted(hist.items())))
    cnt = collections.Counter(); ln = collections.Counter()
    for n in P['home']: cnt[P['home'][n]] += 1; ln[P['home'][n]] += P['lines'][n]
    for c in RANK: print(c, cnt[c], ln[c])
    bad = []
    for n in t1:
        if P['recv'][n] == 'ref' and (mutn[n] or any(P['recv'][c] == 'mut' and P['shome'][c] == P['shome'][n] for c in P['calls'][n] if c in t1)):
            bad.append(n)
    print('&self methods with mutating needs:', bad)
    out = {}
    for n in sorted(P['home']):
        out[n] = dict(crate=P['home'][n], tier1=n in t1, from_appendix_b=P['orig'][n])
        if n in t1:
            out[n].update(struct=P['shome'][n], recv=P['recv'][n], params=[[s, s in mutn[n]] for s in sorted(needs[n], key=lambda s: P['struct_rank'][s])])
    json.dump(out, open(GEN + '/rwp4-t1-plan.json', 'w'), ensure_ascii=False, indent=0)
    json.dump({n: v['crate'] for n, v in out.items()}, open(GEN + '/rwp4-homes.json', 'w'), ensure_ascii=False, indent=0)
