/** 🔎️ Inventories pending semantic rejection assertions through the existing TypeScript parser. */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import {readdirSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const root="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets",ticket="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O";
const files:string[]=[];
function walk(p:string){for(const d of readdirSync(p,{withFileTypes:true})){const f=join(p,d.name);if(d.isDirectory())walk(f);else if(f.endsWith("/🧪️tests/🪶️sqlite/🟦️.ts"))files.push(f);}}
walk(root);
const rows=[];
for(const file of files){
 const s=readFileSync(file,"utf8"),tree=ts.createSourceFile(file,s,ts.ScriptTarget.Latest,true),positions=[];
 function visit(n:ts.Node){if(ts.isCallExpression(n)&&ts.isIdentifier(n.expression)&&n.expression.text==="expect"){let p:ts.Node=n;while(p.parent&&(ts.isPropertyAccessExpression(p.parent)||ts.isCallExpression(p.parent))){p=p.parent;if(ts.isPropertyAccessExpression(p)&&p.name.text==="rejects"){let top:ts.Node=p;while(top.parent&&(ts.isPropertyAccessExpression(top.parent)||ts.isCallExpression(top.parent)))top=top.parent;if(!ts.isAwaitExpression(top.parent)&&!ts.isReturnStatement(top.parent)){let fn:ts.Node|undefined=top.parent;while(fn&&!ts.isFunctionLike(fn))fn=fn.parent;const async=fn&&ts.canHaveModifiers(fn)&&ts.getModifiers(fn)?.some(m=>m.kind===ts.SyntaxKind.AsyncKeyword);positions.push({offset:n.getStart(tree),line:tree.getLineAndCharacterOfPosition(n.getStart(tree)).line+1,async:!!async,text:top.getText(tree)});}break;}}}ts.forEachChild(n,visit);}
 visit(tree);if(positions.length)rows.push({path:file,before:s,positions});
}
const result={scope:"Registered semantic SQLite Source leaves; read-only AST inventory",filesScanned:files.length,rows};
writeFileSync(join(ticket,"🗑️generated/root-semio-source-negative-promise-assertion-census-after.json"),JSON.stringify(result,null,2)+"\n");
const total=rows.reduce((n,r)=>n+r.positions.length,0);
if(total!==0)throw Error("unawaited negative matcher remains: "+total);
const report=join(ticket,"📓️semio-source-negative-promise-runtime-audit.md");
writeFileSync(report,readFileSync(report,"utf8")+"\n## Independent AST After Readback\n\nThe same TypeScript parser revisited "+files.length+" semantic leaves and found zero unawaited rejection assertions. This is structural verification, separate from the actual registered runtime after replay.\n");
console.log(JSON.stringify({filesScanned:files.length,unawaited:rows.reduce((n,r)=>n+r.positions.length,0),files:rows.map(r=>({path:r.path,positions:r.positions.map(p=>({line:p.line,async:p.async}))}))}));
