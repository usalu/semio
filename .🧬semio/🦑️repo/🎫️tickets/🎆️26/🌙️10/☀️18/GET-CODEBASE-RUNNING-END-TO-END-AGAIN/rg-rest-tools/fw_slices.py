import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from lib import *
WS = "Cargo.toml"
ROOTP = "🧰️framework/🛍️products/💻️os"
# ---------------- workflow-run admission (consumer up)
R = ROOTP + "/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run"
RPKG = R + "/📦️packages/🦀️rust"
RUN = "semio-framework-artifact-workflow-run"
ADM = "semio-framework-artifact-workflow-run-admission"
apply_f = R + "/🧬️schema/🧬️mutations/⚡️apply/🦀️.rs"
ap = rd(apply_f)
root = rd(R + "/🦀️.rs")
block = '''#[path = "🧬️schema/🧬️mutations/⚡️apply/🦀️.rs"]
mod apply;
pub use apply::admit_run_operation;
'''
assert block in root
fn_start = root.index("/// 🔒️ The one real write seam for a `RunArtifact`")
fn_end = root.index("#[path = \"🚪️io/🦀️.rs\"]")
checked = root[fn_start:fn_end].rstrip("\n") + "\n"
root2 = root.replace(block, "", 1).replace(root[fn_start:fn_end], "", 1)
wr(R + "/🦀️.rs", root2)
body = ap.replace("use crate::{RunArtifact, RunDiff, RunMutation};\n", "use semio_framework_artifact_workflow_run::{RunArtifact, RunDiff, RunMutation};\n", 1)
body = body.replace("//! ⚡️ The checked run admission seam: a run mutation's own diff is built and its refusals are read here; the diff is applied by\n//! [`crate::apply_run_operation_checked`] through the central applier.\n", "//! ⚡️ The checked run admission seam above the run mutation vocabulary: a run mutation's own diff is built and its refusals are read here; the diff is applied by\n//! [`apply_run_operation_checked`] through the central applier.\n\nextern crate semio_framework_os_kernel as protocol;\n\n", 1)
body = body.rstrip("\n") + "\n\n" + checked
ADMDIR = R + "/⚡️admission"
wr(ADMDIR + "/🦀️.rs", body)
os.remove(ROOT + "/" + apply_f)
try:
    os.rmdir(os.path.dirname(ROOT + "/" + apply_f))
except OSError:
    pass
admpkg = register_crate(WS, RUN, RPKG, ADM, ADMDIR, "Workflow run checked admission seam", "../../🦀️.rs")
t = rd(admpkg + "/Cargo.toml")
t = re.sub(r'(?ms)^\[dev-dependencies\]\n.*?(?=^\[|\Z)', '', t)
wr(admpkg + "/Cargo.toml", t.rstrip("\n") + "\n")
add_dep_line(admpkg + "/Cargo.toml", "dependencies", RUN, '%s = { path = "%s" }' % (RUN, os.path.relpath(os.path.join(ROOT, RPKG), os.path.join(ROOT, admpkg))))
ws_add_dep_inline(WS, ADM, admpkg, RUN) if False else None
# consumers
rc = ROOTP + "/🔨️modules/🏃️run/🧾️run-context/🦀️.rs"
r = rd(rc)
assert "semio_framework_artifact_workflow_run::apply_run_operation_checked(" in r
wr(rc, r.replace("semio_framework_artifact_workflow_run::apply_run_operation_checked(", "semio_framework_artifact_workflow_run_admission::apply_run_operation_checked(", 1))
add_dep_like(ROOTP + "/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml", RUN, ADM, admpkg)
comp = R + "/🧪️tests/🥣️companion/🦀️.rs"
c = rd(comp)
wr(comp, c.replace("pub use semio_framework_artifact_workflow_run::*;\n", "pub use semio_framework_artifact_workflow_run::*;\npub use semio_framework_artifact_workflow_run_admission::{admit_run_operation, apply_run_operation_checked};\n", 1))
add_dep_line(R + "/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml", "dependencies", ADM, '%s = { path = "%s" }' % (ADM, os.path.relpath(os.path.join(ROOT, admpkg), os.path.join(ROOT, R + "/🧪️tests/🥣️companion/📦️packages/🦀️rust"))))
print(run_owners(R, True))
# ---------------- os-config merge policy slice
O = ROOTP + "/🎚️config"
OPKG = O + "/📦️packages/🦀️rust"
OLD = "semio-framework-os-config"
MP = "semio-framework-os-config-merge-policy"
leaf = O + "/🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️.rs"
lt = rd(leaf)
lt = lt.replace("use crate::os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};\nuse super::MergePolicyConfigMutation;\nuse protocol::{Mutation, MutationDiff, ColdMutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};\n",
 "extern crate semio_framework_os_kernel as dsl;\nextern crate semio_framework_os_kernel as protocol;\n\nuse protocol::{ColdDiffAlgebra, ColdMutation, ColdMutationDiff, Mutation, MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};\n", 1)
assert "extern crate" in lt
lt = lt.replace("impl crate::os_spr::ColdMutationKind<", "impl protocol::ColdMutationKind<")
enum = '''
//#region 🔖️MergePolicy
/// 🛡️ Typed, invertible merge-policy mutation vocabulary.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = MergePolicySetting, diff = MergePolicyDiff, schema = "os.config.merge-policy", reference = MergePolicySetting, reference_diff = MergePolicyDiff)]
pub enum MergePolicyConfigMutation {
    ChangeMergePolicy(ChangeMergePolicy),
}
//#endregion 🔖️MergePolicy
'''
anchor = "//#region 🔖️Schema\n"
assert anchor in lt
lt = lt.replace(anchor, enum.lstrip("\n") + "\n" + anchor, 1)
wr(leaf, lt)
mpdir = O + "/🧬️schema/🧬️mutations/🛡️change-merge-policy"
mppkg = register_crate(WS, OLD, OPKG, MP, mpdir, "OS config merge policy setting, leaf and dispatch vocabulary", "../../🦀️.rs")
t = rd(mppkg + "/Cargo.toml")
t = re.sub(r'(?ms)^\[dev-dependencies\]\n.*?(?=^\[|\Z)', '', t)
wr(mppkg + "/Cargo.toml", t.rstrip("\n") + "\n")
# aggregator
agg = O + "/🧬️schema/🧬️mutations/🦀️.rs"
a = rd(agg)
old_enum = '''/// 🛡️ Typed, invertible merge-policy mutation vocabulary.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = MergePolicySetting, diff = MergePolicyDiff, schema = "os.config.merge-policy", reference = MergePolicySetting, reference_diff = MergePolicyDiff)]
pub enum MergePolicyConfigMutation {
    ChangeMergePolicy(ChangeMergePolicy),
}
'''
assert old_enum in a
a = a.replace(old_enum, "", 1)
a = a.replace("pub use super::change_merge_policy::{\n    change_merge_policy, MergePolicyDiff, inverse_merge_policy_config_mutation, inverse_merge_policy_config_mutation_steps, ChangeMergePolicy, MergePolicySetting, MERGE_POLICY_CONFIG_SCHEMA,\n};",
              "pub use super::change_merge_policy::{\n    change_merge_policy, register_merge_policy_config_mutation_descriptors, MergePolicyConfigMutation, MergePolicyDiff, inverse_merge_policy_config_mutation, inverse_merge_policy_config_mutation_steps, ChangeMergePolicy, MergePolicySetting, MERGE_POLICY_CONFIG_SCHEMA,\n};", 1)
assert "register_merge_policy_config_mutation_descriptors, MergePolicyConfigMutation" in a
wr(agg, a)
rootcfg = O + "/🦀️.rs"
rc2 = rd(rootcfg)
mod = '        #[path = "🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️.rs"]\n        pub mod change_merge_policy;\n'
assert mod in rc2
wr(rootcfg, rc2.replace(mod, "        pub use semio_framework_os_config_merge_policy as change_merge_policy;\n", 1))
add_dep_line(OPKG + "/Cargo.toml", "dependencies", MP, '%s = { workspace = true }' % MP)
print(run_owners(O, True))
