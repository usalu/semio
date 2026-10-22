import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import {isBuiltin} from "node:module";
import {captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1,nxAuthorizerInstalledPackagesV1,nxAuthorizerExternalModulesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
const root="/Users/ueli/Documents/semio",L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const entry=process.argv[2]??"🏗️build/⌨️command/⚙️runtime/🟦️.ts";
const s=await captureNxAuthorizerSourcesV1(root,L+entry,new AbortController().signal);
const bytes=nxAuthorizerCapturedBytesV1(s),imports=nxAuthorizerCapturedImportsV1(root,s);
const rows:any[]=[];
for(const [path,b] of bytes){
 if(!/\.[cm]?[jt]sx?$/.test(path))continue;
 const text=Buffer.from(b).toString("utf8").replace(/^#![^\r\n]*/,"");
 const sf=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true);
 const map=imports.get(path)??{};
 const visit=(n:ts.Node)=>{
  const at=()=>sf.getLineAndCharacterOfPosition(n.getStart()).line+1;
  if(ts.isCallExpression(n)){
   const c=n.expression,a0=n.arguments[0];const lit=a0&&ts.isStringLiteralLike(a0)?a0.text:null;
   const isCR=ts.isIdentifier(c)&&c.text==="createRequire";
   const isReq=ts.isIdentifier(c)&&(c.text==="require");
   const isImp=c.kind===ts.SyntaxKind.ImportKeyword;
   const isCRcall=ts.isCallExpression(c)&&ts.isIdentifier(c.expression)&&c.expression.text==="createRequire";
   const isResolve=ts.isPropertyAccessExpression(c)&&c.name.text==="resolve"&&(ts.isIdentifier(c.expression)&&/require/i.test(c.expression.text)||ts.isCallExpression(c.expression)&&ts.isIdentifier(c.expression.expression)&&c.expression.expression.text==="createRequire");
   if(isCR)rows.push({path,line:at(),kind:"createRequire",arg:a0?text.slice(a0.getStart(),a0.getEnd()).slice(0,80):""});
   if(isReq||isImp||isCRcall||isResolve)rows.push({path,line:at(),kind:isReq?"require":isImp?"import()":isCRcall?"createRequire()()":"resolve",lit,covered:lit===null?null:(lit.startsWith("node:")||isBuiltin(lit)||nxAuthorizerInstalledPackagesV1.includes(lit)||nxAuthorizerExternalModulesV1.includes(lit)||Object.hasOwn(map,lit))});
  }
  ts.forEachChild(n,visit);};
 visit(sf);
}
console.log(JSON.stringify(rows));
