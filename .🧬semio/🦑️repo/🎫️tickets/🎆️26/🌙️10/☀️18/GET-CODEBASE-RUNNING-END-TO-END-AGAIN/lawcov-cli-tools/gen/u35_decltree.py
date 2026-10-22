import lib
from lib import S
V = "\ufe0f"
UNIT = "🏗authoring/📦crate-tree/🧱decl-tree"
ROOT = "/w/src/lib.rs"
root_text = '''#[path = "a/m.rs"] mod a;
#[cfg(test)] #[path = "t.rs"] mod t;
mod inline { #[path = "deep.rs"] mod d; mod plain; }
mod plain;
#[path = "comp.rs"] mod component;
pub use component::*;
'''
files = {"/w/src/a/m.rs": '#[path = "n.rs"] mod nested;\n', "/w/src/a/n.rs": "fn n() {}\n", "/w/src/t.rs": "fn t() {}\n", "/w/src/inline/deep.rs": "fn d() {}\n", "/w/src/comp.rs": "fn c() {}\n"}
decls = [
 {"id": "a", "raw": ["a"], "eff": ["a"], "file": "/w/src/a/m.rs", "scope": "/w/src", "inlineRoot": True, "testOnly": False, "isComponent": False, "parentId": None, "depthInFile": 0, "children": ["a::nested"], "inline": False},
 {"id": "a::nested", "raw": ["a", "nested"], "eff": ["a", "nested"], "file": "/w/src/a/n.rs", "scope": "/w/src/a", "inlineRoot": False, "testOnly": False, "isComponent": False, "parentId": "a", "depthInFile": 0, "children": [], "inline": False},
 {"id": "t", "raw": ["t"], "eff": ["t"], "file": "/w/src/t.rs", "scope": "/w/src", "inlineRoot": True, "testOnly": True, "isComponent": False, "parentId": None, "depthInFile": 0, "children": [], "inline": False},
 {"id": "inline", "raw": ["inline"], "eff": ["inline"], "file": None, "scope": "/w/src", "inlineRoot": True, "testOnly": False, "isComponent": False, "parentId": None, "depthInFile": 0, "children": ["inline::d", "inline::plain"], "inline": True},
 {"id": "inline::d", "raw": ["inline", "d"], "eff": ["inline", "d"], "file": "/w/src/inline/deep.rs", "scope": "/w/src/inline", "inlineRoot": True, "testOnly": False, "isComponent": False, "parentId": "inline", "depthInFile": 1, "children": [], "inline": False},
 {"id": "inline::plain", "raw": ["inline", "plain"], "eff": ["inline", "plain"], "file": None, "scope": "/w/src/inline", "inlineRoot": True, "testOnly": False, "isComponent": False, "parentId": "inline", "depthInFile": 1, "children": [], "inline": False},
 {"id": "plain", "raw": ["plain"], "eff": ["plain"], "file": None, "scope": "/w/src", "inlineRoot": True, "testOnly": False, "isComponent": False, "parentId": None, "depthInFile": 0, "children": [], "inline": False},
 {"id": "component", "raw": ["component"], "eff": [], "file": "/w/src/comp.rs", "scope": "/w/src", "inlineRoot": True, "testOnly": False, "isComponent": True, "parentId": None, "depthInFile": 0, "children": [], "inline": False},
]
reach = [("fn f() { super::a(); }", 1), ("mod m { fn f() { super::super::x(); } }", 1), ("mod m { mod n { super::super::super::x; } }", 1), ("super::super::x", 2), ("fn f() {}", 0), ("super::a; mod m { super::b; }", 1), ("mod m { super::a; }", 0), ("x::super::y", 1), ("", 0)]
uf = [
 {"name": "chains merge", "ops": [["union", "a", "b"], ["union", "c", "d"], ["union", "b", "c"]], "groups": [["a", "b", "c", "d"]], "separate": []},
 {"name": "unrelated sets stay apart", "ops": [["union", "a", "b"], ["union", "c", "d"], ["find", "e"]], "groups": [["a", "b"], ["c", "d"], ["e"]], "separate": [["a", "c"], ["a", "e"]]},
 {"name": "a repeated union is a no-op", "ops": [["union", "a", "b"], ["union", "b", "a"], ["union", "a", "a"]], "groups": [["a", "b"]], "separate": []},
 {"name": "a long chain compresses", "ops": [["union", "n%d" % i, "n%d" % (i + 1)] for i in range(40)], "groups": [["n%d" % i for i in range(41)]], "separate": []},
 {"name": "a star", "ops": [["union", "hub", "s%d" % i] for i in range(6)], "groups": [["hub"] + ["s%d" % i for i in range(6)]], "separate": []},
]
slice_root = '#[path = "a.rs"] mod a;\n#[path = "b.rs"] mod b;\n#[path = "c.rs"] mod c;\n'
slice_files = {"/w/src/a.rs": '#[path = "a1.rs"] mod a1;\n', "/w/src/a1.rs": "use super::x;\n", "/w/src/b.rs": "fn f() {}\n", "/w/src/c.rs": '#[cfg(test)] #[path = "ct.rs"] mod tests;\n', "/w/src/ct.rs": "fn t() {}\n"}
slices = {"root": slice_root, "files": slice_files, "expected": [{"members": ["a", "a::a1"], "attach": ["a"]}, {"members": ["b"], "attach": ["b"]}, {"members": ["c", "c::tests"], "attach": ["c"]}]}
corpus = {"version": 1, "rootFile": ROOT, "rootText": root_text, "files": files, "decls": decls, "reach": [{"text": t, "expected": e} for t, e in reach], "unionFind": uf, "slices": slices}
dec = S.obj({"id": S.str, "raw": S.strs(), "eff": S.strs(), "file": S.one(S.str, S.null), "scope": S.str, "inlineRoot": S.bool, "testOnly": S.bool, "isComponent": S.bool, "parentId": S.one(S.str, S.null), "depthInFile": S.int, "children": S.strs(), "inline": S.bool})
schema = S.obj({"version": S.const(1), "rootFile": S.str, "rootText": S.str, "files": S.map(S.str), "decls": S.arr(dec, 1), "reach": S.arr(S.obj({"text": S.str, "expected": S.int}), 1),
 "unionFind": S.arr(S.obj({"name": S.str, "ops": S.arr(S.strs(2)), "groups": S.arr(S.strs(1)), "separate": S.arr(S.strs(2))}), 1), "slices": S.obj({"root": S.str, "files": S.map(S.str), "expected": S.arr(S.obj({"members": S.strs(1), "attach": S.strs(1)}), 1)})})
body = '''const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const platform=(path:string):string=>process.platform==="win32"?"C:"+path.split("/").join("\\\\"):path;
const tableOf=(files:Record<string,string>):Map<string,string>=>new Map(Object.entries(files).map(([path,text])=>[resolve(platform(path)),text]));
const reader=(table:Map<string,string>)=>(path:string):string|null=>table.get(resolve(path))??null;
const posix=(path:string|null):string|null=>path===null?null:path.replace(/^[A-Z]:/u,"").split("\\\\").join("/");
const summary=(decl:ModuleDecl)=>({id:decl.id,raw:[...decl.raw],eff:[...decl.eff],file:posix(decl.file),scope:posix(decl.scope)!,inlineRoot:decl.inlineRoot,testOnly:decl.testOnly,isComponent:decl.isComponent,parentId:decl.parentId,depthInFile:decl.depthInFile,children:[...decl.children],inline:decl.node.inline});
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
test("every module declaration of the crate is found through #[path] mounts with its effective path and scope",()=>{
 const tree=collectDecls(resolve(platform(corpus.rootFile)),corpus.rootText,reader(tableOf(corpus.files)));
 expect([...tree.decls.values()].map(summary)).toEqual(corpus.decls);expect(tree.roots).toEqual(corpus.decls.filter(decl=>decl.parentId===null).map(decl=>decl.id));
 const component=tree.decls.get("component")!;expect(component.globItem).not.toBeNull();expect(tree.decls.get("a")!.globItem).toBeNull();
});
test("a mounted file that cannot be read is skipped without failing",()=>{const tree=collectDecls(resolve(platform(corpus.rootFile)),corpus.rootText,()=>null);expect(tree.decls.size).toBe(corpus.decls.filter(decl=>decl.parentId===null||decl.id.startsWith("inline")).length);expect(tree.decls.get("a")!.children).toEqual([]);});
test("tree-sitter finds the same module ids and nesting",async()=>{
 const parser=await parsed(),table=tableOf(corpus.files),found:string[]=[];
 const walk=(nodes:Node[],raw:string[]):void=>{
  let attributes:Node[]=[];
  for(const node of nodes){
   if(node.type==="attribute_item"){attributes.push(node);continue;}
   if(node.type!=="mod_item"){attributes=[];continue;}
   const name=node.childForFieldName("name")!.text,id=[...raw,name].join("::"),body=node.childForFieldName("body"),texts=attributes.map(attribute=>attribute.text);attributes=[];
   found.push(id);
   if(body!==null)walk(body.namedChildren,[...raw,name]);else{const path=texts.map(text=>/path\\s*=\\s*"([^"]*)"/u.exec(text)?.[1]).filter(value=>value!==undefined).pop();if(path!==undefined){const file=resolve(dirname(raw.length===0?resolve(platform(corpus.rootFile)):[...table.keys()].find(key=>key.endsWith(path.replace(/\\//gu,"/").split("/").pop()!))!),path);const text=table.get(file);if(text!==undefined)walk(parser.parse(text).rootNode.namedChildren,[...raw,name]);}}
  }
 };
 walk(parser.parse(corpus.rootText).rootNode.namedChildren,[]);
 expect(found.sort()).toEqual(corpus.decls.map(decl=>decl.id).sort());
});
for(const row of corpus.reach)test("super reach of "+JSON.stringify(row.text),()=>{expect(superReach(row.text)).toBe(row.expected);});
for(const row of corpus.unionFind)test("union-find: "+row.name,()=>{
 const set=new UnionFind();for(const[op,a,b]of row.ops)if(op==="union")set.union(a!,b!);else set.find(a!);
 for(const group of row.groups){const roots=new Set(group.map(name=>set.find(name)));expect(roots.size,group.join()).toBe(1);}
 for(const[a,b]of row.separate)expect(set.find(a!)).not.toBe(set.find(b!));
 expect(set.find("never-seen")).toBe("never-seen");
});
test("slices keep coupled children and tests with their parent and everything else apart",()=>{
 const table=tableOf(corpus.slices.files),tree=collectDecls(resolve(platform("/w/src/lib.rs")),corpus.slices.root,reader(table)),result=buildSlices(tree,reader(table));
 expect(result.slices.map(slice=>({members:slice.decls.map(decl=>decl.id).sort(),attach:slice.attach.map(decl=>decl.id).sort()}))).toEqual(corpus.slices.expected);expect([...result.namespaces]).toEqual([]);
 const traces:string[]=[];buildSlices(tree,reader(table),message=>traces.push(message));expect(traces.some(message=>message.includes("union"))).toBe(true);
});'''
ts = lib.ts_wrap(UNIT, "🧱️ Contract: the crate declaration tree walks every module of a crate through the root file and its `#[path]` mounts with raw and effective paths, scopes, test flags and component globs, measures `super::` reach, groups declarations into slices that keep coupled children and test modules with their parent, and offers a union-find; the corpus is language-agnostic and tree-sitter's Rust grammar is the independent third-party oracle for the module ids and nesting.",
 'import Parser from "web-tree-sitter";\nimport {createRequire} from "node:module";\nimport {dirname,join,resolve} from "node:path";\nimport {UnionFind,buildSlices,collectDecls,superReach,type ModuleDecl} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts)
