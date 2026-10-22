import json, os, sys
import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🧬mutation-families/🧰support-hoist"
HERE = os.path.dirname(os.path.abspath(__file__))
GOLD = os.path.join(HERE, "u46_goldens.json")
gold = json.load(open(GOLD, encoding="utf8")) if os.path.exists(GOLD) else {"hoist": [], "impl": []}
HEAD = "use crate::schema::{Diff, Snap};\n\n"
ENUM = "#[derive(Clone, Debug, PartialEq, dsl::Mutations)]\npub enum Agg {\n    A(A),\n}\n"
def h(name, text, leaves, eff=None, comp=False, dir="/w/agg", dirTaken=None, defect=""):
    return {"name": name, "aggregate": text, "eff": eff or ["x", "agg"], "isComponent": comp, "leaves": leaves, "taken": dirTaken or [], "defect": defect}
hoists = [
 h("a helper a leaf calls moves into support", HEAD + "fn helper(base: &Snap) -> u8 { 1 }\nconst UNUSED: u8 = 3;\n" + ENUM, ["use crate::x::agg::*;\nfn f() { helper(&s); }\n"]),
 h("a helper pulls in the helpers it calls", HEAD + "pub fn helper(base: &Snap) -> u8 { inner(base) }\nfn inner(base: &Snap) -> u8 { deep() }\nfn deep() -> u8 { 2 }\nfn other() {}\n" + ENUM, ["fn f() { helper(&s); }\n"]),
 h("a helper that names the aggregate stays and so do its callers", HEAD + "fn builds() -> Agg { todo!() }\nfn caller() { builds(); }\nfn plain() {}\n" + ENUM, ["fn f() { caller(); builds(); }\n"]),
 h("nothing for the leaves to call yields no hoist", HEAD + "fn helper() {}\n" + ENUM, ["fn f() { other(); }\n"]),
 h("a name the leaf declares itself is not hoisted", HEAD + "fn helper() {}\n" + ENUM, ["fn helper() {}\nfn f() { helper(); }\n"]),
 h("a glob import in the aggregate keeps everything in place", "use crate::a::*;\nfn helper() {}\n" + ENUM, ["fn f() { helper(); }\n"]),
 h("an existing support module blocks the hoist", "pub mod support;\nfn helper() {}\n" + ENUM, ["fn f() { helper(); }\n"]),
 h("an inherent impl moves with its type", HEAD + "pub struct Acc { n: u8 }\nimpl Acc {\n    pub fn new() -> Self { Acc { n: 0 } }\n}\nimpl<T> Wrap<T> { fn w(&self) {} }\nstruct Wrap<T>(T);\n" + ENUM, ["fn f() { let _ = Acc::new(); }\n"]),
 h("a type with a trait impl in the aggregate stays", HEAD + "pub struct Acc { n: u8 }\nimpl Clone for Acc { fn clone(&self) -> Self { Acc { n: self.n } } }\n" + ENUM, ["fn f() { let _ = Acc { n: 1 }; }\n"]),
 h("lead comments travel with the item and doc comments of the module stay", "//! module docs\n" + HEAD + "// why this exists\n/// builds the base\nfn helper(base: &Snap) -> u8 { 1 }\n" + ENUM, ["fn f() { helper(&s); }\n"]),
 h("imports of moved bodies are copied to the support module", "use crate::schema::{Diff, Snap};\nuse crate::other::Unused;\nuse crate::ids::Id as Ident;\nfn helper(base: &Snap, id: Ident) -> Diff { todo!() }\n" + ENUM, ["fn f() { helper(); }\n"]),
 h("visibility modifiers are re-exported as written", HEAD + "pub fn a1() {}\npub(crate) fn b1() {}\nfn c1() {}\n" + ENUM, ["fn f() { a1(); b1(); c1(); }\n"]),
 h("the mount goes before the first child module", HEAD + "#[path = \"sub/mod.rs\"]\npub mod sub;\nfn helper() {}\n" + ENUM, ["fn f() { helper(); }\n"]),
 h("empty regions are dropped after the cut", HEAD + "//#region helpers\nfn helper() {}\n//#endregion helpers\n" + ENUM, ["fn f() { helper(); }\n"]),
 h("a test-gated helper never moves", HEAD + "#[cfg(test)]\nfn helper() {}\nfn real() {}\n" + ENUM, ["fn f() { helper(); real(); }\n"]),
 h("a later folder emoji is used when the first is taken", HEAD + "fn helper() {}\n" + ENUM, ["fn f() { helper(); }\n"], dirTaken=["\U0001F9F0\ufe0ftaken", "\U0001F6E0\ufe0ftaken"]),
 h("all folder emoji taken yields no hoist", HEAD + "fn helper() {}\n" + ENUM, ["fn f() { helper(); }\n"], dirTaken=["\U0001F9F0\ufe0ftaken", "\U0001F6E0\ufe0ftaken", "\U0001F527\ufe0ftaken", "\U0001F9F2\ufe0ftaken"]),
 h("the aggregate statics and constants move too", HEAD + "const LIMIT: u8 = 9;\nstatic NAME: &str = \"n\";\nenum Kind { A }\ntype Alias = u8;\ntrait Helper {}\nfn used() -> u8 { LIMIT }\n" + ENUM, ["fn f() { used(); NAME; Kind::A; let _: Alias = 0; }\n"]),
]
for i, r in enumerate(hoists):
    r["expect"] = gold["hoist"][i] if i < len(gold["hoist"]) else None
impls = [
 ("impl Name {", "Name"), ("impl<T> Name<T> {", "Name"), ("impl<T: Clone, const N: usize> Wrapper<T, N> where T: Copy {", "Wrapper"), ("unsafe impl Name {", "Name"), ("impl Trait for Name {", None), ("impl<T> Trait<T> for Name<T> {", None), ("impl Name<u8> {", "Name"),
 ("impl<'a> Name<'a> {", "Name"), ("impl Name\nwhere\n    T: Copy,\n{", "Name"), ("impl crate::path::Name {", None), ("impl &Name {", None), ("impl dyn Name {", None), ("impl Name for Other {", None), ("impl Fortune {", "Fortune"), ("impl [u8] {", None),
]
impl_rows = [{"head": head[:-1], "expected": exp} for head, exp in impls]
if "--cases" in sys.argv:
    json.dump({"hoists": hoists}, open(os.path.join(HERE, "..", "scratch", "u46_cases.json"), "w", encoding="utf8"), ensure_ascii=False)
    sys.exit(0)
text = S.str
hoist_expect = S.one(S.null, S.obj({"aggregateText": text, "supportFile": text, "supportText": text, "moved": S.strs()}))
schema = S.obj({"version": S.const(1), "hoists": S.arr(S.obj({"name": text, "aggregate": text, "eff": S.strs(), "isComponent": S.bool, "leaves": S.strs(), "taken": S.strs(), "defect": text, "expect": hoist_expect}), 1), "impls": S.arr(S.obj({"head": text, "expected": S.one(S.null, text)}), 1)})
corpus = {"version": 1, "hoists": hoists, "impls": impl_rows}
body = open(os.path.join(HERE, "u46_body.ts"), encoding="utf8").read()
ts = lib.ts_wrap(UNIT, "🧰️ Contract: the support hoist moves the helper functions, constants and types of an aggregate module that its mutation leaves call (and everything they in turn use) into a support module below the leaves, keeps whatever mentions the aggregate or a test gate in place, copies the imports the moved bodies need and re-exports with the original visibility; inherent impl heads are told from trait impls; the corpus is language-agnostic and tree-sitter's Rust grammar is the independent third-party oracle for the impl heads and for the validity of both produced files.",
 'import Parser from "web-tree-sitter";\nimport {createRequire} from "node:module";\nimport {mkdirSync,mkdtempSync,rmSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join} from "node:path";\nimport {aggregateScope} from "@@' + "🏗authoring/📦crate-tree/🧬mutation-families/🟦.ts" + '@@";\nimport {hoistAggregateSupport,inherentSelf} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts)
