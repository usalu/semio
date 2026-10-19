import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from valuecarve import *
D = V + "/🛬️decode"
DG = D + "/📦️packages/🦀️rust"
N = D + "/🧳️owned/📦️packages/🦀️rust"
os.makedirs(N, exist_ok=True)
# decode root: drop owned
r = D + "/🦀️.rs"
s = rgl.read(r)
old = '#[path="🧳️owned/🦀️.rs"]\npub mod owned;\npub use owned::{NativeReceive,NativeReceiver};\n'
assert old in s
rgl.write(r, s.replace(old, ""))
# decode glue: remove intrinsic/persistent
g = DG + "/🦀️.rs"
t = rgl.read(g)
for l in ["pub use semio_framework_value_intrinsic::{DslValue, Number};\n", "pub use semio_framework_value_persistent::list;\n"]:
    assert l in t
    t = t.replace(l, "")
rgl.write(g, t)
c = rgl.read(DG + "/Cargo.toml")
for l in ['semio-framework-value-intrinsic = { path="../../../🌳️intrinsic/📦️packages/🦀️rust" }\n', 'semio-framework-value-persistent = { path="../../../📋️list/🔗️persistent/📦️packages/🦀️rust" }\n']:
    assert l in c
    c = c.replace(l, "")
rgl.write(DG + "/Cargo.toml", c)
rgl.write(N + "/Cargo.toml", manifest("semio-framework-value-native-owned", "semio_framework_value_native_owned", os.path.relpath(R+"/🧰️framework", N),
 '''semio-framework-value-control-contract = { path = "../../../../🎟️control/🧬️contract/📦️packages/🦀️rust" }
semio-framework-value-intrinsic = { path = "../../../../🌳️intrinsic/📦️packages/🦀️rust" }
semio-framework-value-native-decoding = { path = "../../../📦️packages/🦀️rust" }
semio-framework-value-persistent = { path = "../../../../📋️list/🔗️persistent/📦️packages/🦀️rust" }
semio-framework-value-retained-clone = { path = "../../../../🧬️retained-clone/🧬️contract/📦️packages/🦀️rust" }
semio-framework-value-retirement = { path = "../../../../♻️retirement/📦️packages/🦀️rust" }
semio-framework-value-shared-issuance = { path = "../../../../♻️retirement/🔗️arc-custody/🔒️sealed/📦️packages/🦀️rust" }''',
 '''semio-framework-test-fixtures = { path = "../../../../../🧪️test/🧲️fixture-reader/📦️packages/🦀️rust" }
semio-framework-value-derive = { path = "../../../../✨️derive/📦️packages/🦀️rust" }
semio-framework-value-testing-allocator = { path = "../../../../🧪️testing/💾️allocator/📦️packages/🦀️rust" }
serde_json = { workspace = true }''',
 "Native receiving of retained owners from original input: the receive and receiver traits, record, list, shared, default, field, original and value tree receivers.", "value-native-owned"))
rgl.write(N + "/📋️project.json", rgl.project_json("@semio-tech/value-native-owned-rs"))
rgl.write(N + "/🦀️.rs", '''//! 🧳️ Native receiving of retained owners — wiring only. Domain lives at the owner `🦀️.rs` files.

pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_intrinsic::{DslValue, Number};
pub use semio_framework_value_native_decoding::native_decoding::{NativeDecodeConstructionRecipient, NativeDecodeControl, NativeOwnedDecodeControl};
pub use semio_framework_value_persistent::list;
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_shared_issuance as shared_issuance;

#[path = "../../🦀️.rs"]
pub mod owned;
pub use owned::{NativeReceive, NativeReceiver};

/// 🎟️ The native decoding vocabulary composed with its receivers: the decoding core and the owned receiving tree under one module.
pub mod native_decoding {
    pub use semio_framework_value_native_decoding::native_decoding::*;
    pub use super::owned;
    pub use super::owned::{NativeReceive, NativeReceiver};
}

#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
''')
rgl.register(DG, "semio-framework-value-native-decoding", N, "semio-framework-value-native-owned", "@semio-tech/value-native-decoding-rs", "@semio-tech/value-native-owned-rs", "🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📦️packages/🦀️rust", dep_ws=[])
# facade
fm = V + "/📦️packages/🦀️rust/Cargo.toml"
t = rgl.read(fm)
t = t.replace('semio-framework-value-native-decoding = {', 'semio-framework-value-native-owned = { path = "../../🛬️decode/🧳️owned/📦️packages/🦀️rust" }\nsemio-framework-value-native-decoding = {', 1)
rgl.write(fm, t)
swap(ROOTF, "pub use semio_framework_value_native_decoding::native_decoding;", "pub use semio_framework_value_native_owned::native_decoding;")
