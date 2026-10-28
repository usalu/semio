/** 🧷️ Type-checks the TypeScript files of staged batches on a root and reports only the diagnostics inside those files; aborts above 3 GB RSS: bun nl-tsc.ts <root> <batch dir>... */
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {createRequire} from "node:module";
const [root,...batches]=process.argv.slice(2) as [string,...string[]];
const ts=createRequire(join(root,"package.json"))("typescript");
const files=batches.flatMap(batch=>Object.entries(JSON.parse(readFileSync(join(batch,"manifest.json"),"utf8")) as Record<string,{new:string|null}>).filter(([rel,row])=>row.new!==null&&/\.(?:ts|mts)$/u.test(rel)&&!rel.endsWith(".d.ts")).map(([rel])=>join(root,rel)));
const guard=setInterval(()=>{if(process.memoryUsage().rss>3*1024**3){console.log("[DEBUG] rss over 3 GB, abort");process.exit(3);}},500);
const program=ts.createProgram(files,{noEmit:true,strict:true,skipLibCheck:true,allowImportingTsExtensions:true,resolveJsonModule:true,allowJs:true,checkJs:false,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,target:ts.ScriptTarget.ESNext,types:["bun"],noUncheckedIndexedAccess:true,exactOptionalPropertyTypes:false,lib:["lib.esnext.d.ts","lib.dom.d.ts"]});
const wanted=new Set(files);let count=0;
for(const diagnostic of ts.getPreEmitDiagnostics(program)){
 if(!diagnostic.file||!wanted.has(diagnostic.file.fileName))continue;count++;
 const {line}=diagnostic.file.getLineAndCharacterOfPosition(diagnostic.start??0);
 console.log(`${diagnostic.file.fileName.slice(root.length+1)}:${line+1}: ${ts.flattenDiagnosticMessageText(diagnostic.messageText,"\n").slice(0,300)}`);
}
clearInterval(guard);console.log(`[DEBUG] files ${files.length} diagnostics ${count} rss MB ${Math.round(process.memoryUsage().rss/1048576)}`);
