/** 🔬 Reproduces a failed consumer against a real compiler dependency from an earlier source version. */
import assert from "node:assert/strict";
import {existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {pathToFileURL} from "node:url";
const stage=join(import.meta.dir,"🗑️generated","native-held-retry");
const rows=JSON.parse(readFileSync(join(stage,"overlay.json"),"utf8"));
const source=rows[0];
const work=join(stage,"w"),build=join(work,"b"),target=join(work,"t");
mkdirSync(build,{recursive:true});mkdirSync(target,{recursive:true});
const real=join(stage,"proof.ts");
let helper=source.after.replace(/from "(\.{1,2}\/[^"]+)"/gu,(_full:string,rel:string)=>'from "'+pathToFileURL(resolve(dirname(source.path),rel)).href+'"');
writeFileSync(real,helper);
const { cargoDependencyDiagnosticV1,failedCompilerArtifactDriftV1,runCargoPassAttemptsV1 } = await import(pathToFileURL(real).href);
const provider=join(work,"provider.rs"),consumer=join(work,"consumer.rs");
const library=join(build,"libprovider-123.rlib"), dep=join(build,"provider-123.d");
const old="pub trait Provider {}\n", next="pub trait Provider { fn close(&self); }\n";
const rust=(args:readonly string[])=>Bun.spawnSync(["repo","tools","exec","rustup","--","run","nightly-2026-07-20","rustc","--edition=2024",...args],{stdout:"pipe",stderr:"pipe",env:{...process.env,SEMIO_HOST_CAPACITY:"off"}});
const compileProvider=()=>rust(["--crate-name","provider","--crate-type=lib","-Zchecksum-hash-algorithm=blake3","--emit=link="+library+",dep-info="+dep,provider]);
const compileConsumer=()=>rust(["--crate-name","consumer","--crate-type=lib","--emit=metadata="+join(build,"consumer.rmeta"),"--extern","provider="+library,consumer]);
writeFileSync(provider,old);
writeFileSync(consumer,"pub fn finish(v: &impl provider::Provider) { v.close(); }\n");
const created=compileProvider();
assert.equal(created.exitCode,0,created.stderr.toString());
assert.match(readFileSync(dep,"utf8"),/^# checksum:blake3=/mu);
const lookup=async(files= [library])=>failedCompilerArtifactDriftV1({workspace:work,build,target,filenames:files,read:async(path:string)=>existsSync(path)?new Uint8Array(readFileSync(path)):undefined});
assert.equal(await lookup(),undefined);
writeFileSync(provider,next);
const outcomes:string[]=[];
const terminal=await runCargoPassAttemptsV1((suffix:string)=>{outcomes.push("compile"+suffix);return compileConsumer();},async(failed:ReturnType<typeof compileConsumer>)=>{
  if(failed.exitCode===0)return;
  assert.ok(failed.stderr.toString().split("\n").some(cargoDependencyDiagnosticV1),failed.stderr.toString());
  return lookup();
},async(drift:{path:string;expected:string;observed:string},suffix:string)=>{
  outcomes.push("admit"+suffix);
  assert.equal(drift.path,provider);
  assert.notEqual(drift.expected,drift.observed);
  const admitted=compileProvider();
  assert.equal(admitted.exitCode,0,admitted.stderr.toString());
});
assert.equal(terminal.exitCode,0,terminal.stderr.toString());
assert.deepEqual(outcomes,["compile","admit-retry","compile-retry"]);
assert.equal(await lookup(),undefined);
for(const line of ["error: a failed test","error[E0000]: nondependency","  note: error[E0599]: quoted","error[E0599]:second"]){
  assert.equal(cargoDependencyDiagnosticV1(line),line.startsWith("error[E0599]:"));
}
assert.equal(cargoDependencyDiagnosticV1("\u001b[1m\u001b[31merror[E0599]\u001b[0m: old dependency"),true);
console.log("[DEBUG] Real Rustc diagnostic and dependency checksum recovered once; current inputs do not retry");
