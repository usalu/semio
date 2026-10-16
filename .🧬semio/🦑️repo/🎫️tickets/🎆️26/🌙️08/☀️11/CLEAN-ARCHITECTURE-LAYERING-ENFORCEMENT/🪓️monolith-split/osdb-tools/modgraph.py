"""🕸️ Module-level dependency graph of the db tree by defined-name usage."""
import os,sys,re,collections
from rs import strip,idents,defined_names
D='/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db'
MODS={'ids':'🆔️ids','durability':'💾️durability','policy':'🎚️policy','actor':'🎭️actor','observe':'👁️observe','security':'🔒️security','state':'🔘️state','conflict':'⚔️conflict','preview':'🔮️preview','projection':'📽️projection','cluster':'🌐️cluster','snapshot':'📸️snapshot','index':'🔢️index','query':'🔍️query','compact':'🗜️compact','wal':'📝️wal','sync':'🔄️sync','cli':'⌨️cli','storage':'🗄️storage','artifact':'🗿️artifact','engine':'⚙️engine','faulttest':'🧪️tests/🧯️fault-storage'}
def files_of(m, tests=False):
    base=os.path.join(D,MODS[m]); out=[]
    for root,ds,fs in os.walk(base):
        for f in fs:
            if f=='🦀️.rs':
                p=os.path.join(root,f)
                rel=os.path.relpath(p,base)
                ist=('🧪️tests' in rel)
                if m=='faulttest': ist=False
                if ist==tests: out.append(p)
    return out
def load(m,tests=False):
    t=''
    for p in files_of(m,tests): t+=strip(open(p,encoding='utf-8').read())+'\n'
    return t
if __name__=='__main__':
    defs={};texts={};ttexts={}
    for m in MODS:
        texts[m]=load(m); ttexts[m]=load(m,True)
        defs[m]=defined_names(texts[m])
    owners=collections.defaultdict(set)
    for m,s in defs.items():
        for n in s: owners[n].add(m)
    # also variants/consts etc. skipped
    skip={'new','tests','default','from','drop','fmt','next','get','set','len','run','open','close','read','write','poll','wake','test','main'}
    for label,tx in (('LIB',texts),('TEST',ttexts)):
        print('=====',label)
        for m in MODS:
            used=idents(tx[m])
            edges=collections.Counter()
            for n in used:
                if n in skip: continue
                o=owners.get(n)
                if o and m not in o and len(o)==1:
                    edges[next(iter(o))]+=1
            own=defs[m]
            print(f'{m:12s} ->',', '.join(f'{k}:{v}' for k,v in sorted(edges.items(),key=lambda x:-x[1])))
