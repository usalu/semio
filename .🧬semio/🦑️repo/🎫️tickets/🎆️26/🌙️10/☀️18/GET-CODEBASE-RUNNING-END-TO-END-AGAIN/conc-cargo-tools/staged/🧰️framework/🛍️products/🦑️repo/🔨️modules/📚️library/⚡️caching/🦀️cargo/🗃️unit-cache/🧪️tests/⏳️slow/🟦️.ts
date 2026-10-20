import {spawnSync} from "node:child_process";
/** 🐌️ A compiler shim that delays one crate: `bun slow.ts <crate> <milliseconds> -- <compiler…> <rustc arguments…>`. */
const argv=process.argv.slice(2),separator=argv.indexOf("--"),[crate,milliseconds]=argv.slice(0,separator),rest=argv.slice(separator+1);
const named=rest.indexOf("--crate-name");
if(named>=0&&rest[named+1]===crate)await new Promise(done=>setTimeout(done,Number(milliseconds)));
const environment={...process.env};for(const name of["CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS"])delete environment[name];
const result=spawnSync(rest[0]!,rest.slice(1),{stdio:"inherit",env:environment});
process.exitCode=result.status??1;
