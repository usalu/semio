import json,os,re,subprocess,sys
R="/Users/ueli/Documents/semio/"
C=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
scripts=subprocess.run(["git","ls-files","--","*📜️script.ts"],cwd=R,capture_output=True,text=True).stdout.split("\n")
scripts=[s for s in scripts if s and "/🧪️tests/" not in s and "🗑️" not in s]
texts={s:open(R+s,encoding="utf-8",errors="replace").read() for s in scripts if os.path.isfile(R+s)}
lst=[l for l in open(C+"/env-pass2-tools/list.txt",encoding="utf-8").read().split("\n") if l]
out={}
for i,f in enumerate(lst,1):
    d=os.path.dirname(f).split("/")
    needles=["/".join(d[-2:])+"/🟦️.ts","/".join(d[-2:])]
    hits=[]
    for s,t in texts.items():
        for nd in needles:
            for m in re.finditer(re.escape(nd),t):
                pre=t[max(0,m.start()-1800):m.start()]
                cmds=re.findall(r'args\[0\]\s*===?\s*"([^"]+)"',pre)
                keys=re.findall(r'"([\w:-]+)"\s*:\s*"[^"]*$',pre[-200:])
                hits.append((s,cmds[-1] if cmds else None,keys[-1] if keys else None))
            if hits: break
    out[i]=hits[:6]
json.dump(out,open(C+"/env-pass2-tools/map2.json","w",encoding="utf-8"),ensure_ascii=False,indent=0)
for i,h in out.items():
    print(i,[(x[0].replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/","~/")[-70:],x[1],x[2]) for x in h[:3]])
