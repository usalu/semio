"""🔎 Focused cross-crate privacy scan (direction-aware): non-pub items/types of an upstream crate mentioned by a downstream crate."""
import os,re,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import xscan,ffu
from xscan import *
EDGES=[("model","document"),("model","copy"),("model","mutation_laws"),("model","snapshot_laws"),("document","copy"),("document","mutation_laws"),("document","snapshot_laws")]
def run(root,tests=False):
    ids=dict(ffu.CRATES);files=crate_files(root,ids)
    toks={}
    for cid,fs in files.items():
        s=set()
        for f in fs:
            if not tests and "🧪️" in f:continue
            s|=uses(f)[2]
        toks[cid]=s
    # items
    for up,down in EDGES:
        for f in files[up]:
            if not tests and "🧪️" in f:continue
            L,I=load(f)
            for it in top_items(L,I):
                classify(it)
                nm=it.get('name')
                if nm and it['kind'] in('fn','struct','enum','const','static','type','trait','union') and it['vis']!='pub' and nm in toks[down] and (len(nm)>5 or '_' in nm):
                    print(f"ITEM {up}->{down} {it['vis'] or 'private'} {it['kind']} {nm} ({os.path.relpath(f,root)}:{it['head']+1})")
                if nm and it['kind']=='struct' and nm in toks[down]:
                    priv=[]
                    for x in range(it['head']+1,it['end']):
                        if I[x]['depth']==1 and I[x]['state']=='code':
                            m=re.match(r'^\s*(pub(?:\([^)]*\))?\s+)?(\w+)\s*:',L[x])
                            if m and not (m.group(1) and m.group(1).strip()=='pub'):priv.append(m.group(2))
                    if priv and it['vis']=='pub':print(f"TYPE-WITH-PRIVATE-FIELDS {up}->{down} {nm}: {priv}")
                if it['kind']=='impl' and not re.search(r'\bfor\b',L[it['head']]):
                    m=re.match(r'\s*impl(?:<[^>]*>)?\s+([A-Za-z_]\w*)',L[it['head']])
                    ty=m.group(1) if m else None
                    if ty and ty in toks[down]:
                        for x in range(it['head']+1,it['end']):
                            if I[x]['depth']==1 and I[x]['state']=='code':
                                mm=re.match(r'^\s*(pub(?:\([^)]*\))?\s+)?((?:(?:unsafe|async|const)\s+)*)(fn|const)\s+(\w+)',L[x])
                                if mm and not (mm.group(1) and mm.group(1).strip()=='pub'):print(f"PRIVATE-METHOD {up}->{down} {ty}::{mm.group(4)} ({os.path.relpath(f,root)}:{x+1})")
if __name__=="__main__":
    run(os.path.abspath(sys.argv[1])+"/",tests="--tests" in sys.argv)
