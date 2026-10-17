import sys,os,re,collections
# usage: namecheck.py <new-files.txt>: folder = emoji + U+FE0F + kebab; docstring starts with the folder emoji; unique emoji among the siblings on disk; path <= 240 bytes
R="/Users/ueli/Documents/semio"
bad=0; maxlen=0
for rel in open(sys.argv[1],encoding="utf8").read().split("\n"):
    if not rel: continue
    maxlen=max(maxlen,len(rel.encode("utf8")))
    d=os.path.basename(os.path.dirname(rel))
    m=re.match(r"^(\S+?)️([a-z0-9][a-z0-9-]*)$",d)
    if not m: print("BAD NAME",repr(d)); bad+=1; continue
    emoji=m.group(1)
    first=open(os.path.join(R,rel),encoding="utf8").read().split("\n",1)[0]
    if not first.startswith("/** "+emoji): print("DOC emoji mismatch",rel[-60:],first[:20]); bad+=1
    parent=os.path.dirname(os.path.dirname(os.path.join(R,rel)))
    sib=collections.Counter()
    for n in os.listdir(parent):
        mm=re.match(r"^(\S+?)️",n)
        if mm: sib[mm.group(1)]+=1
    if sib[emoji]>1: print("DUP sibling emoji",emoji,parent[-60:]); bad+=1
print("checked; problems:",bad,"max path bytes:",maxlen)
