import {readFileSync,writeFileSync,renameSync} from "node:fs";
import {resolve,join} from "node:path";
const ticket=resolve(import.meta.dir,".."),owner="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap";
if(process.argv[2]==="bindings-test"){
 const selections=[{SEMIO_LOCKED_LOCALE:"en",SEMIO_LOCKED_TERMINOLOGY:"native"},{SEMIO_LOCKED_LOCALE:"en",SEMIO_LOCKED_TERMINOLOGY:"reuse"},{SEMIO_LOCKED_LOCALE:"de",SEMIO_LOCKED_TERMINOLOGY:"native"},{SEMIO_LOCKED_LOCALE:"de",SEMIO_LOCKED_TERMINOLOGY:"reuse"}];
 edit(owner+"/📦️dependencies/🧬️schema/🔣️.json",source=>{const data=JSON.parse(source);data.required.push("watcherSelections");data.properties.watcherSelections={const:selections};return JSON.stringify(data,null,2)+"\n";});
 edit(owner+"/📦️dependencies/🧫️fixtures/🔣️.json",source=>{const data=JSON.parse(source);data.watcherSelections=selections;return JSON.stringify(data,null,2)+"\n";});
 edit(owner+"/📦️dependencies/🧪️tests/🎛️initial-preparation.feature",source=>source+'  Scenario Outline: Explicit launch selections isolate source watchers\n    Given the same renderer, example and port\n    When its language is <language> and its terminology is <terminology>\n    Then its graph store and socket belong to that exact selection\n    And their identity matches the independent SHA-256 oracle\n    Examples:\n      | language | terminology |\n      | en       | native      |\n      | en       | reuse       |\n      | de       | native      |\n      | de       | reuse       |\n');
 edit(owner+"/📦️dependencies/🧪️tests/🟦️.ts",source=>source+'\n\ntest("explicit languages and terminology isolate watcher graphs and sockets", () => {\n  const selections=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).watcherSelections as Record<string,string>[];\n  const graphs=new Set<string>(),sockets=new Set<string>(),workspace=resolve(owner),target="a:activate-react";\n  for(const selection of selections){\n    const environment={S_OS_PORT:"6064",PLAYGROUND_LOCKED_EXAMPLE_ID:"🎬️demo",...selection},actual=nxWatcherEnvironment(workspace,environment,target);\n    const entries=Object.entries(environment).sort(([a],[b])=>a<b?-1:a>b?1:0),identity=JSON.stringify([workspace.replaceAll("\\\\","/"),target,entries]),hash=new Bun.CryptoHasher("sha256").update(identity).digest("hex").slice(0,24);\n    expect(actual.NX_WORKSPACE_DATA_DIRECTORY.endsWith(hash)).toBe(true);\n    graphs.add(actual.NX_WORKSPACE_DATA_DIRECTORY);sockets.add(actual.NX_SOCKET_DIR);\n  }\n  expect(graphs.size).toBe(selections.length);expect(sockets.size).toBe(selections.length);\n});\n');
}
if(process.argv[2]==="bindings-fix")edit(owner+"/📜️script.ts",source=>source.replace('SEMIO_(?:PLUGIN|RENDERER|BUILD_MODE|LOCALE)$','SEMIO_(?:PLUGIN|RENDERER|BUILD_MODE|LOCALE|LOCKED_LOCALE|LOCKED_TERMINOLOGY)$'));
if(process.argv[2]==="factory-status")edit(owner+"/📜️script.ts",source=>source.replace('if(!watcher)return;','if(!watcher){process.exitCode=cancelled?cancelled==="SIGINT"?130:143:await status;return;}').replace('child.once("close",code=>accept(code??1)); });','child.once("close",code=>accept(code??1)); });void status.catch(()=>{});'));
function edit(path:string,transform:(source:string)=>string):void {
  const before=readFileSync(path,"utf8"),after=transform(before);if(after===before)throw Error("No edit: "+path);
  const stage=join(ticket,"🗑️generated/watch-target-stage");writeFileSync(stage,after);if(readFileSync(path,"utf8")!==before)throw Error("Concurrent edit: "+path);renameSync(stage,path);
}
if(process.argv[2]==="test")edit(owner+"/📦️dependencies/🧪️tests/🟦️.ts",source=>source.replace("activateWhenNxWatcherReady, nxChildEnvironment", "activateWhenNxWatcherReady, waitForNxTargetStart, nxChildEnvironment")+String.raw`

test("initial preparation waits for an exact complete serving target and releases its observers", async () => {
  const child=Object.assign(new EventEmitter(),{stdout:new EventEmitter()}),states:boolean[]=[];let ready=false;
  const prepared=waitForNxTargetStart(child as never,"application:dev").then(()=>{ready=true;});
  for(const chunk of ["> nx run application:dev-tools\n", "\x1b[32m> nx run application:", "dev", "\x1b[0m\r\n"]) {
    child.stdout.emit("data",Buffer.from(chunk.replaceAll("\\n","\n").replaceAll("\\r","\r").replaceAll("\\x1b","\x1b")));
    await new Promise(accept=>setImmediate(accept));states.push(ready);
  }
  expect(states).toEqual([false,false,false,true]);await prepared;
  expect(child.stdout.listenerCount("data")+child.listenerCount("close")+child.listenerCount("error")).toBe(0);
  const failed=Object.assign(new EventEmitter(),{stdout:new EventEmitter()}),failure=waitForNxTargetStart(failed as never,"application:dev");
  failed.emit("close",1);await expect(failure).rejects.toThrow("ended before serving target");
  const source=createSourceFile("bootstrap.ts",readFileSync(resolve(owner,"../📜️script.ts"),"utf8"),ScriptTarget.Latest);
  const operation=source.statements.find(statement=>isFunctionDeclaration(statement)&&statement.name?.text==="waitForNxTargetStart")!;
  const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"target-start-")),module=join(directory,"target.mjs");
  await build({stdin:{contents:operation.getText(source),loader:"ts"},outfile:module,platform:"node",format:"esm"});
  const reference="import {EventEmitter} from 'node:events';import {waitForNxTargetStart} from "+JSON.stringify(pathToFileURL(module).href)+";const child=Object.assign(new EventEmitter(),{stdout:new EventEmitter()}),states=[];let ready=false;const prepared=waitForNxTargetStart(child,'application:dev').then(()=>ready=true);for(const chunk of ['> nx run application:dev-tools\\n','\\x1b[32m> nx run application:','dev','\\x1b[0m\\r\\n']){child.stdout.emit('data',Buffer.from(chunk.replaceAll('\\\\n','\\n').replaceAll('\\\\r','\\r').replaceAll('\\\\x1b','\\x1b')));await new Promise(setImmediate);states.push(ready);}await prepared;console.log(JSON.stringify(states));";
  const oracle=Bun.spawn(["node","--input-type=module","-e",reference],{stdout:"pipe",stderr:"pipe"});
  const [output,errors,code]=await Promise.all([new Response(oracle.stdout).text(),new Response(oracle.stderr).text(),oracle.exited]);
  expect(code,errors).toBe(0);expect(JSON.parse(output)).toEqual(states);
});
`);
if(process.argv[2]==="fix")edit(owner+"/📜️script.ts",source=>source.replace('if(pending.includes("> nx run "+target))finish();','const marker="> nx run "+target;if([" ","\\r","\\n"].some(separator=>pending.includes(marker+separator)))finish();'));
if(process.argv[2]==="fixture")edit(owner+"/📦️dependencies/🧪️tests/🟦️.ts",source=>{
  const start=source.indexOf('  for(const chunk of ["> nx run application:dev-tools');
  const end=source.indexOf('    await new Promise(accept=>setImmediate(accept));states.push(ready);',start);
  if(start<0||end<0)throw Error("Missing chunk fixture");
  source=source.slice(0,start)+String.raw`  const chunks=["> nx run application:dev-tools"+String.fromCharCode(10),String.fromCharCode(27)+"[32m> nx run application:","dev",String.fromCharCode(27)+"[0m"+String.fromCharCode(13,10)];
  for(const chunk of chunks) {
    child.stdout.emit("data",Buffer.from(chunk));
`+source.slice(end);
  return source.replace(/^  const reference=.*$/m,String.raw`  const reference="import {EventEmitter} from 'node:events';import {waitForNxTargetStart} from "+JSON.stringify(pathToFileURL(module).href)+";const child=Object.assign(new EventEmitter(),{stdout:new EventEmitter()}),states=[];let ready=false;const prepared=waitForNxTargetStart(child,'application:dev').then(()=>ready=true);for(const chunk of "+JSON.stringify(chunks)+"){child.stdout.emit('data',Buffer.from(chunk));await new Promise(setImmediate);states.push(ready);}await prepared;console.log(JSON.stringify(states));";`);
});
if(process.argv[2]==="factory-test")edit(owner+"/📦️dependencies/🧪️tests/🟦️.ts",source=>source+String.raw`

test("automatic watcher callbacks register only after initial preparation and honour cancellation", async () => {
  const {startNxWatcherAfterPreparation}=await import("../../📜️script.ts");
  let complete!:()=>void;const prepared=new Promise<void>(accept=>{complete=accept;}),events=["initial"];
  const watcher=startNxWatcherAfterPreparation(prepared,()=>true,()=>{events.push("watcher");return "registered" as never;});
  await new Promise(accept=>setImmediate(accept));expect(events).toEqual(["initial"]);complete();expect(await watcher).toBe("registered");expect(events).toEqual(["initial","watcher"]);
  let started=false;expect(await startNxWatcherAfterPreparation(Promise.resolve(),()=>false,()=>{started=true;return null as never;})).toBeUndefined();expect(started).toBe(false);
  await expect(startNxWatcherAfterPreparation(Promise.reject(Error("initial failure")),()=>true,()=>{started=true;return null as never;})).rejects.toThrow("initial failure");expect(started).toBe(false);
  const source=createSourceFile("bootstrap.ts",readFileSync(resolve(owner,"../📜️script.ts"),"utf8"),ScriptTarget.Latest),operation=source.statements.find(statement=>isFunctionDeclaration(statement)&&statement.name?.text==="startNxWatcherAfterPreparation")!;
  const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"watcher-registration-")),module=join(directory,"watcher.mjs");
  await build({stdin:{contents:operation.getText(source),loader:"ts"},outfile:module,platform:"node",format:"esm"});
  const reference="import {startNxWatcherAfterPreparation} from "+JSON.stringify(pathToFileURL(module).href)+";let complete;const prepared=new Promise(accept=>complete=accept),events=['initial'];const watcher=startNxWatcherAfterPreparation(prepared,()=>true,()=>events.push('watcher'));await new Promise(setImmediate);if(events.length!==1)throw Error('premature registration');complete();await watcher;console.log(JSON.stringify(events));";
  const oracle=Bun.spawn(["node","--input-type=module","-e",reference],{stdout:"pipe",stderr:"pipe"});
  const [output,errors,code]=await Promise.all([new Response(oracle.stdout).text(),new Response(oracle.stderr).text(),oracle.exited]);expect(code,errors).toBe(0);expect(JSON.parse(output)).toEqual(events);
});
`);
if(process.argv[2]==="factory-fix")edit(owner+"/📜️script.ts",source=>source.replace('/** ⏳️ Observes Nx starting the serving target after its prerequisite graph completes. */',`/** 🛡️ Registers automatic source callbacks after the initial prerequisite graph has settled. */
export async function startNxWatcherAfterPreparation(prepared:Promise<void>,active:()=>boolean,start:()=>ReturnType<typeof spawnNxProcess>):Promise<ReturnType<typeof spawnNxProcess>|undefined> {
  await prepared;return active()?start():undefined;
}

/** ⏳️ Observes Nx starting the serving target after its prerequisite graph completes. */`).replace('          NxScript.ensureDaemon(nxCli,this.root,watchEnvironment);','          watcher=await startNxWatcherAfterPreparation(prepared,()=>!finishing&&!cancelled,()=>{\n            NxScript.ensureDaemon(nxCli,this.root,watchEnvironment);').replace('          watcher=launch(["watch"','            const registered=launch(["watch"').replace('          watcher.stdout!.on("data",chunk=>process.stdout.write(chunk));','            return registered;\n          });\n          if(!watcher)return;\n          watcher.stdout!.on("data",chunk=>process.stdout.write(chunk));').replace('activateWhenNxWatcherReady(watcher,prepared,','activateWhenNxWatcherReady(watcher,Promise.resolve(),'));
