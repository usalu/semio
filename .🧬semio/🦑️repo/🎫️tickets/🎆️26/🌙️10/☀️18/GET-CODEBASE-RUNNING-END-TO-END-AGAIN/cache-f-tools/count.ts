import ts from "typescript";
import {readFileSync} from "node:fs";
import {execSync} from "node:child_process";
const names=process.argv[2]!.split(",");
const files=execSync("rg -l -g '*.ts' -g '*.mjs' -e '"+names.join("\\(|")+"\\(' 🧰️framework 🌎️hub ✏️s",{cwd:"/Users/ueli/Documents/semio",encoding:"utf8"}).split("\n").filter(Boolean);
for(const f of files){const src=readFileSync("/Users/ueli/Documents/semio/"+f,"utf8"),tree=ts.createSourceFile(f,src,ts.ScriptTarget.Latest,true);let n=0;const walk=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&names.includes(node.expression.text))n++;ts.forEachChild(node,walk);};walk(tree);if(n)console.log(n,f);}
