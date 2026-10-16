# arity scan: every call site of a method/associated function defined in the instance family passes as many arguments as the definition takes.
# Definitions: all `fn` of the family modules (impl members and free functions). Calls: `.name(`, `::name(` and `name(` over the whole plugin tree.
import sys; sys.path.insert(0, '.')
from lib6 import *
from sig6 import parse_sig, split_args, match_close
import defscan6 as D

def defs_of(raw, M):
    out = []
    for m in re.finditer(r'(?m)^[ \t]*(?:#\[[^\n]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+|async\s+|unsafe\s+|extern\s+"[^"]*"\s+)*fn\s+(\w+)', M):
        s = m.start(); 
        try: sg = parse_sig(raw[s:s + 6000])
        except Exception: continue
        n = len(sg['params']) - (1 if sg['recv'] else 0)
        out.append((sg['name'], n, bool(sg['recv']), s))
    return out

def calls_of(raw, M):
    out = []
    kws = set('if while match for fn return in let loop async move unsafe impl where dyn as else pub use mod struct enum trait type const static ref mut await Some Ok Err None Box Vec Option Result'.split())
    for m in re.finditer(r'(?<![\w])([A-Za-z_]\w*)\s*(?:::\s*<[^()]*?>\s*)?\(', M):
        kw = m.group(1)
        if kw in kws: continue
        k = m.start() - 1
        while k >= 0 and M[k].isspace(): k -= 1
        dot = k >= 0 and M[k] == '.' and not (k >= 1 and M[k - 1] == '.')
        dcolon = k >= 1 and M[k] == ':' and M[k - 1] == ':'
        # a definition head `fn name(`
        k2 = m.start() - 1
        isdef = re.search(r'\bfn\s+$', M[max(0, m.start() - 8):m.start()]) is not None
        if isdef: continue
        ob = m.end() - 1
        cp = match_close(M, ob, '(', ')')
        if cp < 0: continue
        args = split_args(M, raw, ob + 1, cp)
        out.append((kw, len(args), dot, dcolon, m.start()))
    return out

def scan(root=ROOT, fam=None, files=None):
    fam = fam if fam is not None else D.family_files(root)
    famset = set(fam)
    defs = collections.defaultdict(list)
    allf = files if files is not None else [f.replace(PLUGP, '') for f in plug_files()]
    for r in sorted(set(allf) | famset):
        p = root + PLUGP + r
        if not os.path.exists(p): continue
        raw = open(p, encoding='utf-8').read(); M = mask(raw)
        for name, n, hasrecv, pos in defs_of(raw, M):
            defs[name].append((r in famset, n, hasrecv, r))
    viol = []
    checked = 0
    for r in sorted(set(allf) | famset):
        p = root + PLUGP + r
        if not os.path.exists(p): continue
        raw = open(p, encoding='utf-8').read(); M = mask(raw)
        for kw, nargs, dot, dcolon, pos in calls_of(raw, M):
            ds = defs.get(kw)
            if not ds or not any(d[0] for d in ds): continue
            # compare
            ok = False
            for fam_, n, hasrecv, dr in ds:
                if dot and hasrecv and nargs == n: ok = True
                elif dcolon and (not hasrecv and nargs == n or hasrecv and nargs == n + 1): ok = True
                elif not dot and not dcolon and (not hasrecv and nargs == n): ok = True
                elif not dot and not dcolon and hasrecv and nargs == n + 1: ok = True
            checked += 1
            if not ok:
                line = raw.count('\n', 0, pos) + 1
                viol.append((r, line, kw, nargs, sorted({(d[1], d[2]) for d in ds})))
    return viol, checked

def snapshot_files():
    base = SCR + '/pwp6-snap/' + PLUGP
    out = []
    for f in plug_files():
        r = f.replace(PLUGP, '')
        if os.path.exists(base + r): out.append(r)
    return out

def key(v): return (v[2], v[3], tuple(v[4]))

if __name__ == '__main__':
    from methods6 import INSTANCE_FILES
    base_root = SCR + '/pwp6-snap/'
    bv, bc = scan(base_root, fam=INSTANCE_FILES, files=snapshot_files())
    nv, nc = scan()
    bk = collections.Counter(key(v) for v in bv)
    new = []
    cnt = collections.Counter()
    for v in nv:
        k = key(v)
        cnt[k] += 1
        if cnt[k] > bk.get(k, 0): new.append(v)
    print('arity scan: baseline checked', bc, 'violations (false positives of the name matching)', len(bv), '| now checked', nc, 'violations', len(nv), '| NEW', len(new))
    for v in new[:80]: print('  NEW', v[0].split('/')[-3] + '/' + v[0].split('/')[-2], v[1], v[2], 'args', v[3], 'defs', v[4])
