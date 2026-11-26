import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";
const root="/Users/ueli/Documents/semio",config=path.join(root,process.argv[2]),out=process.argv[3];
const cfg=ts.readConfigFile(config,ts.sys.readFile),parsed=ts.parseJsonConfigFileContent(cfg.config,ts.sys,path.dirname(config));
const program=ts.createProgram({rootNames:parsed.fileNames,options:{...parsed.options,noEmit:true}});
const checker=program.getTypeChecker();
const diagnostics=ts.getPreEmitDiagnostics(program).filter(d=>d.file&&d.category===1);
const isTest=f=>/\/🧪️tests\/|\.test\.|\/tests\//.test(f);
const declSide=(sym,seen)=>{
 if(!sym)return false;
 if(sym.flags&ts.SymbolFlags.Alias)try{sym=checker.getAliasedSymbol(sym);}catch{return false;}
 for(const d of sym.declarations??[]){
  const f=d.getSourceFile().fileName;
  if(f.endsWith(".d.mts"))return true;
  if(seen.has(d)||seen.size>60)continue;seen.add(d);
  if(ts.isVariableDeclaration(d)&&d.initializer&&walk(d.initializer,seen))return true;
  if(ts.isParameter(d)||ts.isBindingElement(d)){let p=d.parent;while(p&&!ts.isVariableDeclaration(p)&&!ts.isParameter(p)&&!ts.isSourceFile(p))p=p.parent;if(p&&ts.isVariableDeclaration(p)&&p.initializer&&walk(p.initializer,seen))return true;}
 }
 return false;
};
const walk=(n,seen)=>{let hit=false;const visit=x=>{if(hit)return;if(ts.isIdentifier(x)){let s;try{s=checker.getSymbolAtLocation(x);}catch{}if(declSide(s,seen))hit=true;}if(!hit)ts.forEachChild(x,visit);};visit(n);return hit;};
const rows=[];
for(const d of diagnostics){
 const file=d.file.fileName.replace(root+"/","");
 const {line,character}=d.file.getLineAndCharacterOfPosition(d.start);
 let cls;
 if(!isTest(file))cls="product";
 else{
  let node=(function find(n){let r=n;ts.forEachChild(n,c=>{if(c.getStart()<=d.start&&d.start<c.getEnd())r=find(c);});return r;})(d.file);
  let stmt=node;while(stmt&&!ts.isStatement(stmt)&&!ts.isArrowFunction(stmt)||(stmt&&ts.isBlock(stmt)))stmt=stmt.parent;
  const seen=new Set();
  cls=stmt&&walk(stmt,seen)?"declaration":"fixture";
 }
 rows.push({file,line:line+1,col:character+1,code:d.code,cls,msg:ts.flattenDiagnosticMessageText(d.messageText,"\n").slice(0,300)});
}
fs.writeFileSync(out,JSON.stringify(rows));
const c={};for(const r of rows)c[r.cls]=(c[r.cls]??0)+1;console.log(rows.length,c);
