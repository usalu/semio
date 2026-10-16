# 🧮️ WP4 item model: planner item graph + synthetic statics of component_persistent_local! blocks, orphan-bound assignment, rank table.
import json, re, collections, os, sys
from common import *

CR = json.load(open(GEN + '/pwp0-crates.json'))
RANK = {k: v['rank'] for k, v in CR.items()}
WP4_CRATES = [k for k, v in CR.items() if v['wp'] == 4 and v['kind'] == 'lower']
RANK['actor-host'] = CR['actor-host']['rank']
WP4F = lambda f: (f.startswith('⚛️reactor/') and not f.startswith(('⚛️reactor/🧵️executor/', '⚛️reactor/🪟️surfaces/'))) or f.startswith('🚪️lifetime/')

def synth_statics(items, defs, text):
    out = []
    for f, (L, R) in text.items():
        if not WP4F(f): continue
        n = len(L); i = 0
        while i < n:
            if re.match(r'\s*(?:crate::)?component_persistent_local!\s*\{', L[i]) :
                d = 0; j = i
                while j < n:
                    d += L[j].count('{') - L[j].count('}')
                    if d <= 0 and j > i or (d <= 0 and '{' in L[j] and '}' in L[j]): break
                    j += 1
                k = i + 1
                while k < j:
                    m = re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?static\s+([A-Z_][A-Z0-9_]*)\s*:', L[k])
                    if m:
                        e = k; depth = 0; seen = False
                        while e < j:
                            depth += L[e].count('{') + L[e].count('(') + L[e].count('[') - L[e].count('}') - L[e].count(')') - L[e].count(']')
                            if L[e].rstrip().endswith(';') and depth <= 0: break
                            e += 1
                        s = k
                        while s > i + 1 and R[s - 1].lstrip().startswith(('///', '#[', '//')): s -= 1
                        out.append(dict(file=f, s=s + 1, e=e + 1, kind='static', head=L[k].strip(), name=m.group(1), body='\n'.join(L[k:e + 1])))
                        k = e
                    k += 1
                i = j
            i += 1
    return out

def load():
    files, items, defs, owner = PT.build()
    text = {}
    for p in files:
        its, L, R = PT.outline(ROOT + p); text[p.replace(P, '')] = (L, R)
    owner2 = dict(owner)
    for it in items:
        if it['kind'] == 'impl' and it.get('trait') and it.get('target') and it['id'] not in owner2:
            d = [j for j in defs.get(it['target'], []) if items[j]['kind'] in ('struct', 'enum', 'union', 'type')]
            if len(d) == 1: owner2[it['id']] = d[0]
    size1, adj1, crate1 = PT.graph(items, defs, owner2)
    final1, moves1, _ = PT.simulate(size1, adj1, crate1, items)
    have = {(i['file'], i['defname']) for i in items if i['kind'] == 'static'}
    for sd in synth_statics(items, defs, text):
        if (sd['file'], sd['name']) in have: continue
        it = dict(id=len(items), file=sd['file'], s=sd['s'], e=sd['e'], kind='static', head=sd['head'], name=sd['name'], cfgtest=False, defname=sd['name'], toks=PT.tokens(sd['body']))
        items.append(it); defs[sd['name']].append(it['id'])
    return files, items, defs, owner, owner2, text, final1

if __name__ == '__main__':
    files, items, defs, owner, owner2, text = load()
    st = [i for i in items if i['kind'] == 'static' and WP4F(i['file'])]
    for s in st: print(s['file'], s['s'], s['e'], s['name'])
