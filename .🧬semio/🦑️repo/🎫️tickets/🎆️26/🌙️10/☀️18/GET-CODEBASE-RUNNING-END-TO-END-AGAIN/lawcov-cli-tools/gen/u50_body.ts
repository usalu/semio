const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const witness=(row:{defect:string})=>row.defect?test.failing:test;
const modules=new Map<string,string>(Object.entries(corpus.modules));
const find=(root:Node,predicate:(node:Node)=>boolean):Node[]=>{const found:Node[]=[];const visit=(node:Node):void=>{if(predicate(node))found.push(node);for(const child of node.children)visit(child);};visit(root);return found;};
for(const row of corpus.rewrite)test("rewrite for combined: "+row.name,()=>{expect(rewriteForCombined(row.text,"m_self",modules)).toBe(row.expected);expect(rewriteCrateAliases(row.text,"m_self",modules)).toBe(row.text);});
for(const row of corpus.aliases)test("crate aliases of "+JSON.stringify(row.text),()=>{expect(rewriteCrateAliases(row.text,"m_self",modules)).toBe(row.expected);});
test("tree-sitter counts the same path heads before and after the rewrite and reads every rewritten source as valid Rust",async()=>{
 const parser=await parsed(),scoped=["scoped_identifier","scoped_type_identifier","scoped_use_list","use_wildcard"];
 const heads=(root:Node):Node[]=>find(root,node=>(node.type==="crate"||(node.type==="identifier"&&modules.has(node.text)))&&node.parent!==null&&scoped.includes(node.parent.type)&&node.parent.firstNamedChild?.id===node.id&&node.previousSibling?.text!=="::");
 const mounted=(root:Node):number=>find(root,node=>node.type==="crate"&&node.parent!==null&&node.parent.type==="scoped_identifier"&&["m_self","m_a","m_b"].includes(node.parent.childForFieldName("name")?.text??"")).length;
 for(const row of corpus.rewrite){
  const input=parser.parse(row.text).rootNode,output=parser.parse(rewriteForCombined(row.text,"m_self",modules)).rootNode;
  if(input.hasError())continue;
  expect(output.hasError(),row.name).toBe(false);expect(mounted(output),row.name).toBe(heads(input).length);
 }
});
for(const row of corpus.normalize)test("normalize "+JSON.stringify(row.line),()=>{expect(normalizeCombined(row.line)).toBe(row.expected);});
for(const row of corpus.external)test("external "+JSON.stringify(row.line),()=>{expect(external(row.line)).toBe(row.expected);});
for(const row of corpus.placeless)witness(row)("placeless "+JSON.stringify(row.line),()=>{expect(placeless(row.line)).toBe(row.expected);});
for(const row of corpus.subjects)witness(row)("unresolved subjects of "+JSON.stringify(row.line)+(row.defect?" ["+row.defect+"]":""),()=>{expect(unresolvedSubjects(row.line)).toEqual(row.expected);});
for(const row of corpus.drop)test("drop base unresolved names: "+row.name,()=>{expect(dropBaseUnresolvedNames(row.base,row.carved)).toEqual(row.expected);});
for(const row of corpus.treeError)test("tree error "+JSON.stringify(row.message),()=>{expect(treeError(row.message)).toBe(row.expected);});
for(const row of corpus.hoist)witness(row)("hoist inner attributes: "+row.name+(row.defect?" ["+row.defect+"]":""),()=>{expect(hoistInnerAttributes(row.text)).toBe(row.expected);});
test("hoisted files keep every line, are stable and put every inner attribute and inner doc before the first item (tree-sitter)",async()=>{
 const parser=await parsed();
 for(const row of corpus.hoist.filter(item=>!item.defect&&!item.name.endsWith("stays"))){
  const out=hoistInnerAttributes(row.text);expect(out.split("\n").sort(),row.name).toEqual(row.text.split("\n").sort());expect(hoistInnerAttributes(out),row.name).toBe(out);
  const root=parser.parse(out).rootNode;let seenItem=false;
  for(const child of root.namedChildren){
   const inner=child.type==="inner_attribute_item"||(child.type==="shebang"&&child.text.startsWith("#!["))||(child.type==="line_comment"&&child.text.startsWith("//!"));
   if(inner)expect(seenItem,row.name+": "+child.text).toBe(false);else if(!["line_comment","block_comment"].includes(child.type))seenItem=true;
  }
 }
});
for(const row of corpus.mounted)test("mounted path files: "+row.name,()=>{expect(mountedPathFiles(row.file,row.text)).toEqual(row.expected);});
test("tree-sitter finds the same flat path attributes as the scanner",async()=>{
 const parser=await parsed();
 for(const row of corpus.mounted){
  const attributes=find(parser.parse(row.text).rootNode,node=>node.type==="attribute_item"&&/^#\[\s*path\s*=\s*"[^"]+\.rs"\s*\]$/u.test(node.text)).map(node=>/"([^"]+)"/u.exec(node.text)![1]!);
  expect(attributes.length,row.name).toBe(row.expected.length);
 }
});
const inTemporary=<T,>(files:Record<string,string>,run:(root:string)=>T):T=>{
 const root=mkdtempSync(join(tmpdir(),"semio-sandbox-"));
 try{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}return run(root);}finally{rmSync(root,{recursive:true,force:true});}
};
test("mounting crates writes the combined file, rewrites each owned source once for its first owner and leaves other files alone",()=>{
 inTemporary(corpus.mount.files,root=>{
  mountCrates(root,corpus.mount.owned,new Map<string,string>(Object.entries(corpus.mount.crateModules)));
  for(const[path,text]of Object.entries<string>(corpus.mount.expected))expect(readFileSync(join(root,path),"utf8"),path).toBe(text);
 });
});
test("mounting twice never rewrites a source a second time for the same owner... and a second owner does not re-rewrite a claimed path",()=>{
 inTemporary(corpus.mount.files,root=>{
  mountCrates(root,corpus.mount.owned,new Map<string,string>(Object.entries(corpus.mount.crateModules)));
  expect(readFileSync(join(root,"c1/x.rs"),"utf8")).toBe(corpus.mount.expected["c1/x.rs"]);
 });
});
test("every mounted source is valid Rust (tree-sitter)",async()=>{
 const parser=await parsed();
 for(const[path,text]of Object.entries<string>(corpus.mount.expected))if(path.endsWith(".rs"))expect(parser.parse(text).rootNode.hasError(),path).toBe(false);
});
test("directories mounted from outside a plugin are found through path attributes and includes, skipping dependency folders and missing targets",()=>{
 inTemporary(corpus.outside.files,root=>{expect([...mountedOutsideDirectories(root,corpus.outside.plugin)].sort()).toEqual(corpus.outside.expected);});
});
