import sys, os, json, re
# usage: pst-restore-parser.py <label> <repo-relative leaf> <ParserName>...
# puts the hand-written text of each named parser (from my snapshot of that label) back in place of the generated one
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
root="/Users/ueli/Documents/semio/"
snap=root+F+"/🗑️generated/pst-snap/"
man=json.load(open(snap+"manifest.json",encoding="utf8"))
def prange(src,name):
    m=re.search(r"^export function parse"+re.escape(name)+r"\(",src,re.M)
    if not m: return None
    le=src.find("\n",m.start())
    if le<0: return None
    first=src[m.start():le]
    if first.rstrip().endswith("}") and first.count("{")==first.count("}"): return (m.start(),le+1)
    at=le+1
    while at<len(src):
        nx=src.find("\n",at)
        line=src[at:(len(src) if nx<0 else nx)]
        if line=="}": return (m.start(),len(src) if nx<0 else nx+1)
        if re.match(r"^(?:export |function |const |let |class |interface |type |import |//#)",line): return None
        if nx<0: return None
        at=nx+1
    return None
label,rel=sys.argv[1],sys.argv[2]
k=[k for k,m in man.items() if m["path"]==rel and m["label"]==label][0]
old=open(snap+k+os.path.splitext(rel)[1],encoding="utf8").read()
cur=open(root+rel,encoding="utf8").read()
for name in sys.argv[3:]:
    ro=prange(old,name); rc=prange(cur,name)
    assert ro and rc,(name,ro,rc)
    cur=cur[:rc[0]]+old[ro[0]:ro[1]]+cur[rc[1]:]
    print("restored parse"+name)
open(root+rel,"w",encoding="utf8").write(cur)
