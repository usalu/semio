"""🧪️ quietfc-g: throw-away overlays of the live tree (symlink farm; only the host capacity directories and the build policy are real copies), in the scratchpad: `base` = byte copy of the live files (the diff baseline), `ov` = the working copy the staged change is edited in and the laws run against. Usage: python3 overlay.py build base|ov"""
import os,shutil,sys
REPO="/Users/ueli/Documents/semio"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad"
F="🧰️framework"
LIB=f"{F}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
REAL_DIRS=[f"{F}/🔨️modules/🏃️process/🏗️build/🌡️host-capacity",f"{F}/🔨️modules/🏃️process/🧪️testing/👷️workers",f"{F}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity",f"{LIB}/🌡️host-capacity",f"{LIB}/👷️workers",f"{LIB}/🔧️tools/🏗️bazel/🌡️host-capacity"]
REAL_FILES=[f"{LIB}/🔣️policy.json",f"{LIB}/🔧️tools/🏗️bazel/📜️script.ts",f"{F}/🛍️products/🦑️repo/📓️build-guide.md"]
def farm(OV):
    leaves=set(REAL_DIRS+REAL_FILES);ancestors=set()
    for rel in leaves:
        parts=rel.split("/")
        for index in range(1,len(parts)):ancestors.add("/".join(parts[:index]))
    for prefix in sorted(ancestors,key=lambda row:row.count("/")):
        os.makedirs(os.path.join(OV,prefix),exist_ok=True)
        for name in os.listdir(os.path.join(REPO,prefix)):
            child=prefix+"/"+name
            if name==".DS_Store" or child in ancestors or child in leaves:continue
            os.symlink(os.path.join(REPO,child),os.path.join(OV,child))
def build(name):
    OV=f"{S}/quietfc-g-{name}"
    if os.path.isdir(OV):shutil.rmtree(OV)
    os.makedirs(OV);farm(OV)
    for rel in REAL_DIRS:shutil.copytree(os.path.join(REPO,rel),os.path.join(OV,rel),symlinks=True,ignore=shutil.ignore_patterns("node_modules",".DS_Store"))
    for rel in REAL_FILES:os.makedirs(os.path.dirname(os.path.join(OV,rel)),exist_ok=True);shutil.copy2(os.path.join(REPO,rel),os.path.join(OV,rel))
    for n in ("node_modules","package.json","tsconfig.json"):
        if not os.path.lexists(os.path.join(OV,n)):os.symlink(os.path.join(REPO,n),os.path.join(OV,n))
    print(OV)
if __name__=="__main__":
    if len(sys.argv)==3 and sys.argv[1]=="build" and sys.argv[2] in("base","ov"):build(sys.argv[2])
    else:raise SystemExit(__doc__)
