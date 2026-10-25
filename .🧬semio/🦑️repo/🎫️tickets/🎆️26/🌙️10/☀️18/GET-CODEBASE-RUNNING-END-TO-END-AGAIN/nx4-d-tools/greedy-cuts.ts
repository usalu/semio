import {existsSync,readFileSync,statSync} from "node:fs";
import {dirname,relative,resolve,sep} from "node:path";
import {isBuiltin} from "node:module";
import ts from "typescript";
const root=process.cwd();
const [entry,stepsArg]=process.argv.slice(2);
const steps=Number(stepsArg??10);
const regular=(p:string)=>existsSync(p)&&statSync(p).isFile();
const skip=(p:string)=>p.split(sep).some(x=>x==="node_modules"||x===".cache");
type Edge={to:string;kind:string};
const adj=new Map<string,Edge[]>();
const parse=(path:string)=>{
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
  return out;
};
const res=(from:string,spec:string)=>{
  if(isBuiltin(spec)||/^[a-z][a-z0-9+.-]*:/iu.test(spec))return undefined;
  try{const r=Bun.resolveSync(spec,dirname(from));return skip(r)||!r.startsWith(root)?undefined:r;}catch{if(spec.startsWith(".")){const p=resolve(dirname(from),spec);return regular(p)&&!skip(p)?p:undefined}return undefined;}
};
const start=resolve(root,entry);
const extra=(process.env.EXTRA??"").split(",").filter(Boolean).map(x=>resolve(root,x));
const q=[start,...extra];adj.set(start,[]);for(const x of extra)adj.set(x,[]);
const seen=new Set([start,...extra]);
while(q.length){const p=q.shift()!;const edges:Edge[]=[];for(const {spec,kind} of parse(p)){const r=res(p,spec);if(!r||!regular(r))continue;edges.push({to:r,kind});if(!seen.has(r)){seen.add(r);adj.set(r,[]);q.push(r);}}adj.set(p,edges);}
const banned=(process.env.BAN_NODES??"").split(",").filter(Boolean);
const isBanned=(n:string)=>banned.some(b=>n.includes(b));
const reachWithout=(skipNode:string)=>{const s=new Set([start,...extra]),st=[start,...extra];while(st.length){const u=st.pop()!;for(const e of adj.get(u)!){if(e.to===skipNode||s.has(e.to)||isBanned(e.to))continue;s.add(e.to);st.push(e.to);}}return s;};
const short=(p:string)=>relative(root,p).replace("🧰️framework/","").split("/").slice(-4).join("/");
const total=reachWithout("\0").size;
console.log("closure",total);
const rows:{node:string;dom:number}[]=[];
for(const n of adj.keys()){if(n===start)continue;rows.push({node:n,dom:total-reachWithout(n).size});}
rows.sort((a,b)=>b.dom-a.dom);
const incoming=new Map<string,{from:string;kind:string}[]>();
for(const [u,es] of adj)for(const e of es){const l=incoming.get(e.to)??[];l.push({from:u,kind:e.kind});incoming.set(e.to,l);}
for(const r of rows.slice(0,Number(stepsArg??40))){console.log(String(r.dom).padStart(5),short(r.node),"<-",(incoming.get(r.node)??[]).map(x=>short(x.from).split("/").slice(-2).join("/")+"["+x.kind[0]+"]").join(", ").slice(0,200));}
