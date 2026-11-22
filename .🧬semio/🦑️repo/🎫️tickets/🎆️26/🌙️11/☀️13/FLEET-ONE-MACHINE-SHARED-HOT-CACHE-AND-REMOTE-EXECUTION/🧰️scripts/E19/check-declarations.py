import re,os,sys,subprocess
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import importlib.util
spec=importlib.util.spec_from_file_location("db",os.path.join(os.path.dirname(os.path.abspath(__file__)),"declare-births.py")); db=importlib.util.module_from_spec(spec); spec.loader.exec_module(db)
L=db.LIB
files=subprocess.run("grep -rl 'declareLibraryBuildTestEnvelopeV1({' %s⚡️caching %s🏗️build"%(L,L),shell=True,capture_output=True,text=True).stdout.split("\n")
bad=0
for f in files:
    if not f.endswith(".ts"): continue
    t=open(f).read(); m=re.search(r'declareLibraryBuildTestEnvelopeV1\(\{suite:"([^"]+)"([^}]*)\}',t)
    if not m: continue
    rows=db.row_of(f)
    ids=[r["id"] for r in rows]
    if m.group(1) not in ids:
        bad+=1; print("MISMATCH",f.split("library/")[-1][:100],"declared",m.group(1),"row",ids)
print("checked",len(files),"mismatch",bad)
