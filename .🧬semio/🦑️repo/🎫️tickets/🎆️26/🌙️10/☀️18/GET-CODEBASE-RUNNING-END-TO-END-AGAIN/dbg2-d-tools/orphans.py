import re,sys,collections
R="/Users/ueli/Documents/semio"
D=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d";O=D+"/orig"
TOK=re.compile(r'''//[^\n]*|"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\\n])'|[A-Za-z_][A-Za-z0-9_]*''',re.S)
def idents(t,strings=True):
    c=collections.Counter()
    for x in TOK.findall(t):
        if x.startswith("//"):continue
        if x.startswith('"'):
            if strings:
                for y in re.findall(r"\{([A-Za-z_]\w*)",x):c[y]+=1
        elif re.match(r"[A-Za-z_]",x):c[x]+=1
    return c
only=set(sys.argv[1:])
for l in open(O+"/map.txt",encoding="utf8"):
    n,f=l.rstrip("\n").split(" ",1)
    if not f.endswith((".rs",".ts",".tsx")):continue
    if only and n not in only:continue
    a=open(f"{O}/{n}.bak",encoding="utf8").read();b=open(R+"/"+f,encoding="utf8").read()
    ia,ib=idents(a),idents(b)
    cands=[]
    for k,v in ia.items():
        r=v-ib.get(k,0)
        if r>0 and ib.get(k,0)<=2 and ib.get(k,0)>=1 and len(k)>1 and k not in ("self","true","false","let","mut","std","io","fn","use","pub","crate","super","Some","None","Ok","Err","unwrap","len","as","new","if","else","for","in","while","return","break","println","eprintln","console","log","const","await","async","assert","is","Write","write","the","and","of","to","a"):
            cands.append((k,ib.get(k,0),r))
    if cands:print(n,"/".join(f.split("/")[-4:]),cands[:14])
