"""🧪️ Throw-away overlay for preadmit-g: the live tree with phase-g's staged batch 4 applied (batch 4 is not landed yet) and then (unless PREADMIT_G_BASE_ONLY=1, which builds the baseline for `build_patch.sh`) this lane's staged diff on top, in the scratchpad; the laws run against it without touching the tree. Usage: PHASE_G_CALLSITES=0 python3 overlay.py build"""
import os,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__))
PHASE=os.path.join(HERE,"..","phase-g-tools")
SCRATCH="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad"
OV=os.environ.get("PREADMIT_G_OVERLAY",SCRATCH+"/preadmit-g-ov")
os.environ["PHASE_G_OVERLAY"]=OV
sys.path.insert(0,PHASE)
import overlay as base
F="🧰️framework"
LIB=f"{F}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
base.REAL_DIRS+=[f"{LIB}/🔧️tools/🏗️bazel/🌡️host-capacity"]
base.REAL_FILES+=[f"{LIB}/🔧️tools/🏗️bazel/📜️script.ts"]
base.OV=OV
def build():
    base.build()
    sys.path.insert(0,HERE)
    if os.environ.get("PREADMIT_G_BASE_ONLY")!="1":
        import apply
        apply.patch(OV,False)
    print(OV)
if __name__=="__main__":
    if sys.argv[1:]==["build"]:build()
    else:raise SystemExit(__doc__)
