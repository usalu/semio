import os,sys,subprocess,time,re,concurrent.futures as cf
LIVE="/Users/ueli/Documents/semio";LIB=LIVE+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BUN=LIVE+"/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
SCRIPT=LIB+"/🧪️testing/🌱️genuine-birth/📜️script.ts"
B=LIB+"/🏗️build";PN=B+"/🕸️graph/📥️inference/📦️project-nodes"
BASE=[PN+"/🧪️tests/📥️take/🟦️.ts",PN+"/🧪️tests/📈️inferred/🟦️.ts",PN+"/🧪️tests/🔭️look-job/🟦️.ts",PN+"/🧪️tests/🛣️lanes/🟦️.ts",PN+"/🧪️tests/🧭️parent-routes/🟦️.ts",PN+"/🧪️tests/🚧️read-parent/🟦️.ts",B+"/🧪️tests/🕸️graph/🌱️native-scope/🟦️.ts"]
PRE=[B+"/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts",B+"/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts"]
NEW=[PN+"/🧪️tests/🤝️selected-fleet/🟦️.ts"]
def run(f,limit):
    s=time.time()
    try:
        p=subprocess.run([BUN,SCRIPT,"--file="+f,"--birth=setup","--root="+LIVE],cwd=LIVE,capture_output=True,text=True,timeout=limit)
        out=p.stdout+p.stderr;code=p.returncode
    except subprocess.TimeoutExpired as e:
        out="";code=-9
    pas=re.findall(r"^\s*(\d+) pass",out,re.M);fail=re.findall(r"^\s*(\d+) fail",out,re.M)
    return (os.path.relpath(f,LIB),int(pas[-1]) if pas else -1,int(fail[-1]) if fail else -1,code,round(time.time()-s))
if __name__=="__main__":
    group=sys.argv[1];limit=int(sys.argv[2]) if len(sys.argv)>2 else 170
    files={"base":BASE,"pre":PRE,"new":NEW,"all":BASE+PRE+NEW}[group]
    files=[f for f in files if os.path.exists(f)]
    with cf.ThreadPoolExecutor(4) as ex:
        for r in ex.map(lambda f:run(f,limit),files):print("%-120s pass=%d fail=%d exit=%d %ds"%r,flush=True)
