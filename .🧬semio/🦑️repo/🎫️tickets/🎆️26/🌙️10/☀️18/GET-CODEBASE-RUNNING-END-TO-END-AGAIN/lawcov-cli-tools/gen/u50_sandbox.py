import json, os
import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🧫sandbox"
HERE = os.path.dirname(os.path.abspath(__file__))
MODS = {"dep_a": "m_a", "dep_b": "m_b"}
rw = [
 ("crate paths and dependency paths are rewritten", "use crate::x::Y;\nfn f() { crate::z(); dep_a::q(); }\n", "use crate::m_self::x::Y;\nfn f() { crate::m_self::z(); crate::m_a::q(); }\n"),
 ("use groups keep their braces", "use dep_a::{A, B};\nuse dep_b::*;\n", "use crate::m_a::{A, B};\nuse crate::m_b::*;\n"),
 ("a path behind another path is left alone", "fn f() { a::dep_a::b(); x::crate::y(); }\n", "fn f() { a::dep_a::b(); x::crate::y(); }\n"),
 ("a field or method call is left alone", "fn f() { s.dep_a::<T>(); s.crate::x; }\n", "fn f() { s.dep_a::<T>(); s.crate::x; }\n"),
 ("a macro dollar crate is left alone", "macro_rules! m { () => { $crate::x::y() } }\n", "macro_rules! m { () => { $crate::x::y() } }\n"),
 ("a name that is not followed by a path is left alone", "fn f() { let dep_a = 1; dep_a.f(); crate_x(); }\n", "fn f() { let dep_a = 1; dep_a.f(); crate_x(); }\n"),
 ("strings and comments are left alone", "// crate::x and dep_a::y\nfn f() { let _ = \"crate::x dep_a::y\"; }\n", "// crate::x and dep_a::y\nfn f() { let _ = \"crate::x dep_a::y\"; }\n"),
 ("an extern prelude path is left alone", "fn f() { ::dep_a::x(); }\n", "fn f() { ::dep_a::x(); }\n"),
 ("several rewrites in one line keep their order", "type T = (crate::A, dep_a::B, dep_b::C, crate::D);\n", "type T = (crate::m_self::A, crate::m_a::B, crate::m_b::C, crate::m_self::D);\n"),
 ("text without paths is unchanged", "", ""),
]
rw_rows = [{"name": n, "text": t, "expected": e} for n, t, e in rw]
al = [
 ("use crate as me;\n", "use crate::m_self as me;\n"), ("use dep_a as da;\nuse dep_b as db;\n", "use crate::m_a as da;\nuse crate::m_b as db;\n"), ("use crate::x as y;\nuse dep_a::f as g;\n", "use crate::x as y;\nuse dep_a::f as g;\n"),
 ("pub use crate as r;\n", "pub use crate::m_self as r;\n"), ("use dep_a;\nuse dep_c as c;\n", "use dep_a;\nuse dep_c as c;\n"), ("use  crate   as   z;\n", "use  crate::m_self   as   z;\n"),
]
al_rows = [{"text": t, "expected": e} for t, e in al]
norm = [("error crate::__c_foo_bar::x", "error crate::x"), ("use __c_foo::x;", "use x;"), ("<sandbox>/../.. file", "<sandbox>/.. file"), ("a__c_b_c9::d crate::__c_q", "ad crate"), ("no change", "no change"), ("crate::__c_A::x", "crate::__c_A::x"), ("<sandbox>/..", "<sandbox>/..")]
norm_rows = [{"line": l, "expected": e} for l, e in norm]
ext = [("E0432\tunresolved import `semio_x`", False), ("E0432\tunresolved import `semio_x::y`", True), ("unlinked crate semio_x", False), ("E0433\tcannot find `foo` in the crate root\tf.rs:1", False), ("E0433\tcannot find `foo` in the crate root", True),
 ("E0220\tassociated type `Item` not found for `T`", False), ("E0220\tassociated type `Item` not found for `t`", True), ("E0308\tmismatched types", True), ("E0432\tunresolved import `semio_x`\tf.rs", False), ("E0432\tunresolved import `other_x`", True), ("E0220\tassociated type `Item` not found for `T`\tf.rs:2", False), ("", True)]
ext_rows = [{"line": l, "expected": e} for l, e in ext]
pl = [("src/a.rs\tcannot find derive macro `Foo` in this scope\tlabel\textra", "src/a.rs\tcannot find derive macro `Foo` in this scope"), ("f.rs\tcannot find attribute `x` in this scope\tlabel", "f.rs\tcannot find attribute `x` in this scope"),
 ("f.rs\tcannot find type `T` in this scope\tlabel\textra", "f.rs\tcannot find type `T` in this scope\tlabel"), ("f.rs\tcannot find value `v` in this scope\tl\te", "f.rs\tcannot find value `v` in this scope\tl"), ("f.rs\tcannot find function `f` in this scope\tl\te", "f.rs\tcannot find function `f` in this scope\tl"),
 ("f.rs\tcannot find struct, variant or union type `S` in this scope\tl\te", "f.rs\tcannot find struct, variant or union type `S` in this scope\tl"), ("f.rs\tcannot find type `T` in module `m`\tl\te", "f.rs\tcannot find type `T` in module `m`\tl\te"), ("f.rs\tmismatched types\tl", "f.rs\tmismatched types\tl"),
 ("cannot find attribute `x`", "cannot find attribute `x`"), ("", "")]
pl_rows = [{"line": l, "expected": e, "defect": ""} for l, e in pl]
subj = [
 ("E0433\tcannot find `x` in `mod`", ["x"], ""), ("E0432\tunresolved import `a::b::C`", ["C"], ""), ("E0432\tunresolved imports `a::B`, `c::D`", ["B", "D"], ""), ("E0412\tcannot find type `Foo` in this scope", ["Foo"], ""), ("E0425\tcannot find value `x` in this scope", ["x"], ""),
 ("E0425\tcannot find function `f` in module `m`", ["f"], ""), ("E0433\tfailed to resolve: use of undeclared type `X`", [], ""), ("E0599\tno method named `m` found", [], ""), ("E0432\tunresolved import `a::B`: could not find `B` in `a`", ["B"], ""),
 ("E0422\tcannot find struct, variant or union type `S` in this scope", ["S"], ""), ("E0405\tcannot find trait `Tr` in this scope", ["Tr"], "F36"), ("E0432\tunresolved import `a`", ["a"], ""), ("cannot find type `T` in this scope", [], ""), ("", []  , ""),
 ("E0425\tcannot find value `x` in this scope\tf.rs:3", ["x"], ""),
]
subj_rows = [{"line": l, "expected": e, "defect": d} for l, e, d in subj]
drop = [
 {"name": "a name the base already fails on is dropped in either spelling", "base": ["E0412\tcannot find type `A` in this scope"], "carved": ["E0432\tunresolved import `x::A`", "E0412\tcannot find type `A` in this scope", "E0432\tunresolved import `x::B`", "E0308\tmismatched", "E0432\tunresolved imports `x::A`, `x::B`"], "expected": ["E0432\tunresolved import `x::B`", "E0308\tmismatched", "E0432\tunresolved imports `x::A`, `x::B`"]},
 {"name": "without base failures everything stays", "base": [], "carved": ["E0412\tcannot find type `A` in this scope", "E0308\tm"], "expected": ["E0412\tcannot find type `A` in this scope", "E0308\tm"]},
 {"name": "a line without subjects always stays", "base": ["E0412\tcannot find type `A` in this scope"], "carved": ["E0308\tm", "warning"], "expected": ["E0308\tm", "warning"]},
 {"name": "several subjects need to be known together", "base": ["E0432\tunresolved imports `a::A`, `a::B`"], "carved": ["E0432\tunresolved imports `b::A`, `b::B`", "E0432\tunresolved imports `b::A`, `b::C`"], "expected": ["E0432\tunresolved imports `b::A`, `b::C`"]},
]
tree = [("an inner attribute is not permitted in this context", True), ("couldn't find file x.rs", True), ("file not found for module `m`", True), ("circular modules: a -> b", True), ("couldn't read /x: No such file", True), ("expected `;`, found `}`", True), ("expected item, found `x`", True), ("unexpected closing delimiter: `}`", True),
 ("this file contains an unclosed delimiter", True), ("unclosed delimiter", True), ("mismatched closing delimiter: `)`", True), ("mismatched types", False), ("cannot find value `x` in this scope", False), (" expected x", False), ("", False), ("Expected", False), ("unresolved import `a`", False)]
tree_rows = [{"message": m, "expected": e} for m, e in tree]
hi = [
 ("a file with attributes at the head is unchanged", "//! docs\n#![allow(x)]\nuse a;\nfn f() {}\n", "//! docs\n#![allow(x)]\nuse a;\nfn f() {}\n"),
 ("an inner attribute behind a use is lifted", "use a;\n#![allow(x)]\nfn f() {}\n", "#![allow(x)]\nuse a;\nfn f() {}\n"),
 ("an inner doc line behind a use is lifted", "use a;\n//! late doc\nfn f() {}\n", "//! late doc\nuse a;\nfn f() {}\n"),
 ("an inner doc line behind an item stays", "fn f() {}\n//! x\n", "fn f() {}\n//! x\n"),
 ("lifted lines keep their order", "use a;\n#![a]\nuse b;\n#![b]\nfn f() {}\n", "#![a]\n#![b]\nuse a;\nuse b;\nfn f() {}\n"),
 ("a block comment at the head is skipped", "/* head\nmore */\nuse a;\n#![allow(y)]\n", "/* head\nmore */\n#![allow(y)]\nuse a;\n"),
 ("only comments and blanks come back unchanged", "// a\n\n/* b */\n", "// a\n\n/* b */\n"),
 ("an empty text is unchanged", "", ""),
 ("a plain file is unchanged", "use a;\nfn f() {}\n", "use a;\nfn f() {}\n"),
 ("an inner attribute with a trailing comment is lifted too", "use a;\n#![allow(x)] // why\nfn f() {}\n", "#![allow(x)] // why\nuse a;\nfn f() {}\n"),
 ("an extern crate counts as an item behind which attributes are misplaced", "extern crate a;\n#![no_std]\nfn f() {}\n", "#![no_std]\nextern crate a;\nfn f() {}\n"),
]
hi_rows = []
for n, t, e in hi:
    d = "F37" if "trailing comment" in n else ""
    hi_rows.append({"name": n, "text": t, "expected": e, "defect": d})
mp = [
 {"name": "flat path attributes resolve against the file directory", "file": "a/b/lib.rs", "text": '#[path = "x/y.rs"]\nmod m;\n#[path="../z.rs"] mod n;\n#[path = "dir/file.txt"]\nmod t;\n', "expected": ["a/b/x/y.rs", "a/z.rs"]},
 {"name": "no attributes", "file": "lib.rs", "text": "mod m;\n", "expected": []},
 {"name": "a file at the root", "file": "lib.rs", "text": '#[path = "m/mod.rs"] mod m;\n#[cfg(test)]\n#[path = "t.rs"] mod t;\n', "expected": ["m/mod.rs", "t.rs"]},
 {"name": "repeated mounts are listed as often as written", "file": "a/lib.rs", "text": '#[path = "s.rs"] mod one;\n#[path = "s.rs"] mod two;\n', "expected": ["a/s.rs", "a/s.rs"]},
 {"name": "a path that climbs above the root", "file": "a/lib.rs", "text": '#[path = "../../x.rs"] mod m;\n', "expected": ["../x.rs"]},
 {"name": "a path with dots inside", "file": "a/lib.rs", "text": '#[path = "./b/../c.rs"] mod m;\n', "expected": ["a/c.rs"]},
]
mount = {"files": {"c1/lib.rs": "use crate::x::Y;\nuse dep_b::Z;\nuse crate as me;\n", "c1/x.rs": "use crate::q;\n", "c2/lib.rs": "use dep_b::W;\nuse crate::r;\n", "c2/notes.txt": "crate::untouched\n"}, "owned": [{"module": "m1", "paths": ["c1/lib.rs", "c1/x.rs", "c1/missing.rs"], "root": "c1/lib.rs"}, {"module": "m2", "paths": ["c2/lib.rs", "c1/x.rs", "c2/notes.txt"], "root": "c2/lib.rs"}], "crateModules": {"dep_b": "m2"},
 "expected": {"__combined.rs": '#[path = "c1/lib.rs"]\npub mod m1;\n#[path = "c2/lib.rs"]\npub mod m2;\n', "c1/lib.rs": "use crate::m1::x::Y;\nuse crate::m2::Z;\nuse crate::m1 as me;\n", "c1/x.rs": "use crate::m1::q;\n", "c2/lib.rs": "use crate::m2::W;\nuse crate::m2::r;\n", "c2/notes.txt": "crate::untouched\n"}}
outside = {"plugin": "plug", "files": {"plug/a.rs": '#[path = "../shared/x.rs"]\nmod x;\nconst T: &str = include_str!("../data/t.txt");\n#[path = "inside/y.rs"] mod y;\n#[path = "../nowhere/z.rs"] mod z;\n', "plug/inside/y.rs": "fn y() {}\n", "plug/node_modules/n.rs": '#[path = "../../shared2/q.rs"] mod q;\n', "plug/sub/b.rs": 'include_bytes!("../../shared3/blob.bin");\ninclude!("../../shared4/inc.rs");\n', "shared/x.rs": "fn x() {}\n", "data/t.txt": "t\n", "shared3/blob.bin": "b\n", "shared4/inc.rs": "fn i() {}\n", "shared2/q.rs": "fn q() {}\n"}, "expected": ["data", "shared", "shared3", "shared4"]}
if True:
    pass
text = S.str
schema = S.obj({"version": S.const(1), "modules": S.map(text),
 "rewrite": S.arr(S.obj({"name": text, "text": text, "expected": text}), 1), "aliases": S.arr(S.obj({"text": text, "expected": text}), 1), "normalize": S.arr(S.obj({"line": text, "expected": text}), 1),
 "external": S.arr(S.obj({"line": text, "expected": S.bool}), 1), "placeless": S.arr(S.obj({"line": text, "expected": text, "defect": text}), 1), "subjects": S.arr(S.obj({"line": text, "expected": S.strs(), "defect": text}), 1),
 "drop": S.arr(S.obj({"name": text, "base": S.strs(), "carved": S.strs(), "expected": S.strs()}), 1), "treeError": S.arr(S.obj({"message": text, "expected": S.bool}), 1), "hoist": S.arr(S.obj({"name": text, "text": text, "expected": text, "defect": text}), 1),
 "mounted": S.arr(S.obj({"name": text, "file": text, "text": text, "expected": S.strs()}), 1),
 "mount": S.obj({"files": S.map(text), "owned": S.arr(S.obj({"module": text, "paths": S.strs(), "root": text})), "crateModules": S.map(text), "expected": S.map(text)}),
 "outside": S.obj({"plugin": text, "files": S.map(text), "expected": S.strs()})})
corpus = {"version": 1, "modules": MODS, "rewrite": rw_rows, "aliases": al_rows, "normalize": norm_rows, "external": ext_rows, "placeless": pl_rows, "subjects": subj_rows, "drop": drop, "treeError": tree_rows, "hoist": hi_rows, "mounted": mp, "mount": mount, "outside": outside}
body = open(os.path.join(HERE, "u50_body.ts"), encoding="utf8").read()
ORACLE = open(os.path.join(HERE, "u50_oracle.py"), encoding="utf8").read()
oracle = lib.PY_HEAD.replace("{doc}", "Independent Python model of the sandbox diagnostics filters (external, placeless, subjects, normalization, tree errors) and of the flat #[path] mount resolution over the corpus.") + ORACLE + lib.PY_TAIL
ts = lib.ts_wrap(UNIT, "🧫️ Contract: the sandbox helpers rewrite carved sources for the combined synthetic crate (crate and dependency paths, crate aliases), classify and normalize rustc diagnostics (external, placeless, unresolved subjects, base-known names, tree errors), lift misplaced inner attributes, list flat `#[path]` mounts, mount crates into one combined file and find directories mounted from outside a plugin; the corpus is language-agnostic, tree-sitter's Rust grammar checks every rewritten source and an independent Python model reaches the same verdicts for the diagnostics filters and the mount paths.",
 'import Parser from "web-tree-sitter";\nimport {createRequire} from "node:module";\nimport {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join} from "node:path";\nimport {dropBaseUnresolvedNames,external,hoistInnerAttributes,mountCrates,mountedOutsideDirectories,mountedPathFiles,normalizeCombined,placeless,rewriteCrateAliases,rewriteForCombined,treeError,unresolvedSubjects} from "@@' + UNIT + '/🟦.ts@@";', body, python=True)
lib.write_unit(UNIT, corpus, schema, ts, oracle)
