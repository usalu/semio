import json, os, sys
import lib
from lib import S
from u45_fixture import *
UNIT = "🏗authoring/📦crate-tree/🧬mutation-families"
HERE = os.path.dirname(os.path.abspath(__file__))
GOLD = os.path.join(HERE, "u45_goldens.json")
gold = json.load(open(GOLD, encoding="utf8")) if os.path.exists(GOLD) else {"convert": {}, "scope": [], "decouple": []}
def pm(name, struct, body):
    return body
beta_inverse_import = "\n".join(["use super::Beta;", "use crate::schema::mutations::{alpha, DemoMutation};", "use crate::schema::Snap;", "pub fn inverse(payload: &Beta, base: &Snap) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> {", "    let _ = alpha::Alpha { id: String::new() };", "    Ok(vec![DemoMutation::Beta(Beta { id: payload.id.clone() }), alpha(base.id.clone())])", "}", ""])
beta_inverse_sup = "\n".join(["use super::Beta;", "use crate::schema::mutations::DemoMutation;", "use crate::schema::Snap;", "pub fn inverse(payload: &Beta, _base: &Snap) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> {", "    Ok(vec![DemoMutation::Alpha(super::super::alpha::Alpha { id: payload.id.clone() })])", "}", ""])
delta = add_leaf(FILES, "delta", "Delta")
delta[f"{BASE}/delta/inverse/mod.rs"] = "use super::Delta;\nuse crate::schema::Snap;\npub fn inverse<Op>(payload: &Delta, base: &Snap) -> Result<Vec<Op>, semio_framework_value::ValueError>\nwhere\n    Op: crate::schema::Restoring + From<crate::schema::mutations::alpha::Alpha>,\n{\n    Ok(vec![Op::from(crate::schema::mutations::alpha::Alpha { id: payload.id.clone() })])\n}\n"
assoc = add_leaf(FILES, "delta", "Delta", inverse_body="    use DemoMutation::*;\n    Ok(vec![Delta(Delta { id: payload.id.clone() })])")
assoc[f"{BASE}/delta/mod.rs"] = assoc[f"{BASE}/delta/mod.rs"].replace("protocol::MutationOutcome<Diff>", "protocol::MutationOutcome<<DemoMutation as Mutation<Snap>>::Diff>").replace("use protocol::{MutationKind, SemanticDescriptor};", "use protocol::{Mutation, MutationKind, SemanticDescriptor};")
private = {"lib.rs": '#[path = "agg/mod.rs"]\nmod mutations;\n', "agg/mod.rs": 'use crate::{Diff, Snap};\n#[path = "leaf/mod.rs"]\nmod leaf;\n#[derive(Clone, Debug, PartialEq, dsl::Mutations)]\n#[mutations(snapshot = Snap, diff = Diff, schema = "demo.demo")]\npub enum DemoMutation {\n    Alpha(leaf::Alpha),\n}\n', "agg/leaf/mod.rs": 'use super::*;\n#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]\n#[mutation_leaf(contract = ::protocol)]\npub struct Alpha { pub id: String }\n'}
def direct(name, struct, other, other_struct):
    return {f"{BASE}/{name}/mod.rs": "\n".join([f"//! {name} leaf", f"use super::{other}::{other_struct};", "use crate::schema::mutations::DemoMutation;", "use crate::schema::{Diff, Snap};", "use protocol::{MutationKind, SemanticDescriptor};", "", "#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]", "#[mutation_leaf(contract = ::protocol)]", f"pub struct {struct} {{ pub id: String }}", "", f"impl MutationKind<Snap, DemoMutation> for {struct} {{",
        '    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "x", entity: "y", kind: "z", record: "R" };', "    fn diff(&self, _base: &Snap) -> protocol::MutationOutcome<Diff> { protocol::MutationOutcome::empty() }", "    fn inverse(&self, _base: &Snap) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> {", f"        Ok(vec![DemoMutation::{other_struct}({other_struct} {{ id: self.id.clone() }})])", "    }", "}", ""])}
sib = {"lib.rs": ROOT.split('#[path = "."]\n        pub mod alpha')[0] + "    }\n}\n", "schema/mod.rs": SCHEMA}
sib["lib.rs"] = "\n".join(["//! demo root.", "extern crate semio_framework_os_kernel as protocol;", '#[path = "."]', "pub mod schema {", '    #[path = "schema/mod.rs"]', "    mod component;", "    pub use component::*;", '    #[path = "."]', "    pub mod mutations {", f'        #[path = "{BASE}/mod.rs"]', "        mod component;", "        pub use component::*;", "    }", "}", ""])
sib.update(direct("delta", "Delta", "epsilon", "Epsilon")); sib.update(direct("epsilon", "Epsilon", "delta", "Delta"))
sib[f"{BASE}/mod.rs"] = "\n".join(["//! aggregate", "use crate::schema::{Diff, Snap};", "", f'#[path = "delta/mod.rs"]', "pub mod delta;", f'#[path = "epsilon/mod.rs"]', "pub mod epsilon;", "pub use delta::Delta;", "pub use epsilon::Epsilon;", "", "#[derive(Clone, Debug, PartialEq, dsl::Mutations)]", '#[mutations(snapshot = Snap, diff = Diff, schema = "demo.demo")]', "pub enum DemoMutation {", "    Delta(Delta),", "    Epsilon(Epsilon),", "}", ""])
convert_cases = [
 {"name": "two leaves convert and one stays concrete", "files": FILES},
 {"name": "a companion already generic over its aggregate keeps its bounds", "files": delta},
 {"name": "a private aggregate module stays concrete", "files": private},
 {"name": "a sibling builder is imported from its leaf", "files": with_files(**{f"{BASE}__beta__inverse__mod.rs": beta_inverse_import})},
 {"name": "sibling super paths become absolute", "files": with_files(**{f"{BASE}__beta__inverse__mod.rs": beta_inverse_sup})},
 {"name": "associated diff types and glob variants", "files": assoc},
 {"name": "leaves reaching each other through super are decoupled", "files": sib},
]
for c in convert_cases:
    c["root"] = "lib.rs"
    c["expect"] = gold["convert"].get(c["name"], {"converted": [], "hand": [], "notes": [], "aggregates": [], "edits": {}, "creates": {}})
def af(name, text, expect, extra=None, defect=""):
    files = {"lib.rs": '#[path = "a.rs"]\npub mod a;\n' + ('#[cfg(test)]\n#[path = "t.rs"]\nmod t;\n' if extra else ""), "a.rs": text}
    if extra:
        files["t.rs"] = extra
    return {"name": name, "files": files, "expect": expect, "defect": defect}
V = lambda v, p, path: {"variant": v, "payload": p, "path": path}
AG = lambda name, snap, diff, vs: {"name": name, "file": "a.rs", "effPath": ["a"], "snapshot": snap, "diff": diff, "variants": vs}
aggs = [
 af("one aggregate with options and paths", '#[derive(Clone, dsl::Mutations)]\n#[mutations(snapshot = Snap, diff = crate::Diff, schema = "k")]\npub enum One { A(a::A), B(B) }\n', [AG("One", "Snap", "crate::Diff", [V("A", "A", "a::A"), V("B", "B", "B")])]),
 af("derive order and attributes on variants", '#[derive(dsl::Mutations, Clone)]\npub enum Two {\n    #[allow(dead_code)]\n    A(crate::x::A),\n    #[doc = "b"] B(B),\n}\n', [AG("Two", None, None, [V("A", "A", "crate::x::A"), V("B", "B", "B")])]),
 af("a variant with two fields makes the enum unsupported", '#[derive(dsl::Mutations)]\npub enum Bad { A(X, Y) }\n', [], defect="F29"),
 af("struct and unit variants make the enum unsupported", '#[derive(dsl::Mutations)]\npub enum Bad { A { x: u8 } }\n#[derive(dsl::Mutations)]\npub enum Unit { A, B(B) }\n', []),
 af("two aggregates in one file", '#[derive(dsl::Mutations)]\npub enum First { A(A) }\n#[derive(dsl::Mutations)]\n#[mutations(snapshot = S2)]\npub enum Second { B(B), C(c::C) }\n', [AG("First", None, None, [V("A", "A", "A")]), AG("Second", "S2", None, [V("B", "B", "B"), V("C", "C", "c::C")])]),
 af("a Mutations word outside a derive is ignored", '// Mutations are great\n#[derive(Debug)]\npub enum E { A(A) }\nfn f() { dsl::Mutations::new(); }\n#[derive(dsl::Mutations::Other)]\npub enum F { A(A) }\n', []),
 af("an aggregate in a test-only module is skipped", 'pub fn nothing() {}\n', [], extra='#[derive(dsl::Mutations)]\npub enum T { A(A) }\n'),
 af("an empty enum yields nothing", '#[derive(dsl::Mutations)]\npub enum Empty {}\n', []),
]
scope_rows = [
 {"name": "imports, modules and items", "text": "use crate::a::B;\nuse super::C;\nuse self::d::E;\nuse f::G as H;\npub mod sub;\nmod hidden;\nfn helper() {}\nstruct S;\nmacro_rules! m { () => {} }\nimpl S {}\n", "eff": ["x", "agg"], "isComponent": False},
 {"name": "a component aggregate resolves super from its own path", "text": "use super::C;\nuse self::d::E;\n", "eff": ["x", "agg"], "isComponent": True},
 {"name": "a glob import is flagged", "text": "use crate::a::*;\nuse crate::b::B;\n", "eff": ["x"], "isComponent": False},
 {"name": "test-gated uses are skipped and a component re-export counts as a glob", "text": "#[cfg(test)]\nuse crate::t::T;\n#[cfg(test)]\nmod tests;\n#[path = \"c.rs\"]\nmod component;\npub use component::*;\nstruct Real;\n", "eff": ["x"], "isComponent": False},
 {"name": "a double super cannot be named", "text": "use super::super::X;\nuse crate::{p::Q, r::{s::T as U}};\n", "eff": ["x", "y", "agg"], "isComponent": False},
]
for i, r in enumerate(scope_rows):
    r["expect"] = gold["scope"][i] if i < len(gold["scope"]) else {"names": {}, "glob": False}
AGG_TEXT = "use crate::a::B;\nuse crate::q::Item as Renamed;\npub mod sub;\nfn helper() {}\nstruct Hidden;\n"
decouple_rows = [
 {"name": "imports replace the glob", "aggregate": AGG_TEXT, "eff": ["x", "agg"], "isComponent": False, "leaf": "use super::*;\nuse crate::z::Z;\nfn f(_: B) -> sub::T { Renamed::new() }\n", "depth": 1},
 {"name": "a used item keeps the leaf coupled", "aggregate": AGG_TEXT, "eff": ["x", "agg"], "isComponent": False, "leaf": "use super::*;\nfn f() { helper(); }\n", "depth": 1},
 {"name": "names the leaf declares itself are not imported", "aggregate": AGG_TEXT, "eff": ["x", "agg"], "isComponent": False, "leaf": "use super::*;\nfn B() {}\nfn f() { B(); }\n", "depth": 1},
 {"name": "a leaf without the glob is untouched", "aggregate": AGG_TEXT, "eff": ["x", "agg"], "isComponent": False, "leaf": "use crate::a::B;\nfn f(_: B) {}\n", "depth": 1},
 {"name": "a deeper leaf matches the glob depth", "aggregate": AGG_TEXT, "eff": ["x", "agg"], "isComponent": False, "leaf": "use super::super::*;\nfn f(_: B) {}\n", "depth": 2},
 {"name": "the wrong glob depth is not decoupled", "aggregate": AGG_TEXT, "eff": ["x", "agg"], "isComponent": False, "leaf": "use super::*;\nfn f(_: B) {}\n", "depth": 2},
 {"name": "an aggregate that imports through a glob keeps every leaf coupled", "aggregate": "use crate::a::*;\n", "eff": ["x", "agg"], "isComponent": False, "leaf": "use super::*;\nfn f() {}\n", "depth": 1},
 {"name": "no used name means the glob is just removed", "aggregate": AGG_TEXT, "eff": ["x", "agg"], "isComponent": False, "leaf": "use super::*;\nfn f() {}\n", "depth": 1},
 {"name": "a pub glob is not the aggregate glob", "aggregate": AGG_TEXT, "eff": ["x", "agg"], "isComponent": False, "leaf": "pub use super::*;\nfn f(_: B) {}\n", "depth": 1},
]
for i, r in enumerate(decouple_rows):
    r["expect"] = gold["decouple"][i] if i < len(gold["decouple"]) else None
if "--cases" in sys.argv:
    json.dump({"convert": convert_cases, "scope": scope_rows, "decouple": decouple_rows}, open(os.path.join(HERE, "..", "scratch", "u45_cases.json"), "w", encoding="utf8"), ensure_ascii=False)
    sys.exit(0)
text = S.str
agg_s = S.obj({"name": text, "file": text, "effPath": S.strs(), "snapshot": S.one(S.null, text), "diff": S.one(S.null, text), "variants": S.arr(S.obj({"variant": text, "payload": text, "path": text}))})
scope_entry = S.obj({"kind": S.enum("item", "import", "module")}, ["kind"], extra=True)
scope_expect = S.obj({"names": S.map(S.obj({"kind": S.enum("item", "import", "module"), "statement": text}, ["kind"])), "glob": S.bool})
decouple_expect = S.one(S.null, S.obj({"text": text, "coupled": S.strs()}))
conv_expect = S.obj({"converted": S.strs(), "hand": S.arr(S.obj({"payload": text, "reason": text})), "notes": S.strs(), "aggregates": S.arr(S.obj({"name": text, "snapshot": S.one(S.null, text), "diff": S.one(S.null, text), "variants": S.strs()})), "edits": S.map(text), "creates": S.map(text)})
schema = S.obj({"version": S.const(1), "convert": S.arr(S.obj({"name": text, "root": text, "files": S.map(text), "expect": conv_expect}), 1), "aggregates": S.arr(S.obj({"name": text, "files": S.map(text), "expect": S.arr(agg_s), "defect": text}), 1),
 "scope": S.arr(S.obj({"name": text, "text": text, "eff": S.strs(), "isComponent": S.bool, "expect": scope_expect}), 1), "decouple": S.arr(S.obj({"name": text, "aggregate": text, "eff": S.strs(), "isComponent": S.bool, "leaf": text, "depth": S.int, "expect": decouple_expect}), 1)})
corpus = {"version": 1, "convert": convert_cases, "aggregates": aggs, "scope": scope_rows, "decouple": decouple_rows}
body = open(os.path.join(HERE, "u45_body.ts"), encoding="utf8").read()
ts = lib.ts_wrap(UNIT, "🧬️ Contract: the mutation-family codemod finds every transparent `#[derive(dsl::Mutations)]` aggregate with its snapshot, diff and variant payload paths, reads the names an aggregate module puts in scope, decouples a leaf from its aggregate glob, and converts concrete leaves to leaves generic over their aggregate (keeping concrete builders on the aggregate, leaving unsupported leaves for hand work); the corpus is language-agnostic and tree-sitter's Rust grammar is the independent third-party oracle for the aggregates and for the validity of every produced file.",
 'import Parser from "web-tree-sitter";\nimport {createRequire} from "node:module";\nimport {dirname,join,relative,resolve} from "node:path";\nimport {collectDecls} from "@@' + "🏗authoring/📦crate-tree/🧱decl-tree/🟦.ts" + '@@";\nimport {aggregateScope,convertMutationLeaves,decoupleFromAggregate,findMutationAggregates} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts)
