import {renderCargoProfileConfigurationV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts";
import {spawnSync} from "node:child_process";
const r=renderCargoProfileConfigurationV1({version:1,mode:"prototype",target:"aarch64-apple-darwin",producer:"artifact",operation:"check"},"Cargo.toml","repository");
console.log(r.plan.profile,r.plan.role,JSON.stringify(r.plan.target),JSON.stringify(r.plan.host),r.selectors.length);
const args=["check","--unit-graph","-Z","unstable-options","--offline","-p","semio-framework-hash","--lib","--profile",r.plan.profile,...r.selectors.flatMap(s=>["--config",s])];
const g=spawnSync("cargo",args,{cwd:"/Users/ueli/Documents/semio",encoding:"utf8",maxBuffer:256*1024*1024,env:{...process.env,RUSTUP_TOOLCHAIN:"nightly-2026-07-20"}});
console.log("status",g.status,String(g.stderr).slice(0,500));
if(g.status===0){const units=JSON.parse(g.stdout).units as any[];const by:Record<string,number>={};for(const u of units){const k=u.profile.name+":"+u.profile.incremental+":"+u.target.kind.join("+")+":"+u.platform;by[k]=(by[k]??0)+1;}console.log(by);}
