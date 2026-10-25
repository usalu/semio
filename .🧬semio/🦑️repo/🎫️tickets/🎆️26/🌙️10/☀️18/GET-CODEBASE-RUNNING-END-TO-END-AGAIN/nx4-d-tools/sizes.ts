import {existsSync,readFileSync,statSync} from "node:fs";
import {dirname,relative,resolve,sep} from "node:path";
import {isBuiltin} from "node:module";
import ts from "typescript";
const root=process.cwd();
const entries=process.argv.slice(2).map(x=>resolve(root,x));
const regular=(p:string)=>existsSync(p)&&statSync(p).isFile();
const skip=(p:string)=>p.split(sep).some(x=>x==="node_modules"||x===".cache");
const cache=new Map<string,string[]>();
const boundaries=new Set(JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json","utf8")).spawnBoundaries??[]).size?new Set<string>((JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json","utf8")).spawnBoundaries as string[]).map(x=>resolve(root,x))):new Set<string>();
const specs=(path:string)=>{
  if(cache.has(path))return cache.get(path)!;
  const out:string[]=[];
  if(/\.[cm]?[jt]sx?$/u.test(path)){
    const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);
    const visit=(n:ts.Node)=>{
      if(ts.isImportDeclaration(n)&&(process.env.TYPES==="1"||!n.importClause?.isTypeOnly)&&ts.isStringLiteral(n.moduleSpecifier))out.push(n.moduleSpecifier.text);
      else if(ts.isExportDeclaration(n)&&(process.env.TYPES==="1"||!n.isTypeOnly)&&n.moduleSpecifier&&ts.isStringLiteral(n.moduleSpecifier))out.push(n.moduleSpecifier.text);
      else if(ts.isCallExpression(n)&&n.arguments.length>=1&&ts.isStringLiteralLike(n.arguments[0]!)){const c=n.expression;if(c.kind===ts.SyntaxKind.ImportKeyword||ts.isIdentifier(c)&&["require","__require"].includes(c.text))out.push((n.arguments[0] as ts.StringLiteral).text);}
      else if(ts.isNewExpression(n)&&ts.isIdentifier(n.expression)&&n.expression.text==="URL"&&n.arguments?.length===2&&ts.isStringLiteralLike(n.arguments[0]!)&&n.arguments[1]!.getText()==="import.meta.url")out.push("URL:"+(n.arguments[0] as ts.StringLiteral).text);
      ts.forEachChild(n,visit);
    };visit(file);
  }
  cache.set(path,out);return out;
};
const res=(from:string,spec0:string)=>{
  let spec=spec0;if(spec.startsWith("URL:")){spec=spec.slice(4);const t=resolve(dirname(from),spec);if(boundaries.has(t))return undefined;}
  if(isBuiltin(spec)||/^[a-z][a-z0-9+.-]*:/iu.test(spec))return undefined;
  try{const r=Bun.resolveSync(spec,dirname(from));return skip(r)||!r.startsWith(root)?undefined:r;}catch{if(spec.startsWith(".")){const p=resolve(dirname(from),spec);return regular(p)&&!skip(p)?p:undefined}return undefined;}
};
const union=new Set<string>();
const out:Record<string,string[]>={};
for(const e of entries){
  const seen=new Set([e]),q=[e];
  while(q.length){const p=q.shift()!;for(const spec of specs(p)){const r=res(p,spec);if(!r||seen.has(r)||!regular(r))continue;seen.add(r);q.push(r);}}
  out[relative(root,e)]=[...seen].map(x=>relative(root,x));
  for(const x of seen)union.add(x);
  console.log(String(seen.size).padStart(5),"tests",String([...seen].filter(x=>x.includes("🧪️tests")).length).padStart(3),relative(root,e).replace("🧰️framework/🛍️products/","").slice(-95));
}
console.log("union",union.size);
if(process.env.OUT)await Bun.write(process.env.OUT,JSON.stringify(out));
