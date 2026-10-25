import {existsSync,readFileSync,statSync} from "node:fs";
import {dirname,join,relative,resolve,sep} from "node:path";
import {pathToFileURL,fileURLToPath} from "node:url";
import {isBuiltin} from "node:module";
import ts from "typescript";
const root=process.cwd();
const [entry,...needles]=process.argv.slice(2);
const regular=(p:string)=>existsSync(p)&&statSync(p).isFile();
const skip=(p:string)=>p.split(sep).some(x=>x==="node_modules"||x===".cache");
const specs=(path:string)=>{
  const out:{spec:string;kind:string}[]=[];
  if(!/\.[cm]?[jt]sx?$/u.test(path))return out;
  const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);
  const visit=(n:ts.Node)=>{
    if(ts.isImportDeclaration(n)&&!n.importClause?.isTypeOnly&&ts.isStringLiteral(n.moduleSpecifier))out.push({spec:n.moduleSpecifier.text,kind:"static"});
    else if(ts.isExportDeclaration(n)&&!n.isTypeOnly&&n.moduleSpecifier&&ts.isStringLiteral(n.moduleSpecifier))out.push({spec:n.moduleSpecifier.text,kind:"static"});
    else if(ts.isCallExpression(n)&&n.arguments.length>=1&&ts.isStringLiteralLike(n.arguments[0]!)){const c=n.expression;if(c.kind===ts.SyntaxKind.ImportKeyword||ts.isIdentifier(c)&&["require","__require"].includes(c.text))out.push({spec:(n.arguments[0] as ts.StringLiteral).text,kind:"dynamic"});}
    else if(ts.isNewExpression(n)&&ts.isIdentifier(n.expression)&&n.expression.text==="URL"&&n.arguments?.length===2&&ts.isStringLiteralLike(n.arguments[0]!)&&n.arguments[1]!.getText()==="import.meta.url")out.push({spec:(n.arguments[0] as ts.StringLiteral).text,kind:"url"});
    ts.forEachChild(n,visit);
  };visit(file);return out;
};
const resolveSpec=(from:string,spec:string):string|undefined=>{
  if(isBuiltin(spec)||/^[a-z][a-z0-9+.-]*:/iu.test(spec))return undefined;
  try{
    const r=spec.startsWith(".")?Bun.resolveSync(spec,dirname(from)):Bun.resolveSync(spec,dirname(from));
    return skip(r)||!r.startsWith(root)?undefined:r;
  }catch{
    if(spec.startsWith(".")){const p=resolve(dirname(from),spec);return regular(p)&&!skip(p)?p:undefined}
    return undefined;
  }
};
const skipFrom=(process.env.SKIP_FROM??'').split(',').filter(Boolean);
const parent=new Map<string,{from:string;kind:string}|null>();
const start=resolve(root,entry);parent.set(start,null);const queue=[start];
while(queue.length){const p=queue.shift()!;if(skipFrom.some(x=>p.includes(x)))continue;for(const {spec,kind} of specs(p)){const r=resolveSpec(p,spec);if(!r||parent.has(r)||!regular(r))continue;parent.set(r,{from:p,kind});queue.push(r);}}
const rel=(p:string)=>relative(root,p);
const all=[...parent.keys()].map(rel);
console.log("closure files",all.length,"tests",all.filter(x=>x.includes("🧪️tests")).length);
const chain=(p:string)=>{const rows:string[]=[];let c:string|undefined=p;while(c){const pr=parent.get(c);rows.push(rel(c).split("/").slice(-4).join("/")+(pr?` <-[${pr.kind}]`:""));c=pr?.from;}return rows;};
for(const n of needles){const hits=all.filter(x=>x.includes(n));console.log("needle",n,hits.length);for(const h of hits.slice(0,3)){console.log(chain(resolve(root,h)).join("\n   "));}}
if(!needles.length){const t=all.filter(x=>x.includes("🧪️tests")).slice(0,Number(process.env.SHOW??5));for(const h of t){console.log("--");console.log(chain(resolve(root,h)).join("\n   "));}}
