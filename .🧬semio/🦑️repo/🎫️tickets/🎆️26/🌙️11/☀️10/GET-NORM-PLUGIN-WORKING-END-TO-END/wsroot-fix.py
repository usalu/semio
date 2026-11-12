import sys,re,subprocess
fix=len(sys.argv)>1 and sys.argv[1]=="fix"
files=subprocess.run(["git","ls-files","-z","--cached","--others","--exclude-standard","--","*Cargo.toml"],capture_output=True).stdout.decode().split("\0")
bad=[]
for f in filter(None,files):
    if "/🎫️tickets/" in f or "/target/" in f or "/.cache/" in f: continue
    try: t=open(f,encoding="utf8").read()
    except: continue
    if not re.search(r"^\[workspace\]",t,re.M): continue
    pre=t.split("\n[",1)[0] if not t.startswith("[") else ""
    if re.search(r"^cargo-features\s*=.*trim-paths",pre,re.M): continue
    if re.search(r"^cargo-features",pre,re.M): print("MANUAL",f); continue
    bad.append(f)
    if fix: open(f,"w",encoding="utf8").write('cargo-features = ["trim-paths"]\n'+t)
print(len(bad),"roots lacking"); [print(b) for b in bad] if not fix else None
