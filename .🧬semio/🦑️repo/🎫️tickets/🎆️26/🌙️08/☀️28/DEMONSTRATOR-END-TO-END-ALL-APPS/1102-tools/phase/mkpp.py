import os,sys,shutil,glob,subprocess
sys.path.insert(0,".")
from ph import R,POL,policy_files
PP="/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/pp"
shutil.rmtree(PP,ignore_errors=True)
for p in sorted(glob.glob(POL+"/*.patch")):
    for rel in policy_files(p):
        d=f"{PP}/{rel}"
        if not os.path.exists(d):
            os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copy(f"{R}/{rel}",d)
    r=subprocess.run(["git","apply","--unsafe-paths","-p1",p],cwd=PP,capture_output=True,text=True)
    if r.returncode:print("FAIL",p[-60:],r.stderr[:200])
print("done")
