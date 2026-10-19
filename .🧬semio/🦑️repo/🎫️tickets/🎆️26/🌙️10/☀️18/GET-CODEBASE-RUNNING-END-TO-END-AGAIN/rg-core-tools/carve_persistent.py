import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from valuecarve import *
N = V + "/📋️list/🔗️persistent/📦️packages/🦀️rust"
os.makedirs(N, exist_ok=True)
rgl.write(N + "/Cargo.toml", manifest("semio-framework-value-persistent", "semio_framework_value_persistent", os.path.relpath(R+"/🧰️framework", N),
 '''semio-framework-value-control-contract = { path = "../../../../🎟️control/🧬️contract/📦️packages/🦀️rust" }
semio-framework-value-derive = { path = "../../../../✨️derive/📦️packages/🦀️rust" }
semio-framework-value-intrinsic = { path = "../../../../🌳️intrinsic/📦️packages/🦀️rust" }
semio-framework-value-native-encoding = { path = "../../../../🛫️encode/📦️packages/🦀️rust" }
semio-framework-value-original-aliases = { path = "../../../../♻️retirement/📦️aliases/📦️packages/🦀️rust" }
semio-framework-value-paged-list = { path = "../../../📦️packages/🦀️rust" }
semio-framework-value-retained-clone = { path = "../../../../🧬️retained-clone/🧬️contract/📦️packages/🦀️rust" }
semio-framework-value-retirement = { path = "../../../../♻️retirement/📦️packages/🦀️rust" }
semio-framework-value-shared-issuance = { path = "../../../../♻️retirement/🔗️arc-custody/🔒️sealed/📦️packages/🦀️rust" }''',
 '''semio-framework-test-fixtures = { path = "../../../../../🧪️test/🧲️fixture-reader/📦️packages/🦀️rust" }
semio-framework-value-testing-allocator = { path = "../../../../🧪️testing/💾️allocator/📦️packages/🦀️rust" }
serde_json = { workspace = true }''',
 "Persistent paged lists with their splice, build, traversal and read cursors, original vector concatenation and structural retained equality.", "value-persistent"))
rgl.write(N + "/📋️project.json", rgl.project_json("@semio-tech/value-persistent-rs"))
rgl.write(N + "/🦀️.rs", '''//! 🔗️ Persistent paged lists and structural retained equality — wiring only. Domain lives at the owner `🦀️.rs` files.

use semio_framework_value_control_contract::{ValueError, ValueRefusalKind};
use semio_framework_value_derive::{RetainedEquality, RetireOwned, ToValue};
use semio_framework_value_intrinsic::{DslValue, FromValue, ValueShape};
use semio_framework_value_retained_clone::retained_clone;
use semio_framework_value_retirement::{retirement, RetirementDemand};
use semio_framework_value_shared_issuance as shared_issuance;

#[path = "../../../🌳️roots/🦀️.rs"]
pub mod list;
#[path = "../../../🪢️concat/🦀️.rs"]
pub mod original_vector_concat;
pub use original_vector_concat::OriginalVectorConcat;
#[path = "../../../🟰️equal/🦀️.rs"]
pub mod slice_equality;
pub use slice_equality::{RetainedEqualityElement, RetainedSliceEquality};
#[path = "../../../../🌳️tree/🟰️equal/🦀️.rs"]
pub mod retained_equality;
pub use retained_equality::{RetainedEquality, RetainedEqualityCursor, RetainedEqualityFields, RetainedEqualityOwner, RetainedEqualityStep, RetainedRecordEquality};

#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
pub(crate) mod value {
    pub(crate) use super::observe_retirement_allocations;
}
''')
drop_mount(ROOTF, "📋️list/🌳️roots/🦀️.rs", "list")
s = rgl.read(ROOTF)
old = '#[path="🌳️tree/🟰️equal/🦀️.rs"] pub mod retained_equality;\n'
assert old in s
s = s.replace(old, "pub use semio_framework_value_persistent::{list, retained_equality};\n")
rgl.write(ROOTF, s)
drop_mount(GLUE, "../../📋️list/🪢️concat/🦀️.rs", "original_vector_concat")
swap(GLUE, "pub use original_vector_concat::OriginalVectorConcat;", "pub use semio_framework_value_persistent::original_vector_concat;\npub use original_vector_concat::OriginalVectorConcat;")
drop_mount(GLUE, "../../📋️list/🟰️equal/🦀️.rs", "slice_equality")
swap(GLUE, "pub use slice_equality::{RetainedEqualityElement,RetainedSliceEquality};", "pub use semio_framework_value_persistent::slice_equality;\npub use slice_equality::{RetainedEqualityElement,RetainedSliceEquality};")
fm = V + "/📦️packages/🦀️rust/Cargo.toml"
swap(fm, 'semio-framework-value-registry = {', 'semio-framework-value-persistent = { path = "../../📋️list/🔗️persistent/📦️packages/🦀️rust" }\nsemio-framework-value-registry = {')
rgl.register(V + "/📦️packages/🦀️rust", "semio-framework-value", N, "semio-framework-value-persistent", "@semio-tech/value-rs", "@semio-tech/value-persistent-rs", "🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/📦️packages/🦀️rust", dep_ws=[])
