import re,sys,os
S='✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any'
def cut(text,marker):
    i=text.index(marker); assert text.count(marker)==1,marker
    # include preceding doc-comment lines
    ls=text.rfind('\n',0,i)+1
    while True:
        pl=text.rfind('\n',0,ls-1)+1
        prev=text[pl:ls-1] if ls>0 else ''
        if prev.strip().startswith('///'): ls=pl
        else: break
    j=text.index('{',i); depth=0; k=j
    while True:
        c=text[k]
        if c=='{':depth+=1
        elif c=='}':
            depth-=1
            if depth==0:break
        k+=1
    end=k+1
    if text[end:end+1]=='\n': end+=1
    return text[:ls]+text[end:], text[ls:end]
blocks=[]
def load(p): return open(p,encoding='utf8').read()
def save(p,t): open(p,'w',encoding='utf8').write(t)
bp=f'{S}/🚪️io/💾️binary/🧬️mutations/🦀️.rs'; t=load(bp)
t,b1=cut(t,'impl pack::snapshot::ArtifactPack for Puzzle5dPlaySnapshot {'); blocks.append(b1)
t,b2=cut(t,'impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Puzzle5dPlaySnapshot {'); blocks.append(b2)
t=t.replace('use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;\nuse crate::standards::v1::subsets::any::schema::mutations::*;','use crate::standards::v1::subsets::any::schema::mutations::*;',1)
save(bp,t)
tp=f'{S}/🚪️io/📝️text/🧬️mutations/🦀️.rs'; t=load(tp)
t,b3=cut(t,'impl pack::snapshot::ArtifactDsl for Puzzle5dPlaySnapshot {'); blocks.append(b3)
t=t.replace('use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;\n','',1)
save(tp,t)
sp=f'{S}/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs'; t=load(sp)
t,b4=cut(t,'impl ArtifactSqliteSnapshotHost for Puzzle5dPlaySnapshot{'); blocks.append(b4)
t,b5=cut(t,'impl pack::snapshot::ArtifactSqliteSnapshot for Puzzle5dPlaySnapshot {'); blocks.append(b5)
save(sp,t)
out='''//! 🌉️ Puzzle 5d play snapshot codecs — the pack, DSL text, store and SQLite snapshot capabilities of `Puzzle5dPlaySnapshot`, each delegating to the typed `Puzzle5dSnapshot` authority.

use super::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::io::sqlite::snapshot::Puzzle5dSnapshot;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_value::{ValueError};
use store::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding}};

'''+'\n'.join(blocks)
os.makedirs(f'{S}/✏️editor/📸️snapshot/🌉️play-codecs',exist_ok=True)
save(f'{S}/✏️editor/📸️snapshot/🌉️play-codecs/🦀️.rs',out)
print(len(blocks))
