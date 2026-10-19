import os,re,subprocess,sys,urllib.parse
R="/Users/ueli/Documents/semio"
C=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/vit-tools"
entry=sys.argv[1]
changed=[]
for i in range(80):
    p=subprocess.run(["node","-e",'require("%s")'%entry],cwd=R,capture_output=True,text=True); t=p.stderr
    if p.returncode==0: print("OK"); break
    if "needs an import attribute" in t:
        r=subprocess.run(["python3",C+"/fixjson.py",entry],cwd=R,capture_output=True,text=True).stdout
        for l in r.splitlines():
            if l.startswith("fixed"): changed.append(l)
        print(r.strip().splitlines()[-1] if r.strip() else "")
        continue
    m=re.search(r'file://([^\n:]+\.m?ts):(\d+)',t)
    if "parameter property" in t and m:
        f=urllib.parse.unquote(m.group(1)); r=subprocess.run(["node",C+"/../osplay-tools/fix-pp.mjs",f],capture_output=True,text=True).stdout
        print(r.strip()); changed.append("pp "+r.strip())
        if r.strip().endswith(" 0"): print(t[:800]); break
        continue
    print(t[:1500]); break
print("\n".join(changed))
