#!/usr/bin/env python3
"""🔎 c4p17 live losslessness: line multiset of the original snapshot minus (parent + children) of the live tree, per tag. Normalised: stripped lines, `pub(super) `/`pub(crate) ` widening and `../` include re-bases removed."""
import os, sys, re, collections
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p17 as P, mkspec as M
bad = 0
for tag, f in M.FILES.items():
    orig = open(os.path.join(T, 'orig', tag + '.rs.txt'), encoding='utf-8').read().split('\n')
    base = os.path.join(P.REPO, os.path.dirname(f))
    sp = __import__('specs.' + tag, fromlist=['SPEC']).SPEC
    live = open(os.path.join(P.REPO, f), encoding='utf-8').read().split('\n')
    n = len(live)
    for c in sp['children']:
        t = open(os.path.join(base, c['folder'], '🦀️.rs'), encoding='utf-8').read().split('\n')
        live += t
        n += len(t)
    key = lambda l: re.sub(r'pub\((?:super|crate)\) ', '', l.strip()).replace('../', '')
    a = collections.Counter(key(l) for l in orig)
    b = collections.Counter(key(l) for l in live)
    miss = a - b
    extra = b - a
    print(tag, 'orig', len(orig), 'live', n, 'children', len(sp['children']), 'MISSING', sum(miss.values()), 'extra', sum(extra.values()))
    for l, c in list(miss.items())[:6]:
        print('   missing x%d: %r' % (c, l[:120]))
    bad += sum(miss.values())
print('LOSSLESS OK' if bad == 0 else 'LOSSLESS FAIL', bad)
