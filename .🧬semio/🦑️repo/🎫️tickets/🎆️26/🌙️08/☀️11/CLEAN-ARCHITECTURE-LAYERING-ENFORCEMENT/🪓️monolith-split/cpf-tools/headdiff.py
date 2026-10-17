import sys,subprocess,difflib,re
sys.path.insert(0,__file__.rsplit("/",1)[0])
from edit_lib import *
p=sys.argv[1]; key=sys.argv[2]
cur=read(p)
head=subprocess.run(["git","-c","core.fsmonitor=false","show","HEAD:"+p],capture_output=True,text=True,cwd=ROOT).stdout
def line(t):
    for l in t.split("\n"):
        if key in l: return l
cl,hl=line(cur),line(head)
tok=lambda s: re.findall(r'\w+|\W',s)
a,b=tok(hl or ""),tok(cl or "")
sm=difflib.SequenceMatcher(None,a,b,autojunk=False)
for op,i1,i2,j1,j2 in sm.get_opcodes():
    if op!="equal": print(op,"HEAD:",repr("".join(a[i1:i2])[:900]),"=> NOW:",repr("".join(b[j1:j2])[:1500]))
