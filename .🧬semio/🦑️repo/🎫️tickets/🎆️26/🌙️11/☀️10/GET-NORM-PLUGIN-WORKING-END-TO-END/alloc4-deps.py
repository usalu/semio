import tomllib,subprocess,os,re,sys
out,apply=sys.argv[1],sys.argv[2]=="apply"
A="semio-framework-value-testing-allocator"
ALLOC=os.path.abspath("🧰️framework/🔨️modules/🌱️value/🧪️testing/💾️allocator/📦️packages/🦀️rust")
tomls=[t for t in subprocess.run(["git","-c","core.quotepath=off","ls-files","--cached","--others","--exclude-standard","--","*Cargo.toml"],capture_output=True,text=True).stdout.split("\n") if t and not t.startswith(".🧬semio/")]
pk={}
for t in tomls:
    try:d=tomllib.load(open(t,"rb"))
    except Exception:continue
    if "package" in d:pk[d["package"]["name"]]=(os.path.abspath(t),d)
crates=set()
for l in open(out,encoding="utf-8"):
    p=l.split(" ",2)
    if p[0] in("ok","CONFLICT") and len(p)>1:crates.add(p[1])
done=[]
for n in sorted(crates):
    if n==A or n not in pk:continue
    t,d=pk[n]
    has=any(A in (tb.get("dependencies",{}) if False else {}) for tb in [d])
    tbs=[d]+list(d.get("target",{}).values())
    has=any(A in tb.get("dependencies",{}) or A in tb.get("dev-dependencies",{}) for tb in tbs)
    if has:continue
    done.append(n)
    if not apply:continue
    s=open(t,encoding="utf-8").read()
    rel=os.path.relpath(ALLOC,os.path.dirname(t))
    line=f'{A} = {{ path = "{rel}" }}\n'
    m=re.search(r'(?m)^\[dev-dependencies\]\s*\n',s)
    if m:
        e=re.search(r'(?m)^\[',s[m.end():])
        pos=m.end()+(e.start() if e else len(s)-m.end())
        pre=s[:pos]
        if not pre.endswith("\n"):pre+="\n"
        s=pre+line+("\n" if e else "")+s[pos:] if e else pre+line
    else:
        s=s.rstrip("\n")+"\n\n[dev-dependencies]\n"+line
    open(t,"w",encoding="utf-8").write(s)
print("\n".join(done))
