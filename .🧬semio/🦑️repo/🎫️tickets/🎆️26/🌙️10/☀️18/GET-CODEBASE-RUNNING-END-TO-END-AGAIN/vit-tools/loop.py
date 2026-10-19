import os,re,subprocess,sys,urllib.parse
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
R="/Users/ueli/Documents/semio"
entry=sys.argv[1]
import importlib.util
for i in range(80):
    p=subprocess.run(["node","-e",'require("%s")'%entry],cwd=R,capture_output=True,text=True); t=p.stderr
    if p.returncode==0: print("OK"); break
    if "needs an import attribute" in t:
        subprocess.run(["python3",S+"/fixjson.py",entry],cwd=R); continue
    m=re.search(r'file://([^\n:]+\.m?ts):(\d+)',t)
    if "parameter property" in t and m:
        f=urllib.parse.unquote(m.group(1)); r=subprocess.run(["python3",S+"/fixpp.py",f],capture_output=True,text=True).stdout
        print(r.strip())
        if r.strip().endswith(" 0"): print(t[:800]); break
        continue
    print(t[:1500]); break
