import os,re,sys
sys.path.insert(0,os.path.dirname(__file__))
from crate_split import *
A='/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993'
C=A+'/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks'
base=C+'/📦️packages/🦀️rust/Cargo.toml'
root=open(C+'/🦀️.rs').read()
cut=root.index('#[path = "🧰️structure-helpers/🦀️.rs"]')
head=root[:cut]
core_head=head.replace('//! ⚖️ EN 1993 compliance checks: the clause-by-clause verification behind the compliance report.','//! 🧬️ EN 1993 checks core: shared compliance helpers, scope and structure helpers.').replace('#![allow(async_fn_in_trait)]','#![allow(async_fn_in_trait)]\n#![allow(unused_imports)]')
core_head=re.sub(r'^fn ','pub fn ',core_head,flags=re.M)
core=core_head+'#[path = "../🧰️structure-helpers/🦀️.rs"]\npub mod structure_helpers;\npub use structure_helpers::*;\n'
nc='semio-s-artifact-norm-en1993-checks-core';ns='semio-s-artifact-norm-en1993-checks-special'
pkg=mk_crate(A,C+'/🧬️core',nc,'EN 1993 checks shared helpers and scope',core,base,dev=False)
register(A,pkg,nc)
lines=head.split('\n')
externs=[l for l in lines if l.startswith('extern crate')]
uses=[l for l in lines if l.startswith('use ')]
def mkroot(title,body):
    return f"//! {title}\n\n#![allow(async_fn_in_trait)]\n#![allow(unused_imports)]\n\n"+'\n'.join(externs)+'\n\n'+'\n'.join(uses)+'\n\n'+body
rest=root[cut:]
sp_mount='#[path = "🔗️special-element-checks/🦀️.rs"]\nmod special_element_checks;\nuse special_element_checks::*;\n'
sh_mount='#[path = "🧰️structure-helpers/🦀️.rs"]\nmod structure_helpers;\nuse structure_helpers::*;\n\n'
assert sp_mount in rest and sh_mount in rest
special=mkroot('🔗️ EN 1993 checks special elements: joints, fatigue, fire, cold-formed, plated, shell, tension and special-part checks.',f'pub use {nc.replace("-","_")}::*;\n\n#[path = "../🔗️special-element-checks/🦀️.rs"]\npub mod special_element_checks;\npub use special_element_checks::*;\n')
pkg=mk_crate(A,C+'/🔗️special',ns,'EN 1993 checks for special elements',special,base,extra_deps=[nc],dev=False)
register(A,pkg,ns)
for f in [C+'/🧰️structure-helpers/🦀️.rs',C+'/🔗️special-element-checks/🦀️.rs']:
    t=open(f).read().replace('pub(super) fn','pub fn');open(f,'w').write(t)
rest=rest.replace(sp_mount,'').replace(sh_mount,'')
top=root[:root.index('//#region 🔖️ComplianceHelpers')]
top=top[:top.index('use semio_s_plugin_norm_compliance::AnnexChoice;')]
top+='\n'.join(uses)+f'\n\npub use {ns.replace("-","_")}::*;\n\n'+rest
open(C+'/🦀️.rs','w').write(top)
add_dep(base,ns)
