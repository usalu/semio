import{registerBuildTestLawV1}from"@semio-tech/process/testing/invocation";
import{nativeCallerPhaseV1}from"../../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import{cargoPreparationControlV1}from"../../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import inputSchema from"../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📥️input/🧬️schema/🔣️.json";
import{readOwnedCommandInputV1}from"../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📥️input/🟦️.ts";
import{admitCargoPreparationObservationV1,assertCargoPreparationObservationCurrentV1,cargoPreparationProgramSourcesV1,cargoPreparationResolutionInputsV1,type CargoPreparationResolutionV1}from"../../🟦️.ts";

const policy=inputSchema.definitions.Policy.const,oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const edges=(count:number,width:number):CargoPreparationResolutionV1[]=>Array.from({length:count},(_,index)=>({source:`/repo/🌎️hub/🧩️compositions/🗄️stdio/${"🧪️tests/".repeat(width)}${index}/🟦️.ts`,specifier:`../../${"🔬️unit/".repeat(width)}${index}/🟦️.ts`,selected:`/repo/🌎️hub/🧩️compositions/🗄️stdio/${"🔬️unit/".repeat(width)}${index}/🟦️.ts`}));
const decode=(inputs:ReturnType<typeof cargoPreparationResolutionInputsV1>):CargoPreparationResolutionV1[][]=>inputs.map(input=>JSON.parse(Buffer.from(input.base64,"base64").toString("utf8")));

test("one hub-sized import closure is refused as one text input and travels as binary inputs that concatenate to the same edges",()=>{
 const list=edges(3250,3);
 expect(JSON.stringify(list).length).toBeGreaterThan(policy.maximumBytes);
 expect(()=>readOwnedCommandInputV1({version:1,text:JSON.stringify(list)})).toThrow();
 const inputs=cargoPreparationResolutionInputsV1(list);
 expect(inputs.length).toBeGreaterThanOrEqual(1);
 for(const input of inputs)expect(readOwnedCommandInputV1(input)).toEqual(input);
 expect(decode(inputs).flat()).toEqual(list);
});

test("an empty closure needs no child input and a closure beyond one input splits at the declared byte bound in order",()=>{
 expect(cargoPreparationResolutionInputsV1([])).toEqual([]);
 const list=edges(6000,40),inputs=cargoPreparationResolutionInputsV1(list);
 expect(inputs.length).toBeGreaterThan(1);
 for(const input of inputs){expect(readOwnedCommandInputV1(input)).toEqual(input);expect(Buffer.from(input.base64,"base64").length).toBeLessThanOrEqual(policy.maximumBinaryBytes);}
 expect(decode(inputs).flat()).toEqual(list);
});

test("the chunking agrees with an independent Python decoder on UTF-8, size bound, order and the concatenated JSON digest",()=>{
 const scratch=mkdtempSync(join(tmpdir(),"hubreds-")),list=edges(6000,40),inputs=cargoPreparationResolutionInputsV1(list);
 try{
  const file=join(scratch,"inputs.json");mkdirSync(scratch,{recursive:true});writeFileSync(file,JSON.stringify({limit:policy.maximumBinaryBytes,inputs:inputs.map(input=>input.base64),digest:new Bun.CryptoHasher("sha256").update(JSON.stringify(list)).digest("hex")}));
  const run=spawnSync("python3",["-I",oracle,file],{encoding:"utf8"});
  expect(run.stderr).toBe("");expect(run.status).toBe(0);expect(run.stdout.trim()).toBe(`ok ${inputs.length} ${list.length}`);
 }finally{rmSync(scratch,{recursive:true,force:true});}
});

registerBuildTestLawV1("a real import closure beyond the text input limit is verified current through the actual resolver child","caller-birth",async original=>{
 const control=cargoPreparationControlV1(nativeCallerPhaseV1(original,process.cwd(),createIssuedResourceControlV1(original,"custody:resolution",0)));
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Explicit custody artifact root required");
 const root=join(artifacts,"resolution"),sourceRoot=process.cwd(),door=join(sourceRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts"),entry=join(root,"📜️script.ts");
 mkdirSync(root,{recursive:true});writeFileSync(entry,`import ${JSON.stringify(door)};\n`);
 const program=await cargoPreparationProgramSourcesV1(root,entry,control);
 expect(JSON.stringify(program.resolutions).length).toBeGreaterThan(policy.maximumBytes);
 const observed=admitCargoPreparationObservationV1({version:1,root,script:entry,command:["composition","prepare"],outputs:[],...program});
 await assertCargoPreparationObservationCurrentV1(observed,control);
});
