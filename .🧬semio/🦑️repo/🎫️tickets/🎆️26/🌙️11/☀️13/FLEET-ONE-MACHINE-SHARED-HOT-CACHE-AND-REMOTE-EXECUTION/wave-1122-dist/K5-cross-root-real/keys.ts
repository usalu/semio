import {mkdirSync,readFileSync,readdirSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir,homedir} from "node:os";
import {join} from "node:path";
import {wrapRustcV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
import {readLookupsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts";
const channel="nightly-2026-07-20",rustc=join(homedir(),".rustup","toolchains",readdirSync(join(homedir(),".rustup","toolchains")).find(n=>n.startsWith(channel+"-"))!,"bin","rustc");
const scratch=mkdtempSync(join(tmpdir(),"k5real-"));
for(const root of ["/Users/ueli/Documents/semio","/Users/ueli/.sinc/play"]){
 const cargoHome=join(root,".cache/b/r/m/cargo/home"),pkg=join(cargoHome,"registry/src/index.crates.io-1949cf8c6b5b557f/unicode-ident-1.0.27"),out=join(scratch,root.replace(/\//g,"_"),"deps"),cache=join(scratch,root.replace(/\//g,"_"),"u");
 mkdirSync(out,{recursive:true});mkdirSync(cache,{recursive:true});
 const argv=["--crate-name","unicode_ident","--edition=2021",join(pkg,"src/lib.rs"),"--error-format=json","--json=artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata,link","-C","embed-bitcode=no","-C","codegen-units=256","-C","debug-assertions=off","-C","strip=debuginfo","-C","metadata=abc123abc123abc1","-C","extra-filename=-abc123abc123abc1","--out-dir",out,"-L","dependency="+out,"--cap-lints=allow","--remap-path-prefix="+join(cargoHome,"registry/src")+"="];
 const env={...process.env,CARGO_HOME:cargoHome,RUSTUP_HOME:join(homedir(),".rustup"),SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:"k5",SEMIO_UNIT_CACHE_WORKSPACE_ROOT:root,SEMIO_UNIT_CACHE_INNER:JSON.stringify([rustc]),SEMIO_UNIT_CACHE_TOOLCHAIN:"k5",CARGO_PKG_NAME:"unicode-ident",CARGO_PKG_VERSION:"1.0.27",CARGO_CRATE_NAME:"unicode_ident",CARGO_MANIFEST_DIR:pkg,PWD:pkg,RUSTC_BOOTSTRAP:"1",SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:9",SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:"1",SEMIO_REMOTE_CACHE_RETRIES:"0"} as Record<string,string|undefined>;
 delete env.SEMIO_MACHINE_STATE_DIRECTORY;delete env.SEMIO_UNIT_CACHE_LANE_ROOT;
 const origin=process.cwd();process.chdir(pkg);
 const status=await wrapRustcV1(argv,env,pkg);process.chdir(origin);
 const row=readLookupsV1(cache).find(r=>r.crate==="unicode_ident");
 console.log(root,status,JSON.stringify({key:row?.key,remote:row?.remote,roots:row?.components.roots}));
}
rmSync(scratch,{recursive:true,force:true});
