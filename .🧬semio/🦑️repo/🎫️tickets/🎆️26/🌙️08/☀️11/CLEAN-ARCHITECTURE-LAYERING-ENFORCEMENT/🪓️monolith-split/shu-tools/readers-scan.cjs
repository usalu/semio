// finds string literals ending in 🟦️.ts(x) (not in import/export specifiers) or bare "🟦️.tsx" args in files under given roots
const fs=require("fs"),path=require("path");const ts=require("/Users/ueli/Documents/semio/node_modules/typescript");
const out=[];
const walk=(d)=>{for(const e of fs.readdirSync(d,{withFileTypes:true})){const p=path.join(d,e.name);if(e.isDirectory()){if(!/node_modules|🗑️generated|🧫️fixtures/.test(e.name))walk(p)}else if(/\.tsx?$/.test(e.name)){const t=fs.readFileSync(p,"utf8");const sf=ts.createSourceFile(p,t,ts.ScriptTarget.Latest,true,p.endsWith("x")?ts.ScriptKind.TSX:ts.ScriptKind.TS);
 const v=(n)=>{if(ts.isStringLiteralLike(n)&&/🟦️\.tsx?$/.test(n.text)&&!(n.parent&&(ts.isImportDeclaration(n.parent)||ts.isExportDeclaration(n.parent)||(ts.isCallExpression(n.parent)&&n.parent.expression.kind===ts.SyntaxKind.ImportKeyword)||(ts.isLiteralTypeNode(n.parent)))))out.push(p.replace(process.argv[2],"")+":"+(sf.getLineAndCharacterOfPosition(n.getStart(sf)).line+1)+" "+n.parent.getText(sf).slice(0,160));ts.forEachChild(n,v)};v(sf)}}};
walk(process.argv[2]);console.log(out.join("\n"));
