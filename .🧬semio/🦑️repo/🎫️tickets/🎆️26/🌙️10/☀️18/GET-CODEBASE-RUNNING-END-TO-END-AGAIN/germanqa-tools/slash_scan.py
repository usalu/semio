import sys,json,os,re,collections
sys.path.insert(0,".")
import gqlib
root="/Users/ueli/Documents/semio"
EN=re.compile(r"\b(?:the|of|and|is|are|to|for|with|not|that|this|from|when|which|must|may|by|an|a|in|on|it|its|at|as|be)\b")
DE=re.compile(r"\b(?:der|die|das|und|nicht|ist|sind|wird|werden|mit|von|auf|für|fuer|zu|im|bei|oder|ein|eine|einen|dem|den|des|aus|nur|noch|jede[rsnm]?|ohne|über|ueber|wenn|dass|als|durch|nach|vor|bis|auch|kein|keine)\b")
res=[];ambig=0
def split_candidates(s):
    best=None;score=0
    for m in re.finditer(r" / ",s):
        i=m.start(); l,r=s[:i],s[i+3:]
        le,ld=len(EN.findall(l)),len(DE.findall(l)); re_,rd=len(EN.findall(r)),len(DE.findall(r))
        sc=(le-ld)+(rd-re_)
        if le>=3 and rd>=3 and le>ld and rd>re_ and sc>score: best=(l,r);score=sc
    return best
for rel in gqlib.files_of(root,(".json",)):
    t=open(os.path.join(root,rel),encoding="utf-8").read()
    try: doc=json.loads(t)
    except: continue
    def walk(v,path):
        if isinstance(v,dict):
            for k,x in v.items(): walk(x,path+"/"+k)
        elif isinstance(v,list):
            for i,x in enumerate(v): walk(x,path+f"/{i}")
        elif isinstance(v,str) and " / " in v:
            c=split_candidates(v)
            if c: res.append((rel,path,c))
    walk(doc,"")
print(len(res), len({r for r,_,_ in res}))
by=collections.Counter(p.split("/")[-1] if not p.split("/")[-1].isdigit() else p.split("/")[-2] for r,p,c in res)
print(by.most_common(10))
import pickle; pickle.dump(res,open("../🗑️generated/germanqa/slash.pkl","wb"))
for r,p,(l,rt) in res[:3]: print(r[-60:],p,"\n  EN:",l[:150],"\n  DE:",rt[:150])
