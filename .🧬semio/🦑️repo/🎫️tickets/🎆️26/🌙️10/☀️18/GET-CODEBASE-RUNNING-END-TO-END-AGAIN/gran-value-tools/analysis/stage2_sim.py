"""🧪 Reach and churn-weighted rebuild of the second staged set (facade-tail from the repoint plan, hygiene from the ledger) on top of the first (dead edges, ordered wire, derive resources). Run with `cwd` = the generated folder."""
import sys, json, re, os
from sim3 import *
from tree import ROOT
CR = byname['semio-framework-native-build-compiler-resources']; PRINT = byname['semio-framework-print']
def stage1(E):
    E = clone(S1(E))
    E[N['intrinsic']].discard(N['ordered']); E[N['ordered']].add(N['intrinsic'])
    E[N['derive']].discard(CR); E['NEW:resources'] = {CR}; E[PRINT].add('NEW:resources')
    return E
def stage2(E):
    E = clone(E)
    plan = json.load(open('plan-g5.json'))
    for k in ['ordered', 'snapshot-projection', 'paged-text', 'retirement-queue', 'original-aliases']: E[FACADE].discard(N[k])
    for m in plan['manifests']:
        d = os.path.dirname(os.path.dirname(os.path.dirname(m['path'])))
        d = m['path'][:-len('/Cargo.toml')]
        old = open(os.path.join(ROOT, m['path']), encoding='utf-8').read()
        for name in re.findall(r'^(semio-framework-value-[a-z\-]+)\s*=', m['after'], re.M):
            if name not in old and name in byname:
                if d in C: E[d].add(byname[name])
    for e in json.load(open('../../gran-value-tools/data/hygiene-edges.json')):
        t = byname.get(e['dependency'])
        if t and e['dir'] in C: E[e['dir']].discard(t)
    return E
ch = json.load(open('churn.json'))
W = ['control-contract','allocation-return','retirement','shared-issuance','native-encoding','native-decoding','clone-source','retained-clone','paged-list','ordered','persistent','native-owned','factory','edit','registry','schema','retained-map','paged-text','snapshot-projection','original-aliases','retirement-queue','intrinsic','derive-expansion','derive']
def wm(E):
    num = den = 0
    for k in W:
        w = ch['semio-framework-value-' + k]; num += w * tr(E, [N[k]])[N[k]]; den += w
    w = ch['semio-framework-value']; num += w * tr(E, [FACADE])[FACADE]; den += w
    return round(num / den)
A = stage1(base); B = stage2(A)
print('weighted now', wm(base), 'stage1', wm(A), 'stage2', wm(B))
for k in ['retirement-queue', 'original-aliases', 'paged-text', 'ordered', 'snapshot-projection', 'retained-map', 'persistent', 'native-owned', 'factory', 'intrinsic', 'control-contract', 'derive', 'edit', 'registry', 'schema']:
    print('%-20s %5d %5d %5d' % (k, tr(base, [N[k]])[N[k]], tr(A, [N[k]])[N[k]], tr(B, [N[k]])[N[k]]))
h0 = chain(base); h2 = chain(B)
print('chain control-contract', h0(N['control-contract']), h2(N['control-contract']))
print('mean reach', round(allreach(base), 2), round(allreach(A), 2), round(allreach(B), 2))
