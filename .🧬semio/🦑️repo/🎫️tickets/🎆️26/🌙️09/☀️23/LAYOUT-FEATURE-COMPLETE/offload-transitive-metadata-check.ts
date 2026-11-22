import { deepStrictEqual, ok } from "node:assert/strict";
import { readdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const sourceRoot=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload");
const owner=Object.fromEntries(["🌳️inputs","🗺️paths"].map(name=>[name,join(sourceRoot,name,"🟦️.ts")]));
const drafts=JSON.parse(await Bun.stdin.text()) as Record<string,string>;
const compiler=new Bun.Transpiler({loader:"ts"});

function moduleUrl(source:string,path:string,replacements:Record<string,string>={}):string{
 const absolute=source.replace(/from ["'](\.\.?\/[^"']+)["']/g,(_match:string, relative:string)=>{
  const target=resolve(dirname(path),relative);
  return `from "${replacements[target]??pathToFileURL(target).href}"`;
 });
 return "data:text/javascript;base64,"+Buffer.from(compiler.transformSync(absolute)).toString("base64");
}

const live=await import(pathToFileURL(owner["🗺️paths"]).href);
const revised=await import(moduleUrl(drafts["🗺️paths"],owner["🗺️paths"],{
 [owner["🌳️inputs"]]:moduleUrl(drafts["🌳️inputs"],owner["🌳️inputs"]),
}));
const store=resolve(".cache/b/r/m/u/l/pd14ffe/prototype/build");
const indexmap=join(store,"indexmap/b714a93ca0d43eaf/out");
const hashbrown=join(store,"hashbrown/1ddd3887212c7adf/out");
const decoy=join(store,"hashbrown/08f7708363a36fad/out");
const dependency=join(hashbrown,"libhashbrown-1ddd3887212c7adf.rmeta");
const cargoRegistry=resolve(".cache/b/r/m/cargo/home/registry/src");
const registryRoot=readdirSync(cargoRegistry).find(name=>name.startsWith("index.crates.io-"));
ok(registryRoot,"registry source root");
const manifest=join(cargoRegistry,registryRoot,"petgraph-0.6.5");
const cwd=process.cwd();
const args=[
 "--crate-name","petgraph","--edition=2021",join(manifest,"src/lib.rs"),"--crate-type","lib","--emit=metadata",
 "--out-dir",join(store,"petgraph/4197e8b58fe5a255/out"),
 "--remap-path-prefix",`${cwd}=.`, "-C","extra-filename=-4197e8b58fe5a255",
 ...[indexmap,hashbrown,decoy].flatMap(path=>["-L",`dependency=${path}`]),
 "--extern",`indexmap=${join(indexmap,"libindexmap-b714a93ca0d43eaf.rmeta")}`,
];
const input={args,environment:{CARGO_MANIFEST_DIR:manifest},cwd};
const baseline=live.planOffloadV1(input);
const repaired=revised.planOffloadV1(input);
ok(!("declined" in baseline),JSON.stringify(baseline));
ok(!("declined" in repaired),JSON.stringify(repaired));
ok(![...baseline.files.values()].includes(dependency),"baseline drops transitive metadata");
ok([...repaired.files.values()].includes(dependency),"patched planner ships transitive metadata");
ok(![...repaired.files.values()].some((path:string)=>dirname(path)===decoy),"decoy metadata stays out of the plan");
deepStrictEqual(repaired.arguments,baseline.arguments,"the virtual Rust search paths and order are preserved");
deepStrictEqual(repaired.mappings,baseline.mappings,"the actual dependency-root origins are preserved");
console.log("[DEBUG] The real indexmap/hashbrown metadata closure now crosses separate Cargo -L roots, and excludes the decoy.");
