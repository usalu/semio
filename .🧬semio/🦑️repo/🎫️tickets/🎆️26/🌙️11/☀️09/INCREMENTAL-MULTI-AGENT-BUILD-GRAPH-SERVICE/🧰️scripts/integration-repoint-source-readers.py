import os,re,sys
B="/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
GRAPH=B+"/🕸️graph/🟦️.ts";HELPER=B+"/🕸️graph/🧪️tests/📖️source/🟦️.ts"
files=sys.argv[1:]
for f in files:
    t=open(f,encoding="utf8").read();d=os.path.dirname(f);n=0
    def resolves(x):return os.path.normpath(os.path.join(d,x))==GRAPH
    def sub(m):
        global n
        x=m.group("x")
        if not resolves(x):return m.group(0)
        n+=1;return m.group("pre")+"graphSourceV1()"
    t=re.sub(r'(?P<pre>)readFileSync\(fileURLToPath\(new URL\("(?P<x>[^"]+)",import\.meta\.url\)\),"utf8"\)',sub,t)
    t=re.sub(r'(?P<pre>await )readFile\(new URL\("(?P<x>[^"]+)",import\.meta\.url\),"utf8"\)',sub,t)
    t=re.sub(r'(?P<pre>)readFileSync\(new URL\("(?P<x>[^"]+)",import\.meta\.url\),"utf8"\)',sub,t)
    if n==0:print("NOCHANGE",f);continue
    rel=os.path.relpath(HELPER,d)
    imp='import {graphSourceV1} from "%s";'%rel
    if "graphSourceV1}" not in t:
        i=t.index("\nimport");t=t[:i]+"\n"+imp+t[i:]
    open(f,"w",encoding="utf8").write(t);print("OK",n,f[-70:])
