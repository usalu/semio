import json, os
import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🔬verify"
HERE = os.path.dirname(os.path.abspath(__file__))
V = "️"
MIRROR = "\U0001FA9E" + V + "mirror/\U0001F980" + V + ".rs"
def M(path, file, inline=False, cfg=False, vis="private"):
    return {"path": path, "file": file, "inline": inline, "cfgTest": cfg, "visibility": vis}
resolve = [
 {"name": "mounts, components, inline modules and unresolved modules", "root": "src/lib.rs", "files": {
   "src/lib.rs": '#[path = "a/mod.rs"]\npub mod a;\n#[cfg(test)]\n#[path = "t.rs"]\nmod t;\nmod inl {\n    #[path = "deep.rs"]\n    pub(crate) mod d;\n    mod plain;\n}\n#[path = "comp.rs"]\nmod component;\npub use component::*;\n',
   "src/a/mod.rs": '#[path = "n.rs"]\nmod nested;\n', "src/a/n.rs": "pub fn n() {}\n", "src/t.rs": "fn t() {}\n", "src/inl/deep.rs": "fn d() {}\n", "src/comp.rs": '#[path = "k.rs"]\npub mod k;\n', "src/k.rs": "fn k() {}\n"},
  "expected": [M("a", "src/a/mod.rs", vis="pub"), M("a::nested", "src/a/n.rs"), M("t", "src/t.rs", cfg=True), M("inl", None, True), M("inl::d", "src/inl/deep.rs", vis="pub(crate)"), M("inl::plain", "unresolved"), M("::<component>", "src/comp.rs"), M("k", "src/k.rs", vis="pub")], "error": ""},
 {"name": "the mirror module of the root is walked through transparently", "root": "src/lib.rs", "files": {"src/lib.rs": '#[path = "' + MIRROR + '"]\nmod mirror;\npub mod x;\n', "src/" + MIRROR: '#[path = "m.rs"]\npub mod m;\n', "src/" + MIRROR.rsplit("/", 1)[0] + "/m.rs": "fn m() {}\n"},
  "expected": [M("m", "src/" + MIRROR.rsplit("/", 1)[0] + "/m.rs", vis="pub"), M("x", "unresolved", vis="pub")], "error": ""},
 {"name": "a mounted file that does not exist is an error", "root": "src/lib.rs", "files": {"src/lib.rs": '#[path = "gone.rs"]\nmod gone;\n'}, "expected": [], "error": "Missing module file src/gone.rs"},
 {"name": "path attributes of inline modules move the directory of their children", "root": "src/lib.rs", "files": {"src/lib.rs": '#[path = "d"]\nmod grp {\n    #[path = "f.rs"]\n    mod g;\n}\n', "src/d/f.rs": "fn g() {}\n"}, "expected": [M("grp", None, True), M("grp::g", "src/d/f.rs")], "error": ""},
 {"name": "cfg(test) propagates to every descendant", "root": "src/lib.rs", "files": {"src/lib.rs": "#[cfg(test)]\nmod t {\n    mod u;\n    #[path = \"x.rs\"]\n    mod v;\n}\n", "src/t/x.rs": "fn v() {}\n"}, "expected": [M("t", None, True, True), M("t::u", "unresolved", False, True), M("t::v", "src/t/x.rs", False, True)], "error": ""},
 {"name": "a file module inside a file module resolves against its own file", "root": "src/lib.rs", "files": {"src/lib.rs": '#[path = "p/q.rs"]\nmod q;\n', "src/p/q.rs": '#[path = "r.rs"]\npub mod r;\n', "src/p/r.rs": "fn r() {}\n"}, "expected": [M("q", "src/p/q.rs"), M("q::r", "src/p/r.rs", vis="pub")], "error": ""},
 {"name": "an empty crate has no modules", "root": "src/lib.rs", "files": {"src/lib.rs": "pub fn f() {}\n"}, "expected": [], "error": ""},
]
def mod(path, file, inline=False, cfg=False):
    return {"path": path, "file": file, "inline": inline, "cfgTest": cfg, "visibility": "pub"}
a, b, c = mod("a", "a.rs"), mod("b", "b.rs"), mod("c", "c.rs")
compare = [
 {"name": "an exact split", "original": [a, b], "carved": [[a], [b]], "problems": [], "originalCount": 2, "unionCount": 2},
 {"name": "a missing module", "original": [a, b], "carved": [[a]], "problems": ["module b (b.rs) is missing from the carved crates"], "originalCount": 2, "unionCount": 1},
 {"name": "a changed file", "original": [a], "carved": [[mod("a", "x.rs")]], "problems": ["module a changed: a.rs -> x.rs"], "originalCount": 1, "unionCount": 1},
 {"name": "an extra module", "original": [a], "carved": [[a], [c]], "problems": ["module c appears only in the carved crates"], "originalCount": 1, "unionCount": 2},
 {"name": "a module declared twice with different files", "original": [a], "carved": [[a], [mod("a", "z.rs")]], "problems": ["module a is declared twice: a.rs and z.rs"], "originalCount": 1, "unionCount": 1},
 {"name": "a module declared twice identically is fine", "original": [a], "carved": [[a], [a]], "problems": [], "originalCount": 1, "unionCount": 1},
 {"name": "a cfg(test) change", "original": [a], "carved": [[mod("a", "a.rs", False, True)]], "problems": ["module a changed: a.rs -> a.rs"], "originalCount": 1, "unionCount": 1},
 {"name": "inline modules need only exist somewhere", "original": [mod("m", None, True), a], "carved": [[mod("m", None, True)], [a]], "problems": [], "originalCount": 1, "unionCount": 1},
 {"name": "a missing inline module", "original": [mod("m", None, True), a], "carved": [[a]], "problems": ["inline module m is missing"], "originalCount": 1, "unionCount": 1},
 {"name": "an inline module that became a file module", "original": [mod("m", None, True)], "carved": [[mod("m", "m.rs")]], "problems": ["module m appears only in the carved crates"], "originalCount": 0, "unionCount": 1},
 {"name": "several problems keep their order", "original": [a, b, c], "carved": [[mod("a", "x.rs"), mod("d", "d.rs")], [mod("a", "y.rs")]], "problems": ["module a is declared twice: x.rs and y.rs", "module a changed: a.rs -> x.rs", "module b (b.rs) is missing from the carved crates", "module c (c.rs) is missing from the carved crates", "module d appears only in the carved crates"], "originalCount": 3, "unionCount": 2},
 {"name": "nothing at all", "original": [], "carved": [], "problems": [], "originalCount": 0, "unionCount": 0},
]
plain = [(["crate", "a", "", "__c_x", "b"], "a::b"), ([], ""), (["__c_q"], ""), (["a__c_x", "b"], "a__c_x::b"), (["crate"], ""), (["x", "crate", "y"], "x::y")]
plain_rows = [{"segments": s, "expected": e} for s, e in plain]
E = lambda code, message, label="", file="f.rs", line=1, src="": {"code": code, "message": message, "label": label, "file": file, "line": line, "sourceLine": src}
cascade = [
 {"name": "imports that repeat an unlinked crate re-export are dropped", "entries": [E("E0432", "unresolved import `ext`", "unlinked crate `ext`", src="pub use ext::Thing;"), E("E0432", "unresolved import `crate::m::Thing`", src="use crate::m::Thing;"), E("E0432", "unresolved import `crate::m::Other`", src="use crate::m::Other;"), E("E0308", "mismatched types"), E("E0432", "unresolved imports `a::X`, `b::Y`"), E("E0432", "unresolved import"), E("E0433", "unresolved import `crate::m::Thing`")], "globModules": [], "kept": [2, 3, 4, 5, 6]},
 {"name": "glob modules drop every import below them", "entries": [E("E0432", "unresolved import `crate::g::Zed`"), E("E0432", "unresolved import `crate::h::Zed`")], "globModules": ["g"], "kept": [1]},
 {"name": "a taint from the message alone also counts", "entries": [E("E0432", "unresolved import `x` (unlinked crate)", src="use x::Q;"), E("E0432", "unresolved import `y::Q`", src="use y::Q;")], "globModules": [], "kept": []},
 {"name": "several paths are dropped only when each one is tainted", "entries": [E("E0432", "unresolved import `e`", "unlinked crate", src="use e::{A, B};"), E("E0432", "unresolved imports `m::A`, `m::B`"), E("E0432", "unresolved imports `m::A`, `m::C`")], "globModules": [], "kept": [2]},
 {"name": "no entries", "entries": [], "globModules": [], "kept": []},
 {"name": "without a taint every entry stays", "entries": [E("E0432", "unresolved import `a::B`"), E("E0433", "failed")], "globModules": [], "kept": [0, 1]},
]
diff = [{"name": n, "original": o, "candidate": c2, "onlyOriginal": oo, "onlyCandidate": oc} for n, o, c2, oo, oc in [("multiset difference", ["a", "a", "b"], ["a", "c", "b", "b"], ["a"], ["c", "b"]), ("equal lists", ["a", "b"], ["b", "a"], [], []), ("empty candidate", ["a", "a"], [], ["a", "a"], []), ("empty original", [], ["x", "x"], [], ["x", "x"]), ("both empty", [], [], [], []), ("order of the originals follows first appearance", ["z", "y", "z", "x"], ["y"], ["z", "z", "x"], [])]]
def orphan(name, a_src, b_src, expect):
    return {"name": name, "crates": [{"name": "a", "sources": a_src}, {"name": "b", "sources": b_src}], "expected": expect}
orph = [
 orphan("a foreign trait for a type of another crate", {"a.rs": "pub struct Foo;\n"}, {"b.rs": "impl std::fmt::Display for Foo { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { Ok(()) } }\n"}, [{"crate": "b", "file": "b.rs", "trait": "Display", "selfType": "Foo", "definedIn": ["a"]}]),
 orphan("an inherent impl of a foreign type", {"a.rs": "pub struct Foo;\n"}, {"b.rs": "impl Foo { fn x(&self) {} }\n"}, [{"crate": "b", "file": "b.rs", "trait": None, "selfType": "Foo", "definedIn": ["a"]}]),
 orphan("a local type in the trait arguments satisfies the rule", {"a.rs": "pub struct Foo;\n"}, {"b.rs": "struct Bar;\nimpl From<Bar> for Foo { fn from(_: Bar) -> Foo { Foo } }\n"}, []),
 orphan("a carved trait is accepted", {"a.rs": "pub struct Foo;\npub trait Tr {}\n"}, {"b.rs": "impl Tr for Foo {}\n"}, []),
 orphan("a generic wrapper defined in the same crate", {"a.rs": "pub struct Foo;\n"}, {"b.rs": "struct Wrapper<T>(T);\nimpl<T> std::fmt::Debug for Wrapper<T> { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { Ok(()) } }\n"}, []),
 orphan("a type alias of another crate counts as a type", {"a.rs": "pub type Alias = u8;\n"}, {"b.rs": "impl std::fmt::Display for Alias {}\n"}, [{"crate": "b", "file": "b.rs", "trait": "Display", "selfType": "Alias", "definedIn": ["a"]}]),
 orphan("a type defined in both crates is local to each", {"a.rs": "pub struct Foo;\n"}, {"b.rs": "pub struct Foo;\nimpl std::fmt::Display for Foo {}\n"}, []),
 orphan("an unknown type is not checked", {"a.rs": "pub struct Foo;\n"}, {"b.rs": "impl std::fmt::Display for Mystery {}\n"}, []),
 orphan("an enum of another crate", {"a.rs": "pub enum Kind { A }\n"}, {"b.rs": "impl Default for Kind { fn default() -> Kind { Kind::A } }\n"}, [{"crate": "b", "file": "b.rs", "trait": "Default", "selfType": "Kind", "definedIn": ["a"]}]),
 orphan("impls in the owning crate are fine", {"a.rs": "pub struct Foo;\nimpl std::fmt::Display for Foo {}\nimpl Foo {}\n"}, {"b.rs": "fn f() {}\n"}, []),
]
vis_files = {"u.rs": 'pub struct Pub;\nstruct Priv;\npub(crate) fn crate_only() {}\npub use inner::*;\n#[path = "s.rs"]\nmod sub;\n#[path = "pm.rs"]\npub mod pubmod;\n#[path = "inner.rs"]\npub mod inner;\n', "s.rs": "pub fn in_sub() {}\n", "pm.rs": "pub struct Item;\nstruct Hidden;\n", "inner.rs": "pub struct Globbed;\nstruct GlobHidden;\npub use other::Reexport;\n"}
vis = {"files": vis_files, "root": '#[path = "u.rs"]\npub mod unit;\n', "unitPaths": [["unit"]],
 "callers": {"b/lib.rs": "crate::unit::Pub crate::unit::Priv crate::unit::crate_only crate::unit::sub::in_sub crate::unit::pubmod::Item crate::unit::pubmod::Hidden crate::unit::Globbed crate::unit::GlobHidden crate::unit::inner::Reexport crate::unit::nothing crate::elsewhere::X"},
 "expected": [{"from": "b", "file": "b/lib.rs", "chain": "unit::Priv", "reason": "item Priv in unit is private"}, {"from": "b", "file": "b/lib.rs", "chain": "unit::crate_only", "reason": "item crate_only in unit is pub(crate)"}, {"from": "b", "file": "b/lib.rs", "chain": "unit::sub::in_sub", "reason": "module unit::sub is private"}, {"from": "b", "file": "b/lib.rs", "chain": "unit::pubmod::Hidden", "reason": "item Hidden in unit::pubmod is private"}, {"from": "b", "file": "b/lib.rs", "chain": "unit::GlobHidden", "reason": "item GlobHidden in unit is private"}]}
shared = {"files": {"p1/Cargo.toml": '[package]\nname = "p1"\n\n[lib]\npath = "src/lib.rs"\n', "p1/src/lib.rs": '#[path = "../../shared/s.rs"]\nmod s;\n', "p2/Cargo.toml": '[package]\nname = "p2"\n', "p2/src/lib.rs": '#[path = "../../shared/s.rs"]\nmod s;\n#[path = "own.rs"]\nmod own;\n', "p2/src/own.rs": "fn own() {}\n",
 "p3/Cargo.toml": '[package]\nname = "p3"\n\n[[test]]\nname = "t"\n\n[[bin]]\nname = "x"\n', "p3/src/lib.rs": '#[path = "../../shared/s.rs"]\nmod s;\n', "p3/tests/t.rs": '#[path = "../../shared/t.rs"]\nmod tt;\n', "p3/src/bin/x.rs": '#[path = "../../../shared/t.rs"]\nmod tt;\n',
 "ws/Cargo.toml": '[workspace]\nmembers = ["p1"]\n', "shared/s.rs": "fn s() {}\n", "shared/t.rs": "fn t() {}\n", "broken/Cargo.toml": "this is not toml ===\n"},
 "manifests": ["p1/Cargo.toml", "p2/Cargo.toml", "p3/Cargo.toml", "ws/Cargo.toml", "ghost/Cargo.toml", "broken/Cargo.toml"],
 "all": [{"file": "shared/s.rs", "manifests": ["p1/Cargo.toml", "p2/Cargo.toml", "p3/Cargo.toml"]}, {"file": "shared/t.rs", "manifests": ["p3/Cargo.toml"]}], "within": "p2", "withinExpected": [{"file": "shared/s.rs", "manifests": ["p1/Cargo.toml", "p2/Cargo.toml", "p3/Cargo.toml"]}]}
shared["all"] = [shared["all"][0]]
shared["withinNone"] = []
text = S.str
mod_s = S.obj({"path": text, "file": S.one(S.null, text), "inline": S.bool, "cfgTest": S.bool, "visibility": text})
entry_s = S.obj({"code": text, "message": text, "label": text, "file": text, "line": S.int, "sourceLine": text})
schema = S.obj({"version": S.const(1), "resolve": S.arr(S.obj({"name": text, "root": text, "files": S.map(text), "expected": S.arr(mod_s), "error": text}), 1), "compare": S.arr(S.obj({"name": text, "original": S.arr(mod_s), "carved": S.arr(S.arr(mod_s)), "problems": S.strs(), "originalCount": S.int, "unionCount": S.int}), 1),
 "plain": S.arr(S.obj({"segments": S.strs(), "expected": text}), 1), "cascade": S.arr(S.obj({"name": text, "entries": S.arr(entry_s), "globModules": S.strs(), "kept": S.arr(S.int)}), 1), "diff": S.arr(S.obj({"name": text, "original": S.strs(), "candidate": S.strs(), "onlyOriginal": S.strs(), "onlyCandidate": S.strs()}), 1),
 "orphan": S.arr(S.obj({"name": text, "crates": S.arr(S.obj({"name": text, "sources": S.map(text)})), "expected": S.arr(S.obj({"crate": text, "file": text, "trait": S.one(S.null, text), "selfType": text, "definedIn": S.strs()}))}), 1),
 "visibility": S.obj({"files": S.map(text), "root": text, "unitPaths": S.arr(S.strs()), "callers": S.map(text), "expected": S.arr(S.obj({"from": text, "file": text, "chain": text, "reason": text}))}),
 "shared": S.obj({"files": S.map(text), "manifests": S.strs(), "all": S.arr(S.obj({"file": text, "manifests": S.strs()})), "within": text, "withinExpected": S.arr(S.obj({"file": text, "manifests": S.strs()})), "withinNone": S.arr(S.obj({"file": text, "manifests": S.strs()}))})})
corpus = {"version": 1, "resolve": resolve, "compare": compare, "plain": plain_rows, "cascade": cascade, "diff": diff, "orphan": orph, "visibility": vis, "shared": shared}
body = open(os.path.join(HERE, "u51_body.ts"), encoding="utf8").read()
ORACLE = open(os.path.join(HERE, "u51_oracle.py"), encoding="utf8").read()
oracle = lib.PY_HEAD.replace("{doc}", "Independent Python model of the module tree comparison, the multiset diagnostics difference, plain module paths and unlinked-cascade filtering over the corpus.") + ORACLE + lib.PY_TAIL
ts = lib.ts_wrap(UNIT, "🔬️ Contract: the carve verification resolves the module tree of a crate (path mounts, components, inline modules, cfg(test), the mirror module), compares the tree of an original crate with the union of the carved crates, drops unlinked-crate import cascades from rustc diagnostics, subtracts diagnostic multisets, checks that cross-crate paths resolve through public modules to public items, flags orphan-rule violations and lists files mounted by several packages; the corpus is language-agnostic, tree-sitter's Rust grammar re-resolves the module trees independently and an independent Python model reaches the same verdicts for the comparison, the diff, the plain paths and the cascades.",
 'import Parser from "web-tree-sitter";\nimport {createRequire} from "node:module";\nimport {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join,relative,resolve} from "node:path";\nimport {parseRustSourceTree} from "@@' + "🏗authoring/📦crate-tree/🌳module-tree/🟦.ts" + '@@";\nimport {implHeader} from "@@' + "🏗authoring/📦crate-tree/🗺plan/🧭module-chains/🟦.ts" + '@@";\nimport {checkCrossCrateVisibility,checkOrphanRules,compareModuleTrees,diffDiagnostics,dropUnlinkedCascades,plainModulePath,resolveCrateModules,sharedMountFiles,type OracleEntry,type ResolvedModule} from "@@' + UNIT + '/🟦.ts@@";', body, python=True)
lib.write_unit(UNIT, corpus, schema, ts, oracle)
