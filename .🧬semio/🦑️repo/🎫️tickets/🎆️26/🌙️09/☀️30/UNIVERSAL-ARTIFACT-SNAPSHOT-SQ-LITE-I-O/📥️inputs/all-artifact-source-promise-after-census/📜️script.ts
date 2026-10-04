/** 🔎️ Inventories pending semantic rejection assertions through the existing TypeScript parser. */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import {readdirSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const roots=["/Users/ueli/Documents/semio/✏️s/🔌️plugins","/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules"],ticket="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O";
const files:string[]=[];
function walk(p:string){for(const d of readdirSync(p,{withFileTypes:true})){const f=join(p,d.name);if(d.isDirectory())walk(f);else if(f.endsWith("/🧪️tests/🪶️sqlite/🟦️.ts"))files.push(f);}}
for(const root of roots)walk(root);
const rows=[];
for(const file of files){
 const s=readFileSync(file,"utf8"),tree=ts.createSourceFile(file,s,ts.ScriptTarget.Latest,true),positions=[];
 function visit(n:ts.Node){if(ts.isCallExpression(n)&&ts.isIdentifier(n.expression)&&n.expression.text==="expect"){let p:ts.Node=n;while(p.parent&&(ts.isPropertyAccessExpression(p.parent)||ts.isCallExpression(p.parent))){p=p.parent;if(ts.isPropertyAccessExpression(p)&&p.name.text==="rejects"){let top:ts.Node=p;while(top.parent&&(ts.isPropertyAccessExpression(top.parent)||ts.isCallExpression(top.parent)))top=top.parent;if(!ts.isAwaitExpression(top.parent)&&!ts.isReturnStatement(top.parent)){let fn:ts.Node|undefined=top.parent;while(fn&&!ts.isFunctionLike(fn))fn=fn.parent;const async=fn&&ts.canHaveModifiers(fn)&&ts.getModifiers(fn)?.some(m=>m.kind===ts.SyntaxKind.AsyncKeyword);positions.push({offset:n.getStart(tree),line:tree.getLineAndCharacterOfPosition(n.getStart(tree)).line+1,async:!!async,statement:ts.isExpressionStatement(top.parent),parentKind:ts.SyntaxKind[top.parent.kind],text:top.getText(tree)});}break;}}}ts.forEachChild(n,visit);}
 visit(tree);if(positions.length)rows.push({path:file,before:s,positions});
}
const result={scope:"Registered semantic SQLite Source leaves; read-only AST inventory",filesScanned:files.length,rows};
writeFileSync(join(ticket,"🗑️generated/root-all-artifact-source-negative-promise-assertion-census-after.json"),JSON.stringify(result,null,2)+"\n");
writeFileSync(join(ticket,"📓️all-artifact-source-negative-promise-after-audit.md"),"# All Artifact Source Negative Promise Runtime Audit\n\nRead-only TypeScript AST inventory of "+files.length+" current semantic SQLite Source leaves found "+rows.reduce((n,r)=>n+r.positions.length,0)+" unawaited rejection assertions in "+rows.length+" files. This broader inventory follows the actual Semio74/615 current replay after63 original negative awaits. Runtime BEFORE authorities for other owning packages must be retained separately. Only a matcher used as a direct expression statement inside its existing async owning test is eligible for a narrow await repair; returned/composed promises require readback before any edit. No production or assertion conditions change.\n\n"+rows.map(r=>r.path+"\n\n"+r.positions.map(p=>"Line "+p.line+", async="+p.async+": "+p.text).join("\n\n")).join("\n\n")+"\n");
console.log(JSON.stringify({filesScanned:files.length,unawaited:rows.reduce((n,r)=>n+r.positions.length,0),files:rows.length,eligibleDirectAsync:rows.reduce((n,r)=>n+r.positions.filter(p=>p.async&&p.statement).length,0),nonDirectOrNonAsync:rows.flatMap(r=>r.positions.filter(p=>!p.async||!p.statement).map(p=>({path:r.path,line:p.line,async:p.async,parent:p.parentKind}))).slice(0,30)}));
