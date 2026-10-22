import json, os, sys
import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🧰testkit"
HERE = os.path.dirname(os.path.abspath(__file__))
GOLD = os.path.join(HERE, "u47_goldens.json")
gold = json.load(open(GOLD, encoding="utf8")) if os.path.exists(GOLD) else {"wiring": []}
keys = [("serde = \"1\"", "serde"), ("  tempfile = { version = \"3\" }", "tempfile"), ("serde.workspace = true", "serde"), ("my-crate.path = \"../x\"", "my-crate"), ("a_b = 1", "a_b"), ("# serde = 1", ""), ("[dependencies]", ""), ("", ""), ("1abc = 2", "1abc"), ("a.b.c = 1", ""), ("x=1", "x")]
key_rows = [{"row": r, "key": k, "ident": k.replace("-", "_")} for r, k in keys]
cfg = [
 ("cfg(test)", [], [], True), ('cfg(any(test, feature = "testkit"))', ["testkit"], ["testkit"], True), ('cfg(feature = "testkit")', ["testkit"], ["testkit"], True), ('cfg(not(feature = "testkit"))', [], [], False),
 ('cfg(all(test, feature = "x"))', [], ["x"], False), ('cfg(any(test, feature = "x"))', ["x"], ["x"], True), ('cfg(any(feature = "x", feature = "y", test))', ["x", "y"], ["x", "y"], True), ('cfg(any(feature = "x", feature = "y"))', [], ["x", "y"], False),
 ('cfg(all(feature = "testkit", unix))', ["testkit"], ["testkit"], True), ('cfg(any(test, not(feature = "x")))', [], [], False), ('cfg(not(any(test, feature = "x")))', [], [], False), ('cfg(target_os = "linux")', [], [], False), ("cfg_attr(test, derive(Debug))", [], [], False),
 (' cfg ( any ( test , feature = "a" ) ) ', ["a"], ["a"], True), ('cfg(any(test, feature = "testkit", feature = "other"))', ["testkit", "other"], ["testkit", "other"], True), ('cfg(any(test, feature = "testkit"', [], [], False), ("cfg()", [], [], False), ('cfg(feature="x")', [], ["x"], False),
 ('cfg(all(not(test), feature = "testkit"))', ["testkit"], ["testkit"], True), ('cfg(any(test, all(feature = "z", unix)))', [], ["z"], False), ('cfg(any(all(test), feature = "q"))', [], ["q"], False), ("derive(Debug)", [], [], False), ("", [], [], False), ("cfg(test)  ", [], [], True),
]
cfg_rows = [{"attribute": a, "gate": g, "cfg": c, "testOnly": t} for a, g, c, t in cfg]
only = [{"attributes": a, "expected": e} for a, e in [(["inline(always)", "cfg(test)"], "cfg(test)"), ([], None), (["cfg( test )"], "cfg( test )"), (['cfg(any(test, feature = "testkit"))', "cfg(test)"], 'cfg(any(test, feature = "testkit"))'), (['cfg(feature = "x")'], None), (["cfg(not(test))"], None), (['cfg(feature = "testkit")'], 'cfg(feature = "testkit")')]]
G = lambda f, t, names, method=False, owner=None, use=False: {"features": f, "text": t, "names": names, "method": method, "owner": owner, "use": use}
A = '#[cfg(any(test, feature = "testkit"))]'
K = '#[cfg(feature = "testkit")]'
gates = [
 {"name": "a gated public fn", "text": A + "\npub fn make() -> u8 { 1 }\nfn other() {}\n", "gates": [G(["testkit"], A + "\npub fn make() -> u8 { 1 }", ["make"])], "defect": ""},
 {"name": "an inherent method names its owner", "text": "pub struct S;\nimpl S {\n    " + K + "\n    pub fn probe(&self) {}\n    pub fn real(&self) {}\n}\n", "gates": [G(["testkit"], K + "\n    pub fn probe(&self) {}", ["probe"], True, "S")], "defect": ""},
 {"name": "a re-export lists its leaf names", "text": K + "\npub use crate::inner::{A, B as C};\n", "gates": [G(["testkit"], K + "\npub use crate::inner::{A, B as C};", ["A", "C"], False, None, True)], "defect": ""},
 {"name": "a private use is a use without names", "text": K + "\nuse crate::inner::A;\n", "gates": [G(["testkit"], K + "\nuse crate::inner::A;", [], False, None, True)], "defect": ""},
 {"name": "a private fn declares no exported name", "text": K + "\nfn hidden() {}\n", "gates": [G(["testkit"], K + "\nfn hidden() {}", [])], "defect": ""},
 {"name": "a gated macro names itself", "text": K + "\nmacro_rules! mk { () => {} }\n", "gates": [G(["testkit"], K + "\nmacro_rules! mk { () => {} }", ["mk"])], "defect": ""},
 {"name": "further attributes belong to the item", "text": K + "\n#[derive(Debug)]\npub struct P { x: u8 }\n", "gates": [G(["testkit"], K + "\n#[derive(Debug)]\npub struct P { x: u8 }", ["P"])], "defect": ""},
 {"name": "a trait method without pub names nothing", "text": "pub trait T {\n    " + K + "\n    fn probe(&self);\n}\n", "gates": [G(["testkit"], K + "\n    fn probe(&self);", [], True, "T")], "defect": ""},
 {"name": "a gated static mut", "text": K + "\npub static mut X: u8 = 0;\n", "gates": [G(["testkit"], K + "\npub static mut X: u8 = 0;", ["X"])], "defect": ""},
 {"name": "a restricted visibility exports nothing", "text": K + "\npub(crate) fn inner() {}\n", "gates": [G(["testkit"], K + "\npub(crate) fn inner() {}", [])], "defect": ""},
 {"name": "qualifiers before the keyword are skipped", "text": K + "\npub const fn build() {}\n" + K + "\npub async unsafe fn later() {}\n" + A + "\npub extern \"C\" fn callback() {}\n", "gates": [G(["testkit"], K + "\npub const fn build() {}", ["build"]), G(["testkit"], K + "\npub async unsafe fn later() {}", ["later"]), G(["testkit"], A + "\npub extern \"C\" fn callback() {}", ["callback"])], "defect": ""},
 {"name": "a gated module", "text": K + "\npub mod helpers { pub fn a() {} }\n" + K + "\nmod private_helpers { pub fn a() {} }\n", "gates": [G(["testkit"], K + "\npub mod helpers { pub fn a() {} }", ["helpers"]), G(["testkit"], K + "\nmod private_helpers { pub fn a() {} }", [])], "defect": ""},
 {"name": "a gated statement", "text": "fn f() {\n    " + K + "\n    let x = 1;\n}\n", "gates": [G(["testkit"], K + "\n    let x = 1;", [])], "defect": ""},
 {"name": "test-only and non-gate attributes are not gates", "text": "#[cfg(test)]\nfn t() {}\n#[cfg(feature = \"x\")]\npub fn y() {}\n#[cfg(all(test, feature = \"x\"))]\npub fn z() {}\n", "gates": [], "defect": ""},
 {"name": "a generic trait impl names the implementing type", "text": "impl<T: Clone> Trait<T> for Wrapper<T> {\n    " + K + "\n    pub fn probe(&self) {}\n}\n", "gates": [G(["testkit"], K + "\n    pub fn probe(&self) {}", ["probe"], True, "Wrapper")], "defect": ""},
 {"name": "text without any cfg is skipped", "text": "pub fn plain() {}\n", "gates": [], "defect": ""},
 {"name": "a gated type alias and constant", "text": K + "\npub type Alias = u8;\n" + K + "\npub const LIMIT: u8 = 3;\n", "gates": [G(["testkit"], K + "\npub type Alias = u8;", ["Alias"]), G(["testkit"], K + "\npub const LIMIT: u8 = 3;", ["LIMIT"])], "defect": ""},
 {"name": "a gated impl block", "text": K + "\nimpl S { pub fn m(&self) {} }\n", "gates": [G(["testkit"], K + "\nimpl S { pub fn m(&self) {} }", [])], "defect": ""},
]
decl = [
 ("pub fn a() {}\nstruct B;\nenum C { X }\ntrait D {}\ntype E = u8;\nconst F: u8 = 1;\nstatic G: u8 = 2;\nmod h {}\nunion I { a: u8 }\nmacro_rules! j { () => {} }\n", ["a", "B", "C", "D", "E", "F", "G", "h", "I", "j"], ""),
 ("static mut COUNTER: u8 = 0;\nconst fn g() {}\npub unsafe fn u() {}\nextern \"C\" fn cb() {}\n", ["COUNTER", "g", "u", "cb"], ""),
 ("impl S {\n    fn method() {}\n    const K: u8 = 1;\n    type T = u8;\n}\ntrait Tr {\n    fn m();\n    type A;\n    const C: u8;\n}\n", ["Tr"], ""),
 ("mod outer {\n    pub fn inner() { fn local() {} const L: u8 = 1; }\n    pub mod deeper { pub struct D; }\n}\n", ["outer", "inner", "local", "L", "deeper", "D"], ""),
 ("pub(crate) struct S<T>(T);\nstruct R<'a> { x: &'a u8 }\n", ["S", "R"], ""),
 ("fn f<const N: usize>() {}\nstruct W<const M: usize>;\nimpl<const K: usize> W<K> {}\n", ["f", "W"], "F30"),
 ("fn main() { let r = x.type; let s = a::fn_name; }\n", ["main"], ""),
 ("", [], ""),
]
decl_rows = [{"text": t, "expected": sorted(e), "defect": d} for t, e, d in decl]
manifest = '[package]\nname = "x"\n\n[dependencies]\nserde = "1"\nserde_json = { version = "1", features = ["std"] }\nmy-crate = { path = "../my-crate", features = ["derive"] }\nwork.workspace = true\n\n[dev-dependencies]\ntempfile = "3"\nserde = { version = "1", features = ["derive", "rc"] }\nserde_json = { version = "1", features = ["std", "preserve_order"] }\nmy-crate = { path = "../my-crate", features = ["derive", "testkit"] }\ndev-only.workspace = true\n'
ext = [{"name": "normal, dev-only and dev-features", "manifest": manifest, "dependencies": ['serde = "1"', 'serde_json = { version = "1", features = ["std"] }', 'my-crate = { path = "../my-crate", features = ["derive"] }', "work.workspace = true"], "devDependencies": ['tempfile = "3"', 'serde = { version = "1", features = ["derive", "rc"] }', 'serde_json = { version = "1", features = ["std", "preserve_order"] }', 'my-crate = { path = "../my-crate", features = ["derive", "testkit"] }', "dev-only.workspace = true"]},
 {"name": "no dependency tables", "manifest": '[package]\nname = "x"\n', "dependencies": [], "devDependencies": []},
 {"name": "rows are optional when only dev rows are known", "manifest": '[dev-dependencies]\nonly = "1"\n', "devDependencies": ['only = "1"']},
]
for e in ext:
    e.setdefault("dependencies", [])
    e.setdefault("devDependencies", [])
opt = [("serde = \"1\"", 'serde = { version = "1", optional = true }', ""), ("serde.workspace = true", "serde = { workspace = true, optional = true }", ""), ('serde = { version = "1" }', 'serde = { version = "1", optional = true }', ""), ('serde = { version = "1", optional = true }', 'serde = { version = "1", optional = true }', ""),
 ('  indented = "2"', '  indented = { version = "2", optional = true }', ""), ('x = { path = "../x" }  ', 'x = { path = "../x", optional = true }', ""), ('x = { workspace = true, features = ["a"] }', 'x = { workspace = true, features = ["a"], optional = true }', ""),
 ('x = { version = "1", default-features = false, features = ["a", "b"] }', 'x = { version = "1", default-features = false, features = ["a", "b"], optional = true }', ""), ('x = { version = "1", optional=true }', 'x = { version = "1", optional=true }', ""),
 ('serde = "1" # pinned', 'serde = { version = "1", optional = true } # pinned', "F31"), ("serde = '1'", "serde = { version = '1', optional = true }", "F31")]
opt_rows = [{"row": r, "expected": e, "defect": d} for r, e, d in opt]
req = [{"name": "an integration test needs the gate features its code names", "targetTokens": ["make", "other"], "gated": {"testkit": ["make", "build"], "extra": ["zzz"], "also": ["other"]}, "expected": ["also", "testkit"]}, {"name": "nothing gated", "targetTokens": ["a"], "gated": {}, "expected": []}, {"name": "no names in common", "targetTokens": ["a"], "gated": {"testkit": ["make"]}, "expected": []}]
T1 = '[package]\nname = "x"\n\n[[test]]\nname = "t1"\npath = "tests/t1.rs"\n\n[[test]]\nname = "t2"\nrequired-features = ["a"]\npath = "tests/t2.rs"\n\n[lib]\npath = "src/lib.rs"\n'
with_req = [
 {"name": "a missing row is appended after the last line; an existing one is extended", "manifest": T1, "byTarget": {"t1": ["testkit"], "t2": ["testkit", "a"], "ghost": ["x"]}, "expected": T1.replace('path = "tests/t1.rs"\n', 'path = "tests/t1.rs"\nrequired-features = ["testkit"]\n').replace('required-features = ["a"]', 'required-features = ["a", "testkit"]'), "defect": ""},
 {"name": "no targets change nothing", "manifest": T1, "byTarget": {}, "expected": T1, "defect": ""},
 {"name": "an empty feature list changes nothing", "manifest": T1, "byTarget": {"t1": []}, "expected": T1, "defect": ""},
 {"name": "a section at the end of the file", "manifest": '[[test]]\nname = "t"\npath = "p.rs"', "byTarget": {"t": ["k"]}, "expected": '[[test]]\nname = "t"\npath = "p.rs"\nrequired-features = ["k"]', "defect": ""},
 {"name": "a target without a name is left alone", "manifest": '[[test]]\npath = "p.rs"\n', "byTarget": {"t": ["k"]}, "expected": '[[test]]\npath = "p.rs"\n', "defect": ""},
 {"name": "an existing feature is not repeated", "manifest": '[[test]]\nname = "t"\nrequired-features = ["k"]\n', "byTarget": {"t": ["k"]}, "expected": '[[test]]\nname = "t"\nrequired-features = ["k"]\n', "defect": ""},
 {"name": "a multi-line array is extended as one row", "manifest": '[[test]]\nname = "t3"\nrequired-features = [\n    "a",\n]\npath = "x"\n', "byTarget": {"t3": ["testkit"]}, "expected": '[[test]]\nname = "t3"\nrequired-features = ["a", "testkit"]\npath = "x"\n', "defect": "F32"},
 {"name": "other sections between tests keep their place", "manifest": '[[test]]\nname = "t"\n\n[dependencies]\nx = "1"\n', "byTarget": {"t": ["k"]}, "expected": '[[test]]\nname = "t"\nrequired-features = ["k"]\n\n[dependencies]\nx = "1"\n', "defect": ""},
]
for w in with_req:
    w["byTarget"] = {k: v for k, v in w["byTarget"].items()}
feat_rows = [{"feature": {"name": "testkit", "entries": ["dep:tempfile", "a/testkit"]}, "expected": 'testkit = ["dep:tempfile", "a/testkit"]', "defect": ""}, {"feature": {"name": "empty", "entries": []}, "expected": "empty = []", "defect": ""}, {"feature": {"name": "q", "entries": ['x"y']}, "expected": 'q = ["x\\"y"]', "defect": ""}, {"feature": {"name": "a+b", "entries": ["x"]}, "expected": '"a+b" = ["x"]', "defect": "F33"}]
FM = '[package]\nname = "x"\n\n[features]\ntestkit = ["old"]\nkeep = ["k"]\n\n[dependencies]\nserde = "1"\n'
with_feat = [
 {"name": "an existing row is replaced in place", "manifest": FM, "features": [{"name": "testkit", "entries": ["dep:t", "a/testkit"]}], "expected": FM.replace('testkit = ["old"]', 'testkit = ["dep:t", "a/testkit"]')},
 {"name": "a new row joins the existing section", "manifest": FM, "features": [{"name": "fresh", "entries": ["x"]}], "expected": FM.replace("[features]\n", '[features]\nfresh = ["x"]\n')},
 {"name": "a missing section is created before dependencies", "manifest": '[package]\nname = "x"\n\n[dependencies]\nserde = "1"\n', "features": [{"name": "testkit", "entries": []}], "expected": '[package]\nname = "x"\n\n[features]\ntestkit = []\n\n[dependencies]\nserde = "1"\n'},
 {"name": "a missing section goes to the end without dependencies", "manifest": '[package]\nname = "x"\n', "features": [{"name": "testkit", "entries": ["a/testkit"]}], "expected": '[package]\nname = "x"\n\n[features]\ntestkit = ["a/testkit"]\n'},
 {"name": "a multi-line array is replaced as a whole", "manifest": '[features]\ntestkit = [\n    "a",\n    "b",\n]\nother = []\n', "features": [{"name": "testkit", "entries": ["c"]}], "expected": '[features]\ntestkit = ["c"]\nother = []\n'},
 {"name": "several features keep their order", "manifest": '[dependencies]\n', "features": [{"name": "b", "entries": []}, {"name": "a", "entries": ["x"]}], "expected": '[features]\na = ["x"]\nb = []\n\n[dependencies]\n'},
 {"name": "a feature name that is not a bare key is quoted", "manifest": '[features]\nab = ["keep"]\na_b = ["keep2"]\n', "features": [{"name": "a.b", "entries": ["z"]}], "expected": '[features]\n"a.b" = ["z"]\nab = ["keep"]\na_b = ["keep2"]\n', "defect": "F33"},
]
for w in with_feat:
    w.setdefault("defect", "")
CRATES = lambda *c: list(c)
def crate(name, deps=(), dev=(), owned=None, files=None, ff=None, raw=False):
    pre = (lambda p: p) if raw else (lambda p: name + "/" + p)
    return {"name": name, "ident": name.replace("-", "_"), "dependsOn": list(deps), "devDependsOn": list(dev), "owned": None if owned is None else [pre(p) for p in owned], "files": {pre(k): v for k, v in (files or {}).items()}, "featureFiles": {k: [pre(p) for p in v] for k, v in (ff or {}).items()}}
def ex(key, dependency=True, devOnly=False, devFeatures=(), devRow=None, row=None):
    return {"key": key, "ident": key.replace("-", "_"), "dependency": dependency, "devOnly": devOnly, "devFeatures": list(devFeatures), "devRow": devRow, "row": row}
GATE = '#[cfg(any(test, feature = "testkit"))]\n'
wiring = [
 {"name": "a dev-only external used by gated code becomes an optional dependency", "crates": [crate("a", files={"src/lib.rs": GATE + 'pub fn make() { tempfile::tempdir(); }\n'})], "externals": [ex("tempfile", False, True, [], 'tempfile = "3"', 'tempfile = "3"')], "declared": []},
 {"name": "a gate feature is forwarded to the dependency whose gated names the code uses", "crates": [crate("a", files={"src/lib.rs": GATE + "pub fn make() {}\n"}), crate("b", deps=["a"], files={"src/lib.rs": GATE + "pub fn run() { a::make(); }\n"})], "externals": [], "declared": []},
 {"name": "a consumer that names a gated item without gating its own code consumes the feature", "crates": [crate("a", files={"src/lib.rs": GATE + "pub fn make() {}\n"}), crate("b", deps=["a"], files={"src/lib.rs": "pub fn run() { a::make(); }\n"})], "externals": [], "declared": []},
 {"name": "a toolkit dependency with dev features is forwarded when gated code needs its gated names", "crates": [crate("a", files={"src/lib.rs": GATE + "pub fn make() { serde_json::to_string(&1); }\n"})], "externals": [ex("serde_json", True, False, ["preserve_order"], 'serde_json = { version = "1", features = ["preserve_order"] }', 'serde_json = "1"')], "declared": []},
 {"name": "declared features are not test gates", "crates": [crate("a", files={"src/lib.rs": GATE + "pub fn make() {}\n"})], "externals": [], "declared": ["testkit"]},
 {"name": "a gated method counts when its owner is also named", "crates": [crate("a", files={"src/lib.rs": "pub struct Probe;\nimpl Probe {\n    " + GATE + "    pub fn peek(&self) {}\n}\n"}), crate("b", deps=["a"], files={"src/lib.rs": "pub fn run(p: a::Probe) { p.peek(); }\n"})], "externals": [], "declared": []},
 {"name": "a gated method alone without its owner is not consumed", "crates": [crate("a", files={"src/lib.rs": "pub struct Probe;\nimpl Probe {\n    " + GATE + "    pub fn peek(&self) {}\n}\n"}), crate("b", deps=["a"], files={"src/lib.rs": "pub fn run() { peek(); }\n"})], "externals": [], "declared": []},
 {"name": "a re-export behind a gate is consumed through its root", "crates": [crate("a", files={"src/lib.rs": '#[cfg(feature = "testkit")]\npub use crate::inner::Helper;\n#[cfg(feature = "testkit")]\npub fn probe() {}\n'}), crate("b", deps=["a"], files={"src/lib.rs": "use a::Helper;\nfn f(_: Helper) {}\n"})], "externals": [], "declared": []},
 {"name": "a file mounted by two crates belongs to the crate whose plan owns it", "crates": [crate("a", owned=["src/shared.rs"], files={"src/shared.rs": GATE + "pub fn shared() {}\n"}, raw=True), crate("b", deps=["a"], owned=[], files={"src/shared.rs": GATE + "pub fn shared() {}\n"}, raw=True)], "externals": [], "declared": []},
 {"name": "a file mounted by two crates belongs to the one that depends on no other holder", "crates": [crate("a", files={"src/shared.rs": GATE + "pub fn shared() {}\n"}, raw=True), crate("b", deps=["a"], files={"src/shared.rs": GATE + "pub fn shared() {}\n"}, raw=True)], "externals": [], "declared": []},
 {"name": "files the closure compiles only with the feature count as gated code", "crates": [crate("a", files={"src/lib.rs": GATE + "pub fn make() {}\n", "src/extra.rs": "pub fn other() { tempfile::tempdir(); }\n"}, ff={"testkit": ["src/extra.rs"]})], "externals": [ex("tempfile", False, True, [], 'tempfile = "3"', 'tempfile = "3"')], "declared": []},
 {"name": "an extern crate alias resolves to the dependency", "crates": [crate("a", files={"src/lib.rs": "extern crate tempfile as tf;\n" + GATE + "pub fn make() { tf::tempdir(); }\n"})], "externals": [ex("tempfile", False, True, [], 'tempfile = "3"', 'tempfile = "3"')], "declared": []},
 {"name": "a crate without gates needs no wiring", "crates": [crate("a", files={"src/lib.rs": "pub fn plain() {}\n"})], "externals": [], "declared": []},
 {"name": "dev dependencies carry the consumed features too", "crates": [crate("a", files={"src/lib.rs": GATE + "pub fn make() {}\n"}), crate("b", dev=["a"], files={"src/lib.rs": "pub fn run() { a::make(); }\n"})], "externals": [], "declared": []},
]
for i, w in enumerate(wiring):
    w["expect"] = gold["wiring"][i] if i < len(gold["wiring"]) else {}
build = {"files": {"src/a.rs": GATE + "pub fn make() {}\n" + '#[cfg(feature = "other")]\npub fn thing() {}\n', "src/b.rs": "pub fn plain() {}\n", "src/c.rs": '#[cfg(feature = "testkit")]\nuse x::Y;\n'}, "closures": {"testkit": ["src/a.rs", "src/extra.rs"]}}
if "--cases" in sys.argv:
    json.dump({"wiring": wiring}, open(os.path.join(HERE, "..", "scratch", "u47_cases.json"), "w", encoding="utf8"), ensure_ascii=False)
    sys.exit(0)
text = S.str
gate_s = S.obj({"features": S.strs(1), "text": text, "names": S.strs(), "method": S.bool, "owner": S.one(S.null, text), "use": S.bool})
wiring_expect = S.map(S.obj({"features": S.arr(S.obj({"name": text, "entries": S.strs()})), "optionalDependencies": S.strs(), "consumed": S.map(S.strs()), "gatedNames": S.map(S.strs())}))
ext_s = S.obj({"key": text, "ident": text, "dependency": S.bool, "devOnly": S.bool, "devFeatures": S.strs(), "devRow": S.one(S.null, text), "row": S.one(S.null, text)})
feat_s = S.obj({"name": text, "entries": S.strs()})
schema = S.obj({"version": S.const(1), "keys": S.arr(S.obj({"row": text, "key": text, "ident": text}), 1), "cfg": S.arr(S.obj({"attribute": text, "gate": S.strs(), "cfg": S.strs(), "testOnly": S.bool}), 1), "only": S.arr(S.obj({"attributes": S.strs(), "expected": S.one(S.null, text)}), 1),
 "gates": S.arr(S.obj({"name": text, "text": text, "gates": S.arr(gate_s), "defect": text}), 1), "declared": S.arr(S.obj({"text": text, "expected": S.strs(), "defect": text}), 1),
 "externals": S.arr(S.obj({"name": text, "manifest": text, "dependencies": S.strs(), "devDependencies": S.strs()}), 1), "optional": S.arr(S.obj({"row": text, "expected": text, "defect": text}), 1),
 "required": S.arr(S.obj({"name": text, "targetTokens": S.strs(), "gated": S.map(S.strs()), "expected": S.strs()}), 1), "withRequired": S.arr(S.obj({"name": text, "manifest": text, "byTarget": S.map(S.strs()), "expected": text, "defect": text}), 1),
 "featureRows": S.arr(S.obj({"feature": feat_s, "expected": text, "defect": text}), 1), "withFeatures": S.arr(S.obj({"name": text, "manifest": text, "features": S.arr(feat_s), "expected": text, "defect": text}), 1),
 "wiring": S.arr(S.obj({"name": text, "crates": S.arr(S.obj({"name": text, "ident": text, "dependsOn": S.strs(), "devDependsOn": S.strs(), "owned": S.one(S.null, S.strs()), "files": S.map(text), "featureFiles": S.map(S.strs())})), "externals": S.arr(ext_s), "declared": S.strs(), "expect": wiring_expect}), 1),
 "build": S.obj({"files": S.map(text), "closures": S.map(S.strs())})})
corpus = {"version": 1, "keys": key_rows, "cfg": cfg_rows, "only": only, "gates": gates, "declared": decl_rows, "externals": ext, "optional": opt_rows, "required": req, "withRequired": with_req, "featureRows": feat_rows, "withFeatures": with_feat, "wiring": wiring, "build": build}
body = open(os.path.join(HERE, "u47_body.ts"), encoding="utf8").read()
ORACLE = open(os.path.join(HERE, "u47_oracle.py"), encoding="utf8").read()
oracle = lib.PY_HEAD.replace("{doc}", "Independent Python model of cfg predicates (test-support gate features and every required feature) over the corpus.") + ORACLE + lib.PY_TAIL
ts = lib.ts_wrap(UNIT, "🧰️ Contract: the testkit module reads dependency row keys, classifies `cfg(...)` attributes into test-support gates and required features, scans a file for gated items, statements and methods, lists declared names, reads the external dependencies of a manifest, makes a dependency row optional, plans the per-crate feature wiring of a carve (gate features, optional dependencies, forwarded and consumed features) and edits manifests (`required-features` of tests, `[features]` rows); the corpus is language-agnostic, an independent Python model evaluates the cfg predicates, tree-sitter's Rust grammar checks the gated spans and declared names and `@iarna/toml` checks every edited or read manifest.",
 'import Parser from "web-tree-sitter";\nimport {parse} from "@iarna/toml";\nimport {createRequire} from "node:module";\nimport {dependencyIdent,dependencyRowKey,buildTestkitCrate,cfgFeatures,declaredNames,externalDependencies,optionalDependencyRow,planTestkitWiring,requiredTestFeatures,scanTestGates,testGateFeatures,testOnlyGate,testkitFeatureRow,withFeatureRows,withRequiredFeatures,type TestkitCrate,type ExternalDependency} from "@@' + UNIT + '/🟦.ts@@";', body, python=True)
lib.write_unit(UNIT, corpus, schema, ts, oracle)
