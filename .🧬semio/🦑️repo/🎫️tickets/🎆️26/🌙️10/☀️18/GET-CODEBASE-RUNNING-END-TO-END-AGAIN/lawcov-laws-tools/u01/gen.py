import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
PL = "✏s/🔌plugins"
oracle = open(os.path.join(HERE, "..", "pylib.py"), encoding="utf8").read() + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
MED = "medium"


def b(i, kind, line=0, prio=MED):
    return [i, kind, prio, line]


cases = []


def case(id_, files, expect=None, counts=None, rule="policyApaBreaches", generate=None):
    row = {"id": id_, "rule": rule, "files": files}
    if generate:
        row["generate"] = generate
    if counts is not None:
        row["expectCounts"] = counts
    else:
        row["expect"] = expect
    cases.append(row)


P = PL + "/🔭probe"
case("no-plugins-root", {"README.md": ["# empty"]}, [])
case("clean-plugin", {
    P + "/AGENTS.md": ["a"], P + "/README.md": ["r"], P + "/LICENSE.md": ["l"],
    P + "/🦀.rs": ["pub fn plugin() {}"],
    P + "/📦packages/🦀rust/Cargo.toml": ["[package]", 'name = "probe"', "", "[dependencies]", 'semio-framework-plugin = { path = "../../../../🧰framework/plugin" }'],
    P + "/🗿artifacts/🔸kind/⚙engine/🦀.rs": ["pub fn run() -> u8 { 1 }"],
    P + "/🗿artifacts/🔸kind/ui.ts": ["export const value = 1;"],
}, [])

S = PL + "/🔭shape"
SH = "taxonomy/plugin-closed-shape"
case("closed-shape-entries", {
    S + "/AGENTS.md": ["a"], S + "/README.md": ["r"], S + "/LICENSE.md": ["l"], S + "/🦀.rs": [],
    S + "/🧫fixtures/a.json": [], S + "/🔨modules/m.txt": [], S + "/🎮commands/c.txt": [], S + "/🧬schema/s.json": [],
    S + "/📦packages/p.txt": [], S + "/🗿artifacts/a.txt": [], S + "/📇registry/r.txt": [],
    S + "/🎛apps/a.txt": [], S + "/🔧setup/a.txt": [], S + "/node_modules/x.js": [],
    S + "/.DS_Store": [], S + "/notes.txt": [], S + "/Cargo.toml": [],
}, [
    b("plugin-closed-shape-dir-" + S + "/🎛apps", SH), b("plugin-closed-shape-dir-" + S + "/🔧setup", SH),
    b("plugin-closed-shape-junk-" + S + "/node_modules", SH),
    b("plugin-closed-shape-file-" + S + "/.DS_Store", SH), b("plugin-closed-shape-file-" + S + "/notes.txt", SH), b("plugin-closed-shape-file-" + S + "/Cargo.toml", SH),
])

R1 = P + "/🗿artifacts/🔸kind/🦀.rs"
rust = [
    "use std::fs;",
    "pub fn load() -> String {",
    '    fs::read_to_string("a.txt").unwrap()',
    "}",
    "pub fn env() {",
    '    let v = std::env::var("HOME");',
    "}",
    'pub fn net() { let _ = std::net::TcpStream::connect("x"); }',
    "thread_local! { static X: u8 = 0; }",
    "static mut COUNTER: u32 = 0;",
    "lazy_static! {",
    "    static ref M: Mutex<u8> = Mutex::new(0);",
    "}",
    "static HITS: AtomicU64 = AtomicU64::new(0);",
    "pub fn counter() -> u64 {",
    "    static NEXT: AtomicU64 = AtomicU64::new(0);",
    "    let local: Mutex<u8> = Mutex::new(0);",
    "    NEXT.fetch_add(1, Ordering::SeqCst)",
    "}",
    'let s = "std::fs::read";',
    '// std::env::var("X")',
    "#[cfg(test)]",
    "mod tests {",
    '    fn t() { std::fs::read("x"); }',
    "}",
    "#[test]",
    "fn quick() {",
    '    let _ = File::open("y");',
    "}",
    'pub fn after() { let _ = File::open("z"); }',
]


def pu(sub, line):
    return b("plugin-purity-%s-%s-%d" % (sub, R1, line), "taxonomy/plugin-purity-" + sub, line)


case("rust-purity-lines", {R1: rust}, [
    pu("filesystem-io", 3), pu("env-process-io", 6), pu("network-io", 8), pu("thread-local-state", 9), pu("static-mut-state", 10),
    pu("lazy-static-state", 12 - 1), pu("interior-mutability-mutex", 12), pu("interior-mutability-atomicu64", 14),
    pu("interior-mutability-atomicu64", 16), pu("filesystem-io", 30),
])

T1 = P + "/🗿artifacts/🔸kind/ui.ts"
case("ts-purity-and-state-lane", {T1: [
    'export const a = await fetch("/api");',
    'const ws = new WebSocket("ws://x");',
    '// fetch("/commented")',
    "const keep = 1; // localStorage",
    'indexedDB.open("db");',
    'ephemeralBox("k");',
]}, [
    b("plugin-purity-ts-side-effect-%s-1" % T1, "taxonomy/plugin-purity-ts-side-effect", 1),
    b("plugin-purity-ts-side-effect-%s-2" % T1, "taxonomy/plugin-purity-ts-side-effect", 2),
    b("plugin-purity-ts-side-effect-%s-5" % T1, "taxonomy/plugin-purity-ts-side-effect", 5),
    b("state-lane-storage-%s-5" % T1, "taxonomy/state-lane-storage-outside-config-lane", 5),
    b("state-lane-ephemeral-box-%s-6" % T1, "taxonomy/state-lane-ephemeral-box", 6),
])

S1 = "🧰framework/🔨modules/🔭probe/ui.ts"
S2 = "🧰framework/🔨modules/🖥platform/store.ts"
lane_files = {
    S1: ['localStorage.setItem("a", "b");', '// localStorage.removeItem("a");', " * indexedDB in docblock", 'const t = "sessionStorage";', "const u = 1; /* localStorage */", 'ephemeralMap<string>("x");', 'const v = ephemeralSet("y"); localStorage.clear();'],
    S2: ['localStorage.setItem("a", "b");', 'ephemeralBox("k");'],
    "🧰framework/🛍products/🦑repo/x.ts": ["localStorage.x;"],
    "🧰framework/🔨modules/🔭probe/ui.test.ts": ["localStorage.x;"],
    "🧰framework/🔨modules/🔭probe/🟦.ts": ["localStorage.x;"],
    "🧰framework/🔨modules/🔭probe/🧪tests/a.ts": ["localStorage.x;"],
    "🧰framework/🔨modules/🔭probe/🧪fixtures/a.ts": ["localStorage.x;"],
    "🌎hub/x.ts": ["localStorage.x;"],
}
LK, LE = "taxonomy/state-lane-storage-outside-config-lane", "taxonomy/state-lane-ephemeral-box"
lane_expect = [b("state-lane-storage-%s-1" % S1, LK, 1), b("state-lane-ephemeral-box-%s-6" % S1, LE, 6), b("state-lane-storage-%s-7" % S1, LK, 7), b("state-lane-ephemeral-box-%s-7" % S1, LE, 7), b("state-lane-ephemeral-box-%s-2" % S2, LE, 2)]
case("state-lane-exhaustiveness-direct", lane_files, lane_expect, rule="policyStateLaneExhaustivenessBreaches")
case("state-lane-exhaustiveness-in-aggregate", lane_files, lane_expect)

P2 = PL + "/🔭reg"
RE = P2 + "/🗿artifacts/🔸kind/⚙engine/🦀.rs"
RA = P2 + "/🗿artifacts/🔸kind/🎛apps/🦀.rs"
RS = P2 + "/🗿artifacts/🔸kind/🦀.rs"
EB, VI = "taxonomy/plugin-registration-engine-backlog", "taxonomy/plugin-registration-violation"
case("declarative-registration", {
    RE: ["pub fn init() {", "    register_app_io(spec);", "    use semio_framework_os::host;", '    register_language("x"); // register_app_io(commented)', "}"],
    RA: ["fn a() { register_mesh_exporter(x); }", 'let s = "register_app_io";', "// register_app_io(x)", "use semio_framework_os::Foo;", "fn b() { register_app_io(x); let _ = semio_framework_os::y; }"],
    RS: ["pub fn plugin() -> Plugin {", '    Plugin::builder("x")', "        .setup(|| {})", "        .build()", "}", 'pub fn clean() -> Plugin { Plugin::builder("y").artifact(a).build() }'],
}, [
    b("plugin-registration-engine-backlog-family-call-%s-2" % RE, EB, 2), b("plugin-registration-engine-backlog-os-host-path-%s-3" % RE, EB, 3), b("plugin-registration-engine-backlog-family-call-%s-4" % RE, EB, 4),
    b("plugin-registration-violation-family-call-%s-1" % RA, VI, 1), b("plugin-registration-violation-os-host-path-%s-4" % RA, VI, 4),
    b("plugin-registration-violation-family-call-%s-5" % RA, VI, 5), b("plugin-registration-violation-os-host-path-%s-5" % RA, VI, 5),
    b("plugin-registration-setup-callback-%s-3" % RS, "taxonomy/plugin-registration-setup-callback", 3),
])

P3 = PL + "/🔭dep"
CT = P3 + "/📦packages/🦀rust/Cargo.toml"
fw = '{ path = "../../../../🧰framework/%s" }'
case("plugin-dependency-allowlist", {
    CT: ["[package]", 'name = "dep"', "", "[dependencies]", 'serde = { version = "1" }', "semio-framework-plugin = " + fw % "plugin", "semio-framework-fem = " + fw % "fem",
         'renamed = { path = "../../../../🧰framework/3d", package = "semio-framework-3d" }', "semio-framework-os = " + fw % "os", "serde_json = " + fw % "json", 'local = { path = "../local" }', "",
         "[dev-dependencies]", "semio-framework-testkit = " + fw % "testkit", "", "[target.'cfg(unix)'.dependencies]", "semio-framework-unix = " + fw % "unix"],
    P3 + "/🗿artifacts/🔸kind/Cargo.toml": ["[dependencies]", "semio-framework-fem = " + fw % "fem"],
}, [
    b("plugin-dependency-allowlist-%s-semio-framework-fem" % CT, "taxonomy/plugin-dependency-allowlist", 7),
    b("plugin-dependency-allowlist-%s-semio-framework-os" % CT, "taxonomy/plugin-dependency-os-host", 9),
    b("plugin-dependency-allowlist-%s-semio-framework-unix" % CT, "taxonomy/plugin-dependency-allowlist", 17),
])

P4, P5 = PL + "/🔭eff", PL + "/🔭bb"
EE = P4 + "/🗿artifacts/🔸kind/⚙engine/🦀.rs"
case("effect-capability-parity", {
    P4 + "/🦀.rs": ["pub fn plugin() -> Plugin {", '    Plugin::builder("e").capability(ArtifactKind::Window).build()', "}"],
    EE: ["fn a() { let _ = Effect::OpenWindow; }", "fn b() { let _ = Effect::ClipboardWrite(x); }", "fn c() { let _ = Effect::ClipboardWrite(y); }", "fn d() { let _ = Effect::OpenExternalUrl(u); }",
         "fn e() { let _ = Effect::NotARealEffect; }", 'let s = "Effect::RequestSync";', "#[cfg(test)]", "mod tests {", "    fn t() { let _ = Effect::LoadDocument; }", "}"],
    P5 + "/🦀.rs": ['pub fn plugin() -> Plugin { Plugin::builder("b").local_backbone_storage(x).build() }'],
    P5 + "/🗿artifacts/🔸kind/🦀.rs": ["fn a() { let _ = Effect::RequestSync; }"],
}, [
    b("effect-capability-parity-%s-ClipboardWrite" % P4, "taxonomy/effect-capability-parity", 2),
    b("effect-capability-parity-%s-OpenExternalUrl" % P4, "taxonomy/effect-capability-parity", 4),
])

case("closed-shape-ratchet-ceiling", {}, counts={MED: 41, "high": 1}, generate=[{"path": PL + "/🔭o{i}/stray.txt", "lines": [], "count": 42}])

schema = std_schema("Plugin Architecture Law Corpus", ["policyApaBreaches", "policyStateLaneExhaustivenessBreaches"])
corpus = {"version": 1, "cases": cases}

ts = std_ts(
    "🔌 The artifacts-only plugin architecture law reports exactly the breaches its language-agnostic corpus names (shape, purity, state lanes, registration, dependency allowlist, effect capabilities, ratchet), agrees with an independent Python model, and rejects a malformed corpus.",
    'import { policyApaBreaches, policyStateLaneExhaustivenessBreaches } from "../../🟦.ts";',
    "  policyApaBreaches: (root) => policyApaBreaches(root),\n  policyStateLaneExhaustivenessBreaches: (root) => policyStateLaneExhaustivenessBreaches(root),",
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🔌plugin-architecture", "🔌architecture", ts, corpus, schema, oracle)
print(folder, plen, len(cases))
