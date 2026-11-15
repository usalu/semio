import sys,json,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from laws import *
def worldcmp(tag,base,w):
    nodes=[r for r in w['rootNode'] if base['rootNode'].get(r)!=w['rootNode'][r]]
    keys=[r for r in w['rootKey'] if base['rootKey'].get(r)!=w['rootKey'][r]] if w['rootKey'] else None
    return nodes,keys
out={}
for tag in ('before','after'):
    b=get(tag,'base1')[0]['base']
    u=get(tag,'unref')[0]['unreferenced']
    a=get(tag,'all30')[0]['all30']
    nodes=[r for r in b['rootNode'] if b['rootNode'][r]!=a['rootNode'].get(r)]
    keys=[r for r in b['rootKey'] if b['rootKey'][r]!=a['rootKey'].get(r)]
    g=[x for x in b['groups'] if b['groups'][x]!=a['groups'].get(x)]
    un=[r for r in b['rootNode'] if b['rootNode'][r]!=u['rootNode'].get(r)]
    uk=[r for r in b['rootKey'] if b['rootKey'][r]!=u['rootKey'].get(r)]
    ug=[x for x in b['groups'] if b['groups'][x]!=u['groups'].get(x)]
    out[tag]=dict(owners=len(b['rootNode']),errors=b['errors'],unref=(len(un),len(uk),len(ug)),all30=(len(nodes),len(keys),len(g),len(b['groups'])),changed_nodes=nodes)
    ind={}
    for i in range(5):
        p=S+f'{tag}-ind{i}.json'
        if not os.path.exists(p):continue
        d=json.load(open(p))
        for name,w in d['worlds'].items():
            ch=[r for r in w['rootNode'] if b['rootNode'].get(r)!=w['rootNode'][r]]
            ind[name]=(len(w['rootNode']),len(ch))
    out[tag]['ind']=ind
json.dump(out,open(S+'laws-summary.json','w'),ensure_ascii=False,indent=1)
for tag in out:
    o=out[tag];print(tag,'owners',o['owners'],'unref (nodes,keys,groups)',o['unref'],'all30 (nodes,keys,groups,of)',o['all30'],'individual worlds',len(o['ind']))
