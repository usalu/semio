import re,subprocess,sys,os,json
root="/Users/ueli/Documents/semio"
T=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
f=sys.argv[1];out=sys.argv[2];only=sys.argv[3] if len(sys.argv)>3 else None
src=open(f,encoding='utf8').read()
names=re.findall(r'^test(?:\.skipIf\([^)]*\))?\("([^"]+)"',src,flags=re.M)
res={}
for n in names:
    if only and only not in n: continue
    p=subprocess.run(["bun",T+"/nxpart-tools/run-test.ts","./"+os.path.relpath(f,root),"-t","^"+re.escape(n)+"$"],cwd=root,capture_output=True,text=True,timeout=600)
    txt=p.stdout+p.stderr
    m=re.search(r'(\d+) pass',txt);k=re.search(r'(\d+) fail',txt)
    status="pass" if m and int(m.group(1))>=1 and not (k and int(k.group(1))>0) else "fail"
    err=""
    if status=="fail":
        e=[x for x in re.findall(r'(?:error|Error): ([^\n]{0,200})',txt) if 'lq:test' not in x];err=e[0] if e else txt[-200:]
    res[n]=(status,err)
    print(status,n[:90],"|",err[:150],flush=True)
json.dump(res,open(out,'w'),ensure_ascii=False,indent=1)
