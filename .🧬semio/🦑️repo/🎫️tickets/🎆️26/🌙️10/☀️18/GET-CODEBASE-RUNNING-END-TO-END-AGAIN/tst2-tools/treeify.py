import os,re,sys
sys.path.insert(0,os.path.dirname(__file__))
from ed import edit,ROOT
from urllib.parse import unquote
def resolve(test,rel):
    base=os.path.dirname(os.path.join(ROOT,test))
    return os.path.normpath(os.path.join(base,rel))
def mounts(path):
    try: t=open(path,encoding="utf-8").read()
    except Exception: return False
    return bool(re.search(r'#\[path\s*=\s*"(?!🧪️tests)',t)) or "include!(" in t
RE1=re.compile(r'readFileSync\(\s*new URL\((["\'])([^"\']+\.rs)\1\s*,\s*import\.meta\.url\)\s*,\s*(?:["\']utf-?8["\']|\{\s*encoding:\s*["\']utf-?8["\']\s*\})\s*\)')
RE2=re.compile(r'(await\s+)?Bun\.file\(\s*new URL\((["\'])([^"\']+\.rs)\2\s*,\s*import\.meta\.url\)\s*\)\.text\(\)')
RE3=re.compile(r'(await\s+)?Bun\.file\(\s*frameworkSourcePath\(("|\')([^"\']+\.rs)\2\)\s*\)\.text\(\)')
def run(test,apply):
    p=os.path.join(ROOT,test)
    t=open(p,encoding="utf-8").read(); n=0; changed=[]
    def r1(m):
        nonlocal n
        tgt=resolve(test,m.group(2))
        if mounts(tgt):
            n+=1; changed.append(m.group(2)); return f'readModuleTreeSource(new URL({m.group(1)}{m.group(2)}{m.group(1)},import.meta.url))'
        return m.group(0)
    def r2(m):
        nonlocal n
        tgt=resolve(test,m.group(3))
        if mounts(tgt):
            n+=1; changed.append(m.group(3)); return f'readModuleTreeSource(new URL({m.group(2)}{m.group(3)}{m.group(2)},import.meta.url))'
        return m.group(0)
    def r3(m):
        nonlocal n
        tgt=os.path.join(ROOT,"🧰️framework",m.group(3))
        if mounts(tgt):
            n+=1; changed.append(m.group(3)); return f'readModuleTreeSource(frameworkSourcePath({m.group(2)}{m.group(3)}{m.group(2)}))'
        return m.group(0)
    def r4(m):
        nonlocal n
        tgt=os.path.join(ROOT,"🧰️framework",m.group(2))
        if mounts(tgt):
            n+=1; changed.append(m.group(2)); return f'readModuleTreeSource(frameworkSourcePath({m.group(1)}{m.group(2)}{m.group(1)}))'
        return m.group(0)
    RE4=re.compile(r'readFileSync\(\s*frameworkSourcePath\((["\'])([^"\']+\.rs)\1\)\s*,\s*["\']utf-?8["\']\s*\)')
    t2=RE4.sub(r4,RE3.sub(r3,RE2.sub(r2,RE1.sub(r1,t))))
    print(test[-70:],n,"readers",sorted(set(changed))[:6])
    if apply and n:
        open(p,"w",encoding="utf-8").write(t2)
        edit(test,[],True)
if __name__=="__main__":
    apply=sys.argv[1]=="apply"
    for t in sys.argv[2:]: run(t,apply)
