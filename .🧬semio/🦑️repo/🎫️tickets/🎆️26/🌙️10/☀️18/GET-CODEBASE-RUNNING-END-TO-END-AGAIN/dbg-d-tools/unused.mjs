import ts from "typescript";import fs from "node:fs";
const files=fs.readFileSync(process.argv[2],"utf8").split("\n").filter(f=>/\.tsx?$/.test(f)).map(f=>f.trim()).filter(Boolean);
const opts={target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,strict:true,noEmit:true,skipLibCheck:true,allowImportingTsExtensions:true,resolveJsonModule:true,esModuleInterop:true,isolatedModules:true,noUnusedLocals:true,noUnusedParameters:false,jsx:ts.JsxEmit.ReactJSX,lib:["lib.esnext.d.ts","lib.dom.d.ts"],types:[]};
const prog=ts.createProgram(files,opts);
const map=Object.fromEntries(fs.readFileSync(process.argv[3],"utf8").split("\n").filter(Boolean).map(l=>{const i=l.indexOf(" ");return [l.slice(i+1),l.slice(0,i)]}));
for(const f of files){
  const sf=prog.getSourceFile(f);if(!sf)continue;
  const orig=fs.readFileSync(process.argv[4]+"/"+map[f]+".bak","utf8").split("\n").filter(l=>l.includes("[DEBUG]")).join("\n");
  const all=[...prog.getSyntacticDiagnostics(sf),...prog.getSemanticDiagnostics(sf)];
  for(const d of all){
    if(d.code!==6133&&d.code!==6196&&d.category!==ts.DiagnosticCategory.Error)continue;
    const msg=ts.flattenDiagnosticMessageText(d.messageText,"\n");
    const {line}=sf.getLineAndCharacterOfPosition(d.start??0);
    if(d.code===6133||d.code===6196){const m=/'([^']+)'/.exec(msg);if(m&&orig.includes(m[1]))console.log("UNUSED-AFTER-CUT",f.split("/").slice(-3).join("/"),line+1,msg)}
    else if(d.code<2000)console.log("SYNTAX",f,line+1,msg);
  }
}
console.log("done");
