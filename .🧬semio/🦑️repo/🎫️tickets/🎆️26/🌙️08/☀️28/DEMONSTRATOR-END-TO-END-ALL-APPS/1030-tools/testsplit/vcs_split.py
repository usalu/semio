import os,re
R="/Users/ueli/Documents/semio"
V=R+"/🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs"
base=V+"/🧪️tests/🥣️companion"
pk=base+"/📦️packages/🦀️rust"
vm=V+"/📦️packages/🦀️rust"
assert not os.path.exists(base)
os.makedirs(pk)
COMMON='''pub use semio_framework_os_vcs::*;
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
pub use std::sync::Arc;
'''
def up(depth):return "../"*depth
def write(rel,text):
    p=base+"/"+rel;os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"w").write(text)
def mount(depth,vrel,name,pub=False):
    return '#[path = "%s%s"]\n%smod %s;\n'%(up(depth),vrel,"pub " if pub else "",name)
VP="semio_framework_os_vcs::"
# root
root='''//! 🥣️ Companion tests of the version-graph algebra: history ledger, group visibility, read custody, checkpoint and entity identities, run against the public surface of `semio-framework-os-vcs`.
//!
//! 🧭️ The test sources stay beside their owners and are mounted here with `#[path]` below wrappers that restate the imports of the owning module; every fixture is read at run time from the vcs folder.
#![cfg(test)]

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_replication as protocol;
extern crate semio_framework_pack as pack;

pub use semio_framework_os_vcs::*;
pub use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
pub(crate) use semio_framework_trace_memory_testing as test_allocation;

#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

'''
root+=mount(2,"🧪️tests/🔬️unit/🦀️.rs","tests")
root+=mount(2,"🧪️tests/🧪️fault-params/🦀️.rs","fault_params_tests")
for n,f in [("history_ledger","🗃️history-ledger"),("group_visibility","🪟️group-visibility"),("entity_identity","🚪️io-entity"),("checkpoint_identity","🚪️io-checkpoint")]:
    pass
root+='#[path = "🗃️history-ledger/🦀️.rs"]\nmod history_ledger;\n'
root+='#[path = "🪟️group-visibility/🦀️.rs"]\nmod group_visibility;\n'
root+='#[path = "🪪️entity-identity/🦀️.rs"]\nmod entity_identity;\n'
root+='#[path = "🪪️checkpoint-identity/🦀️.rs"]\nmod checkpoint_identity;\n'
write("🦀️.rs",root)
# history ledger wrapper (depth 3 -> companion/X/🦀️.rs ups=3)
write("🗃️history-ledger/🦀️.rs",'//! 🗃️ History ledger replacement and read laws.\n'+COMMON+mount(3,"🗃️history-ledger/replacements-tests.rs","replacements_tests")+'#[path = "🔗️read/🦀️.rs"]\npub mod history_read;\n')
write("🗃️history-ledger/🔗️read/🦀️.rs",'//! 🔗️ Read custody laws of the history ledger.\n'+COMMON+'pub use super::*;\n'+mount(4,"🔗️read/🧪️tests/🦀️.rs","tests"))
write("🪟️group-visibility/🦀️.rs",'//! 🪟️ Group history visibility laws.\n'+COMMON+mount(3,"🧪️tests/🔬️group-history-visibility/🦀️.rs","group_history_visibility_tests"))
write("🪪️entity-identity/🦀️.rs",'//! 🪪️ Entity identity laws.\n'+COMMON+'pub use semio_framework_os_vcs::io::binary::entity_identity::*;\n#[path = "🎛️control/🦀️.rs"]\npub mod control;\n#[path = "🎟️original/🦀️.rs"]\npub mod original;\n')
write("🪪️entity-identity/🎛️control/🦀️.rs",'//! 🎛️ Controlled entity identity receiving laws.\n'+COMMON+'pub use semio_framework_os_vcs::io::binary::entity_identity::control::*;\n'+mount(4,"🚪️io/💾️binary/🪪️entity-identity/🎛️control/🧪️tests/🦀️.rs","receiving_input_tests"))
write("🪪️entity-identity/🎟️original/🦀️.rs",'//! 🎟️ Original entity identity laws.\n'+COMMON+'pub use semio_framework_os_vcs::io::binary::entity_identity::original::*;\n'+mount(4,"🚪️io/💾️binary/🪪️entity-identity/🎟️original/tests.rs","tests"))
CI="🚪️io/💾️binary/🪪️checkpoint-identity/"
sub=[("commit-facts","commit_facts","commit-facts-tests.rs"),("existing","existing","existing-tests.rs"),("pin-order","pin_order","pin-order-tests.rs"),("repin-alternative","repin_alternative","repin-alternative-tests.rs"),("repin-facts","repin_facts","repin-facts-tests.rs")]
write("🪪️checkpoint-identity/🦀️.rs",'//! 🪪️ Checkpoint identity laws.\n'+COMMON+'pub use semio_framework_os_vcs::io::binary::checkpoint_identity::*;\n'+mount(3,CI+"🧪️tests/🦀️.rs","tests")+'#[path = "🎟️original/🦀️.rs"]\npub mod original;\n')
o='//! 🎟️ Original checkpoint identity laws.\n'+COMMON+'pub use semio_framework_os_vcs::io::binary::checkpoint_identity::original::*;\npub use super::*;\n'+mount(4,CI+"🎟️original/tests.rs","tests")+mount(4,CI+"🎟️original/commit-identity-tests.rs","commit_identity_tests")
for d,m,t in sub:
    o+='#[path = "%s/🦀️.rs"]\npub mod %s;\n'%(d,m)
    write("🪪️checkpoint-identity/🎟️original/%s/🦀️.rs"%d,'//! 🧪️ Original %s laws.\n'%d+COMMON+'pub use semio_framework_os_vcs::io::binary::checkpoint_identity::original::%s::*;\npub use semio_framework_os_vcs::io::binary::checkpoint_identity::original::*;\n'%m+mount(5,CI+"🎟️original/"+t,"tests"))
write("🪪️checkpoint-identity/🎟️original/🦀️.rs",o)
# cargo
c=open(vm+"/Cargo.toml").read()
def fix(l):
    m=re.search(r'path = "([^"]+)"',l)
    if m:
        ab=os.path.normpath(os.path.join(vm,m.group(1)));l=l.replace(m.group(0),'path = "%s"'%os.path.relpath(ab,pk))
    return l
deps=[];dev=[];sec=""
for l in c.split("\n"):
    if l.startswith("["):sec=l;continue
    if not l.strip():continue
    if sec=="[dependencies]":deps.append(fix(l))
    elif sec=="[dev-dependencies]":dev.append(fix(l))
deps=[d for d in deps if not d.startswith("serde = ")]
deps.append('serde = { version = "1.0.219", features = ["derive"] }')
deps.append('semio-framework-os-vcs = { path = "%s", features = ["testkit"] }'%os.path.relpath(vm,pk))
deps.append('semio-framework-os-kernel = { path = "%s" }'%os.path.relpath(R+"/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",pk))
dev=[d for d in dev if not d.startswith("serde = ")]
open(pk+"/Cargo.toml","w").write('''[package]
workspace = "%s"
name = "semio-framework-os-vcs-tests"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Companion tests of the version-graph algebra: history ledger, group visibility, read custody, checkpoint and entity identities against the public surface"
publish = false

[package.metadata.semio]
role = "framework"
id = "os-vcs-tests"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
%s

[dev-dependencies]
%s
'''%(os.path.relpath(R,pk),"\n".join(deps),"\n".join(dev)))
open(pk+"/📋️project.json","w").write('{\n  "name": "@semio-tech/os-vcs-tests-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n')
print("ok",os.path.relpath(vm,pk))
