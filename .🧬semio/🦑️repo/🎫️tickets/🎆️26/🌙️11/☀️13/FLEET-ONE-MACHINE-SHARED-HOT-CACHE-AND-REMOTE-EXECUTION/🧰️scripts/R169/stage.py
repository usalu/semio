import os,re,sys
S=os.path.dirname(os.path.abspath(__file__));M=S+"/mirror";R=S+"/run"
LIVE="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
import shutil
shutil.rmtree(R,ignore_errors=True)
pat=re.compile(r'''(from\s+|import\s*\(\s*)(["'])(\.{1,2}/[^"']*)\2''')
for dp,dn,fn in os.walk(M):
    for f in fn:
        p=os.path.join(dp,f)
        if "🧪️tests" not in p or not f.endswith(".ts"): continue
        rel=os.path.relpath(p,M);livedir=os.path.dirname(os.path.join(LIVE,rel))
        s=open(p,encoding="utf8").read()
        s=pat.sub(lambda m:m.group(1)+m.group(2)+os.path.normpath(os.path.join(livedir,m.group(3)))+m.group(2),s)
        d=os.path.join(R,rel);os.makedirs(os.path.dirname(d),exist_ok=True);open(d,"w",encoding="utf8").write(s)
        for extra in os.listdir(dp):
            if extra.endswith(".py"): shutil.copy2(os.path.join(dp,extra),os.path.dirname(d))
