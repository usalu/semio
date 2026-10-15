import re,sys,json
from plan import *
ROOT='/Users/ueli/Documents/semio'
ORDER=['contract','kinematics','clearance','traversal','interaction','character','staging']
FILES=[ROOT+'/.vscode/launch.json',ROOT+'/.vscode/🧩️launch.seed.jsonc']
ANCHOR='    {\n      "name": "🧪️test🐾️pets🦀️",'
KINDS=[('build','🏗️','4_build','900.0',1400),('check','🔎️','4_gate','922.04',400),('test','🧪️','4_test','903.58',400)]
CONF=[('prototype','🛠️'),('release','🚀️')]

def used_orders():
    used=set()
    for f in FILES:
        for v in re.findall(r'"order": ([0-9.]+)',open(f,encoding='utf-8').read()): used.add(v)
    return used

def allocate():
    used=used_orders(); out={}
    for kind,emoji,group,prefix,start in KINDS:
        n=0
        for c in ORDER:
            for conf,_ in CONF:
                while True:
                    n+=1
                    key=prefix+f'{start+n}'
                    if key not in used: break
                used.add(key); out[(kind,c,conf)]=key
    return out

def entry(kind,emoji,group,c,conf,cemoji,key):
    name=f'🦀️ {crate_name(c)} {emoji}{kind} {cemoji}{conf}'
    cmd=f'bun repo run @semio-tech/pets-{c}-rs:{kind} --configuration={conf} --parallel=1 -- --lib --offline'
    return ('    {\n'
      f'      "name": {json.dumps(name,ensure_ascii=False)},\n'
      '      "type": "node-terminal",\n      "request": "launch",\n'
      f'      "command": {json.dumps(cmd,ensure_ascii=False)},\n'
      '      "cwd": "${workspaceFolder}",\n      "presentation": {\n'
      f'        "group": "{group}",\n        "order": {key}\n      }}\n    }},\n')

def apply():
    orders=allocate()
    for f in FILES:
        s=open(f,encoding='utf-8').read()
        names=[crate_name(c) for c in ORDER]
        if any(f'"🦀️ {n} 🏗️build 🛠️prototype"' in s for n in names):
            print(f,'already has pets routes'); continue
        assert s.count(ANCHOR)==1,f
        i=s.index(ANCHOR); j=s.index('\n    },\n',i)+len('\n    },\n')
        block=''
        for c in ORDER:
            for kind,emoji,group,prefix,start in KINDS:
                for conf,cemoji in CONF:
                    block+=entry(kind,emoji,group,c,conf,cemoji,orders[(kind,c,conf)])
        s=s[:j]+block+s[j:]
        open(f,'w',encoding='utf-8').write(s)
        json.loads(s)
        print(f,'inserted',len(ORDER)*6,'routes; parses')
if __name__=='__main__': apply()
