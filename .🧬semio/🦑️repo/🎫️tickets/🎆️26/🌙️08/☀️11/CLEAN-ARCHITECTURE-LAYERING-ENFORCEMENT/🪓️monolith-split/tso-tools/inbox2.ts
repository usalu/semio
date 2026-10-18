import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
const A="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/";
const FX=A+"📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧪️tests/🧫️fixtures/🔣️.json";
const fx=JSON.parse(readFileSync(FX,"utf8"));
const producers=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts");
const gen=(name:"shardWorkerSource"|"hostShimSource")=>ts.createSourceFile(name+".js",producers[name](),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
const worker=gen("shardWorkerSource"), shim=gen("hostShimSource");
const requests:string[]=[],replies:string[]=[],effects:string[]=[],effectReplies:string[]=[];
const visitWorker=(node:ts.Node):void=>{
  if(ts.isFunctionDeclaration(node)&&node.name?.text==="deliverEffectResult"){const visitReply=(child:ts.Node):void=>{if(ts.isBinaryExpression(child)&&child.left.getText(worker)==="kind"&&ts.isStringLiteral(child.right))effectReplies.push(child.right.text);ts.forEachChild(child,visitReply)};ts.forEachChild(node,visitReply);return}
  if(ts.isBinaryExpression(node)&&node.left.getText(worker)==="kind"&&ts.isStringLiteral(node.right)&&!requests.includes(node.right.text))requests.push(node.right.text);
  if(ts.isSwitchStatement(node)&&node.expression.getText(worker)==="kind")for(const clause of node.caseBlock.clauses)if(ts.isCaseClause(clause)&&ts.isStringLiteral(clause.expression)&&!requests.includes(clause.expression.text))requests.push(clause.expression.text);
  if(ts.isCallExpression(node)&&node.expression.getText(worker)==="self.postMessage"&&node.arguments[0]&&ts.isObjectLiteralExpression(node.arguments[0])){const tag=node.arguments[0].properties.find(item=>ts.isPropertyAssignment(item)&&item.name.getText(worker)==="kind");if(!tag||!ts.isPropertyAssignment(tag)||!ts.isStringLiteral(tag.initializer))throw new Error("Uninventoried worker post");replies.push(tag.initializer.text)}
  ts.forEachChild(node,visitWorker)};
const visitShim=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&["call","effectRequest"].includes(node.expression.getText(shim))&&node.arguments[0]&&ts.isStringLiteral(node.arguments[0]))effects.push(node.arguments[0].text);ts.forEachChild(node,visitShim)};
visitWorker(worker);visitShim(shim);
for(const [k,a] of [["workerRequestKinds",requests],["workerReplyKinds",replies],["hostEffectKinds",effects],["hostReplyKinds",effectReplies]] as const){
  const same=JSON.stringify(a)===JSON.stringify(fx[k]);console.log(k,same?"SAME":"DIFF");if(!same){console.log(" actual ",JSON.stringify(a));console.log(" fixture",JSON.stringify(fx[k]))}
  if(process.argv[2]==="--apply") fx[k]=a;
}
if(process.argv[2]==="--apply") writeFileSync(FX,JSON.stringify(fx,null,2)+"\n");
