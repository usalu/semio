const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const top=(root:Node,type:string):Node[]=>root.namedChildren.filter(child=>child.type===type);
const find=(root:Node,type:string):Node[]=>{const found:Node[]=[];const visit=(node:Node):void=>{if(node.type===type)found.push(node);for(const child of node.children)visit(child);};visit(root);return found;};
const withDirectory=<T,>(taken:readonly string[],run:(directory:string)=>T):T=>{
 const root=mkdtempSync(join(tmpdir(),"semio-support-hoist-")),directory=join(root,"agg");mkdirSync(directory);
 try{for(const name of taken)mkdirSync(join(directory,name));return run(directory);}finally{rmSync(root,{recursive:true,force:true});}
};
const hoist=(row:any,directory:string)=>hoistAggregateSupport({aggregateFile:join(directory,"mod.rs"),aggregateText:row.aggregate,aggregateName:"Agg",leafTexts:row.leaves,scope:aggregateScope(row.aggregate,row.eff,row.isComponent)});
const witness=(row:{defect:string})=>row.defect?test.failing:test;
for(const row of corpus.hoists)witness(row)("hoist: "+row.name+(row.defect?" ["+row.defect+"]":""),()=>{
 withDirectory(row.taken,directory=>{
  const result=hoist(row,directory),shown=result===null?null:{aggregateText:result.aggregateText,supportFile:result.supportFile.slice(directory.length+1).split("\\").join("/"),supportText:result.supportText,moved:[...result.moved]};
  expect(shown).toEqual(row.expect);
  const again=hoist(row,directory);expect(again).toEqual(result);
 });
});
test("every hoist leaves valid Rust behind, moves each body out of the aggregate and into the support file exactly once",async()=>{
 const parser=await parsed();
 for(const row of corpus.hoists){
  if(row.expect===null)continue;
  expect(parser.parse(row.expect.aggregateText).rootNode.hasError(),row.name+" aggregate").toBe(false);expect(parser.parse(row.expect.supportText).rootNode.hasError(),row.name+" support").toBe(false);
  const before=top(parser.parse(row.aggregate).rootNode,"function_item").map(node=>node.childForFieldName("name")!.text),after=top(parser.parse(row.expect.aggregateText).rootNode,"function_item").map(node=>node.childForFieldName("name")!.text),supported=top(parser.parse(row.expect.supportText).rootNode,"function_item").map(node=>node.childForFieldName("name")!.text);
  for(const name of row.expect.moved.filter(moved=>before.includes(moved))){expect(after,row.name+" "+name).not.toContain(name);expect(supported.filter(item=>item===name),row.name+" "+name).toHaveLength(1);}
  for(const name of before.filter(item=>!row.expect!.moved.includes(item)))expect(after,row.name+" "+name).toContain(name);
  expect(find(parser.parse(row.expect.aggregateText).rootNode,"mod_item").map(node=>node.childForFieldName("name")!.text),row.name).toContain("support");
 }
});
for(const row of corpus.impls)test("inherent impl head "+JSON.stringify(row.head),()=>{expect(inherentSelf(row.head)).toBe(row.expected);});
test("tree-sitter agrees which impl heads are inherent and which type they name",async()=>{
 const parser=await parsed();
 for(const row of corpus.impls){
  const impl=find(parser.parse(row.head+"{}").rootNode,"impl_item")[0]!,type=impl.childForFieldName("type");
  const oracle=impl.childForFieldName("trait")!==null||!type||type.type!=="type_identifier"&&type.type!=="generic_type"?null:(type.type==="generic_type"?type.childForFieldName("type")!.text:type.text);
  expect(oracle,row.head).toBe(row.expected);
 }
});
