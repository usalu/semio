import hashlib,os,shutil,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from patch import patch
D=os.path.dirname(os.path.abspath(__file__))
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R105/"
PRD="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/"
LIVE=PRD+"🟦️.ts"
TEST=PRD+"🧪️tests/🤝️pool-reader/🟦️.ts"
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()
def land():
    os.makedirs(S+"backup",exist_ok=True)
    shutil.copyfile(LIVE,S+"backup/pr.ts")
    new=patch(open(LIVE).read())
    open(S+"landed-pr.ts","w").write(new)
    open(LIVE,"w").write(new)
    os.makedirs(os.path.dirname(TEST),exist_ok=True)
    shutil.copyfile(D+"/stage/law-test.ts",TEST)
def restore():
    if sha(LIVE)==sha(S+"landed-pr.ts"):shutil.copyfile(S+"backup/pr.ts",LIVE)
    else:print("live differs from what was landed, not restoring")
    shutil.rmtree(os.path.dirname(TEST),ignore_errors=True)
if __name__=="__main__":
    land() if sys.argv[1]=="land" else restore()
    print(sys.argv[1],"ok")
