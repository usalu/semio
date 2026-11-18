import os,shutil
ROOT="/Users/ueli/Documents/semio"
LIB=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
HERE=os.path.dirname(os.path.abspath(__file__))
PAIRS=[(HERE+"/⏳️lease-wait/🟦️.ts",LIB+"/🪪️graph-state/🧪️tests/⏳️lease-wait/🟦️.ts"),(HERE+"/🚦️no-progress/🟦️.ts",LIB+"/🕸️graph/📥️inference/📦️project-nodes/📥️take/🧪️tests/🚦️no-progress/🟦️.ts")]
import sys
for src,dst in PAIRS:
    if "--remove" in sys.argv:
        shutil.rmtree(os.path.dirname(dst),ignore_errors=True);continue
    os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copy2(src,dst)
print("ok")
