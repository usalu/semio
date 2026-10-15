import os,re,sys,json
from plan import *

ROOT='/Users/ueli/Documents/semio'
ORDER=['contract','kinematics','clearance','traversal','interaction','character','staging']

def member_path(c): return os.path.relpath(pkg_dir(c),ROOT)

def step_members():
    p=ROOT+'/Cargo.toml'
    s=open(p,encoding='utf-8').read()
    m=re.search(r'members = \[(.*?)\n\]',s,re.S)
    items=re.findall(r'"([^"]+)"',m.group(1))
    key=lambda x:x.encode()
    assert items==sorted(items,key=key)
    new=[member_path(c) for c in ORDER if member_path(c) not in items]
    if not new: print('members already present'); return
    allm=sorted(items+new,key=key)
    body='\n'+'\n'.join(f'    "{x}",' for x in allm)
    s2=s[:m.start(1)]+body+s[m.end(1):]
    assert s2.count('members = [')==1
    open(p,'w',encoding='utf-8').write(s2)
    print('inserted',len(new),'members; total',len(allm))

def step_owners():
    p=ROOT+'/📋️build-owners.json'
    s=open(p,encoding='utf-8').read()
    data=json.loads(s)
    owners=data['owners']
    names=[o['name'] for o in owners]
    added=0
    for c in ORDER:
        name=f'@semio-tech/pets-{c}-rs'
        if name in names: continue
        row={'name':name,'root':member_path(c),'manifest':member_path(c)+'/📋️project.json','packages':[]}
        idx=[i for i,o in enumerate(owners) if o['name'].startswith('@semio-tech/pets')]
        pos=next((i for i in idx if owners[i]['name']>name),idx[-1]+1)
        owners.insert(pos,row); added+=1
    out=json.dumps(data,indent=2,ensure_ascii=False)+'\n'
    open(p,'w',encoding='utf-8').write(out)
    print('owners added',added)

def route(name,cmd,group,order):
    return {'name':name,'type':'node-terminal','request':'launch','command':cmd,'cwd':'${workspaceFolder}','presentation':{'group':group,'order':order}}

def routes(order_base):
    rows=[]
    for i,c in enumerate(ORDER):
        crate=crate_name(c); proj=f'@semio-tech/pets-{c}-rs'
        for t,(emoji,group,band) in (('build',('🏗️','4_build',900.0)),('check',('🔎️','4_gate',922.0)),('test',('🧪️','4_test',903.0))):
            for conf,cemoji in (('prototype','🛠️'),('release','🚀️')):
                rows.append((t,crate,proj,conf,emoji,cemoji,group))
    return rows

if __name__=='__main__':
    if sys.argv[1]=='members': step_members()
    if sys.argv[1]=='owners': step_owners()
