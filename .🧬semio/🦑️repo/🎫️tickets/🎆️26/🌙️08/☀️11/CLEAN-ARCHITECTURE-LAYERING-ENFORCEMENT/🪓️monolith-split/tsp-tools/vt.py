import sys,os,json,subprocess,re
root="/Users/ueli/Documents/semio/"
F=root+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/"
cfg,tag=sys.argv[1],sys.argv[2]; extra=sys.argv[3:]
gen=F+"🗑️generated/tsp-vitest-"+tag
os.makedirs(gen,exist_ok=True)
pol=dict(version=1,cwd=os.path.dirname(root+cfg),toolPath=root+"node_modules/vitest/vitest.mjs",runtime="node",coverageRuntime="node",cacheRoot=gen+"/cache",coverageDirectory=gen+"/cov",budgetMs=300000)
env=dict(os.environ,SEMIO_VITEST_POLICY=json.dumps(pol),SEMIO_TEST_LEVEL=os.environ.get("SEMIO_TEST_LEVEL","long"))
r=subprocess.run(["nice","-n","10","perl","-e","alarm 1200; exec @ARGV","--",os.environ.get("VT_RUNTIME","node"),pol["toolPath"],"run","--config",root+cfg,"--testTimeout","60000","--hookTimeout","60000",*extra],cwd=os.path.dirname(root+cfg),env=env,capture_output=True,text=True,errors="replace")
out=r.stdout+r.stderr
open(gen+"/out.log","w").write(out)
tail=[l for l in out.split("\n") if re.search(r"Test Files|Tests |Error|FAIL",l)]
print(tag,r.returncode); print("\n".join(re.sub(r"/Users/ueli/Documents/semio/","",l)[:250] for l in tail[:12]))
