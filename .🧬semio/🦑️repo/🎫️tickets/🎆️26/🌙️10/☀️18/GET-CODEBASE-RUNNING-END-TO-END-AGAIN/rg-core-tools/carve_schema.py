import os, sys, re
sys.path.insert(0, os.path.dirname(__file__))
from valuecarve import *
N = V + "/🧬️schema/📦️packages/🦀️rust"
os.makedirs(N, exist_ok=True)
s = rgl.read(ROOTF)
m = re.search(r'//#region 🔖️SchemaExports\n(.*?)//#endregion 🔖️SchemaExports\n', s, re.S)
assert m
region = m.group(1).replace('include_str!("🧬️schema/', 'include_str!("../../')
s = s[:m.start()] + s[m.end():]
for mod, mount in [("graphql_int64", "🧬️schema/🔗️graphql/🔢️int64/🦀️.rs"), ("intrinsic_json", "🧬️schema/🌳️intrinsic/🔣️json/🦀️.rs")]:
    pat = re.compile(r'#\[path\s*=\s*"' + re.escape(mount) + r'"\]\s*pub mod ' + mod + r';\n')
    s, c = pat.subn("", s)
    assert c == 1, mod
s = s.replace("pub use semio_framework_value_persistent::{list, retained_equality};", "pub use semio_framework_value_persistent::{list, retained_equality};\npub use semio_framework_value_schema::{graphql_int64, intrinsic_json, register_value_schema_exports};", 1)
rgl.write(ROOTF, s)
rgl.write(N + "/Cargo.toml", manifest("semio-framework-value-schema", "semio_framework_value_schema", os.path.relpath(R+"/🧰️framework", N),
 '''semio-framework-schema-registry = { path = "../../../../🧬️schema/📇️registry/📦️packages/🦀️rust" }
semio-framework-value-control-contract = { path = "../../../🎟️control/🧬️contract/📦️packages/🦀️rust" }
semio-framework-value-intrinsic = { path = "../../../🌳️intrinsic/📦️packages/🦀️rust" }''', "",
 "The framework value schema export and the lossless intrinsic JSON and GraphQL Int64 spellings of the value contract.", "value-schema"))
rgl.write(N + "/📋️project.json", rgl.project_json("@semio-tech/value-schema-rs"))
rgl.write(N + "/🦀️.rs", '''//! 🧬️ The value schema export and its lossless spellings — wiring only. Domain lives at the owner `🦀️.rs` files.

use semio_framework_value_control_contract::{ValueError, ValueRefusalKind};
use semio_framework_value_intrinsic::{DslValue, FromValue, Number};

#[path = "../../🔗️graphql/🔢️int64/🦀️.rs"]
pub mod graphql_int64;
#[path = "../../🌳️intrinsic/🔣️json/🦀️.rs"]
pub mod intrinsic_json;

''' + region)
fm = V + "/📦️packages/🦀️rust/Cargo.toml"
swap(fm, 'semio-framework-value-registry = {', 'semio-framework-value-registry = {', 1)
t = rgl.read(fm)
t = t.replace('semio-framework-value-registry = {', 'semio-framework-value-schema = { path = "../../🧬️schema/📦️packages/🦀️rust" }\nsemio-framework-value-registry = {', 1)
rgl.write(fm, t)
rgl.register(V + "/📦️packages/🦀️rust", "semio-framework-value", N, "semio-framework-value-schema", "@semio-tech/value-rs", "@semio-tech/value-schema-rs", "🧰️framework/🔨️modules/🌱️value/🧬️schema/📦️packages/🦀️rust", dep_ws=[])
