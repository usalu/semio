import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import {existsSync,readFileSync,writeFileSync,mkdirSync,rmSync} from "node:fs";
import {createHash} from "node:crypto";
import {dirname,join,resolve,relative,sep,posix} from "node:path";

const ROOT="/Users/ueli/Documents/semio";
const REL="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🟦️.ts";
const OLD=join(ROOT,REL),OLD_DIR=dirname(OLD);
const HERE=import.meta.dir;
const STAGED=join(HERE,"staged");
const groups:Record<string,string>=JSON.parse(readFileSync(join(HERE,"groups.json"),"utf8"));
const DOCS:Record<string,string>={
 "🛂️admission":"Selected graph admission closes owners, sources and producers for exactly the requested targets.",
 "🧭️portable-cli":"The portable CLI, task selections and native caller keep literal arguments and original resources.",
 "📐️portable-model":"The portable graph model agrees with its independent schema, bundler and projection oracles.",
 "📌️pinned-publication":"Pinned graph publications are the only authority for nested consumers, caches and resets.",
 "🔭️observed-facts":"Observed graph facts retain exact reads, retries and replay under original custody.",
 "🔌️nx-engine":"The Nx projection and private engine references agree with the independent Nx implementation.",
 "🚀️dependency-bootstrap":"Build bootstrap and the dependency CLI enter online preparation through their defining modules.",
 "🧊️optional-cache":"The optional graph cache accounts peaks, denial, serialization and cleanup under original control.",
 "🌐️source-discovery":"Portable source and manifest discovery excludes generated sources and honours cancellation.",
 "🎯️source-demand":"Matched inputs, runtime modules and demanded compilation entries keep compiler authority.",
};
const SUPPORT="🧰️support";
const GROUP_ORDER=Object.keys(DOCS);

const SOURCE=process.env.SPLIT_TREE&&existsSync(join(process.env.SPLIT_TREE,REL))?join(process.env.SPLIT_TREE,REL):OLD;
const src=readFileSync(SOURCE,"utf8");
const sha=(t:string|Buffer)=>createHash("sha256").update(t).digest("hex");
const sf=ts.createSourceFile(OLD,src,ts.ScriptTarget.Latest,true);

type Rewrite={start:number;end:number;text:string};
const relSpec=(spec:string,newDir:string)=>{
  const abs=resolve(OLD_DIR,spec);
  let r=relative(newDir,abs).split(sep).join("/");
  if(r==="")r=".";
  if(!r.startsWith("../")&&r!=="."&&r!=="..")r="./"+r;
  if(spec.endsWith("/")&&!r.endsWith("/"))r+="/";
  return r;
};
const isRel=(s:string)=>/^\.\.?(\/|$)/.test(s);
const isImportMeta=(n:ts.Node|undefined,prop:string)=>!!n&&ts.isPropertyAccessExpression(n)&&n.name.text===prop&&ts.isMetaProperty(n.expression);

const rewritesFor=(node:ts.Node,newDir:string):Rewrite[]=>{
  const out:Rewrite[]=[];
  const lit=(n:ts.Node|undefined)=>{
    if(!n||!(ts.isStringLiteral(n)||ts.isNoSubstitutionTemplateLiteral(n))||!isRel(n.text))return;
    out.push({start:n.getStart(sf)+1,end:n.getEnd()-1,text:relSpec(n.text,newDir)});
  };
  const visit=(n:ts.Node)=>{
    if(ts.isImportDeclaration(n)||ts.isExportDeclaration(n))lit((n as any).moduleSpecifier);
    else if(ts.isImportTypeNode(n)&&ts.isLiteralTypeNode(n.argument))lit(n.argument.literal);
    else if(ts.isCallExpression(n)){
      const c=n.expression;
      if(c.kind===ts.SyntaxKind.ImportKeyword)lit(n.arguments[0]);
      else if(ts.isIdentifier(c)&&c.text==="require")lit(n.arguments[0]);
      else if(ts.isIdentifier(c)&&(c.text==="resolve"||c.text==="join")&&isImportMeta(n.arguments[0],"dir"))lit(n.arguments[1]);
    }
    else if(ts.isNewExpression(n)&&n.expression.getText(sf)==="URL"&&n.arguments&&n.arguments.length===2&&isImportMeta(n.arguments[1],"url"))lit(n.arguments[0]);
    ts.forEachChild(n,visit);
  };
  visit(node);
  return out;
};
const applyRewrites=(text:string,base:number,rw:Rewrite[])=>{
  let out=text;
  for(const r of [...rw].sort((a,b)=>b.start-a.start))out=out.slice(0,r.start-base)+r.text+out.slice(r.end-base);
  return out;
};

const identsOf=(nodes:ts.Node[]):Set<string>=>{
  const s=new Set<string>();
  const visit=(n:ts.Node)=>{
    if(ts.isIdentifier(n)){
      const p=n.parent;
      const isName=(ts.isPropertyAccessExpression(p)&&p.name===n)||(ts.isPropertyAssignment(p)&&p.name===n)||(ts.isMethodDeclaration(p)&&p.name===n)||(ts.isBindingElement(p)&&p.propertyName===n)||(ts.isPropertySignature(p)&&p.name===n)||(ts.isQualifiedName(p)&&p.right===n);
      if(!isName)s.add(n.text);
    }
    ts.forEachChild(n,visit);
  };
  for(const n of nodes)visit(n);
  return s;
};

type Stmt={node:ts.Statement;kind:"test"|"helper"|"hook"|"replicate";title?:string;line:number;declared:string[]};
const imports:ts.ImportDeclaration[]=[];
const sideEffects:ts.ImportDeclaration[]=[];
const stmts:Stmt[]=[];
const declaredNames=(st:ts.Statement):string[]=>{
  if(ts.isVariableStatement(st))return st.declarationList.declarations.flatMap(d=>ts.isIdentifier(d.name)?[d.name.text]:[]);
  if(ts.isFunctionDeclaration(st)&&st.name)return [st.name.text];
  if(ts.isTypeAliasDeclaration(st))return [st.name.text];
  return [];
};
for(const st of sf.statements){
  const line=sf.getLineAndCharacterOfPosition(st.getStart(sf)).line+1;
  if(ts.isImportDeclaration(st)){ (st.importClause?imports:sideEffects).push(st); continue; }
  if(ts.isExpressionStatement(st)&&ts.isCallExpression(st.expression)){
    const call=st.expression,callee=call.expression.getText(sf);
    if(callee==="test"||callee==="registerBuildTestLawV1"||callee.startsWith("test.skipIf(")||callee.startsWith("test.each(")){
      const first=call.arguments[0] as ts.StringLiteral;
      stmts.push({node:st,kind:"test",title:first.text,line,declared:[]}); continue;
    }
    if(callee==="beforeAll"){ stmts.push({node:st,kind:"hook",line,declared:[]}); continue; }
  }
  const text=st.getText(sf);
  const declared=declaredNames(st);
  if(!declared.length)throw Error("unclassified statement at line "+line+": "+text.slice(0,80));
  stmts.push({node:st,kind:/import\.meta/.test(text)?"replicate":"helper",line,declared});
}
const tests=stmts.filter(s=>s.kind==="test");
const titles=new Set<string>();
for(const t of tests){ if(titles.has(t.title!))throw Error("duplicate title "+t.title); titles.add(t.title!); if(!groups[t.title!])throw Error("no group for "+t.title); }
for(const t of Object.keys(groups))if(!titles.has(t))throw Error("group row without test: "+t);

const importBindings=(d:ts.ImportDeclaration)=>{
  const c=d.importClause!;
  const def=c.name?.text;
  let ns:string|undefined;const named:{local:string;text:string}[]=[];
  if(c.namedBindings){
    if(ts.isNamespaceImport(c.namedBindings))ns=c.namedBindings.name.text;
    else for(const e of c.namedBindings.elements)named.push({local:e.name.text,text:e.getText(sf)});
  }
  return {def,ns,named,typeOnly:c.isTypeOnly};
};

const fileFor=(newDir:string,docEmoji:string,doc:string,body:Stmt[],opts:{isSupport:boolean;supportNames?:Set<string>;exportAll?:boolean})=>{
  const used=identsOf(body.map(b=>b.node));
  const replicate=stmts.filter(s=>s.kind==="replicate");
  const repUsed=replicate.filter(r=>r.declared.some(n=>used.has(n)));
  for(const r of repUsed)for(const n of identsOf([r.node]))used.add(n);
  const lines:string[]=[];
  lines.push(`/** ${doc} */`);
  const rewrite=(node:ts.Node)=>{
    const rw=rewritesFor(node,newDir);
    const start=node.getFullStart();
    return applyRewrites(node.getFullText(sf),start,rw);
  };
  const importLines:string[]=[];
  for(const d of imports){
    const b=importBindings(d);
    const spec=(d.moduleSpecifier as ts.StringLiteral).text;
    const newSpec=isRel(spec)?relSpec(spec,newDir):spec;
    const q=d.moduleSpecifier.getText(sf)[0];
    const defUsed=b.def&&used.has(b.def)?b.def:undefined;
    const nsUsed=b.ns&&used.has(b.ns)?b.ns:undefined;
    const namedUsed=b.named.filter(n=>used.has(n.local));
    if(!defUsed&&!nsUsed&&!namedUsed.length)continue;
    const parts:string[]=[];
    if(defUsed)parts.push(defUsed);
    if(nsUsed)parts.push("* as "+nsUsed);
    if(namedUsed.length)parts.push("{"+namedUsed.map(n=>n.text).join(",")+"}");
    importLines.push(`import ${b.typeOnly?"type ":""}${parts.join(", ")} from ${q}${newSpec}${q};`);
  }
  lines.push(...importLines);
  return {lines,used,repUsed,rewrite};
};

const outputs:{rel:string;text:string}[]=[];
const mapping:{title:string;group:string;line:number;file:string}[]=[];
const baseDirRel=posix.dirname(REL);
const supportDir=join(OLD_DIR,SUPPORT);
const supportRel=posix.join(baseDirRel,SUPPORT,"🟦️.ts");

const helperStmts=stmts.filter(s=>s.kind==="helper");
const hookStmts=stmts.filter(s=>s.kind==="hook");
const supportBody=[...helperStmts,...hookStmts];
const exportedNames=new Set(helperStmts.flatMap(s=>s.declared));
{
  const f=fileFor(supportDir,"🧰️","Shared fixtures, issued invocations and one-time module warm-up for the build graph behaviour groups.",supportBody,{isSupport:true});
  const out=f.lines.slice();
  out.push("");
  for(const r of f.repUsed)out.push(f.rewrite(r.node).replace(/^\n+/,""));
  for(const s of supportBody){
    let t=f.rewrite(s.node);
    if(s.kind==="helper"){const off=s.node.getStart(sf)-s.node.getFullStart();t=t.slice(0,off)+"export "+t.slice(off);}
    out.push(t.replace(/^\n+/,""));
  }
  outputs.push({rel:supportRel,text:out.join("\n")+"\n"});
}

const usedGroupsByOrder=GROUP_ORDER.filter(g=>tests.some(t=>groups[t.title!]===g));
for(const g of Object.keys(groups).map(t=>groups[t]))if(!GROUP_ORDER.includes(g))throw Error("unknown group "+g);
const groupFileRels:string[]=[];
for(const g of usedGroupsByOrder){
  const newDir=join(OLD_DIR,g);
  const body=tests.filter(t=>groups[t.title!]===g);
  const f=fileFor(newDir,g.codePointAt(0)!.toString(16),DOCS[g].replace(/^/,g.slice(0,g.indexOf("️")+1)+" "),body,{isSupport:false});
  const supportNames=[...exportedNames].filter(n=>f.used.has(n));
  const lines=f.lines.slice();
  const supportSpec=relSpec(join(OLD_DIR,SUPPORT,"🟦️.ts"),newDir);
  lines.push(supportNames.length?`import {${supportNames.join(",")}} from "${supportSpec}";`:`import "${supportSpec}";`);
  lines.push("");
  for(const r of f.repUsed)lines.push(f.rewrite(r.node).replace(/^\n+/,""));
  for(const s of body){
    lines.push(f.rewrite(s.node).replace(/^\n+/,"").replace(/^\n+/,""));
    mapping.push({title:s.title!,group:g,line:s.line,file:posix.join(baseDirRel,g,"🟦️.ts")});
  }
  const rel=posix.join(baseDirRel,g,"🟦️.ts");
  groupFileRels.push(rel);
  outputs.push({rel,text:lines.join("\n")+"\n"});
}

const agg:string[]=["/** 🕸️ Build graph behaviour suite: each group registers its tests on import, after the owned candidate, settlement and verification suites. */"];
for(const d of sideEffects)agg.push(d.getText(sf).replace(/;?$/,";"));
for(const g of usedGroupsByOrder)agg.push(`import "./${g}/🟦️.ts";`);
outputs.push({rel:REL,text:agg.join("\n")+"\n"});

rmSync(STAGED,{recursive:true,force:true});
const tsv:string[]=[`BASE\t${sha(src)}\t${REL}`];
for(const o of outputs){
  const p=join(STAGED,o.rel);mkdirSync(dirname(p),{recursive:true});writeFileSync(p,o.text);
  tsv.push(`FILE\t${sha(o.text)}\t${o.rel}`);
}
writeFileSync(join(HERE,"manifest.tsv"),tsv.join("\n")+"\n");
writeFileSync(join(HERE,"mapping.json"),JSON.stringify({base:REL,tests:mapping,support:supportRel,groups:groupFileRels},null,1));
console.log("tests",tests.length,"helpers",helperStmts.map(s=>s.declared.join("+")).join(","),"hooks",hookStmts.length,"side-effect imports",sideEffects.length);
for(const o of outputs)console.log(o.text.split("\n").length-1,"lines",o.text.length,"bytes",o.rel.split("/").slice(-2).join("/"));
