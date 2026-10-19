import os,subprocess,re,json
os.chdir("/Users/ueli/Documents/semio")
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
suites=[l.strip() for l in open(C+"/tst4-tools/envelope-gated.txt") if l.strip()]
files=subprocess.run(["git","-c","core.quotepath=off","ls-files","--","*📜️script.ts","*script.ts"],capture_output=True,text=True).stdout.split("\n")
files=[f for f in files if f.endswith("script.ts") and f and not f.startswith(".🧬")]
cache={}
for f in files:
    try: cache[f]=open(f,encoding="utf8").read()
    except: pass
for s in suites:
    parts=s.split("/")
    frag="/".join(parts[-3:-1]) if parts[-1] in("🟦️.ts",) else "/".join(parts[-3:])
    frag2="/".join(parts[-4:-1]) if parts[-1]=="🟦️.ts" else frag
    hits=[]
    for f,t in cache.items():
        if frag in t:
            i=t.index(frag)
            hits.append((f.split("📚️library/")[-1][:60],t[max(0,i-100):i+len(frag)+10].replace("\n"," ")[-140:]))
    print("##",s.split("📚️library/")[-1], "::",frag)
    for h in hits[:3]: print("   ",h)
