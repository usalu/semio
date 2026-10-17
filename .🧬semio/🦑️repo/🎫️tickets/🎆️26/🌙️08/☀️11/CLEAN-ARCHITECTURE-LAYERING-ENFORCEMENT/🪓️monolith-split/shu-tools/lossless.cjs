// usage: node lossless.cjs <snapdir>  -> per file: non-import text identical? imported (local,type) multiset identical? 
const fs=require("fs"),path=require("path");const ts=require("/Users/ueli/Documents/semio/node_modules/typescript");
const snap=process.argv[2];
const info=(p,t)=>{const sf=ts.createSourceFile(p,t,ts.ScriptTarget.Latest,true,p.endsWith("x")?ts.ScriptKind.TSX:ts.ScriptKind.TS);
  const binds=[];let rest="";let last=0;const cuts=[];
  for(const s of sf.statements){
    if(ts.isImportDeclaration(s)){cuts.push([s.getStart(sf),s.getEnd()]);const c=s.importClause;if(!c)continue;
      if(c.name)binds.push("default:"+c.name.text);
      if(c.namedBindings){if(ts.isNamespaceImport(c.namedBindings))binds.push("ns:"+c.namedBindings.name.text);else for(const e of c.namedBindings.elements)binds.push(((c.isTypeOnly||e.isTypeOnly)?"type:":"value:")+e.name.text+"="+(e.propertyName??e.name).text)}}
    else if(ts.isExportDeclaration(s)&&s.moduleSpecifier&&s.exportClause&&ts.isNamedExports(s.exportClause)){cuts.push([s.getStart(sf),s.getEnd()]);for(const e of s.exportClause.elements)binds.push("export:"+((s.isTypeOnly||e.isTypeOnly)?"type:":"value:")+e.name.text+"="+(e.propertyName??e.name).text)}
  }
  for(const [a,b] of cuts)rest+=t.slice(last,a),last=b;rest+=t.slice(last);
  return {binds:binds.sort(),rest:rest.replace(/\n\s*\n+/g,"\n").replace(/[ \t]+$/gm,"")};};
let bad=0,n=0;
for(const l of fs.readFileSync(snap+"/SHA.txt","utf8").split("\n").filter(Boolean)){
  const [i,h,p]=l.split("\t");const o=info(p,fs.readFileSync(snap+"/"+i+".orig","utf8"));const c=info(p,fs.readFileSync(p,"utf8"));n++;
  const sameRest=o.rest===c.rest;
  // after the move local names may only become deduped: compare unique sets
  const so=[...new Set(o.binds)],sc=[...new Set(c.binds)];
  const sameBinds=JSON.stringify(so)===JSON.stringify(sc);
  if(!sameRest||!sameBinds){bad++;console.log("DIFF",p.replace(/.*renderer\/|.*Documents\/semio\//,""),sameRest?"":"[non-import text]",sameBinds?"":"[bindings: -"+so.filter(x=>!sc.includes(x)).join(",")+" +"+sc.filter(x=>!so.includes(x)).join(",")+"]")}
}
console.log("files",n,"differing",bad);
