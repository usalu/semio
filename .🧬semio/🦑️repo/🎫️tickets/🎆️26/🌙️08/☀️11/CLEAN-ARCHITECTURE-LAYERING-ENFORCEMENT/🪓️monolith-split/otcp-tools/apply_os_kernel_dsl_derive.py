import os, re, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import edit as E, applylib, compgen

O = "🧰️framework/🛍️products/💻️os/🔨️modules/"
D = O + "🗣️dsl/✨️derive/"
EX = D + "🧩️expand/"
E_SNAKE = "semio_framework_os_kernel_dsl_derive_expand"
E_CRATE = "semio-framework-os-kernel-dsl-derive-expand"
P_MANIFEST = D + "📦️packages/🦀️rust/Cargo.toml"
ctx = applylib.Ctx("os-kernel-dsl-derive", E_SNAKE, P_MANIFEST, E_CRATE, real_map={"component": "testkit::component"})
R = E.ROOT
C = D + "🧪️tests/🌆️companion/"
keys = list(ctx.mounts)
GLUE = D + "📦️packages/🦀️rust/🦀️.rs"
if "--p" in sys.argv:
    t = E.read(GLUE)
    assert t.count("#[path = \"../../🦀️.rs\"]\nmod component;\n\n") == 1
    t2 = t.replace("#[path = \"../../🦀️.rs\"]\nmod component;\n\n", "").replace("component::expand_", E_SNAKE + "::expand_")
    assert "component::" not in t2
    E.pre_copy(GLUE); E.write(GLUE, t2); E.note("edit", GLUE)
    ctx.remove_mounts()
    pc = E.read(P_MANIFEST)
    # production dependency lines of the proc-macro crate move to the expansion library
    a = pc.index("[dependencies]")
    b = pc.index("[dev-dependencies]") if "[dev-dependencies]" in pc else len(pc)
    deps_block = pc[a:b]
    new_block = "[dependencies]\n" + f'{E_CRATE} = {{ path = "../../🧩️expand/📦️packages/🦀️rust" }}\n' + [l for l in deps_block.splitlines() if l.startswith("semio-framework-native-build-compiler-resources")][0] + "\n\n"
    E.pre_copy(P_MANIFEST); E.write(P_MANIFEST, pc[:a] + new_block); E.note("edit", P_MANIFEST)
    # the expansion library
    specs = ctx.specs
    keep = ["semio_framework_ui_locale", "semio_framework_schema_state", "semio_framework_value", "syn", "quote", "proc_macro2", "serde_json", "semio_framework_hash", "semio_framework_native_build_compiler_resources"]
    dep_lines = []
    for ident in keep:
        sec, line = compgen.merged_line(specs[ident])
        dep_lines.append((sec, compgen.reroot(line, 1)))
    pk = EX + "📦️packages/🦀️rust/"
    depth = len(pk.rstrip("/").split("/"))
    desc = "Expansion library of the DSL derives: the product artifact envelope, OS diff codec and source-authorized mutation derive implementations behind the proc-macro crate"
    cargo = compgen.cargo_text(E_CRATE, "dsl-derive-expand", desc, depth, dep_lines, None, None)
    cargo = cargo.replace('publish = false\n', '').replace('{p_name}', '')
    cargo = re.sub(r"\[dependencies\]\nNone = \{[^\n]*\}\n", "[dependencies]\n", cargo)
    cargo = cargo.replace('[lib]\npath = "../../🦀️.rs"', '[lib]\nname = "' + E_SNAKE + '"\npath = "../../🦀️.rs"\n\n[features]\ntestkit = []')
    E.put(pk + "Cargo.toml", cargo)
    E.put(pk + "📋️project.json", compgen.project_json("@semio-tech/dsl-derive-expand-rs"))
    E.put(EX + "🦀️.rs", '''//! 🧩️ Expansion library of the DSL derives: the implementation owner `../🦀️.rs` compiled as an ordinary library, so its units are testable; the proc-macro entry points live in `../📦️packages/🦀️rust/🦀️.rs`.

extern crate proc_macro;

use semio_framework_native_build_compiler_resources as compiler_resources;

#[path = "../🦀️.rs"]
mod component;
pub use component::*;

/// 🪝️ Test hook of the companion crate `semio-framework-os-kernel-dsl-derive-tests`: the private expansion units, public only behind the `testkit` feature.
#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub mod component {
        pub use crate::component::*;
    }
}
''')
title = "Companion tests of the DSL derive expansion library: macro export roster, mutation source authority, leaf and aggregate derives, composite attributes and diff codecs."
if "--show" in sys.argv:
    print(ctx.render_root(R + "/" + C, keys, title, "🌆️", None, ["extern crate proc_macro;"]))
if "--create" in sys.argv:
    print(ctx.companion(C, keys, "semio-framework-os-kernel-dsl-derive-tests", "os-kernel-dsl-derive-tests", "@semio-tech/os-kernel-dsl-derive-tests-rs", "Companion tests of the DSL derive expansion library", title, "🌆️", p_features=("testkit",), p_rel="../../../../🧩️expand/📦️packages/🦀️rust", extra=["extern crate proc_macro;"]))
