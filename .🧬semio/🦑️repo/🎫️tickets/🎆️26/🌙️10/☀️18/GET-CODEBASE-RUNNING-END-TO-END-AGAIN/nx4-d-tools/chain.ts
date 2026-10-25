import {existsSync,readFileSync,statSync} from "node:fs";
import {dirname,relative,resolve,sep} from "node:path";
import {isBuiltin} from "node:module";
import ts from "typescript";
const root=process.cwd();
const [entry,...needles]=process.argv.slice(2);
const regular=(p:string)=>existsSync(p)&&statSync(p).isFile();
const skip=(p:string)=>p.split(sep).some(x=>x==="node_modules"||x===".cache");
const cache=new Map<string,{spec:string;kind:string}[]>();
const specs=(path:string)=>{
  if(cache.has(path))return cache.get(path)!;
  const out:{spec:string;kind:string}[]=[];
  if(/\.[cm]?[jt]sx?$/u.test(path)){
    const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);
    const visit=(n:ts.Node)=>{
      if(ts.isImportDeclaration(n)&&!n.importClause?.isTypeOnly&&ts.isStringLiteral(n.moduleSpecifier))out.push({spec:n.moduleSpecifier.text,kind:"static"});
      else if(ts.isExportDeclaration(n)&&!n.isTypeOnly&&n.moduleSpecifier&&ts.isStringLiteral(n.moduleSpecifier))out.push({spec:n.moduleSpecifier.text,kind:"static"});
      else if(ts.isCallExpression(n)&&n.arguments.length>=1&&ts.isStringLiteralLike(n.arguments[0]!)){const c=n.expression;if(c.kind===ts.SyntaxKind.ImportKeyword||ts.isIdentifier(c)&&["require","__require"].includes(c.text))out.push({spec:(n.arguments[0] as ts.StringLiteral).text,kind:"dynamic"});}
      else if(ts.isNewExpression(n)&&ts.isIdentifier(n.expression)&&n.expression.text==="URL"&&n.arguments?.length===2&&ts.isStringLiteralLike(n.arguments[0]!)&&n.arguments[1]!.getText()==="import.meta.url")out.push({spec:(n.arguments[0] as ts.StringLiteral).text,kind:"url"});
      ts.forEachChild(n,visit);
    };visit(file);
  }
  cache.set(path,out);return out;
};
const res=(from:string,spec:string)=>{
  if(isBuiltin(spec)||/^[a-z][a-z0-9+.-]*:/iu.test(spec))return undefined;
  try{const r=Bun.resolveSync(spec,dirname(from));return skip(r)||!r.startsWith(root)?undefined:r;}catch{if(spec.startsWith(".")){const p=resolve(dirname(from),spec);return regular(p)&&!skip(p)?p:undefined}return undefined;}
};
const blockUrl=process.env.BLOCK_URL==="1",blockDynamic=process.env.BLOCK_DYNAMIC==="1";
const parent=new Map<string,{from:string;kind:string}|null>();
const start=resolve(root,entry);parent.set(start,null);const q=[start];
while(q.length){const p=q.shift()!;for(const {spec,kind} of specs(p)){if(blockUrl&&kind==="url"||blockDynamic&&kind==="dynamic")continue;const r=res(p,spec);if(!r||parent.has(r)||!regular(r))continue;parent.set(r,{from:p,kind});q.push(r);}}
const rel=(p:string)=>relative(root,p);
const chain=(p:string)=>{const rows:string[]=[];let c:string|undefined=p;while(c){const pr=parent.get(c);rows.push(rel(c).replace("🧰️framework/","").split("/").slice(-5).join("/")+(pr?` <-[${pr.kind}]`:""));c=pr?.from;}return rows;};
const all=[...parent.keys()];
for(const n of needles){const hits=all.filter(x=>x.includes(n));console.log("needle",n,hits.length);for(const h of hits.slice(0,2)){console.log(chain(h).join("\n   "));}}
