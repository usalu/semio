import difflib,hashlib,json,os
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
M="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R169/mirror/🏗️build"
files=["🗄️cas/🧬️schema/🔣️.json","🗄️cas/🧬️schema/🟦️.ts","🗄️cas/🌡️partition/🟦️.ts","🗄️cas/🚦️guard/🟦️.ts","🌡️host-capacity/💽️disk/🟦️.ts","🧹️cache-gc/⌨️command/🟦️.ts"]
out=[]
for f in files:
    a=open(os.path.join(L,f),encoding="utf8").read().split("\n");b=open(os.path.join(M,f),encoding="utf8").read().split("\n");full="\n".join(a)
    ops=[o for o in difflib.SequenceMatcher(None,a,b,autojunk=False).get_opcodes() if o[0]!="equal"]
    groups=[list(o[1:]) for o in ops]
    def span(g):
        ctx=2
        while True:
            lo=max(0,g[0]-ctx);hi=min(len(a),g[1]+ctx)
            if full.count("\n".join(a[lo:hi]))==1 or ctx>80:return lo,hi,ctx
            ctx+=2
    while True:
        spans=[span(g) for g in groups];merged=False
        for k in range(len(groups)-1):
            if spans[k][1]>spans[k+1][0]:
                groups[k]=[groups[k][0],groups[k+1][1],groups[k][2],groups[k+1][3]];del groups[k+1];merged=True;break
        if not merged:break
    hunks=[]
    for g,(lo,hi,ctx) in zip(groups,spans):
        i1,i2,j1,j2=g
        hunks.append({"old":"\n".join(a[lo:hi]),"new":"\n".join(b[j1-(i1-lo):j2+(hi-i2)])})
    out.append({"file":f,"baseSha":hashlib.sha256(open(os.path.join(L,f),'rb').read()).hexdigest(),"newSha":hashlib.sha256(open(os.path.join(M,f),'rb').read()).hexdigest(),"hunks":hunks})
    print(f,len(hunks))
json.dump(out,open("patches.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
