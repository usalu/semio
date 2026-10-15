#!/usr/bin/env python3
import sys, os, json, hashlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from regrestore import *
from specs import SPECS

SNAP = os.path.join(TOOLS, '..', '🗑️generated', 'regrestore-snap')
os.makedirs(SNAP, exist_ok=True)


def load(spec):
    O = Orig(orig_text(spec['orig']), spec['id'])
    for inst in O.inst:
        if inst['name'] in spec.get('force_hollow', ()):
            inst['hollow'] = True
    files = tree_files(spec['parent'], spec['folders'], spec['exts'], spec['include'])
    Ts = []
    for f in files:
        Tt = Tfile(os.path.join(ROOT, f))
        Tt.keep_stray = set(spec.get('keep_stray', ()))
        Ts.append(Tt)
    return O, files, Ts


def short(p):
    return p.replace(ROOT + '/', '')


def analyze(spec, verbose=True):
    O, files, Ts = load(spec)
    info = dict(id=spec['id'], orig_names=sorted(O.names()), anom=O.anom, files=[])
    cover = [0] * len(O.code)
    holders = {}
    plans = []
    allsegs = []
    for T in Ts:
        segs = segments_for(O, T)
        cc, oo, added, fixes = plan_file(O, T, segs, T.names)
        T.fixes = fixes
        if T.dropped:
            print('   dropped segs', short(T.path)[-60:], [(d[0]['c0'], d[0]['c1'], d[1]) for d in T.dropped][:6])
        plans.append((T, segs, cc, oo, added))
        allsegs.append((T, segs))
    hol = hollow_plan(O, allsegs)
    if None in hol:
        print('   HOLLOW without location:', hol[None])
    hnames = set()
    for T, segs, cc, oo, added in plans:
        for c, lines, ids in hol.get(T.path, []):
            if any(O.inst[i]['name'] in T.names for i in ids):
                continue
            f = T.fidx[c]
            grp, rel, _ds, _de = gapinfo(T.code, 'rs' if T.path.endswith('.rs') else 'ts')
            if rel:
                _, c1 = expand_segment(grp, c, c, len(T.code))
                f = T.fidx[c1]
            ind = re.match(r'\s*', T.lines[f]).group(0)
            anc = set()
            for i_ in ids:
                p_ = O.inst[i_]['parent']
                while p_ is not None:
                    anc.add(O.inst[p_]['name'])
                    p_ = O.inst[p_]['parent']
            minfo_ = {n_: (k_, nm_) for n_, k_, nm_ in T.marks}
            k_ = f + 1
            while k_ < len(T.lines):
                if k_ in minfo_ and minfo_[k_][0] == 'close' and minfo_[k_][1] not in anc:
                    f = k_
                elif T.lines[k_].strip() != '':
                    break
                k_ += 1
            cc.setdefault(f + 1, []).extend([ind + l for l in lines])
            for i in ids:
                hnames.add(O.inst[i]['name'])
                added.append(i)
        for s in segs:
            for j in range(s['j0'], s['j1'] + 1):
                cover[j] += 1
        for iid in added:
            holders.setdefault(iid, set()).add(short(T.path))
    tree_names = set()
    for T in Ts:
        tree_names |= T.names
    added_names = set()
    for T, segs, cc, oo, added in plans:
        added_names |= {O.inst[i]['name'] for i in added}
    print(f'== {spec["id"]}: orig inst={len(O.inst)} names={len(O.names())} anom={O.anom}')
    print(f'   tree files={len(Ts)} names before={len(tree_names & O.names())} after={len((tree_names | added_names) & O.names())}')
    missing = O.names() - tree_names - added_names
    print(f'   still missing names: {sorted(missing)}')
    unc = [j for j in range(len(O.code)) if cover[j] == 0 and O.code[j] != '']
    print(f'   unclaimed non-blank O code lines: {len(unc)} / {len([x for x in O.code if x])}')
    if verbose:
        for T, segs, cc, oo, added in plans:
            for fx in T.fixes:
                print(f'   fix {short(T.path)[-70:]}: {fx}')
            if added:
                print(f'   + {short(T.path)[-90:]}: {len(added)} added; ' + ', '.join(sorted({O.inst[i]["name"] for i in added}))[:200])
        for inst in O.inst:
            if inst['id'] not in holders:
                print(f'   ! inst {inst["name"]} (O line {inst["line"]}) added nowhere (present in tree: {inst["name"] in tree_names})')
    return O, Ts, plans, unc


def ranges(lst):
    out = []
    for x in lst:
        if out and x == out[-1][1] + 1:
            out[-1][1] = x
        else:
            out.append([x, x])
    return out


def sha(t):
    return hashlib.sha1(t.encode('utf-8')).hexdigest()


def manifest_path(id):
    return os.path.join(SNAP, id + '.json')


def snapshot(spec, Ts):
    mp = manifest_path(spec['id'])
    if os.path.exists(mp):
        return json.load(open(mp))
    man = {short(T.path): dict(full=sha(T.text), stripped=sha(strip_markers(T.text)), names=sorted(T.names)) for T in Ts}
    json.dump(man, open(mp, 'w'), ensure_ascii=False, indent=1)
    return man


def apply(spec, dry=None):
    O, Ts, plans, unc = analyze(spec, verbose=False)
    snapshot(spec, Ts)
    n = 0
    for T, segs, cc, oo, added in plans:
        if not added and not T.fixes:
            continue
        for fx in T.fixes:
            print('   fix', short(T.path)[-60:], fx)
        new = apply_ops(T, cc, oo)
        assert strip_markers(new) == strip_markers(T.text), T.path
        if dry:
            out = os.path.join(dry, sha(T.path) + '.txt')
            open(out, 'w', encoding='utf-8', newline='').write(new)
        elif new == T.text:
            continue
        else:
            cur = read(T.path)
            assert cur == T.text, f'file changed under us: {T.path}'
            open(T.path, 'w', encoding='utf-8', newline='').write(new)
        n += 1
    print(f'applied to {n} files')


def verify(spec):
    O, files, Ts = load(spec)
    man = json.load(open(manifest_path(spec['id'])))
    names = set()
    bad = []
    codeid = True
    nb = 0
    for T in Ts:
        nm, errs = check_balance(T.text)
        names |= nm
        if errs:
            bad.append((short(T.path), errs))
        m = man.get(short(T.path))
        if m is None:
            continue
        if sha(strip_markers(T.text)) != m['stripped']:
            codeid = False
            bad.append((short(T.path), ['code differs from snapshot']))
        if nm:
            nb += 1
    gaps = []
    for T in Ts:
        for s in segments_for(O, T):
            if sum(1 for j in range(s['j0'], s['j1'] + 1) if O.code[j]) < 4:
                continue
            for iid in s['stack']:
                nm = O.inst[iid]['name']
                if nm not in T.names and (short(T.path), nm) not in gaps:
                    gaps.append((short(T.path), nm))
    on = O.names()
    print(f'   member-files-without-marker: {len(gaps)}')
    for p, nm in gaps[:15]:
        print('     GAP', p[-80:], nm)
    print(f'== {spec["id"]}: orig names={len(on)} tree names (all)={len(names)} orig-names-in-tree={len(on & names)} missing={sorted(on - names)}')
    print(f'   files={len(Ts)} files-with-markers={nb} unbalanced={len(bad)} code-identical-to-snapshot={codeid}')
    for p, e in bad[:10]:
        print('   BAD', p[-100:], e[:3])
    return not bad and not (on - names)


def preview(spec):
    O, files, Ts = load(spec)
    d = os.path.join(TOOLS, '..', '🗑️generated', 'regrestore-dry')
    for T in Ts:
        p = os.path.join(d, sha(T.path) + '.txt')
        if not os.path.exists(p):
            continue
        lines = read(p).split('\n')
        print('--', short(T.path)[-100:], len(lines))
        for n, ln in enumerate(lines):
            if MARK.match(ln.rstrip('\r')) and ln not in T.lines:
                nx = lines[n + 1].strip()[:60] if n + 1 < len(lines) else ''
                pv = lines[n - 1].strip()[:60] if n > 0 else ''
                print(f'   {n + 1:6d} {ln.strip()[:40]:40s} prev=[{pv}] next=[{nx}]')


def balance(spec):
    O, files, Ts = load(spec)
    print(f'== {spec["id"]}: files={len(Ts)}')
    for T in Ts:
        nm, errs = check_balance(T.text)
        if errs:
            print('  UNBAL', short(T.path)[-100:], errs[:6])


def check(spec):
    O, files, Ts = load(spec)
    names = set()
    bad = []
    for T in Ts:
        nm, errs = check_balance(T.text)
        names |= nm
        if errs:
            bad.append((short(T.path), errs[:2]))
    on = O.names()
    gaps = []
    for T in Ts:
        for s in segments_for(O, T):
            if sum(1 for j in range(s['j0'], s['j1'] + 1) if O.code[j]) < 4:
                continue
            for iid in s['stack']:
                nm = O.inst[iid]['name']
                if nm not in T.names and (short(T.path), nm) not in gaps:
                    gaps.append((short(T.path), nm))
    print(f'== {spec["id"]}: files={len(Ts)} orig names={len(on)} in-tree={len(on & names)} missing={sorted(on - names)} unbalanced={len(bad)} member-gaps={len(gaps)} anom={O.anom[:3]}')
    for p, e in bad[:6]:
        print('   UNBAL', p[-90:], e)
    for p, nm in gaps[:6]:
        print('   GAP', p[-90:], nm)


def show(spec):
    O, files, Ts = load(spec)
    for T in Ts:
        if not T.marks:
            continue
        print('--', short(T.path)[-110:], len(T.lines))
        for n, k, nm in T.marks:
            print(f'   {n + 1:6d} {k:5s} {nm}')


if __name__ == '__main__':
    cmd = sys.argv[1]
    ids = sys.argv[2:] or list(SPECS)
    for i in ids:
        if cmd == 'analyze':
            O, Ts, plans, unc = analyze(SPECS[i])
            print('   unclaimed ranges (code idx):', ranges(unc)[:40])
        elif cmd == 'apply':
            apply(SPECS[i])
        elif cmd == 'dry':
            d = os.path.join(TOOLS, '..', '🗑️generated', 'regrestore-dry')
            os.makedirs(d, exist_ok=True)
            apply(SPECS[i], d)
        elif cmd == 'verify':
            verify(SPECS[i])
        elif cmd == 'preview':
            preview(SPECS[i])
        elif cmd == 'balance':
            balance(SPECS[i])
        elif cmd == 'check':
            check(SPECS[i])
        elif cmd == 'show':
            show(SPECS[i])
