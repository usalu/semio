# 🧾️ Splits the curation schema root: the artifact and its descriptor stay, the domain (typology, geometry, modules, document helpers, fixtures) moves into the child `🧾️domain`, which the schema nest mounts; the text snapshot codec names the domain instead of the root. usage: python3 cur-domain.py
import os,re,shutil
R='/Users/ueli/Documents/semio/'
ART=R+'✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/'
S=ART+'🏅️standards/🔖️1/🪆️subsets/✳️any/'
ORIG=R+'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/r7c-tools/orig-cur-domain/'
def backup(p):
    b=ORIG+p[len(R):]
    if not os.path.exists(b):
        os.makedirs(os.path.dirname(b),exist_ok=True); shutil.copyfile(p,b)
root=S+'🧬️schema/🦀️.rs'
backup(root)
t=open(root,encoding='utf8').read()
a=t.index('//#region 🔖️Typology')
b=t.index('//#endregion 🏗️Construction\n')+len('//#endregion 🏗️Construction\n')
moved=t[a:b]
head_end=t.index('//#region 🔖️Artifact')
imports=t[t.index('use crate::{CurationSnapshot'):head_end]
reexp_a=t.index('//#region 🔁️Re-exports')
tests_a=t.index('//#region 🧪️Tests')
tests_b=t.index('//#endregion 🧪️Tests\n')+len('//#endregion 🧪️Tests\n')
tests=t[tests_a:tests_b]
new_root=t[:a]+t[b:tests_a]+t[tests_b:]
new_root=new_root.replace('//#region 🔁️Re-exports','//#region 🔁️Domain\n/// 🔁️ The domain module of this schema, exported at the schema path.\npub use super::domain::*;\n//#endregion 🔁️Domain\n\n//#region 🔁️Re-exports',1)
new_root=re.sub(r'\n{4,}','\n\n\n',new_root)
open(root,'w',encoding='utf8').write(new_root)
dom_dir=S+'🧬️schema/🧾️domain/'
os.makedirs(dom_dir,exist_ok=True)
tests=tests.replace('#[path = "🧪️tests/🔬️unit/🦀️.rs"]','#[path = "../🧪️tests/🔬️unit/🦀️.rs"]')
dom='//! 🧾️ Curation schema domain: typology, geometry and world3d helpers, document helpers, the authored sourcing modules and the demo fixtures every codec, window and command of the artifact builds on.\n\n'+imports+'use crate::{CuratedItem, Filters, GeometryRecipe, ObjectKind, ObjectKindExtra};\n\n'+moved.replace('pub(crate) const','pub const').replace('pub(crate) fn','pub fn')+'\n'+tests
open(dom_dir+'🦀️.rs','w',encoding='utf8').write(dom)
# nest mount
nest=ART+'🦀️.rs'
backup(nest)
n=open(nest,encoding='utf8').read()
anchor='''                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
'''
assert n.count(anchor)==1
n=n.replace(anchor,anchor+'''                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧾️domain/🦀️.rs"]
                            pub mod domain;
''')
open(nest,'w',encoding='utf8').write(n)
print('moved',moved.count('\n'),'lines')
