import sys,os,re,subprocess,hashlib
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import split as s
cur=open(s.BOOT+'🦀️.rs',encoding='utf-8').read()
assert cur==s.src or cur.startswith(s.GENERATED_MARK), 'bootstrap root is neither the snapshot nor generated'
out=s.build()
for m,(folder,emoji,desc) in s.MODULES.items():
    d=s.BOOT+folder
    os.makedirs(d,exist_ok=True)
    open(d+'/🦀️.rs','w',encoding='utf-8').write(out[m])
# standalone visibility
pub,_=s.ANALYSIS['access_grants']
ss=open(s.STANDALONE,encoding='utf-8').read()
for i in s.mods['access_grants']:
    r=s.rows[i]
    if r.get('virtual') and i in pub:
        pat='\nasync fn '+r['name']+'('
        if ss.count(pat)==1: ss=ss.replace(pat,'\npub(crate) async fn '+r['name']+'(')
        else: assert ss.count('\npub(crate) async fn '+r['name']+'(')==1,r['name']
open(s.STANDALONE,'w',encoding='utf-8').write(ss)
open(s.BOOT+'🦀️.rs','w',encoding='utf-8').write(out['ROOT'])
print('applied')
