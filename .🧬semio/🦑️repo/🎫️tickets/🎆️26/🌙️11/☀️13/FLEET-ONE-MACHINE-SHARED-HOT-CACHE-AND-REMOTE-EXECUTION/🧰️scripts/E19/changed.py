# coding: utf-8
import sys,subprocess,hashlib,os,re
T=sys.argv[1]; G=T+"/wave-1122/🗑️generated/E19/"; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
R="/Users/ueli/Documents/semio/"
files=set()
for n in ("env-files.txt","envelope-files.txt","env2.txt"):
    files|={x.strip() for x in open(G+n) if x.strip()}
files={f for f in files if os.path.exists(R+f) and "declareLibraryBuildTestEnvelopeV1(" in open(R+f).read()}
files|={L+x.strip() for x in open(T+"/🧰️scripts/E19/touched.txt") if x.strip()}
extra=["⚡️caching/🗂️suites/🔣️.json","⚡️caching/🧪️tests/🧫️fixtures/🧱️command-source/🔣️.json","⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧰️setup/🧪️tests/🐍️.py","🏗️build/🏡️resident/🧬️schema/🔣️.json","🏗️build/🌐️remote-cache/💾️store/🟦️.ts","🏗️build/🌐️remote-cache/🖥️server/🟦️.ts","⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/👁️profile-recheck/🟦️.ts","⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts","⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/🪜️steps/🟦️.ts","⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts","⚡️caching/🚀️bootstrap/📦️dependencies/🪞️adopt/🧪️tests/🟦️.ts","⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/🧪️tests/⏳️busy-funding/🟦️.ts","⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🔭️providers/🟦️.ts"]
files|={L+x for x in extra}
files.add("🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/🧩️component-driver/🟦️.ts")
out=[]
for f in sorted(files):
    p=R+f
    if not os.path.exists(p): continue
    now=hashlib.sha256(open(p,"rb").read()).hexdigest()[:8]
    r=subprocess.run(["git","show","HEAD:"+f],capture_output=True,cwd=R)
    head=hashlib.sha256(r.stdout).hexdigest()[:8] if r.returncode==0 else "new"
    out.append("%s %s -> %s"%(f.replace(L,"").replace("🧰️framework/🔨️modules/🏃️process/",""),head,now))
open(G+"changed.txt","w").write("\n".join(out)+"\n"); print(len(out))
