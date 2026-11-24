import {readFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {spawnSync} from "node:child_process";
import {moduleRemapArgumentsV1,planOffloadV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts";
const W="/Users/ueli/Documents/semio";
for(const name of process.argv.slice(2)){
 const hit=spawnSync("git",["grep","-l",`name = "${name}"`,"--","*.toml"],{cwd:W,encoding:"utf8"}).stdout.split("\n")[0];
 if(!hit){console.log(name,"no manifest");continue;}
 const manifest=dirname(join(W,hit)),toml=readFileSync(join(W,hit),"utf8"),lib=/\[lib\][^\[]*?path\s*=\s*"([^"]+)"/su.exec(toml)?.[1]??"src/lib.rs";
 const env={CARGO_MANIFEST_DIR:manifest,CARGO_PKG_NAME:name,CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:name.replace(/-/g,"_"),SEMIO_UNIT_CACHE_WORKSPACE_ROOT:W};
 const args=["--crate-name",name.replace(/-/g,"_"),"--edition=2021",resolve(manifest,lib),"--crate-type","lib","--emit=metadata","--out-dir",join(W,".cache","o20out"),"--remap-path-prefix",`${manifest}=${name}-0.1.0`];
 const given=moduleRemapArgumentsV1(args,env,manifest,W)??args;
 const t=performance.now(),r=planOffloadV1({args:[...given],environment:env,cwd:manifest}),ms=(performance.now()-t).toFixed(1);
 console.log(name.padEnd(46),"remap+"+(given.length-args.length),("declined" in r)?`${r.declined}: ${r.detail.slice(0,110)}`:`PLANNED files=${r.files.size}`,ms+"ms");
}
