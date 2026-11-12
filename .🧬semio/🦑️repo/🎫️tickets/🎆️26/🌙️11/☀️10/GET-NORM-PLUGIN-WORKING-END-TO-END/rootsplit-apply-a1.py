import json,sys,os,re,importlib.util
T=os.path.dirname(os.path.abspath(__file__))
spec=importlib.util.spec_from_file_location('p',T+'/rootsplit-plan-a1.py');p=importlib.util.module_from_spec(spec);spec.loader.exec_module(p)
S='/tmp/claude-1000/-home-deck-semio/0ec9c8a1-06e8-45c6-b0fa-6d0c7cb86d6b/scratchpad'
names=json.load(open(S+'/names.json'))
APPLY=len(sys.argv)>1 and sys.argv[1]=='apply'
def load(f):
    return json.loads(open(f,encoding='utf8').read())
spec2=importlib.util.spec_from_file_location('j',T+'/rootsplit-jsontext.py');J=importlib.util.module_from_spec(spec2);spec2.loader.exec_module(J)
added={}
def save(f,d): open(f,'w',encoding='utf8').write(json.dumps(d,indent=2,ensure_ascii=False)+'\n')
rootd=load('📋️project.json')
docs={}
mapping=[]
for t,name,args,cands in p.plan:
    proj=cands[0][0]
    f=names[proj]
    d=docs.get(f) or docs.setdefault(f,load(f))
    pick=[x for x in cands if (d['targets'][x[1]].get('metadata',{}).get('semio',{}).get('workspaceCommand')==['verify',name])]
    base=(pick or cands)[0][1]
    if not args: dest=base
    else:
        dest='verify-'+name+'-'+'-'.join(args)
        if dest not in d['targets']:
            nt=json.loads(json.dumps(d['targets'][base]))
            nt['options']['command']=nt['options']['command']+' '+' '.join(args)
            nt.get('metadata',{}).get('semio',{}).pop('workspaceCommand',None)
            if nt.get('metadata',{}).get('semio')=={}: nt['metadata'].pop('semio')
            if nt.get('metadata')=={}: nt.pop('metadata')
            d['targets'][dest]=nt; added.setdefault(f,{})[dest]=nt
    mapping.append((t,proj,dest))
    del rootd['targets'][t]
json.dump(mapping,open(T+'/🗑️generated/rootsplit-a1-mapping.json','w'),ensure_ascii=False,indent=1)
print(len(mapping),len(docs))
if APPLY:
    for f,new in added.items():
        txt=open(f,encoding='utf8').read(); open(f,'w',encoding='utf8').write(J.add_targets(txt,new))
    rr=open('📋️project.json',encoding='utf8').read(); assert json.dumps(json.loads(rr),indent=2,ensure_ascii=False)+'\n'==rr
    save('📋️project.json',rootd)
