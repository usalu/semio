import {launchAgentPlistV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/🍎️launchd/🟨️.mjs";
import {writeFileSync,rmSync} from "node:fs";
import {spawnSync} from "node:child_process";
const label="tech.semio.repo.service.aaaaaaaaaaaa",path="/tmp/"+label+".plist",uid=process.getuid!();
writeFileSync(path,launchAgentPlistV1({label,program:"/bin/sleep",arguments:["20"],workingDirectory:process.cwd(),environment:process.env as never,log:"/tmp/launchd-repro.log"}));
const go=(args:string[])=>{const r=spawnSync("launchctl",args,{encoding:"utf8"});console.log(args.join(" "),"=>",r.status,JSON.stringify((r.stderr+r.stdout).trim()));return r.status;};
go(["bootout","gui/"+uid+"/"+label]);
go(["bootstrap","gui/"+uid,path]);go(["kickstart","gui/"+uid+"/"+label]);go(["bootstrap","gui/"+uid,path]);go(["bootout","gui/"+uid+"/"+label]);go(["bootstrap","gui/"+uid,path]);go(["kickstart","gui/"+uid+"/"+label]);go(["bootout","gui/"+uid+"/"+label]);go(["bootstrap","gui/"+uid,path]);go(["bootout","gui/"+uid+"/"+label]);
rmSync(path);
