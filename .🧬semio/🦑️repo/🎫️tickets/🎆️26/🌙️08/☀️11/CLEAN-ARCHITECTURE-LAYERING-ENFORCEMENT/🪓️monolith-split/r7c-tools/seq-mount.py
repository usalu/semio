# 🪝️ Re-mounts the sequence editor layers: the artifact nest mounts base/host/retained/persistent/import/app, the editor root keeps no item, the subset root names the app layer. Run after seq-lower-root.ts. usage: python3 seq-mount.py
R='/Users/ueli/Documents/semio/'
E=R+'✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
t=open(E,encoding='utf8').read()
if 'use semio_framework_job_publication' in t:
    i=t.index('use semio_framework_job_publication')
    j=t.index('//#region 🧪️UnitTests')
    head=t[:i]
    tail=t[j:]
    new_head='''//! 🖥️ Sequence play app — the editor module root. It owns no item: the artifact nest mounts the layers
//! `🧱️base` (ids, typed media I/O, faults, camera), `🖥️host` (the UI-editing engine), the leaf nodes
//! (`🎮️commands/*`, `🎭️modes/✏️edit/🪟️windows/*`, `📌️panels/*`, `📐️edit-rules`, `🗣️terminology`) and, on top
//! of them, `🧵️retained` (the aggregated command enum), `🔁️persistent`, `🎞️import` and `🎹️app` (the
//! `ArtifactEditor` impl and the manifest stitch). Every layer names the layer it uses by its own module
//! path, so a lower layer never reaches a higher one through this root.

'''
    open(E,'w',encoding='utf8').write(new_head+tail)
N=R+'✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs'
n=open(N,encoding='utf8').read()
anchor='''        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
'''
assert n.count(anchor)==1
if 'pub mod base;' not in n:
    base='🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
    mods=[('🧱️base','base'),('🖥️host','host'),('🧵️retained','retained'),('🔁️persistent','persistent'),('🎞️import','import'),('🎹️app','app')]
    add=''.join(f'        #[path = "{base}{d}/🦀️.rs"]\n        pub mod {m};\n' for d,m in mods)
    n=n.replace(anchor,anchor+add)
n=n.replace('editor::sequence::SequencePlayApp','editor::sequence::app::SequencePlayApp')
old_c='''        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
'''
new_c='''        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        #[cfg(test)]
        pub use component::*;
'''
if old_c in n: n=n.replace(old_c,new_c)
open(N,'w',encoding='utf8').write(n)
S=R+'✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs'
s=open(S,encoding='utf8').read()
s=s.replace('editor_surface::<editor::SequencePlayApp, A>(editor::create_sequence_app())','editor_surface::<editor::app::SequencePlayApp, A>(editor::app::create_sequence_app())')
open(S,'w',encoding='utf8').write(s)
print('mounted')

import os
ART=R+'✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/'
nest=open(N,encoding='utf8').read()
comp_mount='''        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        #[cfg(test)]
        pub use component::*;
'''
if comp_mount in nest:
    nest=nest.replace(comp_mount,'')
    nest=nest.replace('''#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sequence {
''','''#[path = "."]
pub mod editor {
    /// 🖥️ Sequence play app: the artifact nest mounts the layers `🧱️base` (ids, typed media I/O, faults, camera),
    /// `🖥️host` (the UI-editing engine), the leaf nodes (`🎮️commands/*`, `🎭️modes/✏️edit/🪟️windows/*`, `📌️panels/*`,
    /// `📐️edit-rules`, `🗣️terminology`) and, on top of them, `🧵️retained` (the aggregated command enum),
    /// `🔁️persistent`, `🎞️import` and `🎹️app` (the `ArtifactEditor` impl and the manifest stitch). Every layer names the
    /// layer it uses by its own module path.
    #[path = "."]
    pub mod sequence {
''')
    assert 'Sequence play app: the artifact nest' in nest
if '#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]' not in nest:
    t_anchor='''#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
'''
    assert nest.count(t_anchor)==1
    nest=nest.replace(t_anchor,t_anchor+'''/// 🧪️ The mounted-app harness (`context`) and the app-level laws of the sequence editor: they drive the whole app, so they
/// sit above every layer and the leaf tests of commands, windows and panels reach the harness by this path.
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
pub mod editor_unit_tests;
''')
open(N,'w',encoding='utf8').write(nest)
comp=R+'✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
if os.path.exists(comp): os.remove(comp)
cnt=0
for base in [ART, R+'✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🎬️sequence-runtime/']:
    for dp,dn,fn in os.walk(base):
        for f in fn:
            if f.endswith('.rs'):
                p=os.path.join(dp,f)
                s=open(p,encoding='utf8').read()
                if 'crate::editor::sequence::unit_tests::' in s:
                    open(p,'w',encoding='utf8').write(s.replace('crate::editor::sequence::unit_tests::','crate::editor_unit_tests::'))
                    cnt+=1
print('unit_tests repointed in',cnt,'files')
