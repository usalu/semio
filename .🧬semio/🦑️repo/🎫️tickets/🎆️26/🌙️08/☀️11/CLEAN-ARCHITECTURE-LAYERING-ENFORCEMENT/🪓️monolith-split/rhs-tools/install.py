import sys,os,re,hashlib,json,glob,shutil
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from backend_split import T,SC
from rsitems import items,name_kind
from stage import remove_items,dedupe_pred
REPO='/Users/ueli/Documents/semio/'
DRY='--dry' in sys.argv
R_='🧰️framework/🛍️products/🦑️repo/🔨️modules/'
TARGETS={
 'sqlite':'🌎️hub/📇️directory/🪶️sqlite',
 'postgres':'🌎️hub/📇️directory/🐘️postgres',
 'neo4j':'🌎️hub/📇️directory/🌐️neo4j',
 'model':R_+'📐️model/📦️packages/🦀️rust',
 'tickets':R_+'🎫️tickets/📦️packages/🦀️rust',
 'gateway':'🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway',
}
ONLY=[a for a in sys.argv[1:] if not a.startswith('--')]
log=[]
touched=[]

def sha(b):return hashlib.sha256(b).hexdigest()
def read(p):return open(REPO+p,encoding='utf-8').read()
def write(p,t):
    full=REPO+p
    assert len(full.encode('utf-8'))<=240,('path too long',p,len(full.encode('utf-8')))
    if DRY:log.append(('write',p));return
    os.makedirs(os.path.dirname(full),exist_ok=True)
    tmp=full+'.rhs-tmp'
    open(tmp,'w',encoding='utf-8').write(t)
    os.replace(tmp,full)
    touched.append(p)
def edit(p,old,new,count=1):
    t=read(p)
    assert t.count(old)==count,('anchor count',p,old[:80],t.count(old))
    write(p,t.replace(old,new))

def install_stage(name):
    rel=TARGETS[name]
    snap=open(os.path.join(T,'orig/%s.rs.txt'%name),'rb').read()
    live=open(REPO+rel+'/🦀️.rs','rb').read()
    assert sha(snap)==sha(live),('live file changed since snapshot',name)
    st=os.path.join(SC,'rhs-stage',name)
    for e in sorted(os.listdir(st)):
        p=os.path.join(st,e)
        if os.path.isdir(p):
            dst=rel+'/'+e+'/🦀️.rs'
            assert not os.path.exists(REPO+dst) or open(REPO+dst,encoding='utf-8').read()==open(p+'/🦀️.rs',encoding='utf-8').read(),('child exists',dst)
            write(dst,open(p+'/🦀️.rs',encoding='utf-8').read())
    write(rel+'/🦀️.rs',open(os.path.join(st,'🦀️.rs'),encoding='utf-8').read())

def main():
    names=ONLY or list(TARGETS)
    # shared directory core + rows + test imports first when backends are installed
    if any(n in names for n in('sqlite','postgres','neo4j')):
        C='🌎️hub/📇️directory/📜️contract/🦀️.rs'
        ct=read(C)
        anchor='#[path = "../🕰️events/🦀️.rs"]\nmod events;\nuse events::*;\n'
        assert ct.count(anchor)==1
        new_file=open(os.path.join(T,'new/backend-support.rs'),encoding='utf-8').read()
        write('🌎️hub/📇️directory/🛟️backend-support/🦀️.rs',new_file)
        ct=ct.replace(anchor,anchor+'\n#[path = "../🛟️backend-support/🦀️.rs"]\nmod backend_support;\n')
        tail='pub use events::{HubClock, NewDirectoryEvent, now_ms};\n'
        assert ct.endswith(tail),'contract tail changed'
        ct=ct+'pub use backend_support::{AdminEffectPreflightFailure, actor_kind_from_str, actor_kind_to_str, admin_effect_preflight, backend};\n'
        write(C,ct)
        # rows
        RW='🌎️hub/📇️directory/🌐️neo4j/🧰️rows/🦀️.rs'
        rt=read(RW)
        assert sha(rt.encode())==sha(open(os.path.join(T,'orig/neo4j-rows.rs.txt'),'rb').read()),'rows changed'
        rt2,removed=remove_items(rt,dedupe_pred)
        assert len(removed)==7,len(removed)
        used=[n for n in('actor_kind_from_str','actor_kind_to_str','backend','now_ms') if re.search(r'\b%s\b'%n,rt2)]
        assert 'admin_effect_preflight' not in rt2 or True
        anchor2='use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};\n'
        assert rt2.count(anchor2)==1
        rt2=rt2.replace(anchor2,anchor2+'use semio_hub_directory_contract::{%s};\n'%', '.join(sorted(used)))
        write(RW,rt2)
        # tests
        TE=[
         ('🌎️hub/📇️directory/🪶️sqlite/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs',[('use semio_hub_directory_contract::{HubClock, NewDirectoryEvent};\nuse semio_hub_directory_sqlite::{SqliteDirectory, now_ms};','use semio_hub_directory_contract::{HubClock, NewDirectoryEvent, now_ms};\nuse semio_hub_directory_sqlite::SqliteDirectory;'),('use semio_hub_directory_sqlite::now_ms;','use semio_hub_directory_contract::now_ms;',2)]),
         ('🌎️hub/📇️directory/🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs',[('use semio_hub_directory_contract::{ACTIVE_SYNC_SESSION_READ_MAX, AUTH_AUDIT_PAGE_MAX, HubClock, NewDirectoryEvent};','use semio_hub_directory_contract::{ACTIVE_SYNC_SESSION_READ_MAX, AUTH_AUDIT_PAGE_MAX, HubClock, NewDirectoryEvent, now_ms};'),('use semio_hub_directory_sqlite::{SqliteDirectory, now_ms};','use semio_hub_directory_sqlite::SqliteDirectory;')]),
         ('🌎️hub/📇️directory/🐘️postgres/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs',[('use semio_hub_directory_contract::{HubClock, NewDirectoryEvent};','use semio_hub_directory_contract::{HubClock, NewDirectoryEvent, now_ms};'),('use semio_hub_directory_postgres::{PostgresDirectory, now_ms};','use semio_hub_directory_postgres::PostgresDirectory;')]),
         ('🌎️hub/📇️directory/🐘️postgres/🧪️tests/🔬️unit/🦀️.rs',[('use semio_hub_directory_contract::{HubClock, NewDirectoryEvent};','use semio_hub_directory_contract::{HubClock, NewDirectoryEvent, now_ms};'),('use semio_hub_directory_postgres::{PostgresDirectory, now_ms};','use semio_hub_directory_postgres::PostgresDirectory;')]),
         ('🌎️hub/📇️directory/🌐️neo4j/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs',[('use semio_hub_directory_contract::{HubClock, NewDirectoryEvent};','use semio_hub_directory_contract::{HubClock, NewDirectoryEvent, now_ms};'),('use semio_hub_directory_neo4j_rows::now_ms;\n','')]),
         ('🌎️hub/📇️directory/🌐️neo4j/🧪️tests/🔬️unit/🦀️.rs',[('use semio_hub_directory_contract::NewDirectoryEvent;','use semio_hub_directory_contract::{NewDirectoryEvent, now_ms};'),('use semio_hub_directory_neo4j_rows::now_ms;\n','')]),
        ]
        for p,eds in TE:
            t=read(p)
            for e in eds:
                cnt=e[2] if len(e)>2 else 1
                assert t.count(e[0])==cnt,('test anchor',p,e[0][:70],t.count(e[0]))
                t=t.replace(e[0],e[1])
            write(p,t)
    for n in names:
        install_stage(n)
    if 'model' in names:
        pass
    print('dry' if DRY else 'installed',len(log) if DRY else len(touched))
    json.dump(touched if not DRY else [x[1] for x in log],open(os.path.join(T,'installed-files.json'),'w',encoding='utf-8'),ensure_ascii=False,indent=0)
if __name__=="__main__":main()
