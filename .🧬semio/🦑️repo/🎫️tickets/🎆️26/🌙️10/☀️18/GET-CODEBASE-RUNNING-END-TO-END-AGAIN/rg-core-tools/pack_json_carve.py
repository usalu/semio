import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import rgl
R = rgl.ROOT
J = R + "/🧰️framework/🔨️modules/🎒️pack/🔤️json"
FW = R + "/🧰️framework"

def edit(p, old, new, cnt=1):
    s = rgl.read(p)
    assert s.count(old) == cnt, (p, old, s.count(old))
    rgl.write(p, s.replace(old, new))

F = J + "/🎯️float-writer/📦️packages/🦀️rust"
os.makedirs(F, exist_ok=True)
rgl.write(F + "/Cargo.toml", '''[package]
workspace = "../../../../../.."
name = "semio-framework-pack-json-float"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Canonical binary64 and binary32 JSON spellings: byte-identical to the serde_json writer, with exact round-half-to-even digit generation and a fixed scalar text sink."

[package.metadata.semio]
role = "framework"
id = "pack-json-float"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
name = "semio_framework_pack_json_float"
path = "../../🦀️.rs"
''')
rgl.write(F + "/📋️project.json", rgl.project_json("@semio-tech/framework-pack-json-float-rs"))
ff = J + "/🎯️float-writer/🦀️.rs"
edit(ff, "use super::*;\n", "use std::cmp::Ordering;\nuse std::fmt;\n")
s = rgl.read(ff)
s = s.replace("pub(super) struct ScalarText", "pub struct ScalarText").replace("    pub(super) fn new()", "    pub fn new()").replace("    pub(super) fn text(", "    pub fn text(")
rgl.write(ff, s)

S = J + "/🛫️encode/🫳️borrowed/📦️packages/🦀️rust"
os.makedirs(S, exist_ok=True)
rgl.write(S + "/Cargo.toml", '''[package]
workspace = "../../../../../../.."
name = "semio-framework-pack-json-source-sink"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Canonical JSON octets of a borrowed ranked write source: bounded caller output that is retained on refusal."

[package.metadata.semio]
role = "framework"
id = "pack-json-source-sink"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
name = "semio_framework_pack_json_source_sink"
path = "../../🦀️.rs"

[dependencies]
semio-framework-pack-json = { path = "../../../../📦️packages/🦀️rust" }
semio-framework-pack-json-float = { path = "../../../../🎯️float-writer/📦️packages/🦀️rust" }
semio-framework-value = { path = "../../../../../../🌱️value/📦️packages/🦀️rust" }
''')
rgl.write(S + "/📋️project.json", rgl.project_json("@semio-tech/framework-pack-json-source-sink-rs"))
sk = J + "/🛫️encode/🫳️borrowed/🦀️.rs"
edit(sk, "use super::{JsonWriteSource,JsonWriteNode,ScalarText,write_float_to,MAX_DEPTH,ValueError,ValueRefusalKind};\nuse semio_framework_value::NativeEncodeControl;",
     "use semio_framework_pack_json::{JsonWriteSource,JsonWriteNode,MAX_DEPTH};\nuse semio_framework_pack_json_float::{ScalarText,write_float_to};\nuse semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};")

root = J + "/🦀️.rs"
edit(root, '#[path = "🎯️float-writer/🦀️.rs"]\nmod float_writer;\npub use float_writer::*;\n', 'use semio_framework_pack_json_float::{write_float_to, ScalarText};\n')
edit(root, '#[path = "🛫️encode/🫳️borrowed/🦀️.rs"]\nmod borrowed_source_sink;\npub use borrowed_source_sink::write_json_source_into;\n', '')
edit(J + "/📦️packages/🦀️rust/Cargo.toml", 'semio-framework-value = { path = "../../../../🌱️value/📦️packages/🦀️rust" }\n', 'semio-framework-pack-json-float = { path = "../../🎯️float-writer/📦️packages/🦀️rust" }\nsemio-framework-value = { path = "../../../../🌱️value/📦️packages/🦀️rust" }\n')
edit(J + "/🛫️encode/🔣️scalar/🦀️.rs", "crate::write_float_to(value", "semio_framework_pack_json_float::write_float_to(value")
