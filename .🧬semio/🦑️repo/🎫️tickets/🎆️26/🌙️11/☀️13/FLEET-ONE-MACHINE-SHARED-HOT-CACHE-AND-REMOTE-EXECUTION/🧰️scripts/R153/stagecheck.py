# stagecheck.py <group> : rewritten copies (absolute imports; staged siblings remapped) under the fleet ticket scripts/R153; prints their paths
import sys,json,os,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from stagelib import *
g=sys.argv[1];d=f"{S}/g-{g}";man=json.load(open(d+"/manifest.json"))
CH=R+'/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R153';os.makedirs(CH,exist_ok=True)
ext=lambda p:os.path.splitext(p)[1]
outs={m["path"]:f'{CH}/chk-{g}-{m["i"]}{ext(m["path"])}' for m in man}
TS="from\""+R+"/node_modules/typescript/lib/typescript.js\""
for m in man:
    base=os.path.dirname(m["path"]);t=open(f'{d}/{m["i"]}.new',encoding="utf8").read()
    def fix(mo):
        q,p=mo.group(1),mo.group(2);a=os.path.normpath(os.path.join(base,p));return q+outs.get(a,a)+q
    t=re.sub(r'(["\'])(\.\.?/[^"\']*)\1',fix,t)
    t=re.sub(r'new URL\((["\'])(/[^"\']*)\1\s*,\s*import\.meta\.url\)',lambda mo:f'new URL("file://{mo.group(2)}")',t)
    t=t.replace('import.meta.dir',json.dumps(base,ensure_ascii=False)).replace('import.meta.url','"file://'+m["path"]+'"')
    open(outs[m["path"]],"w",encoding="utf8").write(t);print(outs[m["path"]])
