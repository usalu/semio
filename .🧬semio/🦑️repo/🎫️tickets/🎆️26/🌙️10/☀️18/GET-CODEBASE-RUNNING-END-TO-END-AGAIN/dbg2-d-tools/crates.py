import os,collections,re
R="/Users/ueli/Documents/semio"
D=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d"
fs=[l.split(" ",1)[1].strip() for l in open(D+"/orig/map.txt",encoding="utf8")]
res=collections.defaultdict(list);ts=[];js=[]
pat=re.compile(r'^name\s*=\s*"([^"]+)"',re.M)
for f in fs:
    if f.endswith(".rs"):
        d=os.path.dirname(R+"/"+f);found=None
        while len(d)>len(R):
            for cand in (d+"/📦️packages/🦀️rust",d):
                if os.path.exists(cand+"/Cargo.toml") and pat.search(open(cand+"/Cargo.toml",encoding="utf8").read()):
                    found=cand;break
            if found:break
            d=os.path.dirname(d)
        res[found[len(R)+1:] if found else "NONE"].append(f)
    elif f.endswith((".ts",".tsx")):ts.append(f)
    else:js.append(f)
open(D+"/ts.txt","w",encoding="utf8").write("\n".join(ts)+"\n")
open(D+"/json.txt","w",encoding="utf8").write("\n".join(js)+"\n")
out=[]
for k,v in sorted(res.items()):
    nm="?"
    if k!="NONE":
        nm=pat.search(open(R+"/"+k+"/Cargo.toml",encoding="utf8").read()).group(1)
    out.append(nm+"\t"+k)
    print(nm,len(v),k[-60:])
    if k=="NONE":print(v)
open(D+"/crates.txt","w",encoding="utf8").write("\n".join(out)+"\n")
open(D+"/crate-files.txt","w",encoding="utf8").write("\n".join(f"{k}\t{f}" for k,v in sorted(res.items()) for f in v)+"\n")
print(len(ts),len(js),len(res))
