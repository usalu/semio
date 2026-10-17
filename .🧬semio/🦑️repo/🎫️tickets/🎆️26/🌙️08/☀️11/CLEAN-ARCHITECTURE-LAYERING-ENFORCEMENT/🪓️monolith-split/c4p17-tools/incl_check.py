"""🔗 c4p17: every include!/include_str!/include_bytes! literal and every #[path] literal of the split resolves to the same absolute path as in the original.
usage: incl_check.py [stage|live]"""
import os, re, sys, collections
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p17 as P, mkspec as M
mode = sys.argv[1] if len(sys.argv) > 1 else 'stage'
RX = re.compile(r'(include!|include_str!|include_bytes!)\(\s*"([^"\\]+)"|#\[path\s*=\s*"([^"\\]+)"\]')
bad = 0
for tag, f in M.FILES.items():
    base = os.path.join(P.REPO, os.path.dirname(f))
    orig = open(os.path.join(T, 'orig', tag + '.rs.txt'), encoding='utf-8').read()
    def resolved(text, d, skip_own):
        out = collections.Counter()
        for m in RX.finditer(text):
            kind = m.group(1) or '#[path]'
            lit = m.group(2) or m.group(3)
            tgt = os.path.normpath(os.path.join(d, lit))
            out[(kind, tgt)] += 1
        return out
    O = resolved(orig, base, False)
    stage = os.path.join(P.SCR, 'stage', tag) if mode == 'stage' else base
    N = collections.Counter()
    files = [(stage, base)]
    for dp, dn, fn in os.walk(stage):
        pass
    sp = __import__('specs.' + tag, fromlist=['SPEC']).SPEC
    N += resolved(open(os.path.join(stage, '🦀️.rs'), encoding='utf-8').read(), base, False)
    mounts = set()
    for c in sp['children']:
        txt = open(os.path.join(stage, c['folder'], '🦀️.rs'), encoding='utf-8').read()
        r = resolved(txt, os.path.join(base, c['folder']), False)
        N += r
    own = {(k, t) for (k, t) in list(N) if t.startswith(base + os.sep) and t.endswith('/🦀️.rs') and os.path.dirname(os.path.dirname(t)) == base and os.path.basename(os.path.dirname(t)) in {c['folder'] for c in sp['children']}}
    for k in own:
        N.pop(k)
    miss, extra = O - N, N - O
    print(tag, 'literals orig', sum(O.values()), 'new (without own child mounts)', sum(N.values()), 'missing', sum(miss.values()), 'extra', sum(extra.values()))
    for (k, t), c in list(miss.items())[:5] + list(extra.items())[:5]:
        print('   ', k, t[-100:])
    bad += bool(miss or extra)
print('INCLUDE-RESOLUTION', 'OK' if not bad else 'DIFFERENCES')
