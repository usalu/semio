#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from pathlib import Path
import json

TICKET = next(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))
state = json.loads((TICKET / "generators" / "w5_cad_state.json").read_text(encoding="utf-8"))
stdio_dirs = json.loads((TICKET / "generators" / "w5_cad_stdio_dirs.json").read_text(encoding="utf-8"))
CAD = Path(state["cad"])
BUILDER = state["builder"]
DECOMPOSER = state["decomposer"]
TEXT = state["text"]
BINARY = state["binary"]
DESER = state["deser"]
SER = state["ser"]
MUTS = CAD / "🧬️schema" / "🧬️mutations"
SLUGS = ["dwg", "glb", "gltf", "ifc", "json", "obj", "png", "step", "stl"]
CAD_PLUGIN = CAD.parent.parent
GLUE = CAD_PLUGIN / "📦️packages" / "🦀️rust" / "📦️glue.rs"
CARGO = CAD_PLUGIN / "📦️packages" / "🦀️rust" / "Cargo.toml"
TS_BARREL = CAD_PLUGIN / "📦️packages" / "🟦️typescript" / "📦️index.ts"

def mut_mod(dirname: str) -> str:
    slug = "".join(c if c.isascii() and (c.isalnum() or c == "-") else "" for c in dirname)
    return slug.replace("-", "_")

mut_dirs = [c.name for c in sorted(MUTS.iterdir()) if c.is_dir() and c.name not in (TEXT, BINARY)]

parts = []
parts.append("//#region 🗿️Artifacts")
parts.append("#[path = \".\"]")
parts.append("pub mod artifacts {")
parts.append("    #[path = \".\"]")
parts.append("    pub mod cad {")
parts.append('        #[path = "../../🗿️artifacts/📐️cad/🦀️component.rs"]')
parts.append("        mod component;")
parts.append("        pub use component::*;")
parts.append("")
parts.append('        #[path = "../../🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs"]')
parts.append("        mod interaction_spec;")
parts.append("        pub use interaction_spec::*;")
parts.append("")
parts.append(f'        #[path = "../../🗿️artifacts/📐️cad/🧬️schema/🧬️mutations/{TEXT}/🦀️component.rs"]')
parts.append("        pub mod op;")
parts.append(f'        #[path = "../../🗿️artifacts/📐️cad/🧬️schema/📸️snapshot/{TEXT}/🦀️component.rs"]')
parts.append("        pub mod dsl;")
parts.append(f'        #[path = "../../🗿️artifacts/📐️cad/🧬️schema/🧬️mutations/{BINARY}/🦀️component.rs"]')
parts.append("        pub mod spr;")
parts.append("")
parts.append('        #[path = "."]')
parts.append("        pub mod mutations {")
parts.append('            #[path = "../../🗿️artifacts/📐️cad/🧬️schema/🧬️mutations/🦀️component.rs"]')
parts.append("            mod component;")
parts.append("            pub use component::*;")

for d in mut_dirs:
    mod = mut_mod(d)
    base = f"../../🗿️artifacts/📐️cad/🧬️schema/🧬️mutations/{d}"
    parts.append('            #[path = "."]')
    parts.append(f"            pub mod {mod} {{{{")
    parts.append(f'                #[path = "{base}/🦠️mutation/🦀️component.rs"]')
    parts.append("                pub mod mutation;")
    parts.append(f'                #[path = "{base}/🔺️diff/🦀️component.rs"]')
    parts.append("                pub mod diff;")
    parts.append(f'                #[path = "{base}/↩️inverse/🦀️component.rs"]')
    parts.append("                pub mod inverse;")
    parts.append("            }")

parts.append("        }")
parts.append("")
parts.append('        #[path = "../../🗿️artifacts/📐️cad/🧬️schema/🦀️component.rs"]')
parts.append("        pub mod schema;")
parts.append("")
parts.append('        #[path = "."]')
parts.append("        pub mod diff {")
parts.append(f'            #[path = "../../🗿️artifacts/📐️cad/🧬️schema/🔺️diff/{TEXT}/🦀️component.rs"]')
parts.append("            mod component;")
parts.append("            pub use component::*;")
parts.append("")
parts.append('            #[path = "../../🗿️artifacts/📐️cad/🧬️schema/🔺️diff/🦀️component.rs"]')
parts.append("            pub mod schema;")
parts.append("            pub use schema::*;")
parts.append("        }")
parts.append("")
parts.append('        #[path = "."]')
parts.append("        pub mod snapshot {")
parts.append('            #[path = "../../🗿️artifacts/📐️cad/🧬️schema/📸️snapshot/🦀️component.rs"]')
parts.append("            pub mod schema;")
parts.append(f'            #[path = "../../🗿️artifacts/📐️cad/🧬️schema/📸️snapshot/{BINARY}/🦀️component.rs"]')
parts.append("            pub mod pack;")
parts.append("        }")
parts.append("")
parts.append(f'        #[path = "../../🗿️artifacts/📐️cad/{BUILDER}/🦀️component.rs"]')
parts.append("        pub mod builder;")
parts.append(f'        #[path = "../../🗿️artifacts/📐️cad/{DECOMPOSER}/🦀️component.rs"]')
parts.append("        pub mod decomposer;")
parts.append("")
parts.append('        #[path = "."]')
parts.append("        pub mod io {")
parts.append('            #[path = "../../🗿️artifacts/📐️cad/🚪️io/🦀️component.rs"]')
parts.append("            mod component;")
parts.append("            pub use component::*;")
parts.append('            #[path = "."]')
parts.append("            pub mod import {")
parts.append('                #[path = "."]')
parts.append("                pub mod deserializers {")
parts.append('                    #[path = "."]')
parts.append("                    pub mod artifacts {")
for slug in SLUGS:
    dname = stdio_dirs[slug]
    parts.append('                        #[path = "."]')
    parts.append(f"                        pub mod {slug} {{{{")
    parts.append(f'                            #[path = "../../🗿️artifacts/📐️cad/🚪️io/📥️import/{DESER}/🗿️artifacts/{dname}/🦀️component.rs"]')
    parts.append("                            mod component;")
    parts.append("                            pub use component::*;")
    parts.append("                        }")
parts.append("                    }")
parts.append("                }")
parts.append("            }")
parts.append('            #[path = "."]')
parts.append("            pub mod export {")
parts.append('                #[path = "."]')
parts.append("                pub mod serializers {")
parts.append('                    #[path = "."]')
parts.append("                    pub mod artifacts {")
for slug in SLUGS:
    dname = stdio_dirs[slug]
    parts.append('                        #[path = "."]')
    parts.append(f"                        pub mod {slug} {{{{")
    parts.append(f'                            #[path = "../../🗿️artifacts/📐️cad/🚪️io/📤️export/{SER}/🗿️artifacts/{dname}/🦀️component.rs"]')
    parts.append("                            mod component;")
    parts.append("                            pub use component::*;")
    parts.append("                        }")
parts.append("                    }")
parts.append("                }")
parts.append("            }")
for slug in SLUGS:
    parts.append('            #[path = "."]')
    parts.append(f"            pub mod {slug} {{{{")
    parts.append('                #[path = "."]')
    parts.append("                pub mod export {")
    parts.append(f"                    pub use crate::artifacts::cad::io::export::serializers::artifacts::{slug}::*;")
    parts.append("                }")
    parts.append('                #[path = "."]')
    parts.append("                pub mod import {")
    parts.append(f"                    pub use crate::artifacts::cad::io::import::deserializers::artifacts::{slug}::*;")
    parts.append("                }")
    parts.append("            }")
parts.append("        }")
parts.append('        #[path = "."]')
parts.append("        pub mod engine {")
parts.append('            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🦀️component.rs"]')
parts.append("            mod component;")
parts.append("            pub use component::*;")
parts.append("")
parts.append('            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/📥️geometry-import/🦀️component.rs"]')
parts.append("            pub mod geometry_import;")
parts.append('            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🔄️transformation/🦀️component.rs"]')
parts.append("            pub mod transformation;")
parts.append('            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🕹️interaction/🦀️component.rs"]')
parts.append("            pub mod interaction;")
parts.append('            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🔍️construct/🦀️component.rs"]')
parts.append("            pub mod construct;")
parts.append("        }")
parts.append("    }")
parts.append("}")
parts.append("")

new_region = "\n".join(parts) + "\n"
# fix double braces from f-string escaping
new_region = new_region.replace("{{", "{").replace("}}", "}")

text = GLUE.read_text(encoding="utf-8")
start = text.find("//#region 🗿️Artifacts")
end = text.find("//#endregion 🗿️Artifacts")
assert start >= 0 and end >= 0
GLUE.write_text(text[:start] + new_region + text[end:], encoding="utf-8")
print("glue ok", len(new_region))

cargo = CARGO.read_text(encoding="utf-8")
if "semio-s-plugin-stdio" not in cargo:
    cargo = cargo.replace("[dependencies]\n", "[dependencies]\nsemio-s-plugin-stdio = { path = \"../../../🗄️stdio/📦️packages/🦀️rust\", package = \"semio-s-plugin-stdio\" }\n", 1)
    CARGO.write_text(cargo, encoding="utf-8")
    print("cargo dep added")
else:
    print("cargo dep present")

ts_leaf = '🟦️component.ts'
ts_lines = [
    "/** cad facet WASM facades */",
    f"export * as cad_schema from \"../../🗿️artifacts/📐️cad/🧬️schema/{ts_leaf}\";",
    f"export * as cad_snapshot_schema from \"../../🗿️artifacts/📐️cad/🧬️schema/📸️snapshot/{ts_leaf}\";",
    f"export * as cad_snapshot_text from \"../../🗿️artifacts/📐️cad/🧬️schema/📸️snapshot/{TEXT}/{ts_leaf}\";",
    f"export * as cad_snapshot_binary from \"../../🗿️artifacts/📐️cad/🧬️schema/📸️snapshot/{BINARY}/{ts_leaf}\";",
    f"export * as cad_diff from \"../../🗿️artifacts/📐️cad/🧬️schema/🔺️diff/{TEXT}/{ts_leaf}\";",
    f"export * as cad_diff_schema from \"../../🗿️artifacts/📐️cad/🧬️schema/🔺️diff/{ts_leaf}\";",
    f"export * as cad_dsl from \"../../🗿️artifacts/📐️cad/🧬️schema/📸️snapshot/{TEXT}/{ts_leaf}\";",
    f"export * as cad_pack from \"../../🗿️artifacts/📐️cad/🧬️schema/📸️snapshot/{BINARY}/{ts_leaf}\";",
    f"export * as cad_op from \"../../🗿️artifacts/📐️cad/🧬️schema/🧬️mutations/{TEXT}/{ts_leaf}\";",
    f"export * as cad_mutations from \"../../🗿️artifacts/📐️cad/🧬️schema/🧬️mutations/{ts_leaf}\";",
    f"export * as cad_spr from \"../../🗿️artifacts/📐️cad/🧬️schema/🧬️mutations/{BINARY}/{ts_leaf}\";",
    f"export * as cad_builder from \"../../🗿️artifacts/📐️cad/{BUILDER}/{ts_leaf}\";",
    f"export * as cad_decomposer from \"../../🗿️artifacts/📐️cad/{DECOMPOSER}/{ts_leaf}\";",
    f"export * as cad_io from \"../../🗿️artifacts/📐️cad/🚪️io/{ts_leaf}\";",
]
ts = "\n".join(ts_lines) + "\n"
TS_BARREL.write_text(ts, encoding="utf-8")
print("ts ok")
print("muts", [mut_mod(d) for d in mut_dirs])
