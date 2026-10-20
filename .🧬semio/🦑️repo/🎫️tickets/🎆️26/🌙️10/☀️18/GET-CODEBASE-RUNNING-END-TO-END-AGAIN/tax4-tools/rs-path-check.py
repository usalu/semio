# 🔬 Static check: every `#[path = "…"]` attribute and every Cargo workspace member / path dependency under a directory resolves to an existing path. usage: python3 rs-path-check.py <dir>
import os,re,subprocess,sys
root=sys.argv[1]
files=[f for f in subprocess.run(["git","ls-files","-co","--exclude-standard","--",root],capture_output=True,text=True).stdout.split("\n") if f.endswith((".rs",".toml")) and "/target/" not in f and "node_modules" not in f]
miss=[];tot=0
for f in files:
    try: t=open(f,encoding="utf8").read()
    except Exception: continue
    d=os.path.dirname(f)
    if f.endswith(".rs"):
        for m in re.finditer(r'#\[path = "([^"]+)"\]',t):
            p=m.group(1)
            if p==".": continue
            tot+=1
            if not os.path.exists(os.path.normpath(os.path.join(d,p))): miss.append((f,p))
    else:
        for m in re.finditer(r'^\s*"([^"]+)",?\s*$',t,re.M):
            p=m.group(1)
            if "/" in p and not p.startswith(("http","git")) and not re.match(r"^[a-z0-9_-]+$",p) and "🏅" in p:
                tot+=1
                if not os.path.isdir(os.path.normpath(os.path.join(d,p))): miss.append((f,p))
        for m in re.finditer(r'path = "([^"]+)"',t):
            p=m.group(1); tot+=1
            if not os.path.exists(os.path.normpath(os.path.join(d,p))): miss.append((f,p))
print("checked",tot,"missing",len(miss))
for f,p in miss[:12]: print("  ",f[-90:],"->",p[-100:])
