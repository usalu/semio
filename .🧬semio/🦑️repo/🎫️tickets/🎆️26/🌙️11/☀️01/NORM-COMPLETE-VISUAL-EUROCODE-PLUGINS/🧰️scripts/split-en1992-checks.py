import os,re,sys
sys.path.insert(0,os.path.dirname(__file__))
from crate_split import *
A='/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992'
C=A+'/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks'
base=C+'/📦️packages/🦀️rust/Cargo.toml'
root=open(C+'/🦀️.rs').read()
pre=root[root.index('#![allow'):root.index('//#region 🔖️Catalogue')]
pre=pre.replace('#![allow(async_fn_in_trait)]','#![allow(async_fn_in_trait)]\n#![allow(unused_imports)]')
cat=open(C+'/📇️catalogue/🦀️.rs').read()
if 'pub fn registry' in cat:
    i=cat.index('/// 🗄 Every catalogue entry');j=cat.index('/// 🔤 Compact numeric')
    reg=cat[i:j];cat=cat[:i]+cat[j:]
    cat=cat.replace('pub(super) fn f(','pub fn f(')
    os.makedirs(C+'/🗄️registry',exist_ok=True)
    open(C+'/🗄️registry/🦀️.rs','w').write('//! 🗄️ Registry of the EN 1992 verification catalogue: every entry id mapped to its evaluator and the conversion of outcomes into check results. See [`super`].\n\nuse super::*;\n\n'+reg.rstrip()+'\n')
    open(C+'/📇️catalogue/🦀️.rs','w').write(cat)
groups={
 'core':('🧬️core','semio-s-artifact-norm-en1992-checks-core','EN 1992 checks catalogue types, NDP tables, materials and design effects',[],'Foundation: the catalogue case and outcome types, NDP tables, material properties and design-effect combinations.',['📜️ndp-tables','📇️catalogue','🧱️materials-and-time','🧮️design-effects']),
 'resistances':('🏗️resistances','semio-s-artifact-norm-en1992-checks-resistances','EN 1992 section, shear, punching, detailing, fastening and serviceability checks',['core'],'Resistances: section, shear and torsion, punching, detailing, fastenings and serviceability catalogue entries.',['📐️section-resistance','✂️shear-torsion','🕳️punching-and-struts','🔗️detailing','🪝️fastenings','🪟️serviceability']),
 'specials':('🌉️specials','semio-s-artifact-norm-en1992-checks-specials','EN 1992 prestress, fire, bridge and analysis-model checks',['resistances'],'Specials: prestress and fatigue, fire design, bridges and liquid-retaining structures and analysis models.',['🧵️prestress-and-fatigue','🔥️fire-design','🌉️bridges-and-liquid','🏛️analysis-models']),
}
names={k:v[1] for k,v in groups.items()}
def modname(d):return re.sub(r'^[^a-z]*','',d).replace('-','_')
for k,(d,n,desc,below,doc,mods) in groups.items():
    cd=C+'/'+d
    body=f'//! {d[:d.index("️")+1] if False else {"core":"🧬️","resistances":"🏗️","specials":"🌉️"}[k]} EN 1992 checks {k}. {doc}\n\n'+pre
    if below:
        ext=names[below[0]].replace('-','_')
        body+=f'pub use {ext}::*;\n\n'
    for m in mods:
        mn=modname(m)
        body+=f'#[path = "../{m}/🦀️.rs"]\npub mod {mn};\npub use {mn}::*;\n\n'
    body=body.rstrip()+'\n'
    pkg=mk_crate(A,cd,n,desc,body,base,extra_deps=[names[b] for b in below],dev=False)
    register(A,pkg,n)
    for m in mods:
        f=C+'/'+m+'/🦀️.rs';s=open(f).read()
        s=s.replace('pub(super) fn','pub fn')
        s=re.sub(r'^struct ','pub struct ',s,flags=re.M)
        open(f,'w').write(s)
newroot=root[:root.index('//#region 🔖️Catalogue')]
newroot=newroot.replace('//! ⚖️ EN 1992 compliance checks: the clause-by-clause verification behind the compliance report.','//! ⚖️ EN 1992 compliance checks: the clause-by-clause verification behind the compliance report.')
newroot+='pub use semio_s_artifact_norm_en1992_checks_specials::*;\n\n#[path = "🗄️registry/🦀️.rs"]\npub mod catalogue_registry;\npub use catalogue_registry::*;\n\n'+root[root.index('//#region 🔖️ComplianceHelpers'):]
open(C+'/🦀️.rs','w').write(newroot)
add_dep(base,names['specials'])
