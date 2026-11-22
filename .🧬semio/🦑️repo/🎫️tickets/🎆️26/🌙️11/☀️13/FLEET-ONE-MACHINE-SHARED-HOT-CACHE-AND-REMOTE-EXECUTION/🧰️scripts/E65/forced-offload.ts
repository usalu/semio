import {mkdirSync,readFileSync,rmSync,existsSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {fleetRemoteEnvironmentV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {wrapRustcV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";

const workspace="/Users/ueli/Documents/semio",cwd=process.argv[2]!,root=process.argv[3]!,out=process.argv[4]!,mode=process.argv[5]??"always",lane=process.argv[6]??"e65";
rmSync(out,{recursive:true,force:true});mkdirSync(out,{recursive:true});mkdirSync(root,{recursive:true});
const inner=(JSON.parse(readFileSync("/Users/ueli/Documents/semio/.cache/b/r/m/u/offload-rustc.json","utf8")) as {inner:string}).inner;
const environment={...process.env,...fleetRemoteEnvironmentV1({...process.env,SEMIO_WORKSPACE_ROOT:workspace},workspace),SEMIO_OFFLOAD:mode,SEMIO_UNIT_CACHE_ROOT:root,SEMIO_UNIT_CACHE_LANE:lane,SEMIO_UNIT_CACHE_INNER:inner,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:workspace,SEMIO_WORKSPACE_ROOT:workspace,SEMIO_UNIT_CACHE_SLOTS:"0",CARGO_MANIFEST_DIR:cwd,CARGO_PKG_NAME:"e65probe",CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:"e65probe"};
const args=["--crate-name","e65probe","--edition=2021","src/lib.rs","--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata","-C","embed-bitcode=no","-C","debuginfo=1","--remap-path-prefix="+cwd+"=.","--check-cfg","cfg(docsrs,test)","-C","metadata=555e33cb5de15187","-C","extra-filename=-722ade2de31f5fa4","--out-dir",out,"-L","dependency="+out,"-Z","threads=0","--remap-path-scope=all"];
const started=Date.now(),status=await wrapRustcV1(args,environment as Record<string,string>,cwd,{});
console.error("status",status,"ms",Date.now()-started,"outputs",existsSync(out)?"present":"none");
