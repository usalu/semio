import ts from "typescript";import fs from "node:fs";
const [,, listFile, mapFile, origDir, tmpDir]=process.argv;
const files=fs.readFileSync(listFile,"utf8").split("\n").map(f=>f.trim()).filter(f=>/\.tsx?$/.test(f));
const map=Object.fromEntries(fs.readFileSync(mapFile,"utf8").split("\n").filter(Boolean).map(l=>{const i=l.indexOf(" ");return [l.slice(i+1),l.slice(0,i)]}));
const opts={target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,strict:true,noEmit:true,skipLibCheck:true,allowImportingTsExtensions:true,resolveJsonModule:true,esModuleInterop:true,isolatedModules:true,noUnusedLocals:true,jsx:ts.JsxEmit.ReactJSX,lib:["lib.esnext.d.ts","lib.dom.d.ts"],types:[]};
const origs=files.map(f=>{const n=map[f];const p=`${tmpDir}/${n}${f.endsWith("x")?".tsx":".ts"}`;fs.copyFileSync(`${origDir}/${n}.bak`,p);return p});
function diags(list){const prog=ts.createProgram(list,opts);const out=new Map();for(const f of list){const sf=prog.getSourceFile(f);if(!sf)continue;const msgs=[...prog.getSyntacticDiagnostics(sf),...prog.getSemanticDiagnostics(sf)].filter(d=>d.code===6133||d.code===6196||d.code===6192||d.code===6198).map(d=>ts.flattenDiagnosticMessageText(d.messageText,"\n"));out.set(f,msgs);}return out}
const a=diags(origs),b=diags(files.map(f=>f));
files.forEach((f,i)=>{const o=a.get(origs[i])??[],n=b.get(f)??[];const rest=[...o];const extra=[];for(const m of n){const k=rest.indexOf(m);if(k>=0)rest.splice(k,1);else extra.push(m)}if(extra.length)console.log("NEW-UNUSED",f.split("/").slice(-3).join("/"),extra)});
console.log("done",files.length,[...a.values()].flat().length,[...b.values()].flat().length);
