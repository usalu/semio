import os,re,hashlib
LIVE="/Users/ueli/Documents/semio"
LIB=LIVE+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
def sha(t):return hashlib.sha256(t.encode("utf8")).hexdigest()[:12]
def read(p):return open(p,encoding="utf8").read()
def write(p,t):
    os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"w",encoding="utf8").write(t)
def sub1(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:80])
    return text.replace(old,new)
def absolutize(text,origin_dir):
    def r(m):
        s=m.group(2)
        return m.group(1)+os.path.normpath(os.path.join(origin_dir,s))+m.group(3) if s.startswith("../") or s.startswith("./") else m.group(0)
    return re.sub(r'''((?:from\s*|import\s*\(\s*|import\s*)["'])(\.{1,2}/[^"']+)(["'])''',r,text)
