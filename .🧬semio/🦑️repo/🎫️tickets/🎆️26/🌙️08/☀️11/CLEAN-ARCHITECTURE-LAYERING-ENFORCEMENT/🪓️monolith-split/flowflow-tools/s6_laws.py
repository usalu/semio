"""⚖️ Step 6: laws crate roots, the copy crate's retirement-laws wrapper and the test-text edits the split forces. Work tree only."""
import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from ffed import *
def mutation_laws():
    write("🌿️vcs/⚖️laws/🦀️.rs",'''//! ⚖️ Flow mutation catalogue laws: direct-leaf contracts, structural diff ownership and typed mutation retirement, driven through the public document API.
extern crate semio_framework_os_kernel as dsl;
pub use semio_framework_artifact_flow_flow_document::*;

#[cfg(test)]
use crate::os_spr::{ApplyCapability, DiffAlgebra, Identified, MutationApplyResult, MutationDiff};

#[cfg(test)]
#[path = "../🧪️tests/🌿️vcs/🦀️.rs"]
mod flow_direct_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️leaf-contracts/🦀️.rs"]
mod leaf_contracts;

//#region 🧪️Ownership
#[cfg(test)]
#[path = "../🧬️schema/🔺️diff/🧪️tests/🧾️ownership/🦀️.rs"]
mod ownership_tests;
//#endregion 🧪️Ownership

#[cfg(test)]
#[path = "../🧬️schema/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs"]
mod flow_mutation_retirement;
''')
def snapshot_laws():
    write("🚪️io/⚖️laws/🦀️.rs",'''//! ⚖️ Flow snapshot representation laws: the relational and native owner capability, the package schema oracle and the normal Store registration, driven through the public document API.
extern crate semio_framework_os_kernel as store;
pub use semio_framework_artifact_flow_flow_document::*;

#[cfg(test)]
#[path = "../🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;

#[cfg(test)]
#[path = "../../🧪️tests/🌊️flow/🦀️.rs"]
mod package_tests;
''')
def copy_wrapper():
    write("🧵️retained/🧪️tests/🦀️.rs",'''//! 🧪️ Typed Flow retirement laws: exact owner accounting, field custody under independent grants and worker transfer, driven through the document snapshot.

use crate::retained::{FlowOwner, FlowRetirement};
use crate::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

//#region 🧪️RetirementLaws
#[path = "🧵️retained/🦀️.rs"]
mod tests;

#[path = "🧬️fields/🦀️.rs"]
pub(crate) mod field_tests;
//#endregion 🧪️RetirementLaws
''')
    t=read("🧵️retained/📑️copy/🦀️.rs")
    t=replace(t,'#[cfg(test)]\n#[path = "🧪️tests/📑️copy/🦀️.rs"]\nmod tests;','#[cfg(test)]\n#[path = "🧪️tests/📑️copy/🦀️.rs"]\nmod tests;\n\n#[cfg(test)]\n#[path = "../🧪️tests/🦀️.rs"]\nmod retained_laws;')
    write("🧵️retained/📑️copy/🦀️.rs",t)
    c="🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs"
    t=read(c)
    assert t.count("crate::retained::field_tests")==5
    t=t.replace("crate::retained::field_tests","crate::retained_laws::field_tests")
    write(c,t)
    f="🧵️retained/🧪️tests/🧬️fields/🦀️.rs"
    t=read(f)
    t=replace(t,"pub(super) fn observe","pub(crate) fn observe")
    write(f,t)
    r="🧵️retained/🧪️tests/🧵️retained/🦀️.rs"
    t=read(r)
    for v in ("oracle","value","snapshot"):
        t=replace(t,f"FlowOwner::HostSnapshot({v})",f"FlowOwner::HostSnapshot({v}.into())")
    write(r,t)
def retirement_test():
    f="🌿️vcs/🧬️schema/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs"
    t=read(f)
    t=replace(t,"let mut retirement=FlowRetirement::from_owner(FlowOwner::Mutation(mutation));","let mut retirement=ControlledRetirement::new(mutation).map_err(|(error,_)|error).unwrap();")
    t=replace(t,"use crate::retained::{FlowOwner,FlowRetirement};\n","use semio_framework_value::retirement::controlled::ControlledRetirement;\n")
    t=replace(t,"use super::super::{AddSynapse,","use crate::{AddSynapse,")
    write(f,t)
if __name__=="__main__":
    mutation_laws();snapshot_laws();copy_wrapper();retirement_test();print("ok")
