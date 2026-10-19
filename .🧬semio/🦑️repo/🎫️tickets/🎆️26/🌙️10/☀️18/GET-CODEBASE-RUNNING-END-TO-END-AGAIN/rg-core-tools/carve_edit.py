import os, sys, re
sys.path.insert(0, os.path.dirname(__file__))
from valuecarve import *
N = V + "/📋️list/🔺️edit/📦️packages/🦀️rust"
os.makedirs(N, exist_ok=True)
g = rgl.read(GLUE)
mods = [("sort", "../../📶️sort/🦀️.rs", "../../../../📶️sort/🦀️.rs"),
        ("indexed_edit", "../../📋️list/🔺️edit/🦀️.rs", "../../🦀️.rs"),
        ("indexed_collection_commit", "../../📋️list/🔺️edit/🚚️commit/🦀️.rs", "../../🚚️commit/🦀️.rs"),
        ("named_edit", "../../📋️list/🔺️edit/🔑️named/🦀️.rs", "../../🔑️named/🦀️.rs")]
body = ""
for name, old, new in mods:
    pat = re.compile(r'#\[path\s*=\s*"' + re.escape(old) + r'"\]\s*pub mod ' + name + r';\n(pub use ' + name + r'::(?:\{[^}]*\}|[A-Za-z_]+);\n)')
    m = pat.search(g)
    assert m, name
    body += f'#[path = "{new}"]\npub mod {name};\n{m.group(1)}'
    g = g[:m.start()] + f"pub use semio_framework_value_edit::{name};\n" + m.group(1) + g[m.end():]
rgl.write(GLUE, g)
rgl.write(N + "/Cargo.toml", manifest("semio-framework-value-edit", "semio_framework_value_edit", os.path.relpath(R+"/🧰️framework", N),
 '''semio-framework-value-control-contract = { path = "../../../../🎟️control/🧬️contract/📦️packages/🦀️rust" }
semio-framework-value-derive = { path = "../../../../✨️derive/📦️packages/🦀️rust" }
semio-framework-value-intrinsic = { path = "../../../../🌳️intrinsic/📦️packages/🦀️rust" }
semio-framework-value-native-decoding = { path = "../../../../🛬️decode/📦️packages/🦀️rust" }
semio-framework-value-native-encoding = { path = "../../../../🛫️encode/📦️packages/🦀️rust" }
semio-framework-value-persistent = { path = "../../🔗️persistent/📦️packages/🦀️rust" }
semio-framework-value-retained-clone = { path = "../../../../🧬️retained-clone/🧬️contract/📦️packages/🦀️rust" }
semio-framework-value-retirement = { path = "../../../../♻️retirement/📦️packages/🦀️rust" }
semio-framework-value-shared-issuance = { path = "../../../../♻️retirement/🔗️arc-custody/🔒️sealed/📦️packages/🦀️rust" }''',
 '''semio-framework-test-fixtures = { path = "../../../../../🧪️test/🧲️fixture-reader/📦️packages/🦀️rust" }
semio-framework-value-testing-allocator = { path = "../../../../🧪️testing/💾️allocator/📦️packages/🦀️rust" }
serde_json = { workspace = true }''',
 "Indexed and named collection edit plans, key indexes, persistent roots, collection commits and the bounded heap sort that orders them.", "value-edit"))
rgl.write(N + "/📋️project.json", rgl.project_json("@semio-tech/value-edit-rs"))
rgl.write(N + "/🦀️.rs", '''//! 🔺️ Indexed and named collection edit plans over persistent lists — wiring only. Domain lives at the owner `🦀️.rs` files.

use semio_framework_value_control_contract::{ValueError, ValueRefusalKind};
use semio_framework_value_intrinsic::{DslValue, Number};
use semio_framework_value_native_decoding::{native_decoding, NativeDecodeConstructionRecipient, NativeDecodeControl};
use semio_framework_value_persistent::{list, original_vector_concat::OriginalVectorConcat};
use semio_framework_value_retained_clone::retained_clone;
use semio_framework_value_retirement::{artifact_retire_leaf, retirement, RetirementDemand};
use semio_framework_value_shared_issuance as shared_issuance;

''' + body + '''
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
pub(crate) mod value {
    pub(crate) use super::observe_retirement_allocations;
}
''')
fm = V + "/📦️packages/🦀️rust/Cargo.toml"
swap(fm, 'semio-framework-value-persistent = {', 'semio-framework-value-edit = { path = "../../📋️list/🔺️edit/📦️packages/🦀️rust" }\nsemio-framework-value-persistent = {')
rgl.register(V + "/📦️packages/🦀️rust", "semio-framework-value", N, "semio-framework-value-edit", "@semio-tech/value-rs", "@semio-tech/value-edit-rs", "🧰️framework/🔨️modules/🌱️value/📋️list/🔺️edit/📦️packages/🦀️rust", dep_ws=[])
