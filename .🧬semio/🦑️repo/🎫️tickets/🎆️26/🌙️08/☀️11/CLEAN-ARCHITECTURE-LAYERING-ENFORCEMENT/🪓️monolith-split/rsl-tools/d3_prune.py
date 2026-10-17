"""🧹 Removes the `use` leaves of the composer file that only the moved bodies needed (name used in the bundle crate, absent from the remaining composer text, not a trait). Dry run unless `write`. usage: d3_prune.py dry|write"""
import re, os, subprocess, sys, shutil
os.chdir("/Users/ueli/Documents/semio")
S='🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store'
ENG=S+"/⚙️engine/📥️lifted/🏪️store/🦀️.rs"; BUN=S+"/⚙️engine/🧳️bundle/🦀️.rs"
PRE="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/rsl-pre/"
write=sys.argv[1]=="write"
eng=open(ENG,encoding="utf8").read(); bun=open(BUN,encoding="utf8").read()
tok=lambda t:set(re.findall(r'(?<![.\w])(?<!::)[A-Za-z_][A-Za-z0-9_]*',re.sub(r'(?m)^\s*//.*$','',t)))
bun_body="\n".join(l for l in bun.split("\n") if not l.startswith("use "))
bt=tok(bun_body)
def is_trait(n):
    r=subprocess.run(["git","-c","core.fsmonitor=false","-c","core.quotepath=false","grep","-l","-E",r"(pub )?trait "+n+r"\b","--","*.rs",":!.🧬semio"],capture_output=True,text=True).stdout.strip()
    return bool(r)
def split_group(inner):
    depth=0;cur="";parts=[]
    for ch in inner:
        if ch=="{": depth+=1
        if ch=="}": depth-=1
        if ch=="," and depth==0: parts.append(cur);cur=""
        else: cur+=ch
    if cur.strip(): parts.append(cur)
    return [p.strip() for p in parts]
lines=eng.split("\n"); rest_text="\n".join(l for l in lines if not (l.startswith("use ") and l.rstrip().endswith(";")))
rt=tok(rest_text)
removed=[];out=[]
for l in lines:
    if not(l.startswith("use ") and l.rstrip().endswith(";")): out.append(l); continue
    body=l[4:].rstrip().rstrip(";")
    if "{" in body:
        head,rest=body.split("{",1); inner=rest.rsplit("}",1)[0]
        keep=[]
        for p in split_group(inner):
            m=re.match(r'^(.*?)(?:\s+as\s+(\w+))?$',p); n=m.group(2) or m.group(1).split("::")[-1]
            if n not in("_","self") and "{" not in p and n in bt and n not in rt and not is_trait(n): removed.append((n,head+p)); continue
            keep.append(p)
        if not keep: continue
        out.append("use "+head+("{"+", ".join(keep)+"}" if len(keep)>1 or "::" in keep[0] else keep[0])+";")
    else:
        m=re.match(r'^(.*?)(?:\s+as\s+(\w+))?$',body); n=m.group(2) or m.group(1).split("::")[-1]
        if n not in("_","*") and not body.endswith("::*") and n in bt and n not in rt and not is_trait(n): removed.append((n,body)); continue
        out.append(l)
new="\n".join(out)
print(len(removed),"leaves"); [print(" -",n,"|",p) for n,p in removed]
if write:
    cur=open(ENG,encoding="utf8").read(); assert cur==eng
    d=PRE+ENG
    if not os.path.exists(d): os.makedirs(os.path.dirname(d),exist_ok=True); shutil.copy2(ENG,d)
    open(ENG,"w",encoding="utf8").write(new); print("WROTE",len(eng),"->",len(new))
