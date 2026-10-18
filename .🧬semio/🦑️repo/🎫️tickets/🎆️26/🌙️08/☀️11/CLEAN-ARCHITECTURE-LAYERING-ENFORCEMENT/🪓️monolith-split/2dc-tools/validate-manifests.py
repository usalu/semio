"""✅️ Checks every manifest that depends on the compute crate: workspace inheritance resolves, path exists, TOML parses (via bun)."""
import json, os, re, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)
out = subprocess.run(["git","-c","core.fsmonitor=false","-c","core.quotepath=false","grep","-l","-E","^semio-framework-2d-compute[[:space:]]*=","--","✏️s","🌎️hub","🧰️framework","Cargo.toml"],capture_output=True,text=True).stdout.split("\n")
ms=[x for x in out if x.endswith("Cargo.toml")]
untracked=subprocess.run(["git","-c","core.fsmonitor=false","-c","core.quotepath=false","ls-files","--others","--exclude-standard","--","✏️s","🌎️hub","🧰️framework"],capture_output=True,text=True).stdout.split("\n")
ms+= [u for u in untracked if u.endswith("Cargo.toml") and u not in ms and re.search(r"^semio-framework-2d-compute\s*=", open(u,encoding="utf8").read(), re.M)]
print("manifests with compute dep:", len(ms))
bad=0
for m in sorted(ms):
    t=open(m,encoding="utf8").read()
    line=re.search(r"^semio-framework-2d-compute\s*=.*$",t,re.M).group(0)
    if "workspace = true" in line:
        wm=re.search(r'^workspace\s*=\s*"([^"]+)"',t,re.M)
        d=os.path.dirname(m)
        if wm: wr=os.path.normpath(os.path.join(d,wm.group(1),"Cargo.toml"))
        else:
            wr=None;dd=d
            while dd:
                dd=os.path.dirname(dd);c=os.path.join(dd,"Cargo.toml") if dd else "Cargo.toml"
                if os.path.exists(c) and re.search(r"^\[workspace[\].]",open(c,encoding="utf8").read(),re.M): wr=c;break
        wt=open(wr,encoding="utf8").read()
        mm=re.search(r'^\s*(?:\[workspace\.dependencies\.semio-framework-2d-compute\]\s*\n\s*path\s*=\s*"([^"]+)"|semio-framework-2d-compute\s*=\s*\{[^}]*path\s*=\s*"([^"]+)")',wt,re.M)
        if not mm: print("NO WS STANZA",m,wr);bad+=1;continue
        p=mm.group(1) or mm.group(2)
        if not os.path.isfile(os.path.join(os.path.dirname(wr),p,"Cargo.toml")): print("WS PATH BAD",wr,p);bad+=1
    else:
        p=re.search(r'path\s*=\s*"([^"]+)"',line).group(1)
        if not os.path.isfile(os.path.join(os.path.dirname(m),p,"Cargo.toml")): print("PATH BAD",m,p);bad+=1
json.dump(ms,open(sys.argv[1],"w"),ensure_ascii=False)
print("bad",bad)
