import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🧬mutation-families/🌳use-tree"
def leaf(path, leaf): return {"path": path, "leaf": leaf}
flat = [
 ("a plain path", "use a::b;", [leaf(["a", "b"], "b")]), ("a group with alias and nesting", "use a::{b, c as d, e::f};", [leaf(["a", "b"], "b"), leaf(["a", "c"], "d"), leaf(["a", "e", "f"], "f")]), ("self names the parent", "use a::{self, b};", [leaf(["a"], "a"), leaf(["a", "b"], "b")]),
 ("a glob is skipped", "pub use crate::x::*;", []), ("a nested glob is skipped", "use a::{b::*, c};", [leaf(["a", "c"], "c")]), ("a leading group", "use {a::b, c::d};", [leaf(["a", "b"], "b"), leaf(["c", "d"], "d")]), ("a leading colon", "use ::a::b;", [leaf(["a", "b"], "b")]),
 ("not a use", "fn f() {}", []), ("a dangling separator", "use a::;", []), ("a renamed self", "use a::b::{self as s};", [leaf(["a", "b"], "s")]), ("visibility is ignored", "pub(crate) use a::{b, c};", [leaf(["a", "b"], "b"), leaf(["a", "c"], "c")]), ("deep nesting", "use a::{b::{c::d, e}, f};", [leaf(["a", "b", "c", "d"], "d"), leaf(["a", "b", "e"], "e"), leaf(["a", "f"], "f")]),
]
prune = [
 ("a two-leaf group loses one and flattens", "use a::{b, c};", ["b"], "use a::c;"), ("every leaf dropped", "use a::{b, c};", ["b", "c"], None), ("nothing dropped re-renders", "use a::{b, c};", ["z"], "use a::{b, c};"), ("an alias is dropped by its alias name", "use a::{b as x, c};", ["x"], "use a::c;"),
 ("a lone self keeps its group", "use a::{self, b};", ["b"], "use a::{self};"), ("a plain use dropped", "use a::b;", ["b"], None), ("a plain use kept", "use a::b;", ["x"], "use a::b;"), ("nested groups collapse", "use a::{b::{c, d}, e};", ["c"], "use a::{b::d, e};"),
 ("the visibility prefix survives", "pub(crate) use a::{b, c};", ["b"], "pub(crate) use a::c;"), ("a glob left alone merges", "use a::{b, *};", ["b"], "use a::*;"), ("not a use statement", "fn f() {}", ["f"], "fn f() {}"), ("a missing semicolon is returned as is", "use a::{b, c}", ["b"], "use a::{b, c}"),
 ("an alias kept by its original name stays", "use a::{b as x, c};", ["b"], "use a::{b as x, c};"), ("a three-leaf group keeps the group", "use a::{b, c, d};", ["c"], "use a::{b, d};"),
]
corpus = {"version": 1, "flatten": [{"name": n, "statement": s, "expected": e} for n, s, e in flat], "prune": [{"name": n, "statement": s, "drop": d, "expected": e} for n, s, d, e in prune]}
lf = S.obj({"path": S.strs(), "leaf": S.str})
schema = S.obj({"version": S.const(1), "flatten": S.arr(S.obj({"name": S.str, "statement": S.str, "expected": S.arr(lf)}), 1), "prune": S.arr(S.obj({"name": S.str, "statement": S.str, "drop": S.strs(), "expected": S.one(S.str, S.null)}), 1)})
body = '''const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const segments=(node:Node):string[]=>{if(node.type==="scoped_identifier"){const path=node.childForFieldName("path"),name=node.childForFieldName("name")!;return[...(path?segments(path):[]),name.text];}return["identifier","crate","self","super"].includes(node.type)?[node.text]:[];};
const leaves=(statement:string,parser:Parser):{path:string[];leaf:string}[]=>{
 const declaration=parser.parse(statement).rootNode.namedChildren.find(node=>node.type==="use_declaration");if(declaration===undefined)return[];
 const out:{path:string[];leaf:string}[]=[];
 const walk=(node:Node,prefix:string[]):void=>{
  if(node.type==="scoped_use_list"){const path=node.childForFieldName("path"),list=node.childForFieldName("list")!;for(const child of list.namedChildren)walk(child,[...prefix,...(path?segments(path):[])]);return;}
  if(node.type==="use_list"){for(const child of node.namedChildren)walk(child,prefix);return;}
  if(node.type==="use_wildcard")return;
  const alias=node.type==="use_as_clause"?node.childForFieldName("alias")!.text:null,target=node.type==="use_as_clause"?node.childForFieldName("path")!:node,path=[...prefix,...segments(target)],clean=path.at(-1)==="self"?path.slice(0,-1):path;
  out.push({path:clean,leaf:alias??clean.at(-1)!});
 };
 walk(declaration.childForFieldName("argument")!,[]);return out;
};
for(const row of corpus.flatten)test("flatten: "+row.name,()=>{expect(flattenUse(row.statement)).toEqual(row.expected);});
test("tree-sitter flattens the same use leaves",async()=>{const parser=await parsed();for(const row of corpus.flatten.filter(item=>item.statement.includes("use ")&&item.statement.endsWith(";")&&!item.statement.endsWith("::;")))expect(leaves(row.statement,parser),row.name).toEqual(row.expected);});
for(const row of corpus.prune)test("prune: "+row.name,()=>{expect(pruneUseLeaves(row.statement,new Set(row.drop))).toBe(row.expected);});
test("pruned statements are valid Rust that bind exactly the surviving leaves (tree-sitter)",async()=>{
 const parser=await parsed();
 for(const row of corpus.prune.filter(item=>item.statement.endsWith(";")&&item.statement.startsWith(item.statement.includes("pub")?"pub":"use"))){
  const result=pruneUseLeaves(row.statement,new Set(row.drop));if(result===null){expect(leaves(row.statement,parser).filter(item=>!row.drop.includes(item.leaf)),row.name).toEqual([]);continue;}
  expect(parser.parse(result).rootNode.hasError(),result).toBe(false);expect(leaves(result,parser),row.name).toEqual(leaves(row.statement,parser).filter(item=>!row.drop.includes(item.leaf)));
 }
});'''
ts = lib.ts_wrap(UNIT, "🌳️ Contract: the use-tree module flattens a `use` statement into its full leaf paths (glob skipped, `self` naming its parent, aliases as leaves) and prunes named leaves out of groups of any nesting, collapsing single-leaf groups and answering null when nothing is left; the corpus is language-agnostic and tree-sitter's Rust grammar is the independent third-party oracle for the leaves and the validity of every pruned statement.",
 'import Parser from "web-tree-sitter";\nimport {createRequire} from "node:module";\nimport {dirname,join} from "node:path";\nimport {flattenUse,pruneUseLeaves} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts)
