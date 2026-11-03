import os,re,sys
sys.path.insert(0,os.path.dirname(__file__))
from crate_split import *
A='/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997'
C=A+'/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks'
base=C+'/📦️packages/🦀️rust/Cargo.toml'
root=open(C+'/🦀️.rs').read()
pre=root[root.index('#![allow'):root.index('//#region 🔖️ComplianceHelpers')]
pre=pre.replace('#![allow(async_fn_in_trait)]','#![allow(async_fn_in_trait)]\n#![allow(unused_imports)]')
pre+=root[root.index('use semio_s_plugin_norm_compliance::{CheckReport'):root.index('#[path = "📇️catalogue')]
n='semio-s-artifact-norm-en1997-checks-catalogue'
body='//! 📇️ EN 1997 checks catalogue: the geotechnical verification catalogue and the design-approach parameters behind it.\n\n'+pre.rstrip()+'\n\n#[path = "../🧭️design-approach-and-parameters/🦀️.rs"]\nmod design_approach_and_parameters;\npub use design_approach_and_parameters::*;\n\n#[path = "../📇️catalogue/🦀️.rs"]\npub mod catalogue;\n'
pkg=mk_crate(A,C+'/🧬️catalogue-core',n,'EN 1997 geotechnical verification catalogue and design-approach parameters',body,base,dev=False,drop_deps=('semio-s-plugin-norm-app-surface',))
register(A,pkg,n)
f=C+'/📇️catalogue/🦀️.rs';s=open(f).read()
s=s.replace('pub(super) fn','pub fn');open(f,'w').write(s)
old='#[path = "📇️catalogue/🦀️.rs"]\npub mod catalogue;\n\n#[path = "🧭️design-approach-and-parameters/🦀️.rs"]\nmod design_approach_and_parameters;\npub use design_approach_and_parameters::*;\n'
assert old in root
root=root.replace(old,'pub use semio_s_artifact_norm_en1997_checks_catalogue::*;\n')
open(C+'/🦀️.rs','w').write(root)
add_dep(base,n)
