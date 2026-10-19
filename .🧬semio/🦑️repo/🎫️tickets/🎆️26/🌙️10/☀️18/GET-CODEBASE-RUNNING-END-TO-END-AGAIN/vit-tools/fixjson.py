import os,re,subprocess,sys,urllib.parse
R="/Users/ueli/Documents/semio"
entry=sys.argv[1]
def probe():
    p=subprocess.run(["node","-e",'require("%s")'%entry],cwd=R,capture_output=True,text=True)
    t=p.stderr
    m=re.search(r'Module "file://([^"]+)" needs an import attribute',t)
    if m: return ("json",urllib.parse.unquote(m.group(1)),t)
    return ("other" if p.returncode else "ok",None,t)
for i in range(60):
    k,j,t=probe()
    if k!="json":
        print(k); print(t[:1500]); break
    base=os.path.basename(j); fixed=False
    d=os.path.dirname(j)
    for up in range(6):
        d=os.path.dirname(d) if up else d
        out=subprocess.run(["/usr/bin/grep","-rl","--include=*.ts","--include=*.mts","--include=*.tsx","--exclude-dir=node_modules","--exclude-dir=dist","-F",base+'"',d],capture_output=True,text=True).stdout.split("\n")
        for f in out:
            if not f: continue
            rel=os.path.relpath(j,os.path.dirname(f))
            if not rel.startswith("."): rel="./"+rel
            s=open(f,encoding="utf-8").read()
            pat=re.compile(r'^((?:import|export) [^\n]* from "'+re.escape(rel)+r'");',re.M)
            if pat.search(s):
                open(f,"w",encoding="utf-8").write(pat.sub(r'\1 with { type: "json" };',s)); print("fixed",f); fixed=True
        if fixed: break
    if not fixed: print("NO IMPORTER for",j); break
