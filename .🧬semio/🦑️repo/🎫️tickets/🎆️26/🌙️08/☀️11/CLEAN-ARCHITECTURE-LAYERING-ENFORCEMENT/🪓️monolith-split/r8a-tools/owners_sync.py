import json,os,re,sys
A,prefix=sys.argv[1:3]
disk={}
for root,ds,fs in os.walk(A):
    ds[:]=[x for x in ds if x not in ('node_modules','target')]
    if 'Cargo.toml' in fs:
        m=re.search(r'^name\s*=\s*"([^"]+)"',open(os.path.join(root,'Cargo.toml'),encoding='utf8').read(),re.M)
        if m and m.group(1).startswith(prefix): disk[root]=m.group(1)
p='📋️build-owners.json'
raw=open(p,encoding='utf8').read(); d=json.loads(raw)
owners=d['owners']
byroot={r['root']:r for r in owners if r.get('root') in disk or r.get('name','').startswith(prefix)}
keep=[]; used=set(); last=-1
for i,r in enumerate(owners):
    if r.get('name','').startswith(prefix) or r.get('root') in disk:
        last=i
        if r['root'] in disk and r['root'] not in used:
            r['name']=disk[r['root']]; r['manifest']=r['root']+'/Cargo.toml'; used.add(r['root']); keep.append(r)
        continue
    keep.append(r)
add=[{'name':n,'root':r,'manifest':r+'/Cargo.toml','packages':[]} for r,n in sorted(disk.items(),key=lambda x:x[1]) if r not in used]
pos=len([1 for i,r in enumerate(owners[:last+1]) if not (r.get('name','').startswith(prefix) or r.get('root') in disk) or (r['root'] in disk and True)])
idx=max((i for i,r in enumerate(keep) if r.get('root') in disk),default=len(keep)-1)
keep[idx+1:idx+1]=add
d['owners']=keep
open(p,'w',encoding='utf8').write(json.dumps(d,indent=2,ensure_ascii=False)+'\n')
print('disk',len(disk),'added',[a['name'] for a in add],'rows now',len([r for r in keep if r.get('root') in disk]))
