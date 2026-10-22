import json, lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🌳module-tree"
SRC1 = '''//! inner doc
#![allow(dead_code)]
use std::fmt;
pub use crate::a::{b, c as d};

#[cfg(test)]
mod tests {
    fn t() {}
}

/// docs
#[path = "other.rs"]
pub(crate) mod file_mod;

pub mod inline_mod {
    pub struct S;
    pub fn f() {}
    mod deeper { const X: u8 = 1; }
}

#[cfg(all(test, feature = "x"))]
mod only_tests;

const C: u8 = 1;
static ST: u8 = 2;
pub struct P { a: u8 }
enum E { A }
type T = u8;
pub trait Tr {}
impl Tr for P {}
macro_rules! m { () => {} }
extern crate alloc;
pub async fn af() {}
union U { a: u8 }
'''
def mod(name, vis, inline, ct, path, attrs, text, kids, items):
    return {"name": name, "visibility": vis, "inline": inline, "conditionalTest": ct, "pathAttribute": path, "attributes": attrs, "text": text, "children": kids, "items": items}
def it(kind, name, vis, text=None):
    d = {"kind": kind, "name": name, "visibility": vis}
    if text is not None: d["text"] = text
    return d
tree1 = {"innerAttributes": ["#![allow(dead_code)]"],
 "modules": [
  mod("tests", "private", True, True, None, ["cfg(test)"], "#[cfg(test)]\nmod tests {\n    fn t() {}\n}", [], [it("fn", "t", "private")]),
  mod("file_mod", "pub(crate)", False, False, "other.rs", ['path = "other.rs"'], '#[path = "other.rs"]\npub(crate) mod file_mod;', [], []),
  mod("inline_mod", "pub", True, False, None, [], "pub mod inline_mod {\n    pub struct S;\n    pub fn f() {}\n    mod deeper { const X: u8 = 1; }\n}", [mod("deeper", "private", True, False, None, [], "mod deeper { const X: u8 = 1; }", [], [it("const", "X", "private")])], [it("struct", "S", "pub"), it("fn", "f", "pub")]),
  mod("only_tests", "private", False, True, None, ['cfg(all(test, feature = "x"))'], '#[cfg(all(test, feature = "x"))]\nmod only_tests;', [], [])],
 "items": [it("use", None, "private", "use std::fmt;"), it("use", None, "pub", "pub use crate::a::{b, c as d};"), it("const", "C", "private", "const C: u8 = 1;"), it("static", "ST", "private", "static ST: u8 = 2;"), it("struct", "P", "pub", "pub struct P { a: u8 }"), it("enum", "E", "private", "enum E { A }"), it("type", "T", "private", "type T = u8;"), it("trait", "Tr", "pub", "pub trait Tr {}"),
           it("impl", "Tr", "private", "impl Tr for P {}"), it("macro", "m", "private", "macro_rules! m { () => {} }"), it("extern-crate", "alloc", "private", "extern crate alloc;"), it("fn", "af", "pub", "pub async fn af() {}"), it("struct", "U", "private", "union U { a: u8 }")]}
SRC2 = '''pub(super) mod a {
    pub(in crate::x) mod b {
        mod c;
    }
}
#[cfg(test)]
#[path = "x.rs"]
mod m;
#[cfg(not(test))] mod n;
'''
tree2 = {"innerAttributes": [], "modules": [
  mod("a", "pub(super)", True, False, None, [], "pub(super) mod a {\n    pub(in crate::x) mod b {\n        mod c;\n    }\n}", [mod("b", "pub(in crate::x)", True, False, None, [], "pub(in crate::x) mod b {\n        mod c;\n    }", [mod("c", "private", False, False, None, [], "mod c;", [], [])], [])], []),
  mod("m", "private", False, True, "x.rs", ["cfg(test)", 'path = "x.rs"'], '#[cfg(test)]\n#[path = "x.rs"]\nmod m;', [], []),
  mod("n", "private", False, False, None, ["cfg(not(test))"], "#[cfg(not(test))] mod n;", [], [])], "items": []}
SRC3 = ""
tree3 = {"innerAttributes": [], "modules": [], "items": []}
SRC4 = '''pub(crate) const fn cf() {}
unsafe impl Send for X {}
async unsafe fn au() {}
pub extern crate foo;
#[derive(Debug)] struct D;
fn with_body() { mod not_a_module { } }
pub(crate) static mut SM: u8 = 0;
'''
tree4 = {"innerAttributes": [], "modules": [], "items": [it("fn", "cf", "pub(crate)", "pub(crate) const fn cf() {}"), it("impl", "Send", "private", "unsafe impl Send for X {}"), it("fn", "au", "private", "async unsafe fn au() {}"), it("extern-crate", "foo", "pub", "pub extern crate foo;"), it("struct", "D", "private", "#[derive(Debug)] struct D;"), it("fn", "with_body", "private", "fn with_body() { mod not_a_module { } }"), it("static", "mut", "pub(crate)", "pub(crate) static mut SM: u8 = 0;")]}
sources = [{"name": "a mixed file", "text": SRC1, "tree": tree1, "oracle": True}, {"name": "nested and conditional modules", "text": SRC2, "tree": tree2, "oracle": True}, {"name": "an empty file", "text": SRC3, "tree": tree3, "oracle": True}, {"name": "item qualifiers", "text": SRC4, "tree": tree4, "oracle": False}]
defects = [{"name": "an extern \"C\" function is a function", "text": 'pub unsafe extern "C" fn uf() {}\n', "expectedItem": it("fn", "uf", "pub"), "defect": "🌳️module-tree/🟦️.ts:parseRustSourceTree skips the `extern` qualifier but not the ABI string literal that follows it, so `extern \"C\" fn` is classified `other` with no name"}]
cfg = [("cfg(test)", True), ("cfg( test )", True), ('cfg(all(test, feature = "x"))', True), ("cfg(any(test, test))", True), ("cfg(any(test, unix))", False), ("cfg(all(unix, windows))", False), ("cfg(not(test))", False), ('cfg(feature = "test")', False), ("allow(dead_code)", False), ("cfg(all())", False), ("cfg(any())", False),
       ("cfg(all(any(test), unix))", True), ("cfg(all(test))", True), ("  cfg(test)  ", True), ("cfg(any(all(test, unix), test))", True), ("cfg(all(unix, any(test, test)))", True), ("cfg(any(all(test, unix), unix))", False), ("cfg(test, unix)", False), ("cfg(testing)", False), ("cfg()", False), ("cfg_attr(test, derive(Debug))", False), ("cfg(all(test,))", True)]
docs = [
 ("/// a\nfn f() {}\n", "/// a\nfn f() {}\n"), ("fn f() {}\n/// dangling\n", "fn f() {}\n// dangling\n"), ("mod m {\n    fn f() {}\n    /// dangling\n}\n", "mod m {\n    fn f() {}\n    // dangling\n}\n"), ("/// a\n/// b\n\n// plain\n\nfn f() {}\n", "/// a\n/// b\n\n// plain\n\nfn f() {}\n"),
 ("fn f() {}\n/// a\n\n}\n", "fn f() {}\n// a\n\n}\n"), ("/// a\n// plain\n}\n", "// a\n// plain\n}\n"), ("//// four slashes\n}\n", "//// four slashes\n}\n"), ("", ""), ("no docs at all\n", "no docs at all\n"), ("/// a\r\nfn f() {}\r\n", "/// a\r\nfn f() {}\r\n"),
 ("fn f() {\n    /// stray\n    )\n", "fn f() {\n    // stray\n    )\n"), ("/// x\n  ]\n", "// x\n  ]\n")]
item_comments = [{"source": "a\n// c1\n// c2\nfn f() {}\n", "needle": "fn f", "expected": "// c1"}, {"source": "a\n\n// c1\n\nfn f() {}\n", "needle": "fn f", "expected": "fn f"}, {"source": "//! inner\nfn f() {}\n", "needle": "fn f", "expected": "fn f"}, {"source": "x\n/// doc\nfn f() {}\n", "needle": "fn f", "expected": "/// doc"}, {"source": "fn f() {}\n", "needle": "fn f", "expected": "fn f"}, {"source": "// only\n// two\nfn f() {}\n", "needle": "fn f", "expected": "// only"}]
outer_docs = [{"source": "x\n/// a\n/// b\nfn f() {}\n", "needle": "fn f", "expected": "/// a"}, {"source": "/// a\n//// not doc\nfn f() {}\n", "needle": "fn f", "expected": "fn f"}, {"source": "    /// a\n    fn f() {}\n", "needle": "fn f", "expected": "    /// a"}, {"source": "let x = 1; fn f() {}\n", "needle": "fn f", "expected": "fn f"}, {"source": "// plain\nfn f() {}\n", "needle": "fn f", "expected": "fn f"}, {"source": "fn f() {}\n", "needle": "fn f", "expected": "fn f"}]
pathv = [('path = "a/b.rs"', "a/b.rs"), ('path="x"', "x"), ('path = "a\\"b"', 'a\\"b'), ("path = x", None), ('cfg(test)', None), ('path = "a" extra', None), ('pathx = "a"', None), ('path = ""', "")]
leaves = [("use a::b;", ["b"]), ("use a::{b, c as d, e::f};", ["b", "d", "f"]), ("use a::{self, b};", ["a", "b"]), ("pub use crate::x::y as z;", ["z"]), ("pub(crate) use a::*;", []), ("use a::b::{c::{d, e}, f};", ["d", "e", "f"]), ("extern crate a;", []), ("use self::x;", ["x"]), ("use a::{b::*, c};", ["c"]), ("use ::a::b;", ["b"])]
leaf_defects = [{"statement": "use a::{b as c, self as d};", "expected": ["c", "d"], "defect": "🌳️module-tree/🟦️.ts:useLeafNames tests `tokens[a + 2] === \"as\"` for `self as d`, but `as` is at `a + 1`, so a renamed `self` import binds the parent name (`a`) instead of the alias (`d`)"}]
prune = [("use a::{b, c, d};", ["b", "d"], "use a::{b, d};"), ("use a::{b, c, d};", ["c"], "use a::c;"), ("pub use a::{b, c as d};", ["d"], "pub use a::c as d;"), ("use a::{b::{c, d}, e};", ["c", "e"], "use a::{b::c, e};"), ("use a::{b, *};", ["b"], "use a::b;"), ("use a::b;", ["b"], "use a::b;"), ("use a::{b};", ["z"], ""), ("use a::{self, b};", ["a"], "use a;"),
         ("use {a::b, c::d};", ["b", "d"], "use {a::b, c::d};"), ("use a::{/* x */ b, // y\n c};", ["b", "c"], "use a::{b, c};"), ("pub(crate) use a::{b, c};", ["c"], "pub(crate) use a::c;"), ("use a::{b::{c, d}, b::{e}};", ["c", "e"], "use a::b::{c, e};")]
idents = [("fn f(a: u8) -> Vec<T> { let x = a.b; x }", ["T", "Vec", "a", "b", "f", "fn", "let", "u8", "x"]), ("", []), ("// comment only\n", []), ('let s = "string ident";', ["let", "s"])]
refs = [("fn f(a: u8) { a.b; c::d; self::e; Foo { x: 1 }; g(h) }", ["Foo", "a", "c", "e", "f", "fn", "g", "h", "self", "u8"]), ("a::b::c", ["a"]), ("crate::x::y", ["crate", "x"]), ("super::z", ["super", "z"]), ("x.y.z", ["x"]), ("n: T", ["T"])]
corpus = {"version": 1, "sources": sources, "defects": defects, "cfg": [{"attribute": a, "expected": e} for a, e in cfg], "docs": [{"source": a, "expected": b} for a, b in docs], "itemComments": item_comments, "outerDocs": outer_docs, "pathValue": [{"attribute": a, "expected": b} for a, b in pathv],
          "leaves": [{"statement": a, "expected": b} for a, b in leaves], "leafDefects": leaf_defects, "prune": [{"statement": a, "leaves": b, "expected": c} for a, b, c in prune], "identifiers": [{"source": a, "expected": b} for a, b in idents], "references": [{"source": a, "expected": b} for a, b in refs]}
itd = S.obj({"kind": S.str, "name": S.one(S.str, S.null), "visibility": S.str, "text": S.str}, required=["kind", "name", "visibility"])
def modschema(depth=0):
    kids = S.arr(modschema(depth + 1)) if depth < 4 else S.arr(S.anyv)
    return S.obj({"name": S.str, "visibility": S.str, "inline": S.bool, "conditionalTest": S.bool, "pathAttribute": S.one(S.str, S.null), "attributes": S.strs(), "text": S.str, "children": kids, "items": S.arr(itd)})
tree_s = S.obj({"innerAttributes": S.strs(), "modules": S.arr(modschema()), "items": S.arr(itd)})
schema = S.obj({"version": S.const(1), "sources": S.arr(S.obj({"name": S.str, "text": S.str, "tree": tree_s, "oracle": S.bool}), 1), "defects": S.arr(S.obj({"name": S.str, "text": S.str, "expectedItem": itd, "defect": S.str}), 1),
 "cfg": S.arr(S.obj({"attribute": S.str, "expected": S.bool}), 1), "docs": S.arr(S.obj({"source": S.str, "expected": S.str}), 1), "itemComments": S.arr(S.obj({"source": S.str, "needle": S.str, "expected": S.str}), 1), "outerDocs": S.arr(S.obj({"source": S.str, "needle": S.str, "expected": S.str}), 1),
 "pathValue": S.arr(S.obj({"attribute": S.str, "expected": S.one(S.str, S.null)}), 1), "leaves": S.arr(S.obj({"statement": S.str, "expected": S.strs()}), 1), "leafDefects": S.arr(S.obj({"statement": S.str, "expected": S.strs(), "defect": S.str}), 1),
 "prune": S.arr(S.obj({"statement": S.str, "leaves": S.strs(), "expected": S.str}), 1), "identifiers": S.arr(S.obj({"source": S.str, "expected": S.strs()}), 1), "references": S.arr(S.obj({"source": S.str, "expected": S.strs()}), 1)})
body = '''const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const KEYWORDS=new Set(["fn","let","self","crate","super","mod","pub","use","struct","enum","impl","trait","const","static","type","as","mut","ref","in","for","if","else","match","loop","while","return","unsafe","async","await","move","where","dyn","extern","union","macro_rules","true","false"]);
const project=(item:Node,source:string):{kind:string;name:string|null;visibility:string}=>{
 const kinds:Record<string,string>={function_item:"fn",const_item:"const",static_item:"static",struct_item:"struct",union_item:"struct",enum_item:"enum",type_item:"type",trait_item:"trait",impl_item:"impl",macro_definition:"macro",extern_crate_declaration:"extern-crate",use_declaration:"use"};
 const kind=kinds[item.type]??"other",visibility=item.namedChildren.find(child=>child.type==="visibility_modifier")?.text??"private";
 const name=kind==="use"||kind==="impl"||kind==="other"?null:item.childForFieldName("name")?.text??item.namedChildren.find(child=>child.type==="identifier")?.text??null;
 return{kind,name:kind==="impl"?"@impl":name,visibility};
};
const structure=(body:Node[],source:string):{modules:unknown[];items:unknown[]}=>{
 const modules:unknown[]=[],items:unknown[]=[];let attributes:Node[]=[];
 for(const node of body){
  if(node.type==="attribute_item"){attributes.push(node);continue;}
  if(node.type==="line_comment"||node.type==="block_comment")continue;
  if(node.type==="mod_item"){
   const list=node.childForFieldName("body"),inner=list===null?{modules:[],items:[]}:structure(list.namedChildren,source),start=attributes[0]?.startIndex??node.startIndex,texts=attributes.map(attribute=>attribute.text.replace(/^#\\[/u,"").replace(/\\]$/u,""));
   const path=texts.map(text=>/^path\\s*=\\s*"((?:[^"\\\\]|\\\\.)*)"$/u.exec(text)?.[1]).filter(value=>value!==undefined).pop()??null;
   modules.push({name:node.childForFieldName("name")!.text,visibility:node.namedChildren.find(child=>child.type==="visibility_modifier")?.text??"private",inline:list!==null,attributes:texts,pathAttribute:path,text:source.slice(start,node.endIndex),children:inner.modules,items:inner.items});
  }else items.push(project(node,source));
  attributes=[];
 }
 return{modules,items};
};
const strip=(value:unknown):unknown=>{if(Array.isArray(value))return value.map(strip);if(value&&typeof value==="object"){const copy:Record<string,unknown>={};for(const[key,item]of Object.entries(value))if(key!=="text"||!("kind"in value))copy[key]=strip(item);return copy;}return value;};
const real=(source:string)=>{
 const tree=parseRustSourceTree(source),module=(node:RustModuleNode):unknown=>({name:node.name,visibility:node.visibility,inline:node.inline,conditionalTest:node.conditionalTest,pathAttribute:node.pathAttribute,attributes:[...node.attributeTexts],text:source.slice(node.start,node.end),children:node.children.map(module),items:node.items.map(item=>({kind:item.kind,name:item.name,visibility:item.visibility}))});
 return{innerAttributes:tree.innerAttributes.map(attribute=>attribute.text),modules:tree.modules.map(module),items:tree.items.map(item=>({kind:item.kind,name:item.name,visibility:item.visibility,text:source.slice(item.start,item.end)}))};
};
const sorted=(values:Iterable<string>):string[]=>[...values].sort();
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const leafNames=(node:Node):string[]=>{
 const argument=node.childForFieldName("argument")!;
 const walk=(current:Node):string[]=>{
  if(current.type==="identifier"||current.type==="crate"||current.type==="self"||current.type==="super")return current.type==="identifier"?[current.text]:[];
  if(current.type==="scoped_identifier"){const name=current.childForFieldName("name")!;return name.type==="identifier"?[name.text]:[];}
  if(current.type==="use_as_clause")return[current.childForFieldName("alias")!.text];
  if(current.type==="use_wildcard")return[];
  if(current.type==="scoped_use_list"||current.type==="use_list"){const list=current.type==="use_list"?current:current.childForFieldName("list")!;const path=current.type==="scoped_use_list"?current.childForFieldName("path"):null;return list.namedChildren.flatMap(child=>child.type==="self"?[path?.type==="scoped_identifier"?path.childForFieldName("name")!.text:path?.text??""]:walk(child));}
  return[];
 };
 return walk(argument);
};
for(const row of corpus.sources)test("module tree of "+row.name,()=>{expect(real(row.text)).toEqual(row.tree as never);});
for(const row of corpus.defects)test.failing("module tree defect: "+row.name,()=>{const tree=parseRustSourceTree(row.text);expect({kind:tree.items[0]!.kind,name:tree.items[0]!.name,visibility:tree.items[0]!.visibility}).toEqual(row.expectedItem);});
test("tree-sitter's Rust grammar reads the same module and item skeleton",async()=>{
 const parser=await parsed();
 for(const row of corpus.sources.filter(item=>item.oracle)){
  const root=parser.parse(row.text).rootNode;expect(root.hasError(),row.name).toBe(false);const isInner=(node:Node):boolean=>node.type==="inner_attribute_item"||(node.type==="shebang"&&node.text.startsWith("#!["));
  const found=structure(root.namedChildren.filter(node=>!isInner(node)),row.text);
  const expected=real(row.text);
  const withoutCfg=(value:any):any=>{const{conditionalTest:_,...rest}=value;return{...rest,children:value.children.map(withoutCfg),items:value.items.map((item:any)=>({kind:item.kind,name:item.kind==="impl"?"@impl":item.name,visibility:item.visibility}))};};
  const dropCfg=(value:any):any=>{const{conditionalTest:_,...rest}=value;return{...rest,children:value.children.map(dropCfg)};};
  expect((found.modules as any[]).map(dropCfg),row.name).toEqual(expected.modules.map(withoutCfg));
  expect(found.items,row.name).toEqual(expected.items.map(item=>({kind:item.kind,name:item.kind==="impl"?"@impl":item.name,visibility:item.visibility})).map(item=>item.kind==="use"?{...item,name:null}:item));
  expect(root.namedChildren.filter(isInner).map(node=>node.text)).toEqual(expected.innerAttributes);
 }
});
for(const row of corpus.cfg)test("cfg implies test: "+JSON.stringify(row.attribute),()=>{expect(cfgImpliesTest(row.attribute)).toBe(row.expected);});
for(const[index,row]of corpus.docs.entries())test("dangling doc demotion "+index,()=>{expect(demoteDanglingOuterDocs(row.source)).toBe(row.expected);});
for(const row of corpus.itemComments)test("item start over comments: "+JSON.stringify(row.source),()=>{const start=row.source.indexOf(row.needle),at=rustItemStartWithComments(row.source,start);expect(row.source.slice(at,at+row.expected.length)).toBe(row.expected);expect(row.source.indexOf(row.expected)).toBe(at);});
for(const row of corpus.outerDocs)test("outer doc start: "+JSON.stringify(row.source),()=>{const start=row.source.indexOf(row.needle),at=rustOuterDocStart(row.source,start);expect(row.source.slice(at,at+row.expected.length)).toBe(row.expected);});
for(const row of corpus.pathValue)test("path attribute value of "+JSON.stringify(row.attribute),()=>{expect(rustPathAttributeValue(row.attribute)).toBe(row.expected);});
for(const row of corpus.leaves)test("use leaf names of "+JSON.stringify(row.statement),()=>{expect(useLeafNames(row.statement)).toEqual(row.expected);});
for(const row of corpus.leafDefects)test.failing("use leaf names defect "+JSON.stringify(row.statement),()=>{expect(useLeafNames(row.statement)).toEqual(row.expected);});
test("tree-sitter derives the same use leaves",async()=>{
 const parser=await parsed();
 for(const row of corpus.leaves.filter(item=>item.statement.startsWith("use ")||item.statement.includes(" use "))){const root=parser.parse(row.statement).rootNode,declaration=root.namedChildren.find(node=>node.type==="use_declaration")!;expect(leafNames(declaration),row.statement).toEqual(row.expected);}
 for(const row of corpus.leafDefects){const root=parser.parse(row.statement).rootNode;expect(leafNames(root.namedChildren.find(node=>node.type==="use_declaration")!)).toEqual(row.expected);}
});
for(const row of corpus.prune)test("pruned use of "+JSON.stringify(row.statement)+" to "+JSON.stringify(row.leaves),()=>{expect(prunedUse(row.statement,row.leaves)).toBe(row.expected);});
test("every pruned use is valid Rust that binds exactly the kept leaves (tree-sitter)",async()=>{
 const parser=await parsed();
 for(const row of corpus.prune){
  const text=prunedUse(row.statement,row.leaves);if(text==="")continue;const root=parser.parse(text).rootNode;expect(root.hasError(),text).toBe(false);
  const bound=sorted(leafNames(root.namedChildren.find(node=>node.type==="use_declaration")!)),original=sorted(leafNames(parser.parse(row.statement).rootNode.namedChildren.find(node=>node.type==="use_declaration")!).filter(name=>row.leaves.includes(name)));
  expect(bound,text).toEqual(original);
 }
});
for(const row of corpus.identifiers)test("identifier tokens of "+JSON.stringify(row.source),()=>{expect(sorted(identifierSet(row.source))).toEqual(row.expected);});
test("identifier tokens are tree-sitter's identifiers plus Rust keywords",async()=>{
 const parser=await parsed();
 for(const row of corpus.identifiers){const root=parser.parse(row.source).rootNode,named=new Set<string>();const visit=(node:Node):void=>{if(["identifier","type_identifier","field_identifier","primitive_type"].includes(node.type))named.add(node.text);for(const child of node.children)visit(child);};visit(root);const found=identifierSet(row.source);for(const name of named)expect(found.has(name),name).toBe(true);for(const name of found)expect(named.has(name)||KEYWORDS.has(name),name).toBe(true);}
});
for(const row of corpus.references)test("referenced item names of "+JSON.stringify(row.source),()=>{expect(sorted(referencedItemNames(row.source))).toEqual(row.expected);});'''
ts_ = lib.ts_wrap(UNIT, "🌳️ Contract: the Rust module-tree reader parses the module and item skeleton of a file with exact text ranges, attributes, visibilities and `#[path]`, decides whether a `cfg` implies test, demotes dangling doc comments, extends starts over comments, and prunes and names `use` leaves; the corpus is language-agnostic and the tree-sitter Rust grammar is the independent third-party oracle for structure, `use` leaves, validity and identifiers, with `@@O@@` modelling the cfg logic and doc demotion.",
 'import Parser from "web-tree-sitter";\nimport {createRequire} from "node:module";\nimport {dirname,join} from "node:path";\nimport {cfgImpliesTest,demoteDanglingOuterDocs,identifierSet,parseRustSourceTree,prunedUse,referencedItemNames,rustItemStartWithComments,rustOuterDocStart,rustPathAttributeValue,useLeafNames,type RustModuleNode} from "@@' + UNIT + '/🟦.ts@@";', body)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent model of the cfg-implies-test predicate, the dangling-doc demotion, comment-start extension and path attribute reading (stdlib only).") + r'''
def implies(p):
    p = p.strip()
    if p == "test":
        return True
    m = re.match(r"^(all|any)\s*\((.*)\)$", p, re.S)
    if not m:
        return False
    body, args, depth, start = m.group(2), [], 0, 0
    for i, ch in enumerate(body):
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        elif ch == "," and depth == 0:
            args.append(body[start:i].strip())
            start = i + 1
    args.append(body[start:].strip())
    real = [a for a in args if a]
    return any(implies(a) for a in real) if m.group(1) == "all" else bool(real) and all(implies(a) for a in real)


def cfg(attr):
    m = re.match(r"^cfg\s*\((.*)\)$", attr.strip(), re.S)
    return m is not None and implies(m.group(1))


for c in corpus["cfg"]:
    if cfg(c["attribute"]) != c["expected"]:
        failures.append("cfg %r" % c["attribute"])
DOC = re.compile(r"^[ \t]*///(?!/)")


def demote(src):
    lines = src.split("\n")
    skippable = lambda l: l.strip() == "" or (re.match(r"^[ \t]*//", l) is not None and not DOC.match(l))
    i = 0
    while i < len(lines):
        if not DOC.match(lines[i]):
            i += 1
            continue
        end = i
        while end + 1 < len(lines) and DOC.match(lines[end + 1]):
            end += 1
        nxt = end + 1
        while nxt < len(lines) and skippable(lines[nxt]):
            nxt += 1
        if nxt >= len(lines) or re.match(r"^[ \t]*[})\]]", lines[nxt]):
            for k in range(i, end + 1):
                lines[k] = lines[k].replace("///", "//", 1)
        i = end + 1
    return "\n".join(lines)


for c in corpus["docs"]:
    if demote(c["source"]) != c["expected"]:
        failures.append("docs %r" % c["source"])
for c in corpus["pathValue"]:
    m = re.match(r'^path\s*=\s*"((?:[^"\\]|\\.)*)"$', c["attribute"])
    if (m.group(1) if m else None) != c["expected"]:
        failures.append("path %r" % c["attribute"])


def comment_start(src, start):
    cur = start
    while True:
        ls = src.rfind("\n", 0, max(cur - 1, 0)) + 1
        prev = src[ls:cur - 1]
        if cur <= 0 or not re.match(r"^\s*//(?!!)", prev) or prev.strip() == "":
            return cur
        cur = ls


for c in corpus["itemComments"]:
    at = comment_start(c["source"], c["source"].index(c["needle"]))
    if c["source"][at:at + len(c["expected"])] != c["expected"]:
        failures.append("item comments %r" % c["source"])
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts_, oracle)
