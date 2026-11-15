import os,json,subprocess,sys,re
R='/Users/Shared/semio'
SKIP={'.cache','node_modules','.git','target','.🧬semio'}
mans=[]
for d,ds,fs in os.walk(R):
    ds[:]=[x for x in ds if x not in SKIP]
    if 'Cargo.toml' in fs:mans.append(d)
print('manifests',len(mans))
ws=[]
for d in mans:
    try:t=open(d+'/Cargo.toml',encoding='utf8').read()
    except Exception:continue
    if re.search(r'^\[workspace[\].]',t,re.M):ws.append(d)
print('workspace manifests',len(ws))
C=os.path.expanduser('~/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/cargo')
env=dict(os.environ,RUSTC_WRAPPER='')
pk={}
def run(d):
    r=subprocess.run([C,'metadata','--offline','--no-deps','--format-version','1','--config','build.rustc="rustc"','--config','build.rustc-wrapper=""','--manifest-path',d+'/Cargo.toml'],capture_output=True,text=True,env=env,cwd=R)
    if r.returncode:return None,r.stderr[:300]
    return json.loads(r.stdout),None
errs=[]
for d in ws:
    m,e=run(d)
    if m is None:errs.append((d,e));continue
    for p in m['packages']:
        pk[p['manifest_path']]={'name':p['name'],'features':p['features'],'edition':p['edition'],'links':p.get('links'),'deps':[{'name':x['name'],'kind':x['kind'],'path':x['path'],'target':x['target'],'optional':x['optional'],'rename':x['rename'],'features':x['features'],'df':x['uses_default_features']} for x in p['dependencies'] if x.get('path')],'targets':[{'kind':t['kind'],'src':t['src_path']} for t in p['targets']]}
print('packages from workspaces',len(pk),'errors',len(errs))
for e in errs[:5]:print(e)
rest=[d for d in mans if d+'/Cargo.toml' not in pk]
print('manifests not covered',len(rest))
n=0
for d in rest:
    if d+'/Cargo.toml' in pk:continue
    m,e=run(d)
    if m is None:continue
    n+=1
    for p in m['packages']:
        pk[p['manifest_path']]={'name':p['name'],'features':p['features'],'edition':p['edition'],'links':p.get('links'),'deps':[{'name':x['name'],'kind':x['kind'],'path':x['path'],'target':x['target'],'optional':x['optional'],'rename':x['rename'],'features':x['features'],'df':x['uses_default_features']} for x in p['dependencies'] if x.get('path')],'targets':[{'kind':t['kind'],'src':t['src_path']} for t in p['targets']]}
print('extra calls',n,'packages',len(pk))
json.dump(pk,open('oracle-meta.json','w'),ensure_ascii=False)
