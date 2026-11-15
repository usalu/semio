# -*- coding: utf-8 -*-
"""R32 overlay: a scratch root with copies of the needed trees (never a link into the live tree), the payload applied as base or new, and the R32 tests run in it with BUN_RUNTIME_TRANSPILER_CACHE_PATH=0. usage: overlay.py <scratch> prepare | before | after | test"""
import os,re,sys,shutil,subprocess
HERE=os.path.dirname(os.path.abspath(__file__));P=HERE+"/payload";LIVE="/Users/Shared/semio"
S=os.path.abspath(sys.argv[1]);mode=sys.argv[2]
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";R=L+"🏗️build/🏡️resident/";O="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/"
TREES=[(L[:-1],"2m"),("🧰️framework/🔨️modules","400k"),("🧰️framework/📦️packages","400k")]
TESTS=[(O+"🧪️tests/🎯️errno-consensus",["🟦️.ts"]),(O+"🧪️tests/🧭️denied-entry",["🟦️.ts"]),(O+"🧪️tests/📁️descriptor",["🟦️.ts"]),(R+"🧪️tests/🧯️failure",["🟦️.ts"]),(R+"🧪️tests/🛡️consent-service",["🟦️.ts"]),(R+"🧪️tests/🏷️worker-error",["🟦️.ts"]),(R+"🧪️tests/🔐️consent",["🟦️.ts"])]
def files():return [l for l in open(P+"/files.txt",encoding="utf8").read().split("\n") if l]
def put(state):
    for rel in files():
        src=P+("/base/" if state=="before" and "/🧪️tests/" not in rel else "/new/")+rel
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
    for n in ["ajv","fast-deep-equal","fast-uri","json-schema-traverse","require-from-string","typescript","fast-glob","@nodelib","glob-parent","merge2","micromatch","braces","fill-range","to-regex-range","is-number","picomatch","is-glob","is-extglob","run-parallel","queue-microtask","reusify","fastq"]:
        if os.path.exists(LIVE+"/node_modules/"+n) and not os.path.exists(S+"/node_modules/"+n):shutil.copytree(LIVE+"/node_modules/"+n,S+"/node_modules/"+n)
    open(S+"/package.json","w").write('{"name":"r32-overlay","type":"module"}')
def test():
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=S+"/generated"
    for rel,names in TESTS:
        if not os.path.isdir(S+"/"+rel):print("absent",rel[-60:]);continue
        r=subprocess.run([BUN,"test"]+["./"+n for n in names]+["--timeout","180000"],cwd=S+"/"+rel,env=env,capture_output=True,text=True)
        text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")];summary=[l for l in text.splitlines() if re.match(r"^\s*\d+ (pass|fail)$",l)]
        print("ok  " if r.returncode==0 else "RED ",rel.replace(L,"LIB/")[-70:],"|"," ".join(s.strip() for s in summary))
        for f in failed:print("     ",f[:220])
if mode=="prepare":prepare()
elif mode in("before","after"):put(mode)
elif mode=="test":test()
