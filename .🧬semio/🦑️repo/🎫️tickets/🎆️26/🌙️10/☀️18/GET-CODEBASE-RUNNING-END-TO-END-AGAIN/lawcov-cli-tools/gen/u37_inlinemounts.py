import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🌲inline-mounts"
ROOT = "/w/src/lib.rs"
inner = '//! doc\n#[path = "."] mod inner {\n    #[path = "a.rs"] mod y;\n}\n'
cases = [
 {"name": "a pure mount tree file is folded into the root with rebased paths", "root": '#[path = "m/mod.rs"] pub mod m;\nfn keep() {}\n', "files": {"/w/src/m/mod.rs": inner}, "expected": '#[path = "."] pub mod m {\n    #[path = "."] mod inner {\n        #[path = "m/a.rs"] mod y;\n    }\n}\nfn keep() {}\n', "deletes": ["/w/src/m/mod.rs"], "notes": ["folded the mount tree file m/mod.rs into the root module m"], "valid": True},
 {"name": "inner attributes and the file doc are dropped", "root": '#[path = "m/mod.rs"] pub mod m;\n', "files": {"/w/src/m/mod.rs": '#![allow(dead_code)]\n//! doc\n#[path = "."] pub mod inner {}\n'}, "expected": '#[path = "."] pub mod m {\n    #[path = "."] pub mod inner {}\n}\n', "deletes": ["/w/src/m/mod.rs"], "notes": ["folded the mount tree file m/mod.rs into the root module m"], "valid": True},
 {"name": "a file with items of its own stays", "root": '#[path = "m/mod.rs"] pub mod m;\n', "files": {"/w/src/m/mod.rs": 'fn own() {}\n#[path = "."] mod inner {}\n'}, "expected": '#[path = "m/mod.rs"] pub mod m;\n', "deletes": [], "notes": [], "valid": True},
 {"name": "a file without an inline module stays", "root": '#[path = "m/mod.rs"] pub mod m;\n', "files": {"/w/src/m/mod.rs": '#[path = "x.rs"] mod x;\n'}, "expected": '#[path = "m/mod.rs"] pub mod m;\n', "deletes": [], "notes": [], "valid": True},
 {"name": "an inline module with another path stays", "root": '#[path = "m/mod.rs"] pub mod m;\n', "files": {"/w/src/m/mod.rs": '#[path = "x"] mod inner {}\n'}, "expected": '#[path = "m/mod.rs"] pub mod m;\n', "deletes": [], "notes": [], "valid": True},
 {"name": "a test-only mount stays", "root": '#[cfg(test)] #[path = "m/mod.rs"] pub mod m;\n', "files": {"/w/src/m/mod.rs": inner}, "expected": '#[cfg(test)] #[path = "m/mod.rs"] pub mod m;\n', "deletes": [], "notes": [], "valid": True},
 {"name": "a mount that is already the root directory stays", "root": '#[path = "."] pub mod m;\n', "files": {}, "expected": '#[path = "."] pub mod m;\n', "deletes": [], "notes": [], "valid": True},
 {"name": "an unreadable mount stays", "root": '#[path = "gone.rs"] pub mod m;\n', "files": {}, "expected": '#[path = "gone.rs"] pub mod m;\n', "deletes": [], "notes": [], "valid": True},
 {"name": "an inline root module is never folded", "root": 'pub mod m { #[path = "."] mod inner {} }\n', "files": {}, "expected": 'pub mod m { #[path = "."] mod inner {} }\n', "deletes": [], "notes": [], "valid": True},
 {"name": "two mounts fold independently", "root": '#[path = "a/mod.rs"] pub mod a;\n#[path = "b/mod.rs"] pub mod b;\n', "files": {"/w/src/a/mod.rs": '#[path = "."] mod ia {}\n', "/w/src/b/mod.rs": '#[path = "."] mod ib { #[path = "z.rs"] mod z; }\n'}, "expected": '#[path = "."] pub mod a {\n    #[path = "."] mod ia {}\n}\n#[path = "."] pub mod b {\n    #[path = "."] mod ib { #[path = "b/z.rs"] mod z; }\n}\n', "deletes": ["/w/src/a/mod.rs", "/w/src/b/mod.rs"], "notes": ["folded the mount tree file a/mod.rs into the root module a", "folded the mount tree file b/mod.rs into the root module b"], "valid": True},
]
defects = [{"name": "a private mounted module folds into valid Rust", "root": '#[path = "m/mod.rs"] mod m;\n', "files": {"/w/src/m/mod.rs": '#[path = "."] mod inner {}\n'}, "expected": '#[path = "."] mod m {\n    #[path = "."] mod inner {}\n}\n', "defect": "🌲️inline-mounts/🟦️.ts:inlineMountTreeFiles prints `node.visibility` verbatim, which is the word `private` for a module without a modifier (parseRustSourceTree), so the folded module is written as `private mod m {` — not Rust"}]
corpus = {"version": 1, "rootFile": ROOT, "cases": cases, "defects": defects}
cs = S.obj({"name": S.str, "root": S.str, "files": S.map(S.str), "expected": S.str, "deletes": S.strs(), "notes": S.strs(), "valid": S.bool})
schema = S.obj({"version": S.const(1), "rootFile": S.str, "cases": S.arr(cs, 1), "defects": S.arr(S.obj({"name": S.str, "root": S.str, "files": S.map(S.str), "expected": S.str, "defect": S.str}), 1)})
body = '''const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
const platform=(path:string):string=>process.platform==="win32"?"C:"+path.split("/").join("\\\\"):path;
const table=(files:Record<string,string>):Map<string,string>=>new Map(Object.entries(files).map(([path,text])=>[resolve(platform(path)),text]));
const reader=(map:Map<string,string>)=>(path:string):string|null=>map.get(resolve(path))??null;
const posix=(path:string):string=>path.replace(/^[A-Z]:/u,"").split("\\\\").join("/");
const run=(root:string,files:Record<string,string>)=>inlineMountTreeFiles(resolve(platform(corpus.rootFile)),root,reader(table(files)));
for(const row of corpus.cases)test("inline mounts: "+row.name,()=>{const result=run(row.root,row.files);expect(result.rootSource).toBe(row.expected);expect([...result.deletes].map(posix).sort()).toEqual([...row.deletes].sort());expect(result.notes).toEqual(row.notes);});
for(const row of corpus.defects)test.failing("inline mounts defect: "+row.name,()=>{expect(run(row.root,row.files).rootSource).toBe(row.expected);});
test("every folded root is valid Rust (tree-sitter)",async()=>{
 await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);
 for(const row of corpus.cases.filter(item=>item.valid))expect(parser.parse(run(row.root,row.files).rootSource).rootNode.hasError(),row.name).toBe(false);
 for(const row of corpus.defects)expect(parser.parse(row.expected).rootNode.hasError(),row.name).toBe(false);
});
test("folding is idempotent: the folded root has nothing left to fold",()=>{for(const row of corpus.cases.filter(item=>item.deletes.length>0)){const again=run(row.expected,{});expect(again.rootSource,row.name).toBe(row.expected);expect(again.deletes.size).toBe(0);}});'''
ts = lib.ts_wrap(UNIT, "🌲️ Contract: a root-level file module made only of `#[path = \".\"]` inline mounts is folded back into the root as one inline mount wrapper with its relative `#[path]` attributes rebased to the root directory, while impure, test-only, unreadable or already folded mounts stay; the corpus is language-agnostic and tree-sitter's Rust grammar checks that every folded root is valid Rust.",
 'import Parser from "web-tree-sitter";\nimport {createRequire} from "node:module";\nimport {dirname,join,resolve} from "node:path";\nimport {inlineMountTreeFiles} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts)
