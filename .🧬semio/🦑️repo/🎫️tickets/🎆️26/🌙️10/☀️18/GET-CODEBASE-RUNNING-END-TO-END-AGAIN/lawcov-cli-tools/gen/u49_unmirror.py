import json, os, sys
import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🪞unmirror"
HERE = os.path.dirname(os.path.abspath(__file__))
GOLD = os.path.join(HERE, "u49_goldens.json")
gold = json.load(open(GOLD, encoding="utf8")) if os.path.exists(GOLD) else {}
A = {"name": "a-crate", "dir": "a", "root": "a/lib.rs"}
B = {"name": "b-crate", "dir": "b", "root": "b/lib.rs"}
C = {"name": "c-crate", "dir": "c", "root": "c/lib.rs"}
SCH = "pub mod schema { pub struct Item; pub fn make() {} pub mod inner { pub fn deep() {} } }\n"
def case(name, crates, files, targets, api=(), editable=None, defect="", expect=None):
    return {"name": name, "crates": crates, "files": files, "targets": list(targets), "apiRoots": list(api), "editable": editable, "defect": defect, "expect": expect}
cases = [
 case("a glob mirror module is removed and consumers are re-pointed", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "pub mod schema { pub use a_crate::schema::*; }\nuse crate::schema::Item;\npub fn f() -> Item { crate::schema::make(); crate::schema::inner::deep(); Item }\n"}, ["b_crate"]),
 case("an explicit re-export is removed and kept private when used locally", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "pub use a_crate::schema::Item;\npub use a_crate::schema::make;\npub fn f() -> Item { make(); Item }\n"}, ["b_crate"]),
 case("api roots keep their root-level explicit re-exports", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "pub use a_crate::schema::Item;\npub use a_crate::schema::make;\npub fn f() -> Item { make(); Item }\n"}, ["b_crate"], ["b_crate"]),
 case("a consumer crate re-points paths that start in a target", [A, B, C], {"a/lib.rs": SCH, "b/lib.rs": "pub mod schema { pub use a_crate::schema::*; }\n", "c/lib.rs": "use b_crate::schema::Item;\npub fn g() -> Item { b_crate::schema::make(); Item }\n"}, ["b_crate"]),
 case("multi-mounted files are reported with every mounting module and left alone", [A, B], {"a/lib.rs": SCH, "b/lib.rs": '#[path = "shared.rs"]\nmod one;\n#[path = "shared.rs"]\nmod two;\n', "b/shared.rs": "pub use a_crate::schema::Item;\n"}, ["b_crate"], defect="F35", expect={"edits": {}, "report": {"removedLeaves": 0, "repointedLeaves": 0, "repointedChains": 0, "addedImports": 0, "removedModules": 0, "unresolvedMirrors": [], "multiMounted": ["b/shared.rs <- b_crate:/one, b_crate:/two"], "invisible": []}, "audit": {}}),
 case("files that are not editable stay untouched", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "pub use a_crate::schema::Item;\n"}, ["b_crate"], editable=[]),
 case("an unresolvable foreign re-export is reported", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "pub use a_crate::missing::Item;\n"}, ["b_crate"]),
 case("a renamed re-export keeps its alias", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "pub use a_crate::schema::Item as Thing;\npub fn f() -> Thing { Thing }\n"}, ["b_crate"]),
 case("glob imports of a mirror gain the names they used", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "pub mod schema { pub use a_crate::schema::*; }\nmod user {\n    use crate::schema::*;\n    pub fn f() -> Item { make(); Item }\n}\n"}, ["b_crate"]),
 case("a mirror module with its own items stays", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "pub mod schema { pub use a_crate::schema::*; pub fn own() {} }\n"}, ["b_crate"]),
 case("names that are not visible are reported", [A, B], {"a/lib.rs": "pub mod schema { pub struct Item; struct Hidden; }\n", "b/lib.rs": "pub mod schema { pub use a_crate::schema::*; }\nmod user {\n    use crate::schema::*;\n    fn f(_: Hidden, _: Item) {}\n}\n"}, ["b_crate"]),
 case("unresolved leaves and paths are audited", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "use crate::gone::X;\nuse crate::schema::Item;\npub mod schema { pub struct Item; }\nfn f() { crate::nope::f(); }\n"}, ["b_crate"]),
 case("a mirror in a nested inline module is removed with its emptied parents", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "pub mod outer {\n    pub mod schema {\n        pub use a_crate::schema::*;\n    }\n}\npub fn f() -> a_crate::schema::Item { a_crate::schema::Item }\n"}, ["b_crate"]),
 case("paths inside macro arguments are re-pointed like any other", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "pub mod schema { pub use a_crate::schema::*; }\npub fn f() { println!(\"{}\", crate::schema::make()); let _ = crate::schema::inner::deep(); }\n"}, ["b_crate"]),
 case("a public glob re-export of a foreign module keeps the names the file itself uses (mounted file)", [A, B], {"a/lib.rs": SCH, "b/lib.rs": '#[path = "m/mod.rs"]\npub mod m;\n', "b/m/mod.rs": "pub use a_crate::schema::*;\npub fn f() -> Item { Item }\n"}, ["b_crate"], defect="F34", expect={"edits": {"b/m/mod.rs": "use a_crate::schema::Item;\npub fn f() -> Item { Item }\n"}, "report": {"removedLeaves": 1, "repointedLeaves": 0, "repointedChains": 0, "addedImports": 1, "removedModules": 0, "unresolvedMirrors": [], "multiMounted": [], "invisible": []}, "audit": {}}),
 case("a public glob re-export of a foreign module keeps the names the file itself uses (crate root)", [A, B], {"a/lib.rs": SCH, "b/lib.rs": "pub use a_crate::schema::*;\npub fn f() -> Item { make(); Item }\n"}, ["b_crate"], defect="F34", expect={"edits": {"b/lib.rs": "use a_crate::schema::{Item, make};\npub fn f() -> Item { make(); Item }\n"}, "report": {"removedLeaves": 1, "repointedLeaves": 0, "repointedChains": 0, "addedImports": 2, "removedModules": 0, "unresolvedMirrors": [], "multiMounted": [], "invisible": []}, "audit": {}}),
]
for c in cases:
    if c["expect"] is None:
        c["expect"] = gold.get(c["name"], {"edits": {}, "report": {"removedLeaves": 0, "repointedLeaves": 0, "repointedChains": 0, "addedImports": 0, "removedModules": 0, "unresolvedMirrors": [], "multiMounted": [], "invisible": []}, "audit": {}})
if "--cases" in sys.argv:
    json.dump({"cases": cases}, open(os.path.join(HERE, "..", "scratch", "u49_cases.json"), "w", encoding="utf8"), ensure_ascii=False)
    sys.exit(0)
text = S.str
crate = S.obj({"name": text, "dir": text, "root": text})
report = S.obj({"removedLeaves": S.int, "repointedLeaves": S.int, "repointedChains": S.int, "addedImports": S.int, "removedModules": S.int, "unresolvedMirrors": S.strs(), "multiMounted": S.strs(), "invisible": S.strs()})
schema = S.obj({"version": S.const(1), "cases": S.arr(S.obj({"name": text, "crates": S.arr(crate, 1), "files": S.map(text), "targets": S.strs(), "apiRoots": S.strs(), "editable": S.one(S.null, S.strs()), "defect": text, "expect": S.obj({"edits": S.map(text), "report": report, "audit": S.map(S.strs())})}), 1)})
corpus = {"version": 1, "cases": cases}
body = open(os.path.join(HERE, "u49_body.ts"), encoding="utf8").read()
ts = lib.ts_wrap(UNIT, "🪞️ Contract: unmirror deletes the path-preserving re-export layers a carve leaves behind (mirror modules, explicit and glob re-exports of other carved crates), re-points every consumer path, import and glob to the owning crate, keeps designed API roots and own namespaces, leaves multi-mounted and non-editable files alone and reports what it cannot resolve; the corpus is language-agnostic, tree-sitter's Rust grammar checks every edited file and the resolver of a second universe built from the edited sources must resolve every surviving name to the very item it named before.",
 'import Parser from "web-tree-sitter";\nimport {createRequire} from "node:module";\nimport {dirname,join} from "node:path";\nimport {Universe,type Hit} from "@@' + UNIT + '/🔍resolver/🟦.ts@@";\nimport {Unmirror} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts)
