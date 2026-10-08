import {getWorkspaceRoot} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import {join} from "node:path";
import {dirname, relative, resolve} from "node:path";
import {readFileSync, existsSync, mkdirSync, writeFileSync, openSync, closeSync, renameSync} from "node:fs";
import {spawn} from "node:child_process";
import {once} from "node:events";
import ts from "typescript";

const segments=process.argv.slice(2), command=segments.join(" "), root=getWorkspaceRoot();
if(segments[0]?.startsWith("managed-")){
  const mode=segments[0],name=segments[1];
  if(!name||!/^[a-z0-9-]{1,64}$/u.test(name))throw Error("Managed verification requires one scoped job name");
  const directory=join(import.meta.dir,"../🗑️generated/managed-verification",name),statePath=join(directory,"state.json"),cancelPath=join(directory,"cancel.json"),logPath=join(directory,"output.log");
  const read=()=>JSON.parse(readFileSync(statePath,"utf8"));
  const save=(state:Record<string,unknown>)=>{mkdirSync(directory,{recursive:true});writeFileSync(statePath+".next",JSON.stringify(state,null,2)+"\n");renameSync(statePath+".next",statePath);};
  if(mode==="managed-start"){
    if(segments[2]!=="--"||segments[3]!=="nx")throw Error("Managed verification only runs an explicit Bun Nx invocation");
    if(existsSync(statePath))throw Error("Managed job name already exists; inspect it or select a fresh verification cohort");
    mkdirSync(dirname(directory),{recursive:true});mkdirSync(directory);
    save({name,status:"scheduled",pid:process.pid,startedAt:new Date().toISOString(),args:segments.slice(3)});
    const output=openSync(logPath,"wx");
    try{
      const runner=spawn(process.execPath,[import.meta.filename,"managed-execute",name],{cwd:root,env:{...process.env,NX_DAEMON:"false"},detached:true,stdio:["ignore",output,output],windowsHide:true});
      await once(runner,"spawn");runner.unref();
      console.log(JSON.stringify({name,pid:runner.pid,statePath,logPath}));
    }finally{closeSync(output);}
  }else if(mode==="managed-inspect"){
    const state=read();let alive=false;
    if(Number.isSafeInteger(state.pid))try{process.kill(state.pid,0);alive=true;}catch(error){if((error as NodeJS.ErrnoException).code==="EPERM")alive=true;}
    console.log(JSON.stringify({...state,alive,logPath}));
  }else if(mode==="managed-cancel"){
    const state=read();
    if(!["scheduled","running"].includes(state.status))throw Error("Managed verification is already terminal");
    writeFileSync(cancelPath,JSON.stringify({requestedAt:new Date().toISOString()})+"\n");
    console.log(JSON.stringify({name,cancellationRequested:true}));
  }else if(mode==="managed-execute"){
    const state=read(),controller=new AbortController(),evidence:string[]=[];
    save({...state,status:"running",pid:process.pid});
    const poll=setInterval(()=>{if(existsSync(cancelPath))controller.abort();},250),stop=()=>controller.abort();
    process.once("SIGINT",stop);process.once("SIGTERM",stop);
    try{
      const {runOwnedCommand}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts"));
      const {TEST_LEVEL_BUDGET_MS}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts"));
      await runOwnedCommand(process.execPath,state.args,root,`verification:${name}`,TEST_LEVEL_BUDGET_MS.exhaustive,{signal:controller.signal,onLine(line){
        const text=line.replace(/\x1b\[[0-9;]*m/gu,"");
        if(text.length<=8192&&/^(?:\[DEBUG\]|test .+ \.\.\. (?:ok|FAILED)|test result:|\s*\d+ (?:pass|fail)|\s*(?:PASS|FAIL)\s+\[|\s*Summary\s*\[|Ran \d+ tests)/u.test(text)){evidence.push(text);if(evidence.length>256)evidence.shift();}
      }});
      save({...state,pid:process.pid,status:"passed",completedAt:new Date().toISOString(),evidence});
    }catch(error){save({...state,pid:process.pid,status:controller.signal.aborted?"cancelled":"failed",error:String(error),completedAt:new Date().toISOString(),evidence});process.exitCode=1;}
    finally{clearInterval(poll);process.off("SIGINT",stop);process.off("SIGTERM",stop);}
  }else throw Error("managed-start | managed-inspect | managed-cancel");
}else
if(command==="checkpoint-oracle"){
  const {checkpointActorOracle}=await import(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📸️checkpoint/🧪️tests/🔬️unit/🟦️.ts"));
  console.log(`checkpoint-actor-oracle: ${checkpointActorOracle(root)} cases`);
}else if(command==="plugin-oracle"){
  const {timeTravelScenarioOracle}=await import(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🟦️.ts"));
  console.log(`Validated ${timeTravelScenarioOracle(root)} plugin history scenarios`);
}
else if(command==="history-oracles"){
  const base=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests");
  const cases=[["🧪️time-travel","timeTravelScenarioOracle"],["🧪️supersede-ledger","supersedeLedgerOracle"],["🧪️history-alternatives","historyAlternativesOracle"],["🧪️history-label-reload","historyLabelReloadOracle"],["🧪️folder-reload-route","folderReloadRouteOracle"],["🧪️composed-child-history","composedChildHistoryOracle"]] as const;
  const outcomes=await Promise.allSettled(cases.map(async([directory,name])=>{
    const module=await import(join(base,directory,"🟦️.ts"));
    console.log(`${name}: ${module[name](root)} cases`);
  }));
  let failed=0;
  outcomes.forEach((result,index)=>{if(result.status==="rejected"){failed++;console.error(`${cases[index]![1]}: ${String(result.reason)}`);}});
  if(failed)throw Error(`${failed} history oracle(s) failed`);
}else if(command==="browser-transport"){
  const {testBrowserTransportOwnership}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts"));
  await testBrowserTransportOwnership(root);
}else if(command==="browser-inputs"){
  const {testBrowserInputClosure}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts"));
  await testBrowserInputClosure(root);
}else if(command==="browser-ownership"){
  const {testWgpuGeneratorOwnership}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts"));
  await testWgpuGeneratorOwnership(root);
}else if(command==="browser-imports"){
  const taxonomy=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),"utf8"));
  const profile=taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile, owned=new Set<string>(profile.sourceModulePaths), missing=new Map<string,string[]>();
  for(const path of owned){
    if(!/\.(ts|mjs)$/u.test(path))continue;
    const source=readFileSync(join(root,path),"utf8"), emitted=ts.transpileModule(source,{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext}}).outputText;
    const tree=ts.createSourceFile(path,emitted,ts.ScriptTarget.Latest,true);
    for(const item of tree.statements){
      if(!(ts.isImportDeclaration(item)||ts.isExportDeclaration(item))||!item.moduleSpecifier||!ts.isStringLiteral(item.moduleSpecifier)||!item.moduleSpecifier.text.startsWith("."))continue;
      const target=relative(root,resolve(root,dirname(path),item.moduleSpecifier.text)).replaceAll("\\","/");
      if(!existsSync(join(root,target))||owned.has(target))continue;
      const sources=missing.get(target)??[];sources.push(path);missing.set(target,sources);
    }
  }
  console.log(JSON.stringify([...missing].map(([target,sources])=>({target,sources})),null,2));
  if(missing.size)throw Error(`${missing.size} undeclared runtime browser input(s)`);
}else throw Error("checkpoint-oracle | plugin-oracle | history-oracles | browser-imports");
