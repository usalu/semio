import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, relative, resolve } from "node:path";
const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),require=createRequire(join(root,"package.json")),ts=require("typescript"),owner="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo",[command,epoch="3",runEpoch="1"]=process.argv.slice(2),out=join(ticket,"🗑️generated/cargo-nested-owners/regression-"+epoch+(runEpoch==="1"?"":"-"+runEpoch)),sha=(s:string)=>createHash("sha256").update(s).digest("hex");
assert.equal(command,"run");assert.equal(existsSync(out),false);mkdirSync(out,{recursive:true});
const sourcePath=join(ticket,"🗑️generated/cargo-nested-owners/source-"+epoch+".json"),sourceRaw=readFileSync(sourcePath,"utf8"),source=JSON.parse(sourceRaw),checkout=join(out,"checkout"),frames:any[]=[],captured=new Set<string>();
function capture(path:string):void{
 assert.ok(path && !path.startsWith("../") && !path.startsWith("/") && !path.includes("\\"));if(captured.has(path))return;captured.add(path);assert.ok(captured.size<3000,"explicit verification floor exceeded");
 const proposal=source.pairs.find((row:any)=>row.path===path),full=join(root,path),before=existsSync(full)?readFileSync(full,"utf8"):null,body=proposal?proposal.after:before;assert.equal(before,proposal?proposal.before:before);assert.equal(typeof body,"string",path);
 const destination=join(checkout,path);mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,body);frames.push({path,before,body,beforeHash:before===null?null:sha(before),bodyHash:sha(body),proposed:!!proposal});
 if(!/\.(?:ts|mjs|cjs|js)$/u.test(path))return;
 const parsed=ts.createSourceFile(path,body,ts.ScriptTarget.Latest,true);assert.equal(parsed.parseDiagnostics.length,0,path);const imports:string[]=[];
 function visit(node:any):void{
  if((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier))imports.push(node.moduleSpecifier.text);
  if(ts.isImportEqualsDeclaration(node) && ts.isExternalModuleReference(node.moduleReference) && node.moduleReference.expression && ts.isStringLiteral(node.moduleReference.expression))imports.push(node.moduleReference.expression.text);
  if(ts.isCallExpression(node) && ts.isCallExpression(node.expression) && ts.isIdentifier(node.expression.expression) && node.expression.expression.text==="createRequire" && node.arguments[0] && ts.isStringLiteral(node.arguments[0]))imports.push(node.arguments[0].text);
  if(ts.isCallExpression(node) && (node.expression.kind===ts.SyntaxKind.ImportKeyword || ts.isIdentifier(node.expression) && node.expression.text==="require") && node.arguments[0] && ts.isStringLiteral(node.arguments[0]))imports.push(node.arguments[0].text);
  if(ts.isNewExpression(node) && ts.isIdentifier(node.expression) && node.expression.text==="URL" && node.arguments?.[0] && ts.isStringLiteral(node.arguments[0]))imports.push(node.arguments[0].text);
  ts.forEachChild(node,visit);
 }
 visit(parsed);
 for(const reference of imports)if(reference.startsWith(".")){const child=relative(root,resolve(root,dirname(path),reference)).replaceAll("\\","/");capture(child);}
}
for(const path of [owner+"/🧪️tests/🟦️.ts",owner+"/🧪️tests/🪆️nested-owners/🟦️.ts",owner+"/🛠️preparation/📜️script.ts"] )capture(path);
const nestedSchema=owner+"/🧬️schema/🪆️nested-owners/🔣️.json",nestedFixture=owner+"/🧫️fixtures/🪆️nested-owners/🔣️.json";capture(nestedSchema);capture(nestedFixture);capture(owner+"/🧬️schema/📇️discovery/🔣️.json");
for(const row of frames)assert.equal(existsSync(join(root,row.path))?readFileSync(join(root,row.path),"utf8"):null,row.before);assert.equal(readFileSync(sourcePath,"utf8"),sourceRaw);
const argv=[process.execPath,"test",join(checkout,owner,"🧪️tests/🟦️.ts"),join(checkout,owner,"🧪️tests/🪆️nested-owners/🟦️.ts")],env={...process.env,SEMIO_TEST_ARTIFACT_DIR:join(out,"native"),NX_WORKSPACE_ROOT:checkout};
const plan={producer:{path:import.meta.path,source:readFileSync(import.meta.path,"utf8")},sourcePath,sourceRaw,sourceHash:sha(sourceRaw),frames,argv,owningTestSelections:true,wholeRepositoryAcceptance:false};writeFileSync(join(out,"plan.json"),JSON.stringify(plan));
const actual=Bun.spawnSync(argv,{cwd:checkout,env,stdout:"pipe",stderr:"pipe"});writeFileSync(join(out,"actual.log"),Buffer.concat([actual.stdout,actual.stderr]));
const postchecks=frames.map(row=>({path:row.path,checkoutMatch:readFileSync(join(checkout,row.path),"utf8")===row.body,currentMatch:(existsSync(join(root,row.path))?readFileSync(join(root,row.path),"utf8"):null)===row.before}));assert.equal(readFileSync(sourcePath,"utf8"),sourceRaw);
const terminal={sourcePath,sourceHash:sha(sourceRaw),exitCode:actual.exitCode,frames:frames.length,postchecks,wholeRepositoryAcceptance:false};writeFileSync(join(out,"terminal.json"),JSON.stringify(terminal));console.log(JSON.stringify({exitCode:actual.exitCode,frames:frames.length,checkoutGaps:postchecks.filter(x=>!x.checkoutMatch).length,currentGaps:postchecks.filter(x=>!x.currentMatch).length,out}));process.exit(actual.exitCode);
