const Parser=require("web-tree-sitter");
let language;
/** 🌳 Reads visible Rust module items through the existing independent grammar, excluding implementation members. */
async function rustSurfaceRowsV1(source){
 await Parser.init();language??=await Parser.Language.load(require.resolve("tree-sitter-wasms/out/tree-sitter-rust.wasm"));
 const parser=new Parser();parser.setLanguage(language);const tree=parser.parse(source),rows=[];
 try{
  const visit=nodes=>{let conditional=false;for(const node of nodes){
   if(node.type==="attribute_item"){conditional||=/^#\s*\[\s*cfg\s*\(/u.test(node.text);continue}
   if(node.type==="ERROR")throw Error("Unparsed Rust module surface");
   if(node.type==="foreign_mod_item"){const body=node.namedChildren.find(child=>child.type==="declaration_list");if(body)visit(body.namedChildren);conditional=false;continue}
   const visibility=node.namedChildren.find(child=>child.type==="visibility_modifier")?.text.replace(/\s+/gu,"")??"",name=node.childForFieldName("name")?.text;
   if(name&&visibility&&visibility!=="pub(self)")rows.push({name,kind:node.type,visibility,modifiers:node.namedChildren.find(child=>child.type==="function_modifiers")?.text??"",conditional});
   conditional=false;
  }};visit(tree.rootNode.namedChildren);return rows;
 }finally{tree.delete();parser.delete()}
}
module.exports={rustSurfaceRowsV1};
