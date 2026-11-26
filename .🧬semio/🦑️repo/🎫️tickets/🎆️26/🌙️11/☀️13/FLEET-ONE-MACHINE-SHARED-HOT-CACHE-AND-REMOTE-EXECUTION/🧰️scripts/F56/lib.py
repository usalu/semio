import json,os,re,subprocess,sys,time
ROOT='/Users/ueli/Documents/semio'
T=ROOT+'/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION'
F=ROOT+'/🧰️framework/'
H=F+'🔨️modules/🏃️process/🏗️build/🌡️host-capacity/'
P=F+'🛍️products/'
R=P+'🦑️repo/🔨️modules/'
L=R+'📚️library/'
B=L+'🏗️build/'
def sub(path,old,new,count=1):
    s=open(path,encoding='utf8').read()
    n=s.count(old)
    if n!=count: raise SystemExit(f'ANCHOR {n}!={count} in {path[-60:]}: {old[:80]}')
    s=s.replace(old,new)
    tmp=path+'.f56tmp'
    open(tmp,'w',encoding='utf8').write(s); os.replace(tmp,path)
def baseline_drop(files):
    p=F+'🔨️modules/🏃️process/🔖️identity/🧪️tests/🔒️law/🔣️.json'
    d=json.load(open(p,encoding='utf8'))
    for f in files:
        rel=f[len(ROOT)+1:] if f.startswith(ROOT) else f
        if rel not in d['sites']: raise SystemExit('not in baseline '+rel)
        del d['sites'][rel]
    tmp=p+'.f56tmp'
    open(tmp,'w',encoding='utf8').write(json.dumps(d,ensure_ascii=False,indent=1)+'\n'); os.replace(tmp,p)
def baseline_count(f,count):
    p=F+'🔨️modules/🏃️process/🔖️identity/🧪️tests/🔒️law/🔣️.json'
    d=json.load(open(p,encoding='utf8'))
    rel=f[len(ROOT)+1:] if f.startswith(ROOT) else f
    d['sites'][rel]['count']=count
    tmp=p+'.f56tmp'
    open(tmp,'w',encoding='utf8').write(json.dumps(d,ensure_ascii=False,indent=1)+'\n'); os.replace(tmp,p)
def lock(areas):
    for a in sorted(areas):
        d=T+'/landing.lock.'+a
        os.mkdir(d); open(d+'/owner','w').write('F56 '+time.strftime('%H:%M')+'\n')
def unlock(areas):
    for a in sorted(areas):
        d=T+'/landing.lock.'+a
        if open(d+'/owner').read().startswith('F56'):
            os.remove(d+'/owner'); os.rmdir(d)
def run(*c):
    r=subprocess.run(c,cwd=ROOT,capture_output=True,text=True); return r.returncode,r.stdout+r.stderr
def tests(*files):
    for f in files:
        code,out=run('repo','test-file',f); print('TEST',f[-70:],'->',[l for l in out.splitlines() if l.startswith(('passed','failed','test-file:'))] or out[-600:])
def dangling(*folders):
    for f in folders:
        code,out=run('python3',T+'/🧰️scripts/coordinator/dangling-imports.py',f); print('DANGLING',code,out[-300:])
IDTS=F+'🔨️modules/🏃️process/🔖️identity/🟦️.ts'
IDJS=F+'🔨️modules/🏃️process/🔖️identity/🟨️.mjs'
def addimport(path,names,mjs=False,semicolon=True):
    s=open(path,encoding='utf8').read()
    target=IDJS if mjs else IDTS
    rel=os.path.relpath(target,os.path.dirname(path))
    if not rel.startswith('.'): rel='./'+rel
    line='import {%s} from "%s"%s\n'%(','.join(names),rel,';' if semicolon else '')
    if 'identity/' in s.split('\n',3)[0:3].__str__() and ('🔖️identity' in s): 
        raise SystemExit('already imports identity: '+path[-60:])
    lines=s.split('\n')
    idx=0
    for i,l in enumerate(lines[:40]):
        if l.startswith('import ') and l.rstrip().endswith((';','"',"'",'}')) and ' from ' in l: idx=i+1;break
    if idx==0: idx=0
    lines.insert(idx,line.rstrip('\n'))
    tmp=path+'.f56tmp'
    open(tmp,'w',encoding='utf8').write('\n'.join(lines)); os.replace(tmp,path)
