"""🧪️ Builds a throw-away overlay of the process module and the host-capacity library module in the scratchpad (a symlink farm; only the directories the staged change touches are real copies), applies the staged patches to it and prints the root, so the staged laws run against the change without touching the live tree. Usage: python3 overlay.py build | sync"""
import os,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,HERE)
REPO="/Users/ueli/Documents/semio"
OV=os.environ.get("PHASE_G_OVERLAY","/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/phase-g-ov")
FRAMEWORK="🧰️framework"
REAL_DIRS=[
 f"{FRAMEWORK}/🔨️modules/🏃️process/🏗️build/🌡️host-capacity",
 f"{FRAMEWORK}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity",
 f"{FRAMEWORK}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity",
]
REAL_DIRS+=[
 f"{FRAMEWORK}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests",
]
REAL_FILES=[
 f"{FRAMEWORK}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json",
 f"{FRAMEWORK}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts",
 f"{FRAMEWORK}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts",
]

def farm():
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

def build():
    if os.path.isdir(OV):shutil.rmtree(OV)
    os.makedirs(OV)
    farm()
    for rel in REAL_DIRS:
        shutil.copytree(os.path.join(REPO,rel),os.path.join(OV,rel),symlinks=True,ignore=shutil.ignore_patterns("node_modules",".DS_Store"))
    for rel in REAL_FILES:
        os.makedirs(os.path.dirname(os.path.join(OV,rel)),exist_ok=True);shutil.copy2(os.path.join(REPO,rel),os.path.join(OV,rel))
    for name in ("node_modules","package.json","tsconfig.json"):
        if not os.path.lexists(os.path.join(OV,name)):os.symlink(os.path.join(REPO,name),os.path.join(OV,name))
    import stage
    stage.apply(OV)
    print(OV)

if __name__=="__main__":
    if sys.argv[1:]==["build"]:build()
    elif sys.argv[1:]==["sync"]:
        import stage;stage.install_new(OV);stage.flush(OV);print(OV)
    else:raise SystemExit(__doc__)
