import sys,os,re,json,subprocess
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import split as s
SCR='/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/hubo'
def warnings(path):
    txt=open(path,encoding='utf-8').read()
    out=[]
    for m in re.finditer(r'^warning: unused imports?: (.*?)\n\s+--> (.*?):(\d+):(\d+)',txt,re.M):
        names=re.findall(r'`([^`]+)`',m.group(1))
        out.append(([n.split('::')[-1] for n in names],m.group(2)))
    return out
def module_of(path):
    path=path.replace('/Users/ueli/Documents/semio/🌎️hub/','')
    if path.endswith('🏗️bootstrap/🦀️.rs') and '🏗️bootstrap/../' not in path and path.count('/')==1: return 'ROOT'
    for m,(folder,e,d) in s.MODULES.items():
        if '🏗️bootstrap/'+folder+'/' in path: return m
    return None
plain=warnings(SCR+'/n-plain.err');test=warnings(SCR+'/n-test.err')
pr=s.load_prune()
def key(ws):
    d={}
    for names,f in ws:
        m=module_of(f)
        if m:
            for n in names: d.setdefault(m,set()).add(n)
    return d
P=key(plain);T=key(test)
changed=False
for m,names in T.items():
    for n in names:
        e=pr.setdefault(m,{'drop':[],'test':[]})
        if n not in e['drop']: e['drop'].append(n);changed=True
        if n in e['test']: e['test'].remove(n)
for m,names in P.items():
    for n in names:
        e=pr.setdefault(m,{'drop':[],'test':[]})
        if n in T.get(m,set()) or n in e['drop']: continue
        if n not in e['test']: e['test'].append(n);changed=True
json.dump(pr,open(s.PRUNE_PATH,'w'),indent=1,ensure_ascii=False)
print('changed' if changed else 'stable')
