"""🧩️ Splits an EN 199x norm editor into family (identity, compliance family, incremental units), surfaces (modes, panels) and the app wiring root."""
import re,sys
sys.path.insert(0,'.')
from register import *
def split(n):
    f=f"en{n}"; C=f"En{n}"
    root=ART/FAM[f]; ed=root/SUB/"✏️editor"
    rs=ed/"🦀️.rs"; s=rs.read_text(encoding="utf8")
    lines=s.split("\n")
    def region(name):
        a=next(i for i,l in enumerate(lines) if l.startswith("//#region") and name in l)
        b=next(i for i,l in enumerate(lines) if i>a and l.startswith("//#endregion") and name in l)
        return a,b
    ca,cb=region("Constants")
    fa,fb=region("ComplianceFamily")
    consts="\n".join(lines[ca+1:cb])
    fam="\n".join(lines[fa+1:fb])
    fam_crate=f"semio-s-artifact-norm-en{n}-editor-family"; fam_mod=f"semio_s_artifact_norm_en{n}_editor_family"
    sur_crate=f"semio-s-artifact-norm-en{n}-editor-surfaces"; sur_mod=f"semio_s_artifact_norm_en{n}_editor_surfaces"
    fm_mod=f"semio_s_artifact_norm_en{n}_editor_field_meta"
    names=["LABEL","VARIANT","CONTROLLER_ID","DOCUMENT_SCHEMA","ENTITY_KIND_SCENE_KINDS",f"{C}Family","Host"]
    for nm in names[:5]: assert f"pub const {nm}" in consts, nm
    assert f"pub struct {C}Family" in fam and "pub type Host" in fam
    famdir=ed/"🧩️family"; surdir=ed/"🪟️surfaces"
    for d in (famdir,surdir): (d/"📦️packages/🦀️rust").mkdir(parents=True,exist_ok=True)
    famsrc=f'''//! 🧩️ {C} editor family: the identity constants of the editor, its headless `NormFamily` binding and the incremental check units; the surfaces and the app wiring depend on it.

use semio_s_plugin_norm_compliance::NormHost;
use semio_s_artifact_norm_en{n}_model::{C}Snapshot;
use semio_s_artifact_norm_en{n}_mutations::{C}Mutation;

//#region 🔖️Constants
{consts}
//#endregion 🔖️Constants

//#region 🧩️ComplianceFamily
{fam}
//#endregion 🧩️ComplianceFamily

//#region 🪢️Mounts
#[path = "../🧮️incremental/🦀️.rs"]
pub mod incremental;
//#endregion 🪢️Mounts
'''
    (famdir/"🦀️.rs").write_text(famsrc,encoding="utf8")
    srcs="\n".join(l for l in lines)
    new=lines[:ca+1]+[f"pub use {fam_mod}::{{{', '.join(names)}}};"]+lines[cb:fa+1]+lines[fb:]
    t="\n".join(new)
    t=t.replace("use semio_s_plugin_norm_compliance::NormHost;\n","")
    old_inc='#[path = "🧮️incremental/🦀️.rs"]\npub mod incremental;\n'
    assert old_inc in t
    t=t.replace(old_inc,f"pub use {fam_mod}::incremental;\n")
    m1=re.search(r'#\[path = "\."\]\npub mod modes \{.*?\n\}\n#\[path = "\."\]\npub mod panels \{.*?\n\}\n',t,re.S)
    assert m1
    block=m1.group(0)
    t=t.replace(block,f"pub use {sur_mod}::{{modes, panels}};\n")
    t=t.replace(f"pub extern crate {fm_mod} as field_meta;\n",f"pub use {sur_mod}::field_meta;\n") if False else t
    rs.write_text(t,encoding="utf8")
    modes_panels=block.replace('#[path = "."]','#[path = ".."]')
    sursrc=f'''//! 🪟️ {C} editor surfaces: the edit mode with its windows and the panels, mounted from their taxonomy nodes.

pub use {fam_mod}::{{{', '.join(names)}}};
pub extern crate {fm_mod} as field_meta;

//#region 🪢️Mounts
{modes_panels}//#endregion 🪢️Mounts
'''
    (surdir/"🦀️.rs").write_text(sursrc,encoding="utf8")
    return fam_crate,sur_crate,ed,n
if __name__=="__main__":
    for n in sys.argv[1:]:
        print(split(n))
