import sys
sys.argv=[sys.argv[0]]+sys.argv[1:]
import importlib.util,os
spec=importlib.util.spec_from_file_location("b1",os.path.dirname(os.path.abspath(__file__))+"/battery.py");b=importlib.util.module_from_spec(spec);spec.loader.exec_module(b)
O=b.B+"/👁️observation/🧪️tests"
BASE=[O+"/📥️reuse/🟦️.ts",O+"/🧺️presence-batch/🟦️.ts",O+"/🔁️inventory-changed/🟦️.ts",b.B+"/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/📥️take/🟦️.ts"]
NEW=[O+"/🫙️fresh-namespace/🟦️.ts"]
import concurrent.futures as cf
group=sys.argv[1];limit=int(sys.argv[2]) if len(sys.argv)>2 else 170
files={"base":BASE,"new":NEW,"all":BASE+b.PRE+NEW}[group]
files=[f for f in files if os.path.exists(f)]
with cf.ThreadPoolExecutor(4) as ex:
    for r in ex.map(lambda f:b.run(f,limit),files):print("%-120s pass=%d fail=%d exit=%d %ds"%r,flush=True)
