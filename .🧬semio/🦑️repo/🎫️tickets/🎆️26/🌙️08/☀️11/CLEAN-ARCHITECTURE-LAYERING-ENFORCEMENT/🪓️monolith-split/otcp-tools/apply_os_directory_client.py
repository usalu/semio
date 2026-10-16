import os, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import edit as E, applylib, genmirror

O = "🧰️framework/🛍️products/💻️os/🔨️modules/"
D = O + "📇️directory/"
CL = D + "🔌️client/"
ctx = applylib.Ctx("os-directory-client", "semio_framework_os_directory_client", CL + "📦️packages/🦀️rust/Cargo.toml", "semio-framework-os-directory-client")
R = E.ROOT
NATIVE_STREAM = f"{R}/{CL}🧪️tests/🔬️native-streaming/🦀️.rs"
NATIVE_IDENT = f"{R}/{CL}🧪️tests/🔬️native-runtime-identity/🦀️.rs"
IDENT_TESTS = f"{R}/{CL}🪪️runtime/🧪️tests/🪪️runtime/🦀️.rs"

if "--p" in sys.argv:
    ctx.remove_mounts()
    E.edit(CL + "🦀️.rs", '    #[cfg(test)]\n    include!("🧪️tests/🔬️native-runtime-identity/🦀️.rs");\n\n', "")
    E.edit(CL + "🦀️.rs", '\n    #[cfg(test)]\n    include!("🧪️tests/🔬️native-streaming/🦀️.rs");\n', "")
    E.edit(CL + "🦀️.rs", "    #[cfg(test)]\n    pub(crate) fn test(hub_origin: &str, capability: &str) -> Self {", "    #[cfg(any(test, feature = \"testkit\"))]\n    #[doc(hidden)]\n    pub fn test(hub_origin: &str, capability: &str) -> Self {")
    pc = CL + "📦️packages/🦀️rust/Cargo.toml"
    E.edit(pc, "[features]\ndefault = []\n", "[features]\ndefault = []\ntestkit = []\n")
    E.edit(pc, "\n[dev-dependencies]\nsemio-framework-async-macros = { path = \"../../../../../../../🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust\" }\n", "")

native_used = genmirror.used_tokens([NATIVE_STREAM, NATIVE_IDENT, IDENT_TESTS])
host_file = f"{R}/{CL}🦀️.rs"
real = ctx.real
nat_uses = genmirror.host_use_lines(host_file, ("client", "native"), ctx.pkg, real, native_used, ctx="native", traits=ctx.traits)
ctx.node_extras[("client", "native")] = {"uses": nat_uses, "includes": [NATIVE_IDENT, NATIVE_STREAM]}

C = CL + "🧪️tests/🧶️companion/"
keys = ["client", "client::document_http", "identity"]
if "--show" in sys.argv:
    print(ctx.render_root(f"{R}/{C}", keys, "Companion tests of os-directory-client: client unit tests, native transports, document HTTP and identity.", "🧶️"))
if "--create" in sys.argv:
    print(ctx.companion(C, keys, "semio-framework-os-directory-client-tests", "os-directory-client-tests", "@semio-tech/os-directory-client-tests-rs", "Companion tests of os-directory-client: client unit tests, native transports, document HTTP and identity", "Companion tests of os-directory-client: client unit tests, native transports, document HTTP and identity.", "🧶️", p_features=("ureq", "sync", "testkit")))
