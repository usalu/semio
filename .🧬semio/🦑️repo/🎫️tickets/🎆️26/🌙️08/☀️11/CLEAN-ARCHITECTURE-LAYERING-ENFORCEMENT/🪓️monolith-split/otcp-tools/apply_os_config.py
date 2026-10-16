import os, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import edit as E, applylib, fixtures

O = "🧰️framework/🛍️products/💻️os/"
D = O + "🎚️config/"
ctx = applylib.Ctx("os-config", "semio_framework_os_config", D + "📦️packages/🦀️rust/Cargo.toml", "semio-framework-os-config", real_map={"opening_config::component": "opening_config::testkit::component"})
R = E.ROOT
keys = list(ctx.mounts)
half = (len(keys) + 1) // 2
K1, K2 = keys[:half], keys[half:]
C1 = D + "🧪️tests/🌈️companion-1/"
C2 = D + "🧪️tests/🌌️companion-2/"
PK1 = R + "/" + C1 + "📦️packages/🦀️rust"
PK2 = R + "/" + C2 + "📦️packages/🦀️rust"
if "--p" in sys.argv:
    ctx.remove_mounts()
    for f, name in (("🧬️schema/🧬️mutations/📥️admit-local-document/🦀️.rs", "x"), ("🧬️schema/🧬️mutations/📎️attach-local-folder/🦀️.rs", "x")):
        E.edit(D + f, "#[cfg(test)] use semio_framework_value::ToValue;\n", "")
    E.edit(D + "🦀️.rs", "    #[path = \"🧬️schema/🦀️.rs\"]\n    mod component;\n    pub use component::*;\n", "    #[path = \"🧬️schema/🦀️.rs\"]\n    mod component;\n    pub use component::*;\n\n    /// 🪝️ Test hook of the companion crates `semio-framework-os-config-tests-1` and `-2`: the private schema module, public only behind the `testkit` feature.\n    #[cfg(feature = \"testkit\")]\n    #[doc(hidden)]\n    pub mod testkit {\n        pub mod component {\n            pub use super::super::component::*;\n        }\n    }\n")
    pc = D + "📦️packages/🦀️rust/Cargo.toml"
    t = E.read(pc)
    i = t.index("\n[dev-dependencies]\n")
    E.edit(pc, t[i:], "")
    E.edit(pc, "[lib]\nname = \"semio_framework_os_config\"\npath = \"../../🦀️.rs\"\n", "[lib]\nname = \"semio_framework_os_config\"\npath = \"../../🦀️.rs\"\n\n[features]\ntestkit = []\n")
if "--fixtures" in sys.argv:
    for keys_, pk in ((K1, PK1), (K2, PK2)):
        files = sorted({f for k in keys_ for f in ctx.subtree(k)})
        log = []
        n = fixtures.convert_statics(files, pk, log=log)
        print("fixture reads converted:", n, "files", len(log))
extra = ["extern crate semio_framework_os_kernel as dsl;", "extern crate semio_framework_os_kernel as protocol;"]
kernel = None
if "--show" in sys.argv:
    print(ctx.render_root(R + "/" + C1, K1, "Companion tests of os-config (1 of 2).", "🌈️", None, extra))
if "--create" in sys.argv:
    for idx, (C, ks, em, nm, pr) in enumerate(((C1, K1, "🌈️", "1", "1"), (C2, K2, "🌌️", "2", "2")), 1):
        title = f"Companion tests of os-config ({idx} of 2): configuration schema, mutation and text IO tests."
        print(ctx.companion(C, ks, f"semio-framework-os-config-tests-{idx}", f"os-config-tests-{idx}", f"@semio-tech/os-config-tests-{idx}-rs", f"Companion tests of os-config ({idx} of 2): configuration schema, mutation and text IO tests", title, em, p_features=("testkit",), extra=extra, extra_deps=('semio-framework-os-kernel = { workspace = true, features = ["protocol-laws", "mutation-testing"] }',)))
