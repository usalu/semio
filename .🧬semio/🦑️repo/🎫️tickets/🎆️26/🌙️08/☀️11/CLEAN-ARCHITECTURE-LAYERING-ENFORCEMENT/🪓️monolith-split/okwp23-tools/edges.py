# 🕸️ Item-level cross-crate reference edges of the kernel store tree under the planner's crate assignment, by real module-scope name resolution (not by global name uniqueness).
# usage: edges.py [crate ...]   prints for each (selected) crate: dependencies on other store crates, back edges (rank), externs, unresolved names.
import os, re, sys, collections, json
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, model, registry, assign
from okrs import tokenize
import okcrates

RANK = okcrates.RANK
KEYSET = set(RANK) | set(okcrates.COMPANIONS) | {'os-kernel'}

def item_tokens(fm, it):
    toks = [t for t in tokenize(it['text']) if t[0] not in ('ws', 'comment')]
    return toks

def name_uses(toks):
    """Yield (segments, kind) for path starts: `a::b::c`, bare uppercase idents, lowercase idents followed by `(`/`!`/`::`. Skips `.name` and definitions."""
    n = len(toks); i = 0
    while i < n:
        kd, tx, a, b = toks[i]
        if kd != 'id':
            i += 1; continue
        prev = toks[i - 1][1] if i else ''
        prevkd = toks[i - 1][0] if i else ''
        if prev == '.' or (prev == ':' and i >= 2 and toks[i - 2][1] == ':' and False):
            i += 1; continue
        if prev == ':' and i >= 2 and toks[i - 2][1] == ':':
            i += 1; continue
        if tx in ('fn', 'let', 'mut', 'ref', 'pub', 'use', 'mod', 'struct', 'enum', 'trait', 'impl', 'type', 'const', 'static', 'where', 'for', 'in', 'as', 'dyn', 'if', 'else', 'match', 'return', 'loop', 'while', 'move', 'unsafe', 'async', 'await', 'break', 'continue', 'true', 'false', 'self', 'Self', 'extern', 'macro_rules'):
            i += 1; continue
        if prevkd == 'id' and prev in ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'mod', 'let', 'mut', 'macro_rules', 'as'):
            i += 1; continue
        segs = [tx]; j = i + 1
        while j + 2 < n + 1 and j + 1 < n and toks[j][1] == ':' and toks[j + 1][1] == ':' and j + 2 < n and toks[j + 2][0] == 'id':
            segs.append(toks[j + 2][1]); j += 3
        nxt = toks[j][1] if j < n else ''
        upper = tx[0].isupper()
        if tx in ('crate', 'super') or len(segs) > 1 or upper or nxt in ('(', '!') or (nxt == '{' and False):
            yield segs, (toks[i][2])
        i = j if len(segs) > 1 else i + 1

def crate_of_def(res):
    """crate key of a resolved Def (module file + name), via the file model."""
    if res.kind != 'def': return None
    f = res.mod.file
    if not os.path.exists(f): return None
    if f not in model.cof()['items']: return model.crate_of_file(f)
    fm = model.file_model(f)
    it = res.item
    for x in fm.items:
        if x['start'] == it['start']: return x['crate'] or fm.main_crate()
    return fm.main_crate()

def analyse(files=None, verbose=False):
    registry.load()
    w = ns.world()
    edges = collections.defaultdict(lambda: collections.Counter())
    ex = collections.defaultdict(lambda: collections.defaultdict(list))
    externs = collections.defaultdict(collections.Counter)
    unresolved = collections.defaultdict(collections.Counter)
    for f, mod in sorted(w.files.items()):
        if files and f not in files: continue
        planner = f in model.cof()['items']
        fc = model.crate_of_file(f)
        if not fc: continue
        fm = model.file_model(f)
        for it in fm.items:
            if it['kind'] in ('use', 'mod', None, 'extern'): continue
            c = it['crate'] if planner else fc
            if not c: continue
            toks = item_tokens(fm, it)
            seen = set()
            for segs, pos in name_uses(toks):
                key = tuple(segs[:6])
                if key in seen: continue
                seen.add(key)
                r = ns.resolve(mod, segs)
                if r.kind == 'def':
                    d = crate_of_def(r)
                    if d and d != c:
                        edges[c][d] += 1
                        if len(ex[c][d]) < 3: ex[c][d].append((model.short(f)[-50:], it['name'] and it['name'][:30], '::'.join(segs[:4])))
                elif r.kind == 'extern' or r.kind == ns.EXTERN_GLOB:
                    head = (r.extern[0] if r.kind == 'extern' else r.extern[0][0])
                    externs[c][head] += 1
                elif r.kind == 'unresolved':
                    unresolved[c]['::'.join(r.extern[:3])] += 1
    return edges, ex, externs, unresolved

if __name__ == '__main__':
    sel = set(sys.argv[1:])
    edges, ex, externs, unresolved = analyse()
    for c in RANK + ['os-kernel']:
        if sel and c not in sel: continue
        if c not in edges and c not in externs: continue
        rk = RANK.index(c) if c in RANK else 99
        print('##', c)
        for d, n in sorted(edges.get(c, {}).items(), key=lambda kv: (RANK.index(kv[0]) if kv[0] in RANK else 99)):
            dr = RANK.index(d) if d in RANK else 99
            flag = 'BACK' if dr >= rk else 'ok'
            print(f'   -> {d} x{n} {flag}', ex[c][d][:2] if flag == 'BACK' else '')
        print('   externs:', ' '.join(f'{k}x{v}' for k, v in externs[c].most_common(40)))
        if unresolved[c]: print('   unresolved:', ' '.join(f'{k}x{v}' for k, v in unresolved[c].most_common(25)))
