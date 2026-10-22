// usage: bun check-names.ts <root> <listfile> <out.json>   (listfile: one repo-relative .ts path per line)
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {join} from "node:path";
const [root,list,out]=process.argv.slice(2);
const files=readFileSync(list,"utf8").split("\n").filter(Boolean).filter(f=>existsSync(join(root,f))).map(f=>join(root,f));
const found:Record<string,string[]>={};
for(let i=0;i<files.length;i+=200){
  const batch=files.slice(i,i+200);
  const program=ts.createProgram(batch,{noResolve:true,noEmit:true,skipLibCheck:true,types:[],target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,allowImportingTsExtensions:true,resolveJsonModule:true,strict:false,noImplicitAny:false,allowJs:true,checkJs:true,maxNodeModuleJsDepth:0});
  for(const file of batch){
    const source=program.getSourceFile(file);
    if(!source)continue;
    for(const d of program.getSemanticDiagnostics(source)){
      if(d.code!==2304&&d.code!==2552)continue;
      const text=ts.flattenDiagnosticMessageText(d.messageText,"\n");
      const name=/'([^']+)'/.exec(text)?.[1]??text;
      (found[file.slice(root.length+1)]??=[]).push(name);
    }
  }
}
for(const k in found)found[k]=[...new Set(found[k])].sort();
writeFileSync(out,JSON.stringify(found));
console.log("files",files.length,"with names",Object.keys(found).length);
