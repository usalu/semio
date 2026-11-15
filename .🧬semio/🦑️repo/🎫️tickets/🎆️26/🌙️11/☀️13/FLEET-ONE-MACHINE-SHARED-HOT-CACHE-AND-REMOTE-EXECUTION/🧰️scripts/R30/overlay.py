# -*- coding: utf-8 -*-
"""R30 overlay: a scratch root with copies of the needed trees (never a link into the live tree), the payload applied as base or new, and the R30 tests run in it with BUN_RUNTIME_TRANSPILER_CACHE_PATH=0. usage: overlay.py <scratch> prepare | before | after | test [before|after]"""
import os,sys,shutil,subprocess
HERE=os.path.dirname(os.path.abspath(__file__));P=HERE+"/payload";LIVE="/Users/Shared/semio"
S=os.path.abspath(sys.argv[1]);mode=sys.argv[2]
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";W=B+"🪪️graph-state/🙋️wanted/";C=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/♻️retirement/"
TREES=[(L[:-1],"2m"),("🧰️framework/🔨️modules","400k"),("🧰️framework/📦️packages","400k")]
TESTS=[(W+"🧪️tests/🐧️linux-host",["🟦️.ts"]),(C+"🧪️tests/📏️roster-bound",["🟦️.ts"]),(W+"🧪️tests/🍏️idle-grant",["🟦️.ts"]),(W+"🧪️tests/🌍️two-platforms",["🟦️.ts"]),(B+"🕸️graph/🧪️tests/🙋️fleet-read/🌍️small-booking",["🟦️.ts"]),(B+"🕸️graph/🧪️tests/🙋️fleet-read/🍏️macos-reading",["🟦️.ts"]),(B+"🧱️job-limits/🧪️tests",["🟦️.ts"]),(B+"🧠️memory/🧪️tests/🧹️parent-budget",["🟦️.ts"]),(C+"🧪️tests",["🟦️.ts"])]
def files():return [l for l in open(P+"/files.txt",encoding="utf8").read().split("\n") if l]
def put(state):
    for rel in files():
        src=P+("/base/" if state=="before" else "/new/")+rel
        if state=="before" and not os.path.exists(src):
            if os.path.exists(S+"/"+rel):os.remove(S+"/"+rel)
            os.makedirs(os.path.dirname(S+"/"+rel),exist_ok=True);shutil.copyfile(P+"/new/"+rel,S+"/"+rel);continue
        os.makedirs(os.path.dirname(S+"/"+rel),exist_ok=True);shutil.copyfile(src,S+"/"+rel)
def prepare():
    os.makedirs(S,exist_ok=True)
    for t,size in TREES:
        os.makedirs(S+"/"+t,exist_ok=True)
        subprocess.run(["rsync","-a","--max-size="+size,"--exclude=*.wasm","--exclude=*.rlib","--exclude=*.rmeta","--exclude=node_modules","--exclude=target","--exclude=.cache",LIVE+"/"+t+"/",S+"/"+t+"/"],check=True)
    os.makedirs(S+"/node_modules",exist_ok=True)
    for n in ["ajv","fast-deep-equal","fast-uri","json-schema-traverse","require-from-string","typescript"]:
        if not os.path.exists(S+"/node_modules/"+n):shutil.copytree(LIVE+"/node_modules/"+n,S+"/node_modules/"+n)
    open(S+"/package.json","w").write('{"name":"r30-overlay","type":"module"}')
def test():
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
    for rel,names in TESTS:
        r=subprocess.run([BUN,"test"]+["./"+n for n in names]+["--timeout","180000"],cwd=S+"/"+rel,env=env,capture_output=True,text=True)
        text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")];summary=[l for l in text.splitlines() if l.strip().startswith(("pass","fail")) and l.strip().split(" ")[0] in("pass","fail") ]
        print("ok  " if r.returncode==0 else "RED ",rel.replace(L,"LIB/"),"|"," ".join(s.strip() for s in summary))
        for f in failed:print("     ",f[:200])
if mode=="prepare":prepare()
elif mode in("before","after"):put(mode)
elif mode=="test":test()
