import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🪞unmirror/🧵use-tree"
def lf(path, alias=None, glob=False): return {"path": path, "alias": alias, "glob": glob}
src = '''#[cfg(test)]
pub(crate) use a::{b, c as d, e::f};
use crate::x::*;
fn f() {
    use std::fmt::{self, Display};
    use ::core::mem as m;
}
pub use self::g::h;
#[allow(unused)] #[doc = "x"] use k::l;
macro_rules! mm { ($x:ident) => { use $x::y; } }
'''
stmts = [
 {"text": "#[cfg(test)]\npub(crate) use a::{b, c as d, e::f};", "visibility": "pub(crate)", "attributes": ["#[cfg(test)]"], "keyword": "pub(crate) use", "leaves": [lf(["a", "b"]), lf(["a", "c"], "d"), lf(["a", "e", "f"])]},
 {"text": "use crate::x::*;", "visibility": "", "attributes": [], "keyword": "use", "leaves": [lf(["crate", "x"], None, True)]},
 {"text": "use std::fmt::{self, Display};", "visibility": "", "attributes": [], "keyword": "use", "leaves": [lf(["std", "fmt"]), lf(["std", "fmt", "Display"])]},
 {"text": "use ::core::mem as m;", "visibility": "", "attributes": [], "keyword": "use", "leaves": [lf(["core", "mem"], "m")]},
 {"text": "pub use self::g::h;", "visibility": "pub", "attributes": [], "keyword": "pub use", "leaves": [lf(["self", "g", "h"])]},
 {"text": '#[allow(unused)] #[doc = "x"] use k::l;', "visibility": "", "attributes": ["#[allow(unused)]", '#[doc = "x"]'], "keyword": "use", "leaves": [lf(["k", "l"])]},
]
skipped = ["use $x::y;", "use a::{b, ;", "use a::*::b;", "use a::{b c};", "let x = u.use;", "fn f() { s.use(1); }", "use ;", "use a::b"]
binding = [(lf(["a", "b"]), "b"), (lf(["a", "b"], "c"), "c"), (lf(["a"], None, True), None), (lf(["a", "b"], "_"), None), (lf([]), None), (lf(["a"]), "a"), (lf(["a", "_"]), None)]
render = [
 {"name": "leaves of one parent group together", "leaves": [lf(["a", "b"]), lf(["a", "c"], "d"), lf(["x", "y"]), lf(["m"], None, True)], "visibility": "", "attributes": [], "indent": "", "expected": "use a::{b, c as d};\nuse x::y;\nuse m::*;"},
 {"name": "duplicates collapse", "leaves": [lf(["a", "b"]), lf(["a", "b"])], "visibility": "pub", "attributes": [], "indent": "", "expected": "pub use a::b;"},
 {"name": "attributes and indentation precede every statement", "leaves": [lf(["a", "b"]), lf(["x", "y"])], "visibility": "pub(crate)", "attributes": ["#[cfg(test)]", "#[allow(x)]"], "indent": "    ", "expected": "#[cfg(test)]\n    #[allow(x)]\n    pub(crate) use a::b;\n    #[cfg(test)]\n    #[allow(x)]\n    pub(crate) use x::y;"},
 {"name": "root-level names", "leaves": [lf(["a"]), lf(["b"])], "visibility": "", "attributes": [], "indent": "", "expected": "use {a, b};"}, {"name": "a single root-level name", "leaves": [lf(["a"])], "visibility": "", "attributes": [], "indent": "", "expected": "use a;"}, {"name": "no leaves", "leaves": [], "visibility": "", "attributes": [], "indent": "", "expected": ""},
 {"name": "a glob and a name under one parent stay in one group", "leaves": [lf(["p", "q"]), lf(["p"], None, True)], "visibility": "", "attributes": [], "indent": "", "expected": "use p::{q, *};"},
]
corpus = {"version": 1, "source": src, "statements": stmts, "skipped": skipped, "binding": [{"leaf": l, "expected": e} for l, e in binding], "render": render}
leaf = S.obj({"path": S.strs(), "alias": S.one(S.str, S.null), "glob": S.bool})
schema = S.obj({"version": S.const(1), "source": S.str, "statements": S.arr(S.obj({"text": S.str, "visibility": S.str, "attributes": S.strs(), "keyword": S.str, "leaves": S.arr(leaf)}), 1), "skipped": S.strs(1), "binding": S.arr(S.obj({"leaf": leaf, "expected": S.one(S.str, S.null)}), 1),
 "render": S.arr(S.obj({"name": S.str, "leaves": S.arr(leaf), "visibility": S.str, "attributes": S.strs(), "indent": S.str, "expected": S.str}), 1)})
body = '''const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const segments=(node:Node):string[]=>{if(node.type==="scoped_identifier"){const path=node.childForFieldName("path"),name=node.childForFieldName("name")!;return[...(path?segments(path):[]),name.text];}return["identifier","crate","self","super"].includes(node.type)?[node.text]:[];};
const leavesOf=(declaration:Node):{path:string[];alias:string|null;glob:boolean}[]=>{
 const out:{path:string[];alias:string|null;glob:boolean}[]=[];
 const walk=(node:Node,prefix:string[]):void=>{
  if(node.type==="scoped_use_list"){const path=node.childForFieldName("path"),list=node.childForFieldName("list")!;for(const child of list.namedChildren)walk(child,[...prefix,...(path?segments(path):[])]);return;}
  if(node.type==="use_list"){for(const child of node.namedChildren)walk(child,prefix);return;}
  if(node.type==="use_wildcard"){const path=node.namedChildren[0];out.push({path:[...prefix,...(path?segments(path):[])],alias:null,glob:true});return;}
  const alias=node.type==="use_as_clause"?node.childForFieldName("alias")!.text:null,target=node.type==="use_as_clause"?node.childForFieldName("path")!:node,path=[...prefix,...segments(target)];
  if(path.at(-1)==="self")path.pop();out.push({path,alias,glob:false});
 };
 walk(declaration.childForFieldName("argument")!,[]);return out;
};
const declarations=(root:Node):Node[]=>{const found:Node[]=[];const visit=(node:Node):void=>{if(node.type==="use_declaration")found.push(node);for(const child of node.children)visit(child);};visit(root);return found;};
test("every use statement is found, with range, visibility, attributes and leaves",()=>{
 const scanned=scanSource(corpus.source),found=scanUseStatements(scanned);
 expect(found.map(row=>({text:corpus.source.slice(row.start,row.end),visibility:row.visibility,attributes:row.attributes,keyword:corpus.source.slice(row.keywordStart,row.keywordStart+row.visibility.length+(row.visibility?1:0)+3),leaves:row.leaves}))).toEqual(corpus.statements);
 expect(scanned.text).toBe(corpus.source);expect(scanned.tokens.length).toBeGreaterThan(10);
});
for(const text of corpus.skipped)test("a statement that is not a complete use tree is skipped: "+JSON.stringify(text),()=>{expect(scanUseStatements(scanSource(text))).toEqual([]);});
test("tree-sitter finds the same use declarations and leaves",async()=>{
 const parser=await parsed(),root=parser.parse(corpus.source).rootNode,real=scanUseStatements(scanSource(corpus.source)),oracle=declarations(root);
 expect(oracle.length).toBe(real.length);for(const[index,declaration]of oracle.entries())expect(leavesOf(declaration),declaration.text).toEqual(real[index]!.leaves);
});
for(const row of corpus.binding)test("binding of "+JSON.stringify(row.leaf),()=>{expect(leafBinding(row.leaf)).toBe(row.expected);});
for(const row of corpus.render)test("render: "+row.name,()=>{expect(renderUseLeaves(row.leaves,row.visibility,row.attributes,row.indent)).toBe(row.expected);});
test("rendered statements are valid Rust that scan back to the same leaves (tree-sitter and the scanner)",async()=>{
 const parser=await parsed();
 for(const row of corpus.render.filter(item=>item.leaves.length>0)){
  const text=renderUseLeaves(row.leaves,row.visibility,row.attributes,row.indent);expect(parser.parse(text).rootNode.hasError(),row.name).toBe(false);
  const leaves=scanUseStatements(scanSource(text)).flatMap(statement=>statement.leaves),key=(leaf:{path:string[];alias:string|null;glob:boolean})=>JSON.stringify([leaf.path,leaf.alias,leaf.glob]);
  expect(new Set(leaves.map(key)),row.name).toEqual(new Set(row.leaves.map(key)));
  expect(declarations(parser.parse(text).rootNode).flatMap(leavesOf).map(key).sort(),row.name).toEqual([...new Set(row.leaves.map(key))].sort());
 }
});'''
ts = lib.ts_wrap(UNIT, "🧵️ Contract: the unmirror use-tree module scans every `use` statement of a source (module level and function local, attributes and visibility included, macro variables and malformed trees skipped), names the binding of a leaf and renders leaves back grouped by parent; the corpus is language-agnostic and tree-sitter's Rust grammar is the independent third-party oracle for the declarations, leaves and the validity of rendered statements.",
 'import Parser from "web-tree-sitter";\nimport {createRequire} from "node:module";\nimport {dirname,join} from "node:path";\nimport {leafBinding,renderUseLeaves,scanSource,scanUseStatements} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts)
