import ts from "/home/deck/semio/node_modules/typescript/lib/typescript.js";
import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
const dir=process.argv[2];let src=readFileSync(process.argv[3],"utf8");
const pinAssign="{sealedGenerationPinTextV1=encodeGenerationPinV1({owner:generationPinOwnerV1(process.env),...record});}";if(!src.includes(pinAssign))throw Error("pin assign");src=src.replace(pinAssign,"{recordSealedGenerationPinV1(encodeGenerationPinV1({owner:generationPinOwnerV1(process.env),...record}));}");
const modules=[
 ["🔐️authority",[[12,12],[57,90],[305,308]]],
 ["📜️contract",[[91,141],[803,809]]],
 ["🛑️custody",[[143,183],[372,378]]],
 ["🕊️release",[[184,211]]],
 ["📁️workspace",[[212,303],[663,686]]],
 ["🧭️discovery",[[309,353],[602,662]]],
 ["📍️pin",[[354,362]]],
 ["🏷️retention",[[363,371],[379,393]]],
 ["🕰️host",[[394,499]]],
 ["🤝️prefetch",[[500,601]]],
 ["📥️load",[[687,802],[810,849]]],
 ["🎯️resolve",[[850,872]]],
];
const sf=ts.createSourceFile("x.ts",src,ts.ScriptTarget.Latest,true);
const line=n=>sf.getLineAndCharacterOfPosition(n).line+1;
const moduleOf=l=>modules.find(([,rs])=>rs.some(([a,b])=>l>=a&&l<=b))?.[0];
const imports=[],stmts=[];
for(const s of sf.statements){
  if(ts.isImportDeclaration(s)){imports.push(s);continue;}
  const names=[];
  const add=n=>{if(ts.isIdentifier(n))names.push(n.text);};
  if(ts.isVariableStatement(s))for(const d of s.declarationList.declarations)add(d.name);
  else if(s.name)add(s.name);
  const startLine=line(s.getStart(sf));
  stmts.push({s,names,startLine,mod:moduleOf(startLine),isType:ts.isTypeAliasDeclaration(s)||ts.isInterfaceDeclaration(s),exported:!!(ts.getCombinedModifierFlags?.(s)&ts.ModifierFlags.Export),text:null});
}
for(const st of stmts)if(!st.mod&&!ts.isExpressionStatement(st.s)||!st.mod)console.error("UNASSIGNED",st.startLine,st.names);
const ids=n=>{const out=new Set();const v=x=>{if(ts.isIdentifier(x))out.add(x.text);ts.forEachChild(x,v);};v(n);return out;};
const declaredIn=new Map();for(const st of stmts)for(const n of st.names)declaredIn.set(n,st);
const needed=new Set();
for(const st of stmts){st.uses=ids(st.s);for(const u of st.uses){const d=declaredIn.get(u);if(d&&d.mod!==st.mod){needed.add(u);}}}
const origExports=[];
for(const st of stmts)if(st.exported)for(const n of st.names)origExports.push([n,st]);
const importInfo=imports.map(i=>{const c=i.importClause,spec=i.moduleSpecifier.text,typeOnly=c?.isTypeOnly;const items=[];if(c?.name)items.push({kind:"default",name:c.name.text});if(c?.namedBindings){if(ts.isNamespaceImport(c.namedBindings))items.push({kind:"ns",name:c.namedBindings.name.text});else for(const e of c.namedBindings.elements)items.push({kind:"named",name:e.name.text,prop:e.propertyName?.text,type:typeOnly||e.isTypeOnly});}return {spec,typeOnly,items,attrs:i.attributes?src.slice(i.attributes.getStart(sf),i.attributes.end):null};});
const out={};
for(const [m] of modules){
  const own=stmts.filter(s=>s.mod===m);
  let body=src.slice(0,0);
  const parts=[];
  for(const st of own){
    const from=st.s.getFullStart(),startPos=st.s.getStart(sf);
    let text=src.slice(from,st.s.end);
    const idx=startPos-from;
    const isNeeded=st.names.some(n=>needed.has(n));
    if(isNeeded&&!st.exported)text=text.slice(0,idx)+"export "+text.slice(idx);
    parts.push(text);
  }
  body=parts.join("\n");
  const used=new Set();for(const st of own)for(const u of st.uses)used.add(u);
  const lines=[];
  for(const ii of importInfo){
    const its=ii.items.filter(i=>used.has(i.name));if(!its.length)continue;
    const spec=ii.spec.startsWith(".")?"../"+ii.spec:ii.spec;
    const def=its.find(i=>i.kind==="default"),ns=its.find(i=>i.kind==="ns"),named=its.filter(i=>i.kind==="named");
    const bits=[];if(def)bits.push(def.name);if(ns)bits.push("* as "+ns.name);
    if(named.length)bits.push("{"+named.map(n=>(n.type&&!ii.typeOnly?"type ":"")+(n.prop?n.prop+" as "+n.name:n.name)).join(",")+"}");
    lines.push(`import${ii.typeOnly?" type":""} ${bits.join(",")} from ${JSON.stringify(spec)}${ii.attrs?" "+ii.attrs:""};`);
  }
  const cross=new Map();
  for(const st of own)for(const u of st.uses){const d=declaredIn.get(u);if(d&&d.mod!==m){if(!cross.has(d.mod))cross.set(d.mod,new Map());cross.get(d.mod).set(u,d.isType);}}
  for(const [dm,names] of cross)lines.push(`import {${[...names].map(([n,t])=>(t?"type ":"")+n).join(",")}} from "../${dm}/🟦️.ts";`);
  let text=lines.join("\n")+"\n"+body;
  text=text.replace(/new URL\("(\.\.?\/[^"]*)"/g,'new URL("../$1"');
  out[m]={text,deps:[...cross.keys()]};
  mkdirSync(`${dir}/${m}`,{recursive:true});
  writeFileSync(`${dir}/${m}/🟦️.ts`,text.endsWith("\n")?text:text+"\n");
}
const byMod=new Map();
for(const [n,st] of origExports){if(!byMod.has(st.mod))byMod.set(st.mod,[]);byMod.get(st.mod).push(st.isType?"type "+n:n);}
const entry=modules.map(([m])=>byMod.has(m)?`export {${byMod.get(m).join(",")}} from "./${m}/🟦️.ts";`:null).filter(Boolean).join("\n")+"\n";
writeFileSync(`${dir}/🟦️.ts`,entry);
console.error(JSON.stringify(Object.fromEntries(Object.entries(out).map(([k,v])=>[k,[v.text.length,v.deps]])),null,0));
console.error("needed",[...needed].join(" "));
const bare="new URL(physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))";
const rd=m=>readFileSync(`${dir}/${m}/🟦️.ts`,"utf8");
const wr=(m,t)=>writeFileSync(`${dir}/${m}/🟦️.ts`,t);
let a=rd("🔐️authority");
a=a.replace("new URL(path,physicalSourceOriginV1(",'new URL("../"+path,physicalSourceOriginV1(');
const marker="/** 🗂️ The graph implementation files";
if(!a.includes(marker)||!a.includes("["+bare))throw Error("authority markers");
const names=modules.map(([m])=>m);
a=a.replace(marker,`/** 🧱️ Every file of the graph entry and its split modules: the loaded implementation covers each of them. */\nexport const graphSelfUrlsV1=():URL[]=>{const origin=physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL);return ["../🟦️.ts",...${JSON.stringify(names)}.map(name=>\`../\${name}/🟦️.ts\`)].map(path=>new URL(path,origin));};\n`+marker);
a=a.replace("["+bare,"[...graphSelfUrlsV1()");
wr("🔐️authority",a);
let l=rd("📥️load");
if(!l.includes("["+bare))throw Error("load marker");
l=l.replace("["+bare,"[...graphSelfUrlsV1()");
l=l.replace(/import \{([^}]*)\} from "\.\.\/🔐️authority\/🟦️\.ts";/,(x,g)=>`import {${g},graphSelfUrlsV1} from "../🔐️authority/🟦️.ts";`);
wr("📥️load",l);
