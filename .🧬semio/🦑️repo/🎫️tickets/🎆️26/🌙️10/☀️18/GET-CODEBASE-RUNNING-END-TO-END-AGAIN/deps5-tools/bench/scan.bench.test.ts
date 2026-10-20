import {withBuildTestInvocationV1} from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import {readFileSync,lstatSync} from "node:fs";
import {join} from "node:path";
import {pathToFileURL} from "node:url";

const ROOT=process.cwd(),LIB=join(ROOT,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),DEPS=join(LIB,"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage"),FW=join(ROOT,"🧰️framework/🔨️modules");
const OLD=join(DEPS,"🟦️.ts"),NEW=process.env.DEPS5_NEW_SCAN??"",GEN=process.env.DEPS5_GENERATION??"ecafa95add9056aaed6a";

test("deps5 scan benchmark",async()=>{
 const ts=await import("typescript"),routing=await import(join(FW,"🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")),leases=await import(join(FW,"🏃️process/🔒️leases/🟦️.ts")),descriptors=await import(join(FW,"📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts")),installSchema=JSON.parse(readFileSync(join(DEPS,"🧬️schema/📥️install.json"),"utf8"));
 const extract=(file:string)=>{const source=readFileSync(file,"utf8"),tree=ts.createSourceFile("x.ts",source,ts.ScriptTarget.Latest,true),fn=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="scanJavascriptDependencyInstalledPathsV1");if(!fn)throw Error("scan missing in "+file);return ts.transpileModule(fn.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;};
 const variants:{name:string;code:string;schema:any}[]=process.env.DEPS5_SKIP_OLD?[]:[{name:"old",code:extract(OLD),schema:installSchema}];
 if(NEW){const schema=structuredClone(installSchema);schema.$defs.InstalledPathProtocol.const.scanProgressWorkUnits=Number(process.env.DEPS5_SCAN_UNITS??4096);variants.push({name:"new",code:extract(NEW),schema});if(process.env.DEPS5_NOLIGHT){const c=extract(NEW).replace(/const light = \(\) => \{[^}]*\};/u,"const light = () => {};");if(c===extract(NEW))throw Error("no light");variants.push({name:"nolight",code:c,schema});}}
 const root=join(ROOT,".cache/b/d/m/j",GEN),results:Record<string,unknown>={};
 await withBuildTestInvocationV1(async original=>{
  const control=routing.createIssuedResourceControlV1(original,"deps5:scan",0),counters={lstat:0,checks:0};
  const custody=()=>{counters.checks++;routing.requireIssuedResourceControlInvocationV1(control,original);routing.requireIssuedResourceControlRefinementV1(control,control);leases.checkResourceLeaseControl(control);leases.requireMaterializedPathV1(root);};
  for(const variant of variants)for(let round=0;round<Number(process.env.DEPS5_ROUNDS??1);round++){
   counters.lstat=0;counters.checks=0;let publishes=0;
   const counted=(path:string,options?:unknown)=>{counters.lstat++;return (lstatSync as any)(path,options);};
   const scan=new Function("installSchema","createHash","lstatSync","withArtifactReadDirectoryV1","join","requireIssuedResourceControlV1","checkResourceLeaseControl",variant.code+";return scanJavascriptDependencyInstalledPathsV1;")(variant.schema,createHash,counted,descriptors.withArtifactReadDirectoryV1,join,routing.requireIssuedResourceControlV1,leases.checkResourceLeaseControl) as (root:string,control:any,check:()=>void)=>Promise<any>;
   const counting=Object.freeze({signal:control.signal,remainingMilliseconds:()=>control.remainingMilliseconds(),publish:async(p:any)=>{publishes++;await control.publish(p);},yieldContinuation:()=>control.yieldContinuation()});
   routing.requireIssuedResourceControlV1;
   const t=performance.now(),cpu=process.cpuUsage(),result=await scan(root,control,custody),ms=performance.now()-t,used=process.cpuUsage(cpu);
   const row={ms:Math.round(ms),cpuMs:Math.round((used.user+used.system)/1000),lstats:counters.lstat,checks:counters.checks,entries:result.entries,sha256:result.sha256.slice(0,12),directories:result.directories,files:result.files,links:result.links};
   ((results[variant.name] ??= []) as any[]).push(row);
   console.error("[bench] deps5 bench",variant.name,JSON.stringify(row));
  }
 });
 const old=(results.old as any[]|undefined)?.[0];if(results.new&&old)expect((results.new as any[])[0].sha256).toBe(old.sha256);
 console.error("[bench] deps5 bench summary",JSON.stringify(results));
},3000000);
