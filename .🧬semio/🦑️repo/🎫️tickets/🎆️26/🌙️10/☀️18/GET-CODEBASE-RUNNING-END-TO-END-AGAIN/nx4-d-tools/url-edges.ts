import {existsSync,readFileSync,statSync} from "node:fs";
import {dirname,relative,resolve,sep} from "node:path";
import {isBuiltin} from "node:module";
import ts from "typescript";
const root=process.cwd();
const [entry]=process.argv.slice(2);
const skipFrom=(process.env.SKIP_FROM??"").split(",").filter(Boolean);
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
const closure=(seed:string,banned:Set<string>=new Set(),blockUrl=false)=>{const seen=new Set([seed]),q=[seed];while(q.length){const p=q.shift()!;if(skipFrom.some(x=>p.includes(x)))continue;for(const {spec,kind} of specs(p)){if(blockUrl&&kind==="url")continue;const r=res(p,spec);if(!r||seen.has(r)||banned.has(r)||!regular(r))continue;seen.add(r);q.push(r);}}return seen;};
const start=resolve(root,entry);
const full=closure(start);
const noUrl=closure(start,new Set(),true);
console.log("full",full.size,"static+dynamic only",noUrl.size);
const rows:{edge:string;adds:number}[]=[];
const urlEdges:{from:string;to:string}[]=[];
for(const p of full)for(const {spec,kind} of specs(p))if(kind==="url"){const r=res(p,spec);if(r&&regular(r))urlEdges.push({from:p,to:r});}
for(const e of urlEdges){const without=closure(start,new Set([e.to]));rows.push({edge:relative(root,e.from).split("/").slice(-3).join("/")+" => "+relative(root,e.to).split("/").slice(-4).join("/"),adds:full.size-without.size});}
rows.sort((a,b)=>b.adds-a.adds);
for(const r of rows.slice(0,25))console.log(String(r.adds).padStart(5),r.edge);
if(process.env.DUMP){
  const banned=new Set((process.env.BAN??"").split(",").filter(Boolean).map(x=>resolve(root,x)));
  const c=closure(start,banned,process.env.BLOCK_URL==="1");
  const now=Date.now()/1000;const rowsM=[...c].map(p=>({p:relative(root,p),age:now-statSync(p).mtimeMs/1000}));
  console.log("closure",c.size);
  for(const w of [600,1800,3600,3*3600,6*3600,86400])console.log("changed within",w,"s:",rowsM.filter(r=>r.age<w).length);
  const top=new Map<string,number>();for(const r of rowsM){const k=r.p.split("/").slice(0,5).join("/");top.set(k,(top.get(k)??0)+1);}
  for(const [k,v] of [...top].sort((a,b)=>b[1]-a[1]).slice(0,12))console.log(v,k);
  for(const r of rowsM.filter(r=>r.age<3600).sort((a,b)=>a.age-b.age).slice(0,40))console.log(Math.round(r.age),r.p.split("/").slice(-5).join("/"));
}
