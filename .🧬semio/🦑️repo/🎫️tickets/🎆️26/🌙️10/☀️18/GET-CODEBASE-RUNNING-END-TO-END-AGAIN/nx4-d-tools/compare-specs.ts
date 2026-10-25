import {readFileSync} from "node:fs";
import {resolve,dirname} from "node:path";
import ts from "typescript";
const root=process.cwd();
const I=resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference");
const parser=await import(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📥️inference/🟨️.mjs"));
const files=process.argv.slice(2);
let diffs=0;
for(const f of files){
  const path=resolve(root,f),text=readFileSync(path,"utf8");
  const product=new Set<string>([...parser.commandSourceImports(path,text)].map((x:any)=>typeof x==="string"?x:JSON.stringify(x)));
  const sf=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true),mine=new Set<string>();
  const visit=(n:ts.Node)=>{
    if(ts.isImportDeclaration(n)&&ts.isStringLiteral(n.moduleSpecifier))mine.add(n.moduleSpecifier.text);
    else if(ts.isExportDeclaration(n)&&n.moduleSpecifier&&ts.isStringLiteral(n.moduleSpecifier))mine.add(n.moduleSpecifier.text);
    else if(ts.isCallExpression(n)&&n.arguments.length>=1&&ts.isStringLiteralLike(n.arguments[0]!)){const c=n.expression;if(c.kind===ts.SyntaxKind.ImportKeyword||ts.isIdentifier(c)&&["require","__require"].includes(c.text))mine.add((n.arguments[0] as ts.StringLiteral).text);}
    ts.forEachChild(n,visit);
  };visit(sf);
  const onlyProduct=[...product].filter(x=>!mine.has(x)),onlyMine=[...mine].filter(x=>!product.has(x));
  if(onlyProduct.length||onlyMine.length){diffs++;console.log(f.slice(-70),"onlyProduct",JSON.stringify(onlyProduct).slice(0,300),"onlyMine",JSON.stringify(onlyMine).slice(0,200));}
}
console.log("files",files.length,"diffs",diffs);
