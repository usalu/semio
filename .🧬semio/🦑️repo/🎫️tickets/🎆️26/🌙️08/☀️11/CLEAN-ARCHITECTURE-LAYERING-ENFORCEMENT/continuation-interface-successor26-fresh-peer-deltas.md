# Fresh Physical Peer Deltas

Current full bodies retained in authored26 peer-cut-custody. No historical actor attributed.

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts

```diff
--- 
+++ 
@@ -10,9 +10,9 @@
 const sha=(path:string):string=>{const info=lstatSync(path);if(info.isSymbolicLink()||!info.isFile())throw Error("Preparation source must be a physical file");return createHash("sha256").update(readFileSync(path)).digest("hex");};
 /** 🔗️ Prepares once, then owns unchanged Cargo update and locked fetch under one current exclusive custody. */
-export async function withPreparedCargoDependencyPairV1(root:string,manifest:string,signal:AbortSignal,runner:CargoDependencyPairRunnerV1,sourcePaths:readonly string[]=[]):Promise<void> {
+export async function withPreparedCargoDependencyPairV1(root:string,manifest:string,signal:AbortSignal,runner:CargoDependencyPairRunnerV1,sourcePaths:readonly string[]=[],packages:readonly string[]=[]):Promise<void> {
  signal.throwIfAborted();const directory=join(root,".🧬semio/🦑️repo/⚡️cache/cargo-preparation",randomUUID()),lease=await acquireQueuedResourceLease({directory:join(root,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"),resource:`cargo-preparation:${root}`,mode:"exclusive",owner:randomUUID(),signal,onWait:()=>console.log(`[cargo-preparation] waiting for ${manifest}`)});
  try {
  mkdirSync(directory,{recursive:true});const scope=cargoWorkspaceForManifest(root,manifest),sources=[...new Set([fileURLToPath(import.meta.url),fileURLToPath(new URL("../🟦️.ts",import.meta.url)),fileURLToPath(new URL("./📜️script.ts",import.meta.url)),fileURLToPath(new URL("./🧾️custody/🟦️.ts",import.meta.url)),fileURLToPath(new URL("./🧾️custody/🧬️schema/🔣️.json",import.meta.url)),fileURLToPath(new URL("../🧬️schema/🏃️invocation/🔣️.json",import.meta.url)),fileURLToPath(new URL("../🧬️schema/🛠️preparation/🔣️.json",import.meta.url)),...sourcePaths])].map(path=>({path,sha256:sha(path)}));
- const prepared=prepareCargoOwners(root,scope,[],directory);publishCargoWorkspaceMembership(root,cargoWorkspaceForManifest(root,scope.manifest),"write");
+ const prepared=prepareCargoOwners(root,scope,packages,directory);publishCargoWorkspaceMembership(root,cargoWorkspaceForManifest(root,scope.manifest),"write");
  const scopes=discoverCargoWorkspaces(root),ownerRoster=JSON.stringify(scopes.map(scope=>scope.manifest)),members=prepared.owners.map(owner=>{const scope=scopes.find(scope=>scope.directory===owner);if(!scope)throw Error("Prepared workspace disappeared");return{owner,roster:JSON.stringify(cargoWorkspaceMembers(root,scope))};}),inputs=[...new Set([...prepared.manifests,...members.flatMap(member=>{const scope=scopes.find(scope=>scope.directory===member.owner)!;return cargoWorkspaceMembers(root,scope).map(pkg=>pkg.manifest);})])].map(path=>cargoPreparationInputV1(root,resolve(root,path),"file"));
  const current=():void=>{signal.throwIfAborted();for(const source of sources)if(sha(source.path)!==source.sha256)throw Error(`Prepared source changed: ${source.path}`);for(const input of inputs)if(cargoPreparationInputV1(root,input.path,input.kind).sha256!==input.sha256)throw Error(`Prepared manifest changed: ${input.path}`);const currentScopes=discoverCargoWorkspaces(root);if(JSON.stringify(currentScopes.map(scope=>scope.manifest))!==ownerRoster)throw Error("Prepared workspace roster changed");for(const member of members){const scope=currentScopes.find(scope=>scope.directory===member.owner);if(!scope||JSON.stringify(cargoWorkspaceMembers(root,scope))!==member.roster)throw Error("Prepared member roster changed");}for(const recipe of prepared.recipes)assertCargoPreparationObservationCurrentV1(recipe);};
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts

```diff
--- 
+++ 
@@ -315,5 +315,5 @@
   if(!names.length && !args.includes("--workspace") && typeof selectedPackage==="string")names.push(selectedPackage);
   if (source.workspace?.metadata?.semio?.repository !== undefined) {
-    const result=Bun.spawnSync([process.execPath,fileURLToPath(new URL("./🛠️preparation/📜️script.ts",import.meta.url)),"prepare","--manifest",owner.manifest,...names.flatMap(name=>["--package",name])],{cwd:root,env:{...environment,NX_WORKSPACE_ROOT:root},stdout:"pipe",stderr:"inherit"});
+    const result=Bun.spawnSync([process.execPath,fileURLToPath(new URL("./🛠️preparation/📜️script.ts",import.meta.url)),"synchronize","--manifest",owner.manifest,...names.flatMap(name=>["--package",name])],{cwd:root,env:{...environment,NX_WORKSPACE_ROOT:root},stdout:"pipe",stderr:"inherit"});
     if (result.stdout.byteLength) process.stderr.write(result.stdout);
     if(result.exitCode!==0)throw new Error(`Selected Cargo preparation failed: ${owner.manifest} (${result.exitCode})`);
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/📜️script.ts

```diff
--- 
+++ 
@@ -1,3 +1,5 @@
 #!/usr/bin/env bun
+import {withPreparedCargoDependencyPairV1} from "./🟦️.ts";
+import {runOwnedCommand} from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
 import {randomUUID,createHash} from "node:crypto";
 import {readFileSync} from "node:fs";
@@ -37,4 +39,17 @@
 }
 
+/** 🔗️ Synchronizes the actual selected owner lock after one fresh local recipe closure. */
+export class SynchronizeScript extends Script {
+ async run(args:string[]):Promise<void> {
+  if(args.length<2||args[0]!=="--manifest"||(args.length-2)%2||args.slice(2).some((value,index)=>index%2===0?value!=="--package":!value))throw Error("synchronize --manifest <selected-owner> [--package <name>...]");
+  const packages=args.filter((_,index)=>index>=3&&index%2===1);if(new Set(packages).size!==packages.length)throw Error("Duplicate selected Cargo package");
+  const controller=new AbortController();let cancelled:NodeJS.Signals|undefined;
+  const stop=(signal:NodeJS.Signals):void=>{cancelled??=signal;controller.abort();};const interrupt=():void=>stop("SIGINT"),terminate=():void=>stop("SIGTERM");process.once("SIGINT",interrupt);process.once("SIGTERM",terminate);
+  try {const owner=cargoWorkspaceForManifest(this.root,args[1]!);await withPreparedCargoDependencyPairV1(this.root,owner.manifest,controller.signal,(argv,cwd,signal)=>runOwnedCommand("cargo",argv,cwd,"cargo:selected-dependencies",0,{signal}),[],packages);}
+  catch(error){if(!cancelled)throw error;process.exitCode=cancelled==="SIGINT"?130:143;}
+  finally{process.off("SIGINT",interrupt);process.off("SIGTERM",terminate);}
+ }
+}
+
 if(import.meta.main){
  if(process.argv[2]==="capture-program"){
@@ -44,4 +59,4 @@
   const edges=JSON.parse(await Bun.stdin.text());if(!Array.isArray(edges)||edges.some(edge=>typeof edge.source!=="string"||typeof edge.specifier!=="string"))throw Error("Invalid program resolution request");
   console.log(JSON.stringify(edges.map(edge=>Bun.resolveSync(edge.specifier,dirname(edge.source)))));
- }else await new ScriptRouter(getWorkspaceRoot()).register("prepare",PreparationScript).run(process.argv.slice(2));
+ }else await new ScriptRouter(getWorkspaceRoot()).register("prepare",PreparationScript).register("synchronize",SynchronizeScript).run(process.argv.slice(2));
 }
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧬️schema/🏃️invocation/🔣️.json

```diff
--- 
+++ 
@@ -57,4 +57,29 @@
       }
     }
-  ]
+  ],
+  "$defs": {
+    "CargoPreparationSelectionV1": {
+      "type": "object",
+      "additionalProperties": false,
+      "required": [
+        "manifest",
+        "packages"
+      ],
+      "properties": {
+        "manifest": {
+          "type": "string",
+          "pattern": "^(?!/)(?![A-Za-z]:/)(?!.*\\\\)(?!.*(?:^|/)\\.\\.(?:/|$)).*Cargo\\.toml$"
+        },
+        "packages": {
+          "type": "array",
+          "uniqueItems": true,
+          "items": {
+            "type": "string",
+            "minLength": 1,
+            "pattern": "^[^\\u0000]+$"
+          }
+        }
+      }
+    }
+  }
 }
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts

```diff
--- 
+++ 
@@ -198,4 +198,21 @@
 
 
+    const scoped=fixture.selectedPreparation;
+    const selectionSchema=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧬️schema/🏃️invocation/🔣️.json"),"utf8")).$defs.CargoPreparationSelectionV1;
+    assert.equal(new (require("ajv"))({strict:true}).compile(selectionSchema)(scoped.selection),true);
+    writeFileSync(join(root,"Cargo.toml"),fixture.cargo.preparedFiles["Cargo.toml"].replace('members = [".", "generated"]','members = [".", "generated", "unrelated"]').replace('member-manifests = ["Cargo.toml", "generated/Cargo.toml"]','member-manifests = ["Cargo.toml", "generated/Cargo.toml", "unrelated/Cargo.toml"]'));
+    writeFileSync(recipe,parent.replace(ancestor.parentRead,""));
+    writeFileSync(join(root,"generated/Cargo.toml"),fixture.cargo.preparedFiles["generated/Cargo.toml"]+ordered.metadata);writeFileSync(join(root,"generated/📜️script.ts"),ordered.script);
+    mkdirSync(join(root,"unrelated"),{recursive:true});writeFileSync(join(root,"unrelated/Cargo.toml"),scoped.unusedManifest);writeFileSync(join(root,"unrelated/lib.rs"),"");writeFileSync(join(root,"unrelated/📜️script.ts"),scoped.unusedScript);
+    let scopedCalls=0;
+    await withPreparedCargoDependencyPairV1(root,scoped.selection.manifest,controller.signal,async(args:string[])=>{
+      scopedCalls++;assert.equal(readFileSync(join(root,"generated/child-output.txt"),"utf8"),"7");
+      const actual=Bun.spawnSync(["cargo",...args,"--offline"],{cwd:root,stdout:"pipe",stderr:"pipe"});assert.equal(actual.exitCode,0,actual.stderr.toString());
+      const metadata=Bun.spawnSync(["cargo","metadata","--locked","--offline","--no-deps","--format-version=1"],{cwd:root,stdout:"pipe",stderr:"pipe"});assert.equal(metadata.exitCode,0,metadata.stderr.toString());assert.deepEqual(JSON.parse(metadata.stdout.toString()).packages.map((pkg:any)=>pkg.name).sort(),scoped.expectedPackages);
+      const tree=Bun.spawnSync(["cargo","tree","--locked","--offline","--prefix","none","--format","{p}","-p",...scoped.selection.packages],{cwd:root,stdout:"pipe",stderr:"pipe"});assert.equal(tree.exitCode,0,tree.stderr.toString());assert.deepEqual(tree.stdout.toString().trim().split("\n").map(line=>line.split(" ")[0]).sort(),scoped.expectedTreePackages);
+      assert.deepEqual(require("smol-toml").parse(readFileSync(join(root,"Cargo.lock"),"utf8")),Bun.TOML.parse(readFileSync(join(root,"Cargo.lock"),"utf8")));
+    },[],scoped.selection.packages);
+    assert.equal(scopedCalls,2);
+
     passed = true;
   } finally {
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts

```diff
--- 
+++ 
@@ -108,5 +108,4 @@
  * [[CARGO_RELAY_SCRIPT]] (POSIX), so it never writes to an inherited, possibly non-blocking descriptor ([[cargoStreamingStatus]]). */
 function runCmdInternal(cmd: string, args: string[], opts: RunCmdOpts): number {
-  if (cmd === "cargo") prepareCargoWorkspaceInvocation(getWorkspaceRoot(), args, opts.cwd ?? process.cwd());
   const budgetMs = opts.budgetMs ?? defaultBudgetMs(cmd);
   const formattedArgs = [...args];
@@ -120,4 +119,5 @@
   }
   const relayed = cmd === "cargo" && (process.platform !== "win32" || observesCargoInvocation(formattedArgs, opts.env ?? process.env));
+  if (cmd === "cargo" && !relayed) prepareCargoWorkspaceInvocation(getWorkspaceRoot(), args, opts.cwd ?? process.cwd(), opts.env ?? process.env);
   const result = relayed
     ? spawnSync(process.versions.bun ? process.execPath : "bun", [CARGO_RELAY_SCRIPT, "relay", ...formattedArgs], {
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📜️script.ts

```diff
--- 
+++ 
@@ -8,5 +8,5 @@
 import { getWorkspaceRoot } from "../🟦️.ts";
 import { publishCargoWorkspaceMemberships, cargoRepositoryPackages } from "./🟦️.ts";
-import {PreparationScript} from "./🛠️preparation/📜️script.ts";
+import {PreparationScript,SynchronizeScript} from "./🛠️preparation/📜️script.ts";
 
 /** 📣️ Checks or publishes current native owner membership through its authored regular-manifest recipe. */
@@ -66,3 +66,3 @@
   }
 }
-if (import.meta.main) await new ScriptRouter(getWorkspaceRoot()).register("capability-contract-check", CapabilityContractScript).register("runtime-contract-check",RuntimeContractScript).register("runtime-input-check",RuntimeInputScript).register("bun-contract-check",BunContractScript).register("members", MembersScript).register("prepare",PreparationScript).register("contract-check", ContractScript).register("queued-contract-check", QueuedContractScript).register("native-input-check", NativeInputScript).run(process.argv.slice(2));
+if (import.meta.main) await new ScriptRouter(getWorkspaceRoot()).register("capability-contract-check", CapabilityContractScript).register("runtime-contract-check",RuntimeContractScript).register("runtime-input-check",RuntimeInputScript).register("bun-contract-check",BunContractScript).register("members", MembersScript).register("prepare",PreparationScript).register("synchronize",SynchronizeScript).register("contract-check", ContractScript).register("queued-contract-check", QueuedContractScript).register("native-input-check", NativeInputScript).run(process.argv.slice(2));
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts

```diff
--- 
+++ 
@@ -7,5 +7,4 @@
 import { CARGO_RELAY_BUDGET_ENV, cargoStreamingStatus, runCmdStatus } from "../../../🏃️process/🟦️.ts";
 import { repositoryProcessOwnerContextV1, repositoryVitestPolicyV1, repositoryCargoTestPolicyV1, repositoryWasmBuildPolicyV1 } from "../../../🟦️.ts";
-import { prepareCargoWorkspaceInvocation } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
 import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
 import { runRepositoryCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
@@ -40,9 +39,6 @@
     const index = args.indexOf("--manifest");
     const manifest = index >= 0 ? args[index + 1] : undefined;
-    if(tool==="owner-command"||tool==="repository-test-body"){
+    if(tool==="owner-command"){
       const request=nativeOwnerTestManifestRequestV1(args.slice(1)),path=resolve(this.repoRoot,request.manifest),cwd=resolve(this.repoRoot,request.cwd);
-      const consuming=tool==="repository-test-body";
-      if(consuming && (request.command!=="bun"&&request.command!==process.execPath || resolve(cwd,request.args[0]??"")!==resolve(cwd,"📜️script.ts") || request.args[1]!=="test"))throw Error("Repository test body requires its exact owned Bun script test route");
-      if(!consuming)prepareCargoWorkspaceInvocation(this.repoRoot,["test","--manifest-path",path],cwd);
       const cargo = Bun.TOML.parse(readFileSync(path, "utf8")) as { package?: { name?: string }; workspace?: object };
       if (!cargo.package?.name && !cargo.workspace) throw Error(`Native owner requires a package or workspace manifest: ${manifest}`);
@@ -52,5 +48,4 @@
       const policies=request.testManifests.map(manifest=>{
         const selected=resolve(this.repoRoot,manifest);
-        if(!consuming)prepareCargoWorkspaceInvocation(this.repoRoot,["test","--manifest-path",selected],cwd);
         return repositoryCargoTestPolicyV1(selected,cwd);
       });
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body/🟦️.ts

```diff
--- 
+++ 
@@ -1,6 +1,9 @@
 import {test,expect} from "bun:test";
 import {readFileSync} from "node:fs";
-import {resolve,dirname} from "node:path";
+import {resolve,dirname,join,relative,isAbsolute} from "node:path";
 import ts from "typescript";
+import {transformSync} from "esbuild";
+import Ajv from "ajv";
+import {parse as parseJsonc} from "jsonc-parser";
 import {nativeOwnerTestManifestRequestV1} from "../../🗺️owner-test-manifests/🟦️.ts";
 const owner=resolve(import.meta.dir,"../.."),fixture=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/📬️test-body/🔣️.json"),"utf8"));
@@ -11,5 +14,5 @@
 test("typed owner transport prepares only at actual current Cargo consumption",async()=>{
  const oracle=Bun.spawnSync(["node","--eval",`const cases=${JSON.stringify(fixture.cases)};console.log(JSON.stringify(cases.map(row=>row.route==='owner-command'?(row.consume?['owned','prepare:2','cargo:2']:['owned']):['refused'])))`]);expect(oracle.exitCode).toBe(0);expect(JSON.parse(new TextDecoder().decode(oracle.stdout))).toEqual(fixture.cases.map((row:any)=>row.expected));
- for(const compile of [(text:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(text),(text:string)=>ts.transpileModule(text,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).outputText]){
+ for(const compile of [(text:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(text),(text:string)=>ts.transpileModule(text,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).outputText,(text:string)=>transformSync(text,{loader:"ts",target:"es2022"}).code]){
   for(const row of fixture.cases){
    const records:string[]=[];let version=1;
@@ -25,13 +28,56 @@
 });
 
-test("canonical Nx target route requires explicit current Cargo consumer ownership",()=>{
+test("canonical Nx inference delegates every owned command without eager or special test preparation",()=>{
  const syntax=ts.createSourceFile("nx.mjs",readFileSync(resolve(library,"🟨️.mjs"),"utf8"),ts.ScriptTarget.Latest,true);
- const declaration=syntax.statements.find((node:any)=>node.name?.text==="nativeOwnerExecutionRoute")!.getText(syntax).replace(/^export /,"");
- const classify=new Function(declaration+";return nativeOwnerExecutionRoute;")();
- for(const row of fixture.targets){let result;try{result=classify(row.target);}catch{result="refused";}expect(result,row.id).toBe(row.expected);}
- const roots=["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust","✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust","✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust","✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust","✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust"];
+ const declaration=syntax.statements.find((node:any)=>node.name?.text==="projectWithDefaults")!.getText(syntax);
+ const manifest={package:{name:"owner"},workspace:{}},closure=["/repo/🏃️process/🧪️testing/🦀️cargo/🟦️.ts"];
+ const bindings={existsSync:()=>true,join,resolve,dirname,relative,isAbsolute,readToml:()=>manifest,readFileSync:()=>"",SCRIPT_BASENAME:"📜️script.ts",DEFAULT_EXECUTOR:"nx:run-commands",LIBRARY_ROOT:"/repo/library",POLICY:{targetDefaults:{},nxSerialTargets:[]},withWasmTooling:(targets:any)=>targets,rootCommandTargets:()=>({}),cargoTargets:()=>({}),componentTargets:()=>({}),printDocumentTargets:()=>({}),generatorContractInputs:()=>({}),generatorOutputOwners:()=>[],genericCommandFallbackInputs:()=>[],withLeveledTestTargets:(targets:any)=>targets,targetPolicy:(_name:string,target:any)=>target,targetWithDefaults:(target:any,root:string)=>({...target,options:{cwd:root,...target.options}}),nativeTargetCommandInputs:()=>closure,targetScriptClosure:()=>closure,nxPath:(path:string)=>path.replaceAll("\\","/"),nativeLockInputs:()=>[],genericTargetCommandInputs:()=>[],generatorOutputCouplingInputs:()=>[],projectInputs:()=>({})};
+ for(const compile of [(text:string)=>text,(text:string)=>transformSync(text,{loader:"js",target:"es2022"}).code]){
+  const infer=new Function(...Object.keys(bindings),declaration+";return projectWithDefaults;")(...Object.values(bindings));
+  for(const row of fixture.targets){
+   const result=infer({name:"owner",targets:{test:row.target}},"owner","/repo/owner","/repo"),command=result.targets.test.options.command;
+   expect(command,row.id).toContain(" native owner-command --manifest ");
+   expect(command,row.id).not.toContain("repository-test-body");
+   const expected=row.target.options.command.replace(/^bun\s+("[^"\n]+"|'[^'\n]+'|[^\s]+)/u,(_:string,path:string)=>`bun ${JSON.stringify(resolve("/repo/owner",path.replace(/^["']|["']$/g,"")))}`);
+   expect(command.endsWith(` -- ${expected}`),row.id).toBe(true);
+   expect(result.targets.test.options.cwd).toBe(".");
+   expect(result.targets.test.inputs).toContain("nativeTestSources");
+  }
+ }
+ const schema=JSON.parse(readFileSync(resolve(owner,"🧬️schema/🗺️owner-test-manifests/🔣️.json"),"utf8")),admit=new Ajv({strict:true}).compile(schema);
+ for(const row of fixture.cases.filter((row:any)=>row.route==="owner-command")){
+  const request=nativeOwnerTestManifestRequestV1(["--manifest","owner/Cargo.toml","--cwd","owner","--",row.command,...row.args]);
+  expect(Boolean(admit(request)),row.id).toBe(true);
+ }
  const repository=resolve(library,"../../../../..");
- for(const root of roots){const project=JSON.parse(readFileSync(resolve(repository,root,"📋️project.json"),"utf8"));expect(classify(project.targets.test),project.name).toBe("repository-test-body");}
- for(const root of ["🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust","🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"]){const project=JSON.parse(readFileSync(resolve(repository,root,"📋️project.json"),"utf8"));expect(classify(project.targets.test),project.name).toBe("owner-command");}
- console.log("[DEBUG] Nx5 verified Cargo-consuming test owners/level siblings selected;2 direct neutral driver owners preserve eager preparation; custom declarations refused");
+ for(const root of fixture.testOwners){
+  const project=JSON.parse(readFileSync(resolve(repository,root,"📋️project.json"),"utf8"));
+  expect(project.targets.test.metadata?.semio?.nativePreparation,project.name).toBeUndefined();
+ }
+ console.log("[DEBUG] actual complete Nx target inference and independent esbuild agree; generic/test/build argv preserved; Ajv admits current owner requests;5 redundant metadata declarations absent");
 });
+
+test("canonical selected native callers preserve compiler argv and keep full setup explicit",()=>{
+ const repository=resolve(library,"../../../../..");
+ const oracle=Bun.spawnSync(["node","--eval",`const fs=require('node:fs'),path=require('node:path');const root=process.argv[1],rows=JSON.parse(process.argv[2]);console.log(JSON.stringify(rows.map(row=>{const target=JSON.parse(fs.readFileSync(path.join(root,row.project),'utf8')).targets[row.target];return{...row,command:target.options.command,dependencies:target.dependsOn}})))`,repository,JSON.stringify(fixture.callerTargets)]);
+ expect(oracle.exitCode).toBe(0);expect(JSON.parse(new TextDecoder().decode(oracle.stdout))).toEqual(fixture.callerTargets);
+ for(const row of fixture.callerTargets){
+  const target=JSON.parse(readFileSync(resolve(repository,row.project),"utf8")).targets[row.target];
+  expect(target.options.command,row.target).toBe(row.command);
+  expect(target.dependsOn,row.target).toEqual(row.dependencies);
+  expect(target.dependsOn,row.target).not.toContain("workspace:deps-cargo");
+ }
+ const workspace=JSON.parse(readFileSync(resolve(repository,"📋️project.json"),"utf8"));
+ expect(workspace.targets["deps-cargo"].options.command).toMatch(/ sync cargo$/u);
+ expect(workspace.targets["deps-cargo"].cache).toBe(false);
+ const target=JSON.parse(readFileSync(resolve(library,"🗂️workspaces/🦀️cargo/📋️project.json"),"utf8")).targets.synchronize;
+ expect(target.options.command).toBe("bun ./📜️script.ts synchronize");expect(target.options.forwardAllArgs).toBe(true);expect(target.cache).toBe(false);
+ for(const path of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){
+  const errors:any[]=[],launch=parseJsonc(readFileSync(resolve(repository,path),"utf8"),errors);expect(errors).toEqual([]);
+  const selected=launch.configurations.filter((row:any)=>row.command?.includes("@semio-tech/cargo-workspaces:synchronize"));
+  expect(selected.length,path).toBe(1);expect(selected[0].command).toContain('-- --manifest "${input:cargoSynchronizationManifest}"');
+  expect(launch.inputs.filter((row:any)=>row.id==="cargoSynchronizationManifest").length,path).toBe(1);
+ }
+ const router=readFileSync(resolve(library,"🗂️workspaces/🦀️cargo/📜️script.ts"),"utf8");expect(router).toContain('.register("synchronize",SynchronizeScript)');
+ console.log("[DEBUG]4 real native caller vectors retain compiler argv/tool prerequisites; Node JSON oracle agrees; explicit global setup and mandatory-manifest Nx/live/seed synchronization route admitted");
+});
```
