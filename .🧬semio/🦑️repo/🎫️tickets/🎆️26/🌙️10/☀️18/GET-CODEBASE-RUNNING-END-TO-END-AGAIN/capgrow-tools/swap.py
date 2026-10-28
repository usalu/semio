"""🔀️ capgrow: puts the batch files of the mirror aside and the live base back (`base`), or the batch back (`batch`), for red/green on one mirror: python3 -I swap.py <mirror> base|batch."""
import os,shutil,sys
mirror,mode=sys.argv[1],sys.argv[2];live="/Users/ueli/Documents/semio";aside=os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),"🗑️generated","capgrow","aside")
for rel in open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"files.txt"),encoding="utf8").read().split("\n"):
    if not rel:continue
    target=os.path.join(mirror,rel);kept=os.path.join(aside,rel.replace("/","__"))
    if mode=="base":
        os.makedirs(aside,exist_ok=True)
        if os.path.exists(target):shutil.copy2(target,kept)
        if os.path.exists(os.path.join(live,rel)):shutil.copy2(os.path.join(live,rel),target)
        elif os.path.exists(target):os.remove(target)
    else:
        if os.path.exists(kept):shutil.copy2(kept,target)
print(mode,"done")
