import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
oracle = open(os.path.join(HERE, "..", "pylib.py"), encoding="utf8").read() + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []


def case(id_, rule, files, **kw):
    row = {"id": id_, "rule": rule, "files": files}
    row.update(kw)
    cases.append(row)


def b(i, kind, line=0):
    return [i, kind, "high", line]


case("cargo-manifests-skip-generated-and-dependency-directories", "policyDiscoverCargoTomlFiles",
     {"Cargo.toml": [], "a/Cargo.toml": [], "a/b/Cargo.toml": [], "node_modules/x/Cargo.toml": [], "target/y/Cargo.toml": [], "a/Cargo.lock": []},
     expectList=["Cargo.toml", "a/Cargo.toml", "a/b/Cargo.toml"])
DB = "protocol-migration/db-server-only"
case("db-crates-are-server-only", "policyDbServerOnlyBreaches", {
    "app/Cargo.toml": ["[dependencies]", 'db_core = { path = "../db/core" }', 'db = { path = "../db", features = ["x"] }', 'dbx = { path = "../dbx" }', 'db_store = { version = "1" }', 'db_net_v2 = { path = "../n" }'],
    "Cargo.toml": ['db_root = { path = "db/root" }'],
    "db/core/Cargo.toml": ['db_core = { path = "../x" }'],
    "hub/app/Cargo.toml": ['db_core = { path = "../x" }'],
}, expect=[b("db-server-only-app/Cargo.toml-db_core", DB), b("db-server-only-app/Cargo.toml-db", DB), b("db-server-only-app/Cargo.toml-db_net_v2", DB), b("db-server-only-Cargo.toml-db_root", DB)])
ST = "os-state-authority/item-scope-global"
IM = "os-state-authority/id-minting"
SM = "os-state-authority/authority-struct-map"
S = "✏s/🔌plugins/🔭p/🦀.rs"
state = [
    "static mut COUNTER: u32 = 0;",
    "thread_local! { static X: u8 = 0; }",
    "lazy_static! {",
    "    static ref M: Mutex<u8> = Mutex::new(0);",
    "}",
    "static CACHE: Mutex<Vec<u8>> = Mutex::new(Vec::new());",
    "const LAZY: OnceLock<u8> = OnceLock::new();",
    "static NAMES: &[&str] = &[];",
    "pub struct SessionStore {",
    "    seq: u32,",
    "    items: HashMap<u32, String>,",
    "    ids: AtomicU64,",
    "}",
    "pub struct Plain {",
    "    items: HashMap<u32, String>,",
    "}",
    "fn f() {",
    "    static LOCAL: Mutex<u8> = Mutex::new(0);",
    "    let seq: u32 = 1;",
    "}",
    "#[cfg(test)]",
    "mod tests {",
    "    static mut T: u8 = 0;",
    "}",
]
case("os-state-authority-globals-ids-and-authority-maps", "policyOsStateAuthorityBreaches", {
    S: state,
    "🌎hub/x.rs": ["static mut H: u8 = 0;"], "compose/y.rs": ["static mut H: u8 = 0;"], "🧰framework/🛍products/💻os/z.rs": ["static mut H: u8 = 0;"],
    "🧰framework/other/w.rs": ["static mut W: u8 = 0;"],
}, expect=[
    b("os-state-authority-static-mut-%s-1" % S, ST, 1), b("os-state-authority-thread-local-%s-2" % S, ST, 2), b("os-state-authority-lazy-static-%s-3" % S, ST, 3),
    b("os-state-authority-static-Mutex-%s-6" % S, ST, 6), b("os-state-authority-static-OnceLock-%s-7" % S, ST, 7),
    b("os-state-authority-id-mint-%s-10" % S, IM, 10), b("os-state-authority-map-field-%s-11-SessionStore" % S, SM, 11), b("os-state-authority-id-mint-%s-12" % S, IM, 12),
    b("os-state-authority-static-mut-🧰framework/other/w.rs-1", ST, 1),
])
DA = "os-state-authority/document-app-shape"
app = lambda n: ["impl DocumentApp for %s {" % n, "    type Snapshot = S;", "    type Mutation = M;", "}"]
case("document-apps-carry-no-state", "policyDocumentAppShapeBreaches", {
    "a/stateful.rs": ["pub struct Counter { value: u32 }"] + app("Counter"),
    "a/unit.rs": ["pub struct Unit;"] + app("Unit"),
    "a/tuple.rs": ["", "pub struct T(u8);"] + app("T"),
    "a/empty.rs": ["struct Empty { /* none */ }"] + app("Empty"),
    "a/nomutation.rs": ["struct Half { a: u8 }", "impl DocumentApp for Half {", "    type Snapshot = S;", "}"],
    "a/tested.rs": ["#[cfg(test)]", "mod tests {", "    struct X { a: u8 }"] + ["    " + l for l in app("X")] + ["}"],
    "🌎hub/stateful.rs": ["pub struct Counter { value: u32 }"] + app("Counter"),
}, expect=[b("document-app-shape-a/stateful.rs-Counter", DA, 1), b("document-app-shape-a/tuple.rs-T", DA, 2)])
case("pack-files-are-forbidden", "policyNoPackFilesBreaches", {"x.pack": [], "a/b.pack": [], "a/c.txt": [], "node_modules/d.pack": []},
     expect=[b("no-pack-file-x.pack", "pack-migration/no-pack-file"), b("no-pack-file-a/b.pack", "pack-migration/no-pack-file")])
RS = "budget/no-raw-spawn"
spawn_ts = [
    'import { spawnSync } from "node:child_process";',
    'const r = spawnSync("git", ["x"]);',
    'execSync("ls"); Bun.spawn(["a"]); spawn("b")',
    '// spawnSync("x")',
    'const s = "spawnSync(x)";',
    "const t = `spawn(${x})`;",
    "child.spawn(a);",
]
ROOTMOD = "🧰framework/🛍products/🦑repo/🔨modules/📚library/🎛commands/x/🟦.ts"
PKGMOD = "🧰framework/🛍products/💻os/📦packages/🦀rust/🎛commands/y/🟦.ts"
case("raw-spawn-ignores-comments-and-strings", "policyRawSpawnBreaches", {
    "📜script.ts": spawn_ts, ROOTMOD: ["spawnSync(1);"], PKGMOD: ["execSync(2);"], "other/🟦.ts": ["spawnSync(1);"], "sub/📜script.ts": ["spawn(1);"],
    "🧰framework/🛍products/🦑repo/🔨modules/📚library/🎛commands/x/🧪tests/🟦.ts": ["spawnSync(1);"],
}, expect=[b("no-raw-spawn-📜script.ts-2", RS, 2), b("no-raw-spawn-📜script.ts-3", RS, 3), b("no-raw-spawn-📜script.ts-7", RS, 7), b("no-raw-spawn-%s-1" % ROOTMOD, RS, 1), b("no-raw-spawn-%s-1" % PKGMOD, RS, 1), b("no-raw-spawn-sub/📜script.ts-1", RS, 1)])
case("raw-spawn-line-numbers-survive-block-comments", "policyRawSpawnBreaches", {"📜script.ts": ["/* c */", "spawnSync(1);"]},
     expect=[b("no-raw-spawn-📜script.ts-2", RS, 2)], defect="🟦.ts:365-369 drops the comment delimiters from the stripped text, so offsets taken from it no longer index the original content and the reported line shifts")
BN = "budget/no-budget-null"
case("budget-null-literals", "policyBudgetNullBreaches", {"📜script.ts": ["run({ budgetMs: null });", "run({ budgetMs:null, budgetMs: null });", "run({ budgetMs: 5 });"], "other/🟦.ts": ["budgetMs: null"]},
     expect=[b("no-budget-null-📜script.ts-1", BN, 1), b("no-budget-null-📜script.ts-2", BN, 2)])
for id_, src, out in [
    ("line-comments-strings-and-templates-are-blanked", 'a // c\nb "x" \'y\' `z${1}` c', "a     \nb" + " " * 17 + "c"),
    ("escapes-stay-inside-their-string", 'p "a\\"b" q', "p " + " " * 6 + " q"),
]:
    case("strip-" + id_, "policyStripTsCommentsAndStrings", {}, input=src, expectList=[out])
case("strip-block-comments-keep-their-length", "policyStripTsCommentsAndStrings", {}, input="a /* c */ b", expectList=["a" + " " * 9 + "b"],
     defect="🟦.ts:363-370 skips the two delimiter characters on both sides without emitting blanks, so a one-line block comment shrinks by four characters")

rules = ["policyDiscoverCargoTomlFiles", "policyDbServerOnlyBreaches", "policyOsStateAuthorityBreaches", "policyDocumentAppShapeBreaches", "policyNoPackFilesBreaches", "policyRawSpawnBreaches", "policyBudgetNullBreaches", "policyStripTsCommentsAndStrings"]
schema = std_schema("Authority Bans Law Corpus", rules, {"input": {"type": "string"}})
ts = std_ts(
    "🚧 The authority-ban laws report exactly the breaches their language-agnostic corpus names over synthetic repositories, agree with an independent Python model, and two recorded comment-stripping defects are pinned as failing rows.",
    'import { policyBudgetNullBreaches, policyDbServerOnlyBreaches, policyDiscoverCargoTomlFiles, policyDocumentAppShapeBreaches, policyNoPackFilesBreaches, policyOsStateAuthorityBreaches, policyRawSpawnBreaches, policyStripTsCommentsAndStrings } from "../../🟦.ts";',
    '''  policyDiscoverCargoTomlFiles: (root) => policyDiscoverCargoTomlFiles(root),
  policyDbServerOnlyBreaches: (root) => policyDbServerOnlyBreaches(root),
  policyOsStateAuthorityBreaches: (root) => policyOsStateAuthorityBreaches(root),
  policyDocumentAppShapeBreaches: (root) => policyDocumentAppShapeBreaches(root),
  policyNoPackFilesBreaches: (root) => policyNoPackFilesBreaches(root),
  policyRawSpawnBreaches: (root) => policyRawSpawnBreaches(root),
  policyBudgetNullBreaches: (root) => policyBudgetNullBreaches(root),
  policyStripTsCommentsAndStrings: (_root, row) => [policyStripTsCommentsAndStrings(row.input)],''',
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🚧authority-bans", "🚧bans", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
