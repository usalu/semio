import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🗺plan/🧭module-chains"
chains = [
 ("a plain use", "use crate::a::b;", [["a", "b"]]), ("a group with a nested path", "use crate::a::{b, c::d};", [["a", "b"], ["a", "c", "d"]]), ("a group right after crate", "use crate::{a, b::c};", [["a"], ["b", "c"]]),
 ("nested groups", "use crate::a::{b::{c, d}, e};", [["a", "b", "c"], ["a", "b", "d"], ["a", "e"]]), ("expression paths", "fn f() { crate::a::g(); let _ = crate::h; }", [["a", "g"], ["h"]]), ("extern crate is not a path", "extern crate alloc;\nuse foo::crate::x;", []),
 ("a glob keeps its parent chain", "use crate::a::*;", [["a"]]), ("an alias keeps the path", "use crate::a::b as c;", [["a", "b"]]), ("comments and strings are not paths", "// crate::a::b\nlet s = \"crate::x::y\";", []), ("two paths in one call", "crate::a::b::c(crate::d::e)", [["a", "b", "c"], ["d", "e"]]),
 ("a path inside a type", "fn f(x: crate::m::T) -> crate::n::U {}", [["m", "T"], ["n", "U"]]), ("a bare crate", "let c = crate;", []),
]
alias = [("a used alias", "use crate::a::b;\nfn f() { b::c::d(); }", [["a", "b", "c", "d"]]), ("a renamed alias", "use crate::a::b as bb;\nfn f() { bb::x(); }", [["a", "b", "x"]]), ("a glob is no alias", "use crate::*;\nfn f() { x::y(); }", []),
         ("grouped aliases", "use crate::a::{b, c as d};\nfn f() { b::x(); d::y::z(); }", [["a", "b", "x"], ["a", "c", "y", "z"]]), ("a super use is no crate alias", "use super::a::b;\nb::c();", []), ("a one-segment target", "use crate::b;\nfn f() { b::c(); }", [["b", "c"]]),
         ("method-like and use positions are skipped", "use crate::a::b;\nfn f() { x.b::c(); }\nuse b::d;", []), ("no use at all", "fn f() { a::b(); }", [])]
root_src = '''pub mod schema { pub use super::a::b::*; }
pub mod other { pub use crate::x::y; pub use crate::x::y; }
mod mixed { pub use super::q; fn f() {} }
mod withpath;
#[path = "p.rs"] mod pathed { pub use super::z; }
mod empty {}
mod nest { pub use super::a::b; mod inner { pub use super::a::b; } }
mod badnest { pub use super::a::b; mod inner { pub use super::a::c; } }
'''
root_expected = [{"name": "schema", "targets": [["a", "b"]], "text": "pub mod schema { pub use super::a::b::*; }", "nested": []}, {"name": "other", "targets": [["x", "y"], ["x", "y"]], "text": "pub mod other { pub use crate::x::y; pub use crate::x::y; }", "nested": []}, {"name": "nest", "targets": [["a", "b"]], "text": "mod nest { pub use super::a::b; mod inner { pub use super::a::b; } }", "nested": ["inner"]}]
graphs = [
 {"name": "a chain lists dependencies first", "nodes": ["a", "b", "c"], "edges": {"a": ["b"], "b": ["c"]}, "components": [["c"], ["b"], ["a"]]},
 {"name": "a cycle forms one component", "nodes": ["a", "b", "c"], "edges": {"a": ["b"], "b": ["a"], "c": ["a"]}, "components": [["a", "b"], ["c"]]},
 {"name": "a self loop is one component", "nodes": ["a"], "edges": {"a": ["a"]}, "components": [["a"]]}, {"name": "disconnected nodes", "nodes": ["x", "y"], "edges": {}, "components": [["x"], ["y"]]},
 {"name": "nodes known only from edges are visited", "nodes": ["a"], "edges": {"a": ["z"]}, "components": [["a"], ["z"]]}, {"name": "an empty graph", "nodes": [], "edges": {}, "components": []},
 {"name": "two cycles joined by a one-way edge", "nodes": ["a", "b", "c", "d"], "edges": {"a": ["b", "c"], "b": ["a"], "c": ["d"], "d": ["c"]}, "components": [["a", "b"], ["c", "d"]]},
]
qual = [
 ("a head used in a use and a call", "use a::b;\nfn f() { a::g(); b::h(); }", ["a"], [], "use crate::a::b;\nfn f() { crate::a::g(); b::h(); }"), ("an excluded head", "use a::b;\nfn f() { a::g(); }", ["a"], ["a"], "use a::b;\nfn f() { a::g(); }"),
 ("a group that follows a path continues it", "use std::{a::x, b};", ["a"], [], "use std::{a::x, b};"), ("a leading group does not", "use {a::x, b::y};", ["a", "b"], [], "use {crate::a::x, crate::b::y};"),
 ("field, macro and nested paths are not heads", "x.a::y();\n$a::z;\nfoo::a::w;", ["a"], [], "x.a::y();\n$a::z;\nfoo::a::w;"), ("only the first segment of a path", "a::b::c", ["a", "b"], [], "crate::a::b::c"), ("self and super paths", "self::a::b; super::a::c;", ["a"], [], "self::a::b; super::a::c;"),
 ("a variable named like a module", "let a = 1; a::f();", ["a"], [], "let a = 1; crate::a::f();"), ("a group inside a path group", "use b::{a::x};", ["a", "b"], [], "use crate::b::{a::x};"), ("mixed groups", "use {b::{a::x}, a::y};", ["a", "b"], [], "use {crate::b::{a::x}, crate::a::y};"), ("no modules", "use a::b;", [], [], "use a::b;"), ("an unlisted head", "use z::b;", ["a"], [], "use z::b;"),
]
sup = [
 {"name": "super paths resolve against the module chain", "text": "fn f() { super::a::b(); super::super::c(); }", "from": 0, "to": 999, "excluded": [], "base": ["m", "n"], "expected": [["m", "a", "b"], ["c"]]},
 {"name": "more supers than modules are ignored", "text": "fn f() { super::super::super::c(); }", "from": 0, "to": 999, "excluded": [], "base": ["m"], "expected": []},
 {"name": "excluded ranges are skipped", "text": "fn f() { super::a(); }", "from": 0, "to": 999, "excluded": [[9, 20]], "base": ["m"], "expected": []},
 {"name": "a group continues the resolved chain", "text": "use super::{a, b::c};", "from": 0, "to": 999, "excluded": [], "base": ["m", "n"], "expected": [["m", "a"], ["m", "b", "c"]]},
 {"name": "a super that follows a path is not a head", "text": "x::super::a;", "from": 0, "to": 999, "excluded": [], "base": ["m"], "expected": []}
]
sup.append({"name": "the range limits the scan", "text": "super::a; super::b;", "from": 10, "to": 999, "excluded": [], "base": ["m", "n"], "expected": [["m", "b"]]})
rel = [("a mixed text", "fn f() { a::b::c(); self::d::e(); x.y::z::w(); $m::n::o; q::r; }", [["a", "b", "c"], ["q", "r"]]), ("a type path", "let v = Vec::new();", [["Vec", "new"]]), ("keywords are not heads", "crate::a::b; super::c::d;", []), ("one segment", "fn f() { a(); }", []), ("an empty text", "", [])]
heads = [("a use and calls", "use a::b; use c; use d as e; use {f, g::h, i as j}; k::l(); self::m(); super::n(); x.o::p(); crate::q::r();", ["a", "c", "crate", "d", "f", "g", "k", "self", "super"]), ("no paths", "let x = 1;", []), ("a path with a call", "a::b::c();", ["a"]), ("self paths", "self::x; super::y;", ["self", "super"]), ("a name after self that is itself a head", "self::x::y(); super::z::w();", ["self", "super", "x", "z"])]
impls = [
 ("an inherent impl", "impl Foo {}", {"trait": None, "self": "Foo", "traitArgs": []}), ("a trait impl", "impl Display for Foo {}", {"trait": "Display", "self": "Foo", "traitArgs": []}), ("generics and where", "impl<T: Clone> From<Bar<T>> for Baz<T> where T: Debug {}", {"trait": "From", "self": "Baz", "traitArgs": ["Bar", "T"]}),
 ("qualified paths", "impl fmt::Display for crate::m::Foo {}", {"trait": "Display", "self": "Foo", "traitArgs": []}), ("a dyn self type", "impl Tr for dyn Other {}", {"trait": "Tr", "self": "Other", "traitArgs": []}), ("an unsafe impl", "unsafe impl Send for X {}", {"trait": "Send", "self": "X", "traitArgs": []}),
 ("a local type in the trait arguments", "impl<T> Trait<Local<T>> for Vec<T> {}", {"trait": "Trait", "self": "Vec", "traitArgs": ["Local", "T"]}), ("no impl", "fn f() {}", None), ("a closing double angle", "impl<T> Tr<Vec<T>> for X<Y<T>> {}", {"trait": "Tr", "self": "X", "traitArgs": ["Vec", "T"]}),
]
node = lambda name, path=None: {"name": name, "pathAttribute": path}
bases = [{"parent": "/w/crate/src", "node": node("m"), "expected": "/w/crate/src/m"}, {"parent": "/w/crate/src", "node": node("m", "other"), "expected": "/w/crate/src/other"}, {"parent": "/w/crate/src", "node": node("m", "../up"), "expected": "/w/crate/up"}, {"parent": "/w", "node": node("m", "/abs"), "expected": "/abs"}]
files = [{"fileDir": "/w/src", "inline": "/w/src/m", "inFile": True, "node": node("x", "a/b.rs"), "expected": "/w/src/a/b.rs"}, {"fileDir": "/w/src", "inline": "/w/src/m", "inFile": False, "node": node("x", "a/b.rs"), "expected": "/w/src/m/a/b.rs"}, {"fileDir": "/w/src", "inline": "/w/src/m", "inFile": True, "node": node("x"), "expected": None}, {"fileDir": "/w/src", "inline": "/w/src/m", "inFile": False, "node": node("x"), "expected": None}]
corpus = {"version": 1, "chains": [{"name": n, "source": s, "expected": e} for n, s, e in chains], "aliases": [{"name": n, "source": s, "expected": e} for n, s, e in alias], "rootAlias": {"source": root_src, "expected": root_expected}, "graphs": graphs,
          "qualify": [{"name": n, "text": t, "modules": m, "exclude": x, "expected": e} for n, t, m, x, e in qual], "super": sup, "relative": [{"name": n, "text": t, "expected": e} for n, t, e in rel], "heads": [{"name": n, "text": t, "expected": e} for n, t, e in heads],
          "impls": [{"name": n, "text": t, "expected": e} for n, t, e in impls], "bases": bases, "files": files}
nodeS = S.obj({"name": S.str, "pathAttribute": S.one(S.str, S.null)})
chains_s = S.arr(S.arr(S.str))
schema = S.obj({"version": S.const(1), "chains": S.arr(S.obj({"name": S.str, "source": S.str, "expected": chains_s}), 1), "aliases": S.arr(S.obj({"name": S.str, "source": S.str, "expected": chains_s}), 1),
 "rootAlias": S.obj({"source": S.str, "expected": S.arr(S.obj({"name": S.str, "targets": chains_s, "text": S.str, "nested": S.strs()}))}),
 "graphs": S.arr(S.obj({"name": S.str, "nodes": S.strs(), "edges": S.map(S.strs()), "components": chains_s}), 1),
 "qualify": S.arr(S.obj({"name": S.str, "text": S.str, "modules": S.strs(), "exclude": S.strs(), "expected": S.str}), 1),
 "super": S.arr(S.obj({"name": S.str, "text": S.str, "from": S.int, "to": S.int, "excluded": S.arr(S.arr(S.int, 2)), "base": S.strs(), "expected": chains_s, "skip": S.bool}, required=["name", "text", "from", "to", "excluded", "base", "expected"]), 1),
 "relative": S.arr(S.obj({"name": S.str, "text": S.str, "expected": chains_s}), 1), "heads": S.arr(S.obj({"name": S.str, "text": S.str, "expected": S.strs()}), 1),
 "impls": S.arr(S.obj({"name": S.str, "text": S.str, "expected": S.one(S.obj({"trait": S.one(S.str, S.null), "self": S.one(S.str, S.null), "traitArgs": S.strs()}), S.null)}), 1),
 "bases": S.arr(S.obj({"parent": S.str, "node": nodeS, "expected": S.str}), 1), "files": S.arr(S.obj({"fileDir": S.str, "inline": S.str, "inFile": S.bool, "node": nodeS, "expected": S.one(S.str, S.null)}), 1)})
body = '''const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const segments=(node:Node):string[]=>{
 if(node.type==="identifier"||node.type==="type_identifier"||node.type==="crate"||node.type==="self"||node.type==="super")return[node.text];
 if(node.type==="scoped_identifier"||node.type==="scoped_type_identifier"){const path=node.childForFieldName("path"),name=node.childForFieldName("name")!;return[...(path?segments(path):[]),name.text];}
 return[];
};
const treeChains=(node:Node,out:string[][]):void=>{
 const rooted=(parts:string[]):string[]|null=>parts[0]==="crate"?parts.slice(1):null;
 const flatten=(current:Node,prefix:string[]):void=>{
  if(current.type==="scoped_use_list"){const path=current.childForFieldName("path"),list=current.childForFieldName("list")!,head=[...prefix,...(path?segments(path):[])];for(const child of list.namedChildren)flatten(child,head);return;}
  if(current.type==="use_list"){for(const child of current.namedChildren)flatten(child,prefix);return;}
  if(current.type==="use_as_clause"){flatten(current.childForFieldName("path")!,prefix);return;}
  if(current.type==="use_wildcard"){const path=current.namedChildren[0];const parts=[...prefix,...(path?segments(path):[])];const chain=rooted(parts);if(chain&&chain.length)out.push(chain);return;}
  const parts=[...prefix,...segments(current)],chain=rooted(parts);if(chain&&chain.length)out.push(chain);
 };
 if(node.type==="use_declaration"){flatten(node.childForFieldName("argument")!,[]);return;}
 if((node.type==="scoped_identifier"||node.type==="scoped_type_identifier")){const parts=segments(node),chain=rooted(parts);if(chain&&chain.length)out.push(chain);return;}
 for(const child of node.children)treeChains(child,out);
};
const sets=(rows:string[][]):string[]=>rows.map(row=>row.join("::")).sort();
const node=(row:{name:string;pathAttribute:string|null}):RustModuleNode=>({name:row.name,pathAttribute:row.pathAttribute} as unknown as RustModuleNode);
for(const row of corpus.bases)test("inline module base "+row.parent+" + "+JSON.stringify(row.node),()=>{expect(inlineModuleBase(resolve(row.parent),node(row.node))).toBe(resolve(row.expected));});
for(const row of corpus.files)test("external module file "+JSON.stringify(row.node)+" in file scope "+row.inFile,()=>{expect(externalModuleFile(resolve(row.fileDir),resolve(row.inline),row.inFile,node(row.node))).toBe(row.expected===null?null:resolve(row.expected));});
for(const row of corpus.chains)test("crate-relative chains: "+row.name,()=>{expect(crateRelativeChains(row.source)).toEqual(row.expected);});
test("tree-sitter reads the same crate-rooted paths",async()=>{
 const parser=await parsed();
 for(const row of corpus.chains){const root=parser.parse(row.source).rootNode;if(root.hasError())continue;const found:string[][]=[];treeChains(root,found);expect(sets(found),row.name).toEqual(sets(row.expected));}
});
for(const row of corpus.aliases)test("alias chains: "+row.name,()=>{expect(aliasChains(row.source)).toEqual(row.expected);});
test("root alias modules keep only pure re-export modules with consistent children",()=>{
 const tree=parseRustSourceTree(corpus.rootAlias.source),found=rootAliasModules(corpus.rootAlias.source,tree.modules);
 expect([...found.entries()].map(([name,value])=>({name,targets:value.targets,text:value.text,nested:[...value.nested].sort()}))).toEqual(corpus.rootAlias.expected);
});
for(const row of corpus.graphs)test("strongly connected components: "+row.name,()=>{
 const edges=new Map(Object.entries(row.edges).map(([from,to])=>[from,new Set(to)] as [string,Set<string>])),found=stronglyConnected(row.nodes,edges);
 expect(found.map(component=>[...component].sort()).sort((a,b)=>a.join()<b.join()?-1:1)).toEqual([...row.components].sort((a,b)=>a.join()<b.join()?-1:1));
 const position=new Map(found.flatMap((component,index)=>component.map(name=>[name,index] as [string,number])));
 for(const[from,to]of edges)for(const target of to)if(position.get(from)!==position.get(target))expect(position.get(target)!,from+"->"+target).toBeLessThan(position.get(from)!);
});
test("graphlib's Tarjan finds the same components",()=>{
 for(const row of corpus.graphs){
  const graph=new Graph({directed:true});for(const name of row.nodes)graph.setNode(name);for(const[from,to]of Object.entries(row.edges))for(const target of to)graph.setEdge(from,target);
  const found=alg.tarjan(graph).map(component=>[...component].sort()).sort((a,b)=>a.join()<b.join()?-1:1);expect(found,row.name).toEqual([...row.components].sort((a,b)=>a.join()<b.join()?-1:1));
 }
});
for(const row of corpus.qualify)test("module heads: "+row.name,()=>{expect(qualifyModuleHeads(row.text,new Set(row.modules),new Set(row.exclude))).toBe(row.expected);});
test("qualified text stays valid Rust and only inserts crate:: prefixes (tree-sitter)",async()=>{
 const parser=await parsed();
 for(const row of corpus.qualify){const before=parser.parse(row.text).rootNode.hasError(),text=qualifyModuleHeads(row.text,new Set(row.modules),new Set(row.exclude));if(!before)expect(parser.parse(text).rootNode.hasError(),row.name).toBe(false);expect(text.split("crate::").join("")).toBe(row.text.split("crate::").join(""));}
});
for(const row of corpus.super)test("super chains: "+row.name,()=>{expect(superChains(row.text,row.from,row.to,row.excluded as [number,number][],row.base)).toEqual(row.expected);});
for(const row of corpus.relative)test("relative head chains: "+row.name,()=>{expect(relativeHeadChains(row.text)).toEqual(row.expected);});
for(const row of corpus.heads)test("path head names: "+row.name,()=>{expect([...pathHeadNames(row.text)].sort()).toEqual(row.expected);});
for(const row of corpus.impls)test("impl header: "+row.name,()=>{expect(implHeader(row.text)).toEqual(row.expected);});
test("tree-sitter reads the same impl headers",async()=>{
 const parser=await parsed(),last=(node:Node|null):string|null=>{if(node===null)return null;if(node.type==="type_identifier"||node.type==="identifier")return node.text;if(node.type==="scoped_type_identifier")return node.childForFieldName("name")!.text;if(node.type==="generic_type")return last(node.childForFieldName("type"));if(node.type==="dynamic_type")return last(node.childForFieldName("trait"));if(node.type==="reference_type")return last(node.childForFieldName("type"));return null;};
 const args=(node:Node|null):string[]=>{if(node===null||node.type!=="generic_type")return[];const out:string[]=[];const visit=(current:Node):void=>{if(current.type==="type_identifier")out.push(current.text);for(const child of current.children)visit(child);};visit(node.childForFieldName("type_arguments")!);return out;};
 for(const row of corpus.impls.filter(item=>item.expected!==null)){
  const root=parser.parse(row.text).rootNode,item=root.namedChildren.find(candidate=>candidate.type==="impl_item"||candidate.type==="unsafe_item"||candidate.type==="function_item");if(item===undefined||item.type!=="impl_item")continue;
  const trait=item.childForFieldName("trait"),self=item.childForFieldName("type");
  expect({trait:last(trait),self:last(self),traitArgs:args(trait)},row.name).toEqual(row.expected as never);
 }
});'''
ts_ = lib.ts_wrap(UNIT, "🧭️ Contract: the Rust module-chain reader resolves inline and external module files, lists `crate::` chains, alias chains, root alias modules, `super::` chains, relative heads and path heads of a source, orders a dependency graph into strongly connected components, prefixes module heads with `crate::` and parses `impl` headers; the corpus is language-agnostic, tree-sitter's Rust grammar and graphlib's Tarjan are the independent third-party oracles.",
 'import Parser from "web-tree-sitter";\nimport {Graph,alg} from "graphlib";\nimport {createRequire} from "node:module";\nimport {dirname,join,resolve} from "node:path";\nimport {parseRustSourceTree,type RustModuleNode} from "@@🏗authoring/📦crate-tree/🌳module-tree/🟦.ts@@";\nimport {aliasChains,crateRelativeChains,externalModuleFile,implHeader,inlineModuleBase,pathHeadNames,qualifyModuleHeads,relativeHeadChains,rootAliasModules,stronglyConnected,superChains} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts_)
