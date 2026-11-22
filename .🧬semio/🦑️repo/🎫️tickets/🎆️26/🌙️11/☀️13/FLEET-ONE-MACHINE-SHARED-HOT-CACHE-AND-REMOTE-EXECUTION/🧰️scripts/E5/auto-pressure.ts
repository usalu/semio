import {mkdirSync,readFileSync,rmSync} from "node:fs";
import {join} from "node:path";
import {wrapRustcV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
import {testHostProbeV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🧪️tests/🧰️workspace/🟦️.ts";

const workspace="/Users/ueli/Documents/semio",cwd=process.argv[2]!,root=join(workspace,".cache/b/r/m/u"),out=process.argv[3]!;
rmSync(out,{recursive:true,force:true});mkdirSync(out,{recursive:true});
const inner=(JSON.parse(readFileSync(join(root,"offload-rustc.json"),"utf8")) as {inner:string}).inner;
const environment={...process.env,SEMIO_OFFLOAD:"auto",SEMIO_UNIT_CACHE_ROOT:root,SEMIO_UNIT_CACHE_LANE:"e5auto",SEMIO_UNIT_CACHE_INNER:inner,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:workspace,SEMIO_WORKSPACE_ROOT:workspace,SEMIO_UNIT_CACHE_SLOTS:"0",CARGO_MANIFEST_DIR:cwd,CARGO_PKG_NAME:"e5probe",CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:"e5probe"};
const args=["--crate-name","e5probe","--edition=2021","src/lib.rs","--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata","-C","embed-bitcode=no","-C","debuginfo=1","--remap-path-prefix="+cwd+"=.","--check-cfg","cfg(docsrs,test)","-C","metadata=555e33cb5de15187","-C","extra-filename=-722ade2de31f5fa4","--out-dir",out,"-L","dependency="+out,"-Z","threads=0","--remap-path-scope=all"];
const status=await wrapRustcV1(args,environment as Record<string,string>,cwd,{admission:{probe:testHostProbeV1("critical-pressure-gigabyte-swap")}});
console.error("status",status);
