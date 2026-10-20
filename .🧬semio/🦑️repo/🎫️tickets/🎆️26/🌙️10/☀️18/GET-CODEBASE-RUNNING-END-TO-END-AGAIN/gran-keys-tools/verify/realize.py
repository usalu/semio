import os,shutil,subprocess,sys
SB=os.environ.get("SB","/private/tmp/gk/sb"); ROOT="/Users/ueli/Documents/semio"
for rel in sys.argv[1:]:
    p=os.path.join(SB,rel)
    if os.path.islink(p): os.unlink(p)
    else: shutil.rmtree(p)
    subprocess.run(["cp","-Rc",os.path.join(ROOT,rel),p],check=True); print("realized",rel)
