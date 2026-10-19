import os,re,subprocess,sys,collections
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/pfx-oracle"
A=S+"/a/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
B=S+"/b/🪆️subsets"
def patch(root):
    for dp,dn,fn in os.walk(root):
        for f in fn:
            if f.endswith(".rs"):
                p=os.path.join(dp,f);s=open(p,encoding="utf-8").read();o=s
                s=re.sub(r"(?m)^[ \t]*extern crate [^\n]*;\n","",s)
                s=re.sub(r"(?m)^([ \t]*)(?:pub(?:\([a-z]+\))?[ \t]+)?use (?!crate|self|super)[A-Za-z0-9_:{} ,]*::\*;","\\1const _: () = ();",s)
                if s!=o:open(p,"w",encoding="utf-8").write(s)
def run(root,d):
    crate=re.sub(r"[^a-z0-9_]","_",d.encode("ascii","ignore").decode() or "c")+"x"
    r=subprocess.run(["rustc","+nightly-2026-07-20","--edition","2021","--crate-type","lib","--crate-name","pfxoracle","--emit=metadata","-o","/dev/null","--error-format=short",root+"/"+d+"/🦀️.rs"],capture_output=True,text=True)
    out=collections.Counter()
    for l in r.stderr.splitlines():
        m=re.match(r"(.*?):(\d+):(\d+): (error.*)",l)
        if m:
            path=m.group(1).replace(root,"")
            out[(path,m.group(4)[:160])]+=1
    return out
if __name__=="__main__":
    if "--patch" in sys.argv:
        patch(A);patch(B);print("patched")
    else:
        for d in sys.argv[1:]:
            a=run(A,d);b=run(B,d)
            new=b-a;gone=a-b
            print(d,"A",sum(a.values()),"B",sum(b.values()),"new",sum(new.values()),"gone",sum(gone.values()))
            for k,v in list(new.items())[:12]:print("  NEW",v,k)
            for k,v in list(gone.items())[:5]:print("  GONE",v,k)
