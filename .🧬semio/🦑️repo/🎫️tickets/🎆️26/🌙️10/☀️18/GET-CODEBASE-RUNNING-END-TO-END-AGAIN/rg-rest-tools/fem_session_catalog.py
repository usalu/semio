import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from lib import *
P2 = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d"
SD = P2 + "/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session"
CAT = SD + "/🪪️session-constants-and-owner-catalog"
cf = CAT + "/🦀️.rs"
c = rd(cf)
assert "use super::*;\n" in c
c = c.replace("use super::*;\n", "use semio_s_fem_analyses_model::MountedAnalysisSupport;\n", 1)
c = c.replace("pub(super)", "pub")
c = c.replace("\nstruct MountedOwnerClaim {", "\npub struct MountedOwnerClaim {", 1)
mb = rd(SD + "/🏗️model-build-job/🦀️.rs")
defs = '''pub(super) const SESSION_MAXIMUM_ITEMS: usize = SESSION_MAXIMUM_INPUT_ITEMS + MountedProcessOwnerCatalog::fixed().items() + MountedProcessOwnerCatalog::fixed().roots();

pub(super) const SESSION_MAXIMUM_BYTES: usize = SESSION_MAXIMUM_INPUT_BYTES + MountedProcessOwnerCatalog::fixed().roots() * SESSION_OWNER_PAGE_BYTES;

'''
assert defs in mb
pub_defs = defs.replace("pub(super)", "pub")
c = c.replace("//#region 🔖️Contract\n", "//#region 🔖️Contract\n" + pub_defs, 1) if False else c
# place after SESSION_OWNER_PAGE_BYTES definition
anchor = "pub const SESSION_OWNER_PAGE_BYTES: usize = 16_384;\n\n"
assert anchor in c
c = c.replace(anchor, anchor, 1)
# defs need MountedProcessOwnerCatalog (defined later in the file) - consts may reference items defined later in the module
c = c.rstrip("\n").replace("//#endregion 🔖️Contract", pub_defs.rstrip("\n") + "\n//#endregion 🔖️Contract", 1) + "\n"
wr(cf, c)
mb2 = mb.replace(defs, "", 1)
wr(SD + "/🏗️model-build-job/🦀️.rs", mb2)
NEW = "semio-s-artifact-fem-2d-editor-session-catalog"
npkg = register_crate(P2 + "/Cargo.toml", "semio-s-artifact-fem-2d-editor-session", SD + "/📦️packages/🦀️rust", NEW, CAT, "Fem 2d editor session capacities and process owner catalog", "../../🦀️.rs")
root = rd(SD + "/🦀️.rs")
old_a = '''#[path = "🪪️session-constants-and-owner-catalog/🦀️.rs"]
mod session_constants_and_owner_catalog;
pub use session_constants_and_owner_catalog::{ConsumerKey, FEM2D_MOUNTED_JOB_KIND};
use session_constants_and_owner_catalog::{
'''
assert old_a in root
root = root.replace(old_a, '''pub use semio_s_artifact_fem_2d_editor_session_catalog::{ConsumerKey, FEM2D_MOUNTED_JOB_KIND};
use semio_s_artifact_fem_2d_editor_session_catalog::{
    SESSION_MAXIMUM_BYTES, SESSION_MAXIMUM_ITEMS,
''', 1)
root = root.replace("#[cfg(test)]\nuse session_constants_and_owner_catalog::MountedOwnerClass;\n", "#[cfg(test)]\nuse semio_s_artifact_fem_2d_editor_session_catalog::MountedOwnerClass;\n", 1)
old_b = '''use model_build_job::{
    bounded_string_capacities, ModelBuildStage, MountedIdentity, MountedModelBuild, MountedStage,
    SESSION_MAXIMUM_BYTES, SESSION_MAXIMUM_ITEMS,
};'''
assert old_b in root
root = root.replace(old_b, '''use model_build_job::{
    bounded_string_capacities, ModelBuildStage, MountedIdentity, MountedModelBuild, MountedStage,
};''', 1)
wr(SD + "/🦀️.rs", root)
# tests
tf = SD + "/🧪️tests/🔬️unit/🦀️.rs"
t = rd(tf)
t = t.replace('        include_str!("../../🪪️session-constants-and-owner-catalog/🦀️.rs"),\n', "", 1)
assert '        "MountedOwnerClass::AssemblyDofStrings",\n' in t
t = t.replace('        "MountedOwnerClass::AssemblyDofStrings",\n', "", 1)
wr(tf, t)
mf = SD + "/📦️packages/🦀️rust/Cargo.toml"
add_dep_line(mf, "dependencies", NEW, '%s = { path = "%s" }' % (NEW, os.path.relpath(os.path.join(ROOT, npkg), os.path.join(ROOT, os.path.dirname(mf)))))
print(run_owners(P2, True))
