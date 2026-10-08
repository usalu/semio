## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts
```diff
--- 

+++ 

@@ -262,6 +262,6 @@

  const visit=(manifest:string):void=>{
  if(finished.has(manifest)||visiting.has(manifest))return;visiting.add(manifest);
- const start=timed?performance.now():0,{owner,pkg,current}=binding(manifest);
- for(const dependency of localDependencies(manifest,roots.has(manifest))){pending.add(dependency);visit(dependency);}
+ const start=timed?performance.now():0,{owner,pkg}=binding(manifest);if(!names.length&&owner.directory===selected.directory)roots.add(manifest);
+ while(true){const dependencies=localDependencies(manifest,roots.has(manifest));for(const dependency of dependencies)pending.add(dependency);const unfinished=dependencies.filter(path=>!finished.has(path)&&!visiting.has(path));if(!unfinished.length)break;for(const dependency of unfinished)visit(dependency);}
  diagnostic("closure",manifest,start,pending.size);
  const value=document(manifest).package?.metadata?.semio?.preparation;if(value===undefined){visiting.delete(manifest);finished.add(manifest);return;}
@@ -280,5 +280,12 @@

  };
  for(const manifest of pending)visit(manifest);
- if(!names.length)while(true){const additions=members(cargoWorkspaceForManifest(root,selected.manifest)).filter(pkg=>!finished.has(pkg.manifest));if(!additions.length)break;for(const pkg of additions){roots.add(pkg.manifest);pending.add(pkg.manifest);visit(pkg.manifest);}}
+ while(true){
+ documents.clear();inventories.clear();scopes=discoverCargoWorkspaces(root);nativeScopes.clear();for(const scope of scopes)nativeScopes.set(scope.manifest,scope);
+ if(!names.length)for(const pkg of members(cargoWorkspaceForManifest(root,selected.manifest)))roots.add(pkg.manifest);
+ const closure=new Set(roots);for(const manifest of closure)for(const dependency of localDependencies(manifest,roots.has(manifest)))closure.add(dependency);
+ const before=executed.size;for(const manifest of closure){pending.add(manifest);visit(manifest);}if(executed.size!==before)continue;
+ for(const manifest of closure){const {owner,pkg,current}=binding(manifest),value=current.package?.metadata?.semio?.preparation;if(value===undefined)continue;const recipe=parseCargoPreparation(value),script=physical(root,slash(relative(root,resolve(root,pkg.directory,recipe.script)))),key=JSON.stringify([script,...recipe.command]);if(slash(relative(resolve(root,owner.directory),script)).startsWith("../")||!executed.has(key))throw Error(`Prepared recipe closure changed: ${manifest}`);}
+ break;
+ }
  for(const recipe of recipes)assertCargoPreparationObservationCurrentV1(recipe);
  diagnostic("finished",selected.manifest,started,pending.size);
```
## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts
```diff
--- 

+++ 

@@ -189,4 +189,9 @@

     });
     assert.equal(ancestorCalls,2);
+    rmSync(join(root,"additional"),{recursive:true,force:true});writeFileSync(join(root,"Cargo.toml"),fixture.cargo.preparedFiles["Cargo.toml"]);writeFileSync(join(root,"generated/Cargo.toml"),fixture.cargo.preparedFiles["generated/Cargo.toml"]+ordered.metadata);writeFileSync(join(root,"generated/📜️script.ts"),ordered.script);
+    const changed=fixture.changedPreparation,changedManifest=fixture.cargo.preparedFiles["generated/Cargo.toml"]+ordered.metadata.replace('command=["prepare"]',`command=${JSON.stringify(changed.command)}`),change=`const target=resolve(process.cwd(),"generated/Cargo.toml"),content=${JSON.stringify(changedManifest)};writeFileSync(target,content);observeCargoPreparationOutputV1(target,content);`;
+    writeFileSync(recipe,ordered.script.replace("completeCargoPreparationObservationV1();",change+"\ncompleteCargoPreparationObservationV1();"));
+    let changedCalls=0;await assert.rejects(withPreparedCargoDependencyPairV1(root,"Cargo.toml",controller.signal,async()=>{changedCalls++;}),new RegExp(changed.expected));assert.equal(changedCalls,0);
+
 
 
```
## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json
```diff
--- 

+++ 

@@ -30052,8 +30052,4 @@

         "🚚️move": "move-morph-target-attribute",
         "🔀️reorder": "reorder-morph-target-attributes"
-      },
-      "📸️snapshot": {
-        "📸️set": "set-snapshot",
-        "🩹️patch": "patch-snapshot"
       }
     }
```
## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts
```diff
--- 

+++ 

@@ -3068,5 +3068,6 @@

   const constraints = ["enum", "const", "minLength", "maxLength", "pattern", "format", "minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum", "multipleOf", "items", "additionalItems", "minItems", "maxItems", "uniqueItems", "contains", "required", "additionalProperties", "patternProperties", "propertyNames", "minProperties", "maxProperties", "dependencies", "not", "if", "then", "else"];
   const annotations = ["type", "title", "description", "default", "examples", "readOnly", "writeOnly", "$id", "$comment", "$schema"];
-  if (typeof node.$ref === "string" || object(node.properties) || node.type !== undefined && (constraints.some(key => Object.hasOwn(node, key)) || Object.keys(node).every(key => annotations.includes(key)))) return true;
+  const propertiesAuthority = object(node.properties) && (Object.keys(node.properties).length > 0 || node.type !== undefined || Object.keys(node).every(key => annotations.includes(key) || constraints.includes(key) || ["properties", "$defs", "definitions", "allOf", "anyOf", "oneOf"].includes(key) || key.startsWith("x-")));
+  if (typeof node.$ref === "string" || propertiesAuthority || node.type !== undefined && (constraints.some(key => Object.hasOwn(node, key)) || Object.keys(node).every(key => annotations.includes(key)))) return true;
   return ["allOf", "anyOf", "oneOf"].some(key => Array.isArray(node[key]) && node[key].length > 0 && node[key].every(child => typeof child === "boolean" || schemaRootAuthority(child)))
     || ["$defs", "definitions"].some(key => object(node[key]) && Object.values(node[key]).some(schemaRootAuthority));
```
## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts
```diff
--- 

+++ 

@@ -7441,5 +7441,5 @@

     const taxonomy = library.loadCatalogTaxonomy();
     const casesPath = join(import.meta.dir, "../../🧫️fixtures/🧬️schema-scope-catalog/🔣️.json");
-    const cases = JSON.parse(readFileSync(casesPath, "utf8")) as { contract: string; cases: { id: string; files: Record<string, unknown>; expected: { scopes: Record<string, { path: string; level: string; exports: Record<string, { file: string; facet: string }>; dependsOn: string[] }>; diagnosticCodes: string[]; placementPaths: string[] }; schemaGrammar?: Record<string, boolean> }[] };
+    const cases = JSON.parse(readFileSync(casesPath, "utf8")) as { contract: string; cases: { id: string; files: Record<string, unknown>; expected: { scopes: Record<string, { path: string; level: string; exports: Record<string, { file: string; facet: string }>; dependsOn: string[] }>; diagnosticCodes: string[]; placementPaths: string[] }; schemaGrammar?: Record<string, boolean>; schemaCompile?: Record<string, boolean> }[] };
     const authority = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🧬️schema-scope-catalog/🔣️.json"), "utf8"));
     const ajv = new Ajv({ strict: true });
@@ -7455,4 +7455,9 @@

         }
         for (const [path, expected] of Object.entries(row.schemaGrammar ?? {})) expect(new Ajv({ strict: false }).validateSchema(row.files[path]), row.id).toBe(expected);
+        for (const [path, expected] of Object.entries(row.schemaCompile ?? {})) {
+          let accepted = true;
+          try { new Ajv({ strict: true, strictTypes: false }).compile(row.files[path]); } catch { accepted = false; }
+          expect(accepted, row.id).toBe(expected);
+        }
         const inventory = library.inventorySchemaScopes(root, taxonomy);
         const scopes = Object.fromEntries(Object.entries(inventory.catalog.scopes).map(([id, scope]) => [id, { path: scope.path, level: scope.level, exports: scope.exports, dependsOn: [...scope.dependsOn] }]));
```
## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts
```diff
--- 

+++ 

@@ -3,4 +3,6 @@

 import { cargoInputDigestV1, cargoDirectoryEntriesV1 } from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
 import { blake3Hex } from "../../../../../../../🔨️modules/🔏️hash/🟦️.ts";
+import type {FileObservationControlV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🟦️.ts";
+import {cmdBudgetMs} from "../../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
 import { homedir } from "node:os";
 import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";
@@ -361,7 +363,7 @@

 /** 🛡️ Executes actual Cargo target metadata and runtime source graphs, retaining qualified evidence rather than inferring compilation. */
 /** 🎭️ Resolves the actual Dev-selected closed actor generation and its original package/factory compiler custody. */
-export async function runtimeSelectedActorsV1(root:string,config:{actorProvenanceOwner:string;actorCatalogOwner:string;actorDevelopmentOwner:string;actorFactoryOwner:string;actorDescriptorOwner:string;actorChildOwner:string},context:{read:(path:string)=>string|undefined;current:(observation:RuntimeCargoObservationV1)=>boolean;fixtureCollections:readonly string[];checkCancellation:()=>void}){
+export async function runtimeSelectedActorsV1(root:string,config:{actorProvenanceOwner:string;actorCatalogOwner:string;actorDevelopmentOwner:string;actorFactoryOwner:string;actorDescriptorOwner:string;actorChildOwner:string},context:{read:(path:string)=>string|undefined;current:(observation:RuntimeCargoObservationV1)=>boolean;fixtureCollections:readonly string[];checkCancellation:()=>void;observation:FileObservationControlV1}){
   context.checkCancellation();const [publicationOwner,catalogOwner,developmentOwner,factory,descriptorOwner]=await Promise.all([config.actorProvenanceOwner,config.actorCatalogOwner,config.actorDevelopmentOwner,config.actorFactoryOwner,config.actorDescriptorOwner].map(path=>import(join(root,path))));
-  const dataRoot=publicationOwner.trustedCatalogDataRootV1(root,"development"),published=publicationOwner.readTrustedCatalogPublicationProvenanceV1(dataRoot),requested=developmentOwner.LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES,selection=catalogOwner.trustedBootstrapSelectPackages(requested),profile=catalogOwner.trustedBootstrapProfileIdV1(selection),generationRoot=join(dataRoot,"trusted-catalog","generations",published.record.generationId),bundle=JSON.parse(context.read(relative(root,published.record.bundle.path))!),childSource=context.read(config.actorChildOwner);
+  const dataRoot=publicationOwner.trustedCatalogDataRootV1(root,"development"),published=await publicationOwner.readTrustedCatalogPublicationProvenanceV1(dataRoot,context.observation),requested=developmentOwner.LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES,selection=catalogOwner.trustedBootstrapSelectPackages(requested),profile=catalogOwner.trustedBootstrapProfileIdV1(selection),generationRoot=join(dataRoot,"trusted-catalog","generations",published.record.generationId),bundle=JSON.parse(context.read(relative(root,published.record.bundle.path))!),childSource=context.read(config.actorChildOwner);
   const fail=(detail:string):never=>{throw Error("Selected production actor: "+detail);},hash=(bytes:string|Uint8Array)=>createHash("sha256").update(bytes).digest("hex"),path=(value:string)=>relative(root,resolve(value)).replaceAll("\\","/"),inside=(owner:string,value:string)=>{const part=relative(owner,value);return part!==""&&!isAbsolute(part)&&part.split(/[\\/]/u)[0]!=="..";};
   const bytes=(claim:{path:string;sha256:string;byteLength:number},owner?:string):Buffer=>{context.checkCancellation();if(!isAbsolute(claim.path)||owner&&!inside(owner,claim.path))fail("byte owner escaped");const stat=lstatSync(claim.path);if(!stat.isFile()||stat.isSymbolicLink()||!Number.isSafeInteger(claim.byteLength)||claim.byteLength<1||claim.byteLength>128*1024*1024||stat.size!==claim.byteLength||!/^[a-f0-9]{64}$/u.test(claim.sha256))fail("bounded original byte claim refused");const value=readFileSync(claim.path);if(value.length!==claim.byteLength||hash(value)!==claim.sha256)fail("original selected bytes changed");context.read(path(claim.path));return value;};
@@ -387,20 +389,21 @@

   }
   if(!owners.length)fail("selected Dev generation has no closed actors");
-  const recheck=()=>{context.checkCancellation();const current=publicationOwner.readTrustedCatalogPublicationProvenanceV1(dataRoot);if(current.path!==published.path||JSON.stringify(current.record)!==JSON.stringify(published.record))fail("selected pointer or bundle changed during proof");for(const witness of witnesses as any[]){bytes(witness.producer.compiler);bytes(witness.producer.runtime);for(const input of witness.producer.inputs)bytes(input);}if(observations.some(observation=>!context.current(observation)))fail("original production compiler/resource inputs changed during proof");};recheck();
+  const recheck=async(observation:FileObservationControlV1)=>{context.checkCancellation();const current=await publicationOwner.readTrustedCatalogPublicationProvenanceV1(dataRoot,observation);if(current.path!==published.path||JSON.stringify(current.record)!==JSON.stringify(published.record))fail("selected pointer or bundle changed during proof");for(const witness of witnesses as any[]){bytes(witness.producer.compiler);bytes(witness.producer.runtime);for(const input of witness.producer.inputs)bytes(input);}if(observations.some(observation=>!context.current(observation)))fail("original production compiler/resource inputs changed during proof");};await recheck(context.observation);
   return{dataRoot,requested,profile,published,dynamicImports:{[config.actorChildOwner]:owners},observations,witnesses,recheck};
 }
 
 /** 🔎️ Supplies the current native storage and typed input owners to selected actor verification. */
-function runtimeDevelopmentActorContextV1(root:string,read:(path:string)=>string|undefined,fixtureCollections:readonly string[],checkCancellation:()=>void){
+function runtimeDevelopmentActorContextV1(root:string,read:(path:string)=>string|undefined,fixtureCollections:readonly string[],checkCancellation:()=>void,observation:FileObservationControlV1){
   const current=(observation:RuntimeCargoObservationV1)=>{checkCancellation();return runtimeCargoObservationCurrentV1(observation,{root,cwd:observation.cwd,outDirectory:"",buildDirectory:cargoDirectories(root).build,cargoHome:resolve(process.env.CARGO_HOME??join(homedir(),".cargo")),digest:path=>cargoInputDigestV1(path).sha256??undefined,fixtureCollections,read:path=>{checkCancellation();try{return readFileSync(resolve(root,path))}catch{return undefined}},directoryEntries:path=>{checkCancellation();return cargoDirectoryEntriesV1(resolve(root,path))?.map(([name,kind,symlinkTarget])=>({path:join(resolve(root,path),name),kind:kind as RuntimeCompilerResourceEntryV1["kind"],symlinkTarget}));}});};
-  return{read,current,fixtureCollections,checkCancellation};
+  return{read,current,fixtureCollections,checkCancellation,observation};
 }
 
 /** 🧾️ Verifies the existing selected Dev actors through their current owners without acquiring or publishing evidence. */
-export async function loadCurrentDevelopmentActorsV1(root:string,checkCancellation:()=>void){
+export async function loadCurrentDevelopmentActorsV1(root:string,observation:FileObservationControlV1){
+  const checkCancellation=()=>{if(observation.cancelled())throw Error("Selected production actor observation cancelled");};
   checkCancellation();root=resolve(root);const config=JSON.parse(readFileSync(join(import.meta.dir,"../🔣️.json"),"utf8")),taxonomy=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),"utf8"));
   const fixtureCollections=["fixtures","test-tube-fixtures"].flatMap(id=>["fixtures","examples"].filter(slug=>new RegExp(taxonomy.semanticDirectoryKinds[id].slugPattern,"u").test(slug)).map(slug=>taxonomy.semanticDirectoryKinds[id].emoji+slug)),identities=new Map<string,string>();
   const read=(path:string):string|undefined=>{checkCancellation();const absolute=resolve(root,path);if(!existsSync(absolute))return;const actual=realpathSync(absolute),physical=relative(root,actual).replaceAll("\\","/");if(runtimeFixturePathV1(physical,fixtureCollections))throw Error("Selected production actor: physical input belongs to a fixture collection");let bytes:Buffer;try{bytes=readFileSync(actual)}catch{return;}const sha256=createHash("sha256").update(bytes).digest("hex"),previous=identities.get(actual);if(previous&&previous!==sha256)throw Error("Selected production actor: input changed during read-only verification");identities.set(actual,sha256);return bytes.toString("utf8");};
-  const selected=await runtimeSelectedActorsV1(root,config,runtimeDevelopmentActorContextV1(root,read,fixtureCollections,checkCancellation)),recheck=()=>{checkCancellation();selected.recheck();for(const [path,sha256]of identities)if(cargoInputDigestV1(path).sha256!==sha256)throw Error("Selected production actor: input changed after read-only verification");};recheck();return{...selected,recheck};
+  const selected=await runtimeSelectedActorsV1(root,config,runtimeDevelopmentActorContextV1(root,read,fixtureCollections,checkCancellation,observation)),recheck=async(control:FileObservationControlV1)=>{checkCancellation();await selected.recheck(control);for(const [path,sha256]of identities)if(cargoInputDigestV1(path).sha256!==sha256)throw Error("Selected production actor: input changed after read-only verification");};await recheck(observation);return{...selected,recheck};
 }
 
@@ -429,4 +432,5 @@

   process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
   const checkCancellation = (): void => { if (signal.signal.aborted) throw Error("Runtime graph canceled"); };
+  const observationStarted=Date.now(),observationBudget=cmdBudgetMs(),observation:FileObservationControlV1={maxBytes:128*1024*1024,maxWork:65536,chunkBytes:1024*1024,cancelled:()=>signal.signal.aborted,remainingMs:()=>(observationBudget>0?observationBudget:86_400_000)-(Date.now()-observationStarted),onProgress:step=>console.log(`[runtime graph] physical file ${step.phase} ${step.bytes}/${step.totalBytes}`)};
   const read = (path: string): string | undefined => {
     checkCancellation(); const absolute = resolve(root, path);
@@ -458,5 +462,5 @@

       observations.push(...completed);
     }
-    const loadActors=()=>runtimeSelectedActorsV1(root,config,runtimeDevelopmentActorContextV1(root,read,fixtureCollections,checkCancellation));
+    const loadActors=()=>runtimeSelectedActorsV1(root,config,runtimeDevelopmentActorContextV1(root,read,fixtureCollections,checkCancellation,observation));
     try{try{actorSelection=await loadActors();}catch(error){
       if(provenance||artifacts||dynamic)throw error;
@@ -626,5 +630,5 @@

     findings.push({code:"runtime-unresolved-edge",path:config.guestChecks,detail:`Actual runtime witness acquisition/verification failed: ${String(error)}`});
   } finally {
-    try{actorSelection?.recheck();}catch(error){findings.push({code:"runtime-input-mismatch",path:config.actorProvenanceOwner,detail:String(error)});}
+    try{await actorSelection?.recheck(observation);}catch(error){findings.push({code:"runtime-input-mismatch",path:config.actorProvenanceOwner,detail:String(error)});}
     for(const observation of observations)for(const resource of observation.compilerResources)findings.push(...runtimeCompilerResourceInputsV1(observation,resource,{root,digest:path=>cargoInputDigestV1(path).sha256??undefined,fixtureCollections,read:path=>{try{return readFileSync(path)}catch{return undefined}},entries:path=>cargoDirectoryEntriesV1(path)??undefined}).findings);
     for(const[path,sha256]of identities)try{if(createHash("sha256").update(readFileSync(resolve(root,path))).digest("hex")!==sha256)findings.push({code:"runtime-input-mismatch",path,detail:"Source/resource changed before actual runtime graph completion"});}catch{findings.push({code:"runtime-input-mismatch",path,detail:"Source/resource disappeared before actual runtime graph completion"});}
```
## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🧪️tests/🟦️.ts
```diff
--- 

+++ 

@@ -403,5 +403,5 @@

 test("ordinary selected Dev actor acquisition refuses an absent production ledger",async()=>{
  const {runtimeSelectedActorsV1}=await import("../🔎️verification/🟦️.ts"),root=process.cwd(),config=JSON.parse(readFileSync(join(owner,"🔣️.json"),"utf8")),directory=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,`missing-actor-ledger-${crypto.randomUUID()}`),previous=process.env.OS_HUB_DATA;mkdirSync(join(directory,"trusted-catalog"),{recursive:true});process.env.OS_HUB_DATA=directory;
- try{await expect(runtimeSelectedActorsV1(root,config,{read:path=>{try{return readFileSync(resolve(root,path),"utf8")}catch{return undefined}},current:()=>false,fixtureCollections:["🧫️fixtures"],checkCancellation:()=>{}})).rejects.toThrow("current.json");const {LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES}=await import("../../../../../../../../🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts");expect(LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES.split(",")).toHaveLength(6);}finally{if(previous===undefined)delete process.env.OS_HUB_DATA;else process.env.OS_HUB_DATA=previous;}
+ try{const started=Date.now(),observation={maxBytes:128*1024*1024,maxWork:65536,chunkBytes:1024*1024,cancelled:()=>false,remainingMs:()=>60_000-(Date.now()-started),onProgress:()=>{}};await expect(runtimeSelectedActorsV1(root,config,{read:path=>{try{return readFileSync(resolve(root,path),"utf8")}catch{return undefined}},current:()=>false,fixtureCollections:["🧫️fixtures"],checkCancellation:()=>{},observation})).rejects.toThrow("current.json");const {LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES}=await import("../../../../../../../../🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts");expect(LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES.split(",")).toHaveLength(6);}finally{if(previous===undefined)delete process.env.OS_HUB_DATA;else process.env.OS_HUB_DATA=previous;}
 });
 
@@ -410,5 +410,5 @@

  const {loadCurrentDevelopmentActorsV1}=await import("../🔎️verification/🟦️.ts"),vectors=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).developmentActorReads,previous=process.env.OS_HUB_DATA;
  expect(typeof loadCurrentDevelopmentActorsV1).toBe("function");
- for(const row of vectors){const directory=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,`read-only-actors-${crypto.randomUUID()}`);mkdirSync(join(directory,"trusted-catalog"),{recursive:true});process.env.OS_HUB_DATA=directory;let checked=0;try{await expect(loadCurrentDevelopmentActorsV1(process.cwd(),()=>{checked++;if(row.cancelled)throw Error(row.expected);})).rejects.toThrow(row.expected);expect(checked).toBeGreaterThan(0);expect(readdirSync(directory)).toEqual(["trusted-catalog"]);expect(readdirSync(join(directory,"trusted-catalog"))).toEqual([]);}finally{if(previous===undefined)delete process.env.OS_HUB_DATA;else process.env.OS_HUB_DATA=previous;}}
+ for(const row of vectors){const directory=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,`read-only-actors-${crypto.randomUUID()}`);mkdirSync(join(directory,"trusted-catalog"),{recursive:true});process.env.OS_HUB_DATA=directory;let checked=0;const started=Date.now();try{await expect(loadCurrentDevelopmentActorsV1(process.cwd(),{maxBytes:128*1024*1024,maxWork:65536,chunkBytes:1024*1024,cancelled:()=>{checked++;if(row.cancelled)throw Error(row.expected);return false;},remainingMs:()=>60_000-(Date.now()-started),onProgress:()=>{}})).rejects.toThrow(row.expected);expect(checked).toBeGreaterThan(0);expect(readdirSync(directory)).toEqual(["trusted-catalog"]);expect(readdirSync(join(directory,"trusted-catalog"))).toEqual([]);}finally{if(previous===undefined)delete process.env.OS_HUB_DATA;else process.env.OS_HUB_DATA=previous;}}
  const source=ts.createSourceFile("verification.ts",readFileSync(join(owner,"🔎️verification/🟦️.ts"),"utf8"),ts.ScriptTarget.Latest,true),definition=source.statements.find(statement=>ts.isFunctionDeclaration(statement)&&statement.name?.text==="loadCurrentDevelopmentActorsV1")!;expect(!!definition).toBe(true);const calls:string[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node))calls.push(node.expression.getText(source));ts.forEachChild(node,visit)};visit(definition);expect(calls).toContain("runtimeSelectedActorsV1");expect(calls.some(call=>/runTool|spawn|prepareCargo/u.test(call))).toBe(false);
 });
```
## 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/🔌️providers/🧪️tests/🟦️.ts
```diff
--- 

+++ 

@@ -10,7 +10,6 @@

 export function runMutationInventoryProviderChecksV1(): number {
   const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
-  const schema = read("../🧬️schema/🔣️.json"), vectors = read("../🧫️fixtures/🔣️.json"), corpusSchema = read("../🧫️fixtures/🔣️schema.json");
-  const ajv = new Ajv({ strict: true }).addSchema(schema), oracle = ajv.compile(schema);
-  assert.equal(Boolean(ajv.compile(corpusSchema)(vectors)), true);
+  const schema = read("../🧬️schema/🔣️.json"), vectors = read("../🧫️fixtures/🔣️.json");
+  const oracle = new Ajv({ strict: true }).compile(schema);
   for (const vector of vectors.cases) {
     assert.equal(Boolean(oracle(vector.value)), vector.valid, vector.name);
```
## 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/🔌️providers/🧫️fixtures/🔣️schema.json
```diff
--- 

+++ 

@@ -1,121 +0,0 @@

-{
-  "$schema": "http://json-schema.org/draft-07/schema#",
-  "type": "object",
-  "additionalProperties": false,
-  "required": [
-    "schema",
-    "cases",
-    "selectionCases",
-    "discoveryCases"
-  ],
-  "properties": {
-    "schema": {
-      "const": "semio.repository.mutation-inventory-provider-vectors/v1"
-    },
-    "cases": {
-      "type": "array",
-      "minItems": 5,
-      "items": {
-        "type": "object",
-        "additionalProperties": false,
-        "required": [
-          "name",
-          "value",
-          "valid"
-        ],
-        "properties": {
-          "name": {
-            "type": "string",
-            "minLength": 1
-          },
-          "value": {},
-          "valid": {
-            "type": "boolean"
-          }
-        }
-      }
-    },
-    "selectionCases": {
-      "type": "array",
-      "minItems": 4,
-      "items": {
-        "type": "object",
-        "additionalProperties": false,
-        "required": [
-          "name",
-          "providers",
-          "owner",
-          "expected"
-        ],
-        "properties": {
-          "name": {
-            "type": "string",
-            "minLength": 1
-          },
-          "providers": {
-            "type": "array",
-            "items": {
-              "$ref": "urn:semio:repository:mutation-inventory-provider:v1"
-            }
-          },
-          "owner": {
-            "type": "string",
-            "minLength": 1
-          },
-          "expected": {
-            "anyOf": [
-              {
-                "type": "integer",
-                "minimum": 0
-              },
-              {
-                "type": "null"
-              },
-              {
-                "const": "ambiguous"
-              }
-            ]
-          }
-        }
-      }
-    },
-    "discoveryCases": {
-      "type": "array",
-      "minItems": 1,
-      "items": {
-        "type": "object",
-        "additionalProperties": false,
-        "required": [
-          "name",
-          "files",
-          "expected",
-          "refused"
-        ],
-        "properties": {
-          "name": {
-            "type": "string",
-            "minLength": 1
-          },
-          "files": {
-            "type": "object",
-            "propertyNames": {
-              "$ref": "urn:semio:repository:mutation-inventory-provider:v1#/definitions/path"
-            },
-            "additionalProperties": {
-              "type": "string"
-            }
-          },
-          "expected": {
-            "type": "array",
-            "items": {
-              "$ref": "urn:semio:repository:mutation-inventory-provider:v1"
-            }
-          },
-          "refused": {
-            "type": "boolean"
-          }
-        }
-      }
-    }
-  }
-}
```