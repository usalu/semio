const fs=require("node:fs"),path=require("node:path"),ts=require("typescript"),directory=__dirname,input=JSON.parse(fs.readFileSync(path.join(directory,"🗑️generated/schema-definitions-binding-plan.json"),"utf8")),rows=[],manual=[];
const isOld=n=>ts.isImportDeclaration(n)&&ts.isStringLiteral(n.moduleSpecifier)&&n.moduleSpecifier.text.includes("ajv/dist/2020");
for(const row of input.files){
 const approved=JSON.parse(fs.readFileSync(path.join(directory,"schema-manual-constructors.json"),"utf8")).rows.filter(r=>r.path===row.path).map(r=>r.offset),selected=(row.constructors??[]).filter(c=>c.schemas.length&&c.unresolved.length===0||approved.includes(c.start));if(!selected.length)continue;
 const text=fs.readFileSync(row.path,"utf8"),ast=ts.createSourceFile(row.path,text,ts.ScriptTarget.Latest,true),edits=[],existing=ast.statements.find(n=>ts.isImportDeclaration(n)&&ts.isStringLiteral(n.moduleSpecifier)&&n.moduleSpecifier.text==="ajv"&&n.importClause?.name),name=existing?.importClause.name.text??"AjvDraft07";let alias=false;
 const factories=new Set(selected.map(c=>c.factoryStart));
 for(const key of factories){
  const all=(row.constructors??[]).filter(c=>c.factoryStart===key),chosen=selected.filter(c=>c.factoryStart===key),owner=chosen[0];
  if(chosen.length===all.length)edits.push({start:owner.factoryStart,end:owner.factoryEnd,old:owner.module,replacement:"ajv"});
  else{alias=true;for(const c of chosen)edits.push({start:c.start,end:c.end,old:text.slice(c.start,c.end),replacement:name});}
 }
 function inspect(node){if(ts.isNewExpression(node)&&selected.some(c=>c.start===node.expression.getStart(ast))){let chain=node;while(chain.parent&&(ts.isPropertyAccessExpression(chain.parent)||ts.isCallExpression(chain.parent))){chain=chain.parent;if(ts.isCallExpression(chain)&&ts.isPropertyAccessExpression(chain.expression)&&chain.expression.name.text==="addMetaSchema"&&chain.arguments.length===1&&ts.isIdentifier(chain.arguments[0])){const alias=chain.arguments[0].text,owner=ast.statements.find(n=>ts.isImportDeclaration(n)&&ts.isStringLiteral(n.moduleSpecifier)&&n.moduleSpecifier.text==="ajv/dist/refs/json-schema-draft-07.json"&&n.importClause?.name?.text===alias);if(owner)edits.push({start:chain.expression.expression.end,end:chain.end,old:text.slice(chain.expression.expression.end,chain.end),replacement:""});}}}ts.forEachChild(node,inspect);}inspect(ast);
 if(alias&&!existing){if(text.includes("AjvDraft07"))throw Error("Hand-authored import name needed "+row.path);edits.push({start:0,end:0,old:"",replacement:'import AjvDraft07 from "ajv";\n'});}
 let proposed=text;for(const edit of [...edits].sort((a,b)=>b.start-a.start||b.end-a.end))proposed=proposed.slice(0,edit.start)+edit.replacement+proposed.slice(edit.end);
 const tree=ts.createSourceFile(row.path,proposed,ts.ScriptTarget.Latest,true);
 for(const declaration of tree.statements.filter(n=>ts.isImportDeclaration(n)&&ts.isStringLiteral(n.moduleSpecifier)&&n.moduleSpecifier.text==="ajv/dist/refs/json-schema-draft-07.json"&&n.importClause?.name)){
  const alias=declaration.importClause.name.text;let uses=0;function count(node){if(node===declaration)return;if(ts.isIdentifier(node)&&node.text===alias)uses++;ts.forEachChild(node,count);}count(tree);if(!uses){const original=ast.statements.find(n=>ts.isImportDeclaration(n)&&n.importClause?.name?.text===alias);edits.push({start:original.getFullStart(),end:original.end,old:text.slice(original.getFullStart(),original.end),replacement:""});}
 }
 rows.push({path:row.path,sha256:row.sha256,edits,constructors:selected.length});
}
for(const row of input.files){const constructors=(row.constructors??[]).filter(c=>c.unresolved.length||!c.schemas.length);if(constructors.length&&(row.constructors??[]).some(c=>c.schemas.length))manual.push({path:row.path,constructors});}
fs.writeFileSync(path.join(directory,"🗑️generated/schema-ajv-source-constructors.json"),JSON.stringify({version:1,rows,manual},null,2)+"\n");console.log(JSON.stringify({files:rows.length,constructors:rows.reduce((n,r)=>n+r.constructors,0),manualFiles:manual.length}));
