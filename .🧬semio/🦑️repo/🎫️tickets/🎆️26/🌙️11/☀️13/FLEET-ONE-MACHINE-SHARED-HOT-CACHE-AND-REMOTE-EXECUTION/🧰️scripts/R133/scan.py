import os,re,sys,collections
ROOT="/Users/ueli/Documents/semio/🧰️framework"
SCOPES=[ROOT+"/🛍️products/🦑️repo/🔨️modules/📚️library",ROOT+"/🔨️modules/🏃️process"]
EXT=(".ts",".mjs")
def files():
    for s in SCOPES:
        for d,dn,fn in os.walk(s):
            dn[:]=[x for x in dn if x not in("node_modules","target",".cache") and "🧪️tests" not in x and "🧫️fixtures" not in x]
            for f in fn:
                if f.endswith(EXT) and not f.endswith(".d.ts") and not f.endswith(".d.mts"): yield os.path.join(d,f)
PAT=re.compile(r'''(?:"\.cache|"\.nx|'\.cache|'\.nx|`[^`]*\.cache|`[^`]*\.nx\b|/\.cache/|/\.nx/)''')
JOIN=re.compile(r'''\b(join|resolve)\(\s*([A-Za-z_$][\w$.]*(?:\([^()]*\))?)\s*,\s*("\.cache[^"]*"|"\.nx[^"]*")''')
if __name__=="__main__":
    mode=sys.argv[1] if len(sys.argv)>1 else "summary"
    tot=collections.Counter();rest=[]
    for f in files():
        t=open(f,encoding="utf8").read()
        lines=t.split("\n")
        for i,l in enumerate(lines):
            if PAT.search(l):
                js=JOIN.findall(l)
                rem=JOIN.sub("",l)
                tot["lines"]+=1
                tot["join-forms"]+=len(js)
                if PAT.search(rem): rest.append((f,i+1,l.strip()[:230]))
    print(tot,"remaining lines:",len(rest))
    if mode=="rest":
        for f,i,l in rest: print(f.replace(ROOT,"")[-90:],i,l)
