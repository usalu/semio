import json,os,sys,glob
root=sys.argv[1]
def dups(v,path,out):
    if isinstance(v,dict):
        m=v.get("modified")
        if isinstance(m,list):
            ids=[x.get("id") for x in m if isinstance(x,dict)]
            if len(ids)!=len(set(ids)): out.append(path+"/modified repeated ids")
        for k,x in v.items(): dups(x,path+"/"+k,out)
    elif isinstance(v,list):
        for x in v: dups(x,path+"[]",out)
for art in sorted(glob.glob(os.path.join(root,"*"))):
    fx=glob.glob(art+"/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations")
    if not fx: continue
    fx=fx[0]; d=n=o=w=b=0; ex=[]
    for leaf in sorted(os.listdir(fx)):
        for case in sorted(os.listdir(os.path.join(fx,leaf))):
            c=os.path.join(fx,leaf,case)
            if not os.path.isdir(c): continue
            b+=1
            f=os.path.join(c,"🔣️.diff.json")
            if os.path.isfile(f):
                out=[];dups(json.load(open(f,encoding="utf-8")),"",out)
                if out: d+=1; ex.append(leaf+"/"+case)
            try: oc=json.load(open(os.path.join(c,"🔣️.outcome.json")))
            except Exception: continue
            if oc.get("status")=="no-op" and not os.path.isfile(f): n+=1
            mj=open(os.path.join(c,"🔣️.mutation.json"),encoding="utf-8").read()
            if '"mutation"' in mj.split("\n")[1] if "\n" in mj else False: o+=1
    print(os.path.basename(art),"bundles",b,"dup-modified-diffs",d,"noop-without-diff",n,"old-mutation-key",o,ex[:2])
