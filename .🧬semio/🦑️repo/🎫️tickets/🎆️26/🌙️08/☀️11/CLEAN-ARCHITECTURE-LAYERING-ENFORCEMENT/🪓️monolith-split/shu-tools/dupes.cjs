const fs=require("fs");const ts=require("/Users/ueli/Documents/semio/node_modules/typescript");
const dupes=(p,t)=>{const sf=ts.createSourceFile(p,t,ts.ScriptTarget.Latest,true,p.endsWith("x")?ts.ScriptKind.TSX:ts.ScriptKind.TS);const m=new Map();
 for(const s of sf.statements)if(ts.isImportDeclaration(s)&&s.importClause&&s.importClause.namedBindings&&ts.isNamedImports(s.importClause.namedBindings)&&!s.importClause.name){const k=s.moduleSpecifier.text+"|"+!!s.importClause.isTypeOnly;m.set(k,(m.get(k)||0)+1)}
 return [...m].filter(([k,v])=>v>1).map(([k])=>k)};
let tot=0;
for(const snap of process.argv.slice(2)){for(const l of fs.readFileSync(snap+"/SHA.txt","utf8").split("\n").filter(Boolean)){const [i,h,p]=l.split("\t");
 const o=dupes(p,fs.readFileSync(snap+"/"+i+".orig","utf8")),c=dupes(p,fs.readFileSync(p,"utf8"));
 const nw=c.filter(x=>!o.includes(x));if(nw.length){tot++;console.log("NEW DUPE",p.replace(/.*Documents\/semio\//,"").slice(-90),nw)}}}
console.log("files with new dupes:",tot)
