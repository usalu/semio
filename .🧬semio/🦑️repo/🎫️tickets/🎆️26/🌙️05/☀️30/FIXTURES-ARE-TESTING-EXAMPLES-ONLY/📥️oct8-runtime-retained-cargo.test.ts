import{test,expect}from"bun:test";
import{readFileSync,existsSync,readdirSync}from"node:fs";
import{createHash}from"node:crypto";
import{resolve,join,dirname}from"node:path";
import{homedir}from"node:os";
import{runtimeCargoProvenanceV1,runtimeCargoUnitInputsV1,runtimeCargoInvocationV1,runtimeRetainedBuildResourceInputsV1,runtimeCargoObservationCurrentV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts";
const root=process.cwd();
for(const name of ["real-cargo-capture-jm1OzI/compiler/semio-cargo-provenance/cargo-unit-provenance-cargo-artifacts-8EUO60.json","real-cargo-capture-jm1OzI/compiler/semio-cargo-provenance/cargo-unit-provenance-cargo-relay-5c168a09-3866-483a-970a-7c0c6772e0d0.json"])test(`actual retained three Cargo units validate ${name}`,()=>{
const path=resolve(import.meta.dir,"🗑️generated/oct8-plugin-artifacts/"+name);
const [receipt]=runtimeCargoProvenanceV1(readFileSync(path,"utf8"));
expect(receipt!.units).toHaveLength(3);
expect(receipt!.buildScripts).toHaveLength(1);
expect(receipt!.buildResources).toHaveLength(1);
const resource=receipt!.buildResources![0]!;
const context={root,cwd:receipt!.cwd,outDirectory:resource.out_dir,read:(path:string)=>{try{return readFileSync(resolve(root,path))}catch{return undefined}},directory:(path:string)=>{try{return readdirSync(resolve(root,path)).map(name=>path+"/"+name)}catch{return undefined}}};
const observedResources=runtimeRetainedBuildResourceInputsV1(receipt!,resource,context);
expect(observedResources.findings).toEqual([]);
expect(observedResources.verified).toBe(true);
expect(observedResources.observations.map(row=>row.kind).sort()).toEqual(["copy","directory","read"]);
expect(runtimeRetainedBuildResourceInputsV1(receipt!,resource,{...context,read:()=>Buffer.from("both changed")}).verified).toBe(false);
expect(runtimeCargoInvocationV1(receipt!,root).target).toBeNull();
const digest=(path:string)=>{try{return createHash("sha256").update(readFileSync(resolve(root,path))).digest("hex")}catch{return undefined}};
for(const unit of receipt!.units){const bound=runtimeCargoUnitInputsV1(receipt!,unit,root,digest);expect(bound.findings).toEqual([]);expect(bound.verified).toBe(true);expect(unit.message.profile.test).toBe(false);}
const bin=receipt!.units.find(unit=>unit.message.target.kind.includes("bin"))!;
if(bin.artifacts[0]!.stagedPath){expect(existsSync(bin.artifacts[0]!.path)).toBe(false);expect(existsSync(bin.artifacts[0]!.stagedPath!)).toBe(true);}else expect(existsSync(bin.artifacts[0]!.path)).toBe(true);
expect(bin.inputs.some(input=>input.path.startsWith(receipt!.buildScripts[0]!.out_dir+"/"))).toBe(true);
console.log("[DEBUG] actual3 Cargo units retained complete .d/currentSHA/original+stagedSHA after capture deletion");
});

test("actual durable no-optin compiler ledger refuses absent artifacts and selected package units",async()=>{
const path=resolve(import.meta.dir,"🗑️generated/oct8-plugin-artifacts/real-cargo-capture-jm1OzI/compiler/semio-cargo-provenance/cargo-unit-provenance-cargo-relay-f5fb9803-b72f-408a-b92e-d7349508d6a8.json");let [receipt]=runtimeCargoProvenanceV1(readFileSync(path,"utf8"));
const digest=(path:string)=>{try{return createHash("sha256").update(readFileSync(resolve(root,path))).digest("hex")}catch{return undefined}};
const context={root,cwd:receipt!.cwd,outDirectory:"",buildDirectory:receipt!.buildDirectory!,cargoHome:resolve(process.env.CARGO_HOME??join(homedir(),".cargo")),digest,read:(path:string)=>{try{return readFileSync(resolve(root,path))}catch{return undefined}},directory:(path:string)=>{try{return readdirSync(resolve(root,path)).map(name=>path+"/"+name)}catch{return undefined}}};
if(!runtimeCargoObservationCurrentV1(receipt!,context)){
const{cargoStreamingStatus}=await import(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts")),environment={...process.env,CARGO_TARGET_DIR:join(dirname(receipt!.buildDirectory!),"relay-target"),CARGO_BUILD_BUILD_DIR:receipt!.buildDirectory!};delete environment.SEMIO_TEST_ARTIFACT_DIR;
const ledger=dirname(path),before=new Set(readdirSync(ledger));expect(await cargoStreamingStatus(receipt!.args,receipt!.cwd,environment,10000)).toBe(0);
const fresh=readdirSync(ledger).find(name=>!before.has(name)&&name.startsWith("cargo-unit-provenance-cargo-relay-"));expect(fresh).toBeDefined();[receipt]=runtimeCargoProvenanceV1(readFileSync(join(ledger,fresh!),"utf8"));
}
expect(runtimeCargoObservationCurrentV1(receipt!,context)).toBe(true);
const artifact=receipt!.units[0]!.artifacts[0]!.path;
expect(runtimeCargoObservationCurrentV1(receipt!,{...context,digest:(path:string)=>path===artifact?undefined:digest(path)})).toBe(false);
expect(runtimeCargoObservationCurrentV1({...receipt!,invocationInputs:[]},context)).toBe(false);
const primary=receipt!.units.find(unit=>unit.message.target.kind.includes("bin"))!;
const name=(Bun.TOML.parse(readFileSync(primary.message.manifest_path!,"utf8")) as any).package.name,selected={...receipt!,args:[...receipt!.args,"-p",name]};
expect(runtimeCargoObservationCurrentV1(selected,context)).toBe(true);
expect(runtimeCargoObservationCurrentV1({...selected,units:selected.units.filter(unit=>unit!==primary)},context)).toBe(false);
console.log("[DEBUG] actual no-optin durable Cargo receipt binds current configuration/source/artifacts; missing artifact/package/input evidence refuses automatic reuse");
});
