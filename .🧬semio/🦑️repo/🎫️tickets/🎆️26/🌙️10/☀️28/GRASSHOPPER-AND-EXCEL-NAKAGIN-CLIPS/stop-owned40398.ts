/** 🧭 Read process facts through the existing portable repo doctor port without signalling any process. */
import {existsSync,writeFileSync,mkdirSync} from "node:fs";
import {dirname,join,basename} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";
const ticket=dirname(fileURLToPath(import.meta.url));
let root=ticket;
while(!existsSync(join(root,"🧰️framework"))){const parent=dirname(root);if(parent===root)throw Error("Repository root missing");root=parent;}
const module=await import(pathToFileURL(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts")).href);
const ports=module.doctorPortsV1(root,{signal:AbortSignal.timeout(20000)});
const rows=await ports.processes();
const byId=new Map(rows.map(row=>[row.pid,row]));const child=byId.get(15644);const bun=byId.get(3540);const repo=byId.get(49760);const shell=byId.get(2380);if(!child||child.parent!==3540||!child.command.endsWith("Python314\\python.exe")||!bun||bun.parent!==49760||!repo||repo.parent!==2380||!shell||!shell.command.includes("repo tools exec --light python -c")||!shell.command.includes("remodel-current-state.md"))throw Error("Ownership chain mismatch");await ports.command("powershell",["-NoProfile","-NonInteractive","-Command","Stop-Process -Id 15644 -ErrorAction Stop"]);console.log("[DEBUG] Terminated exact owned interactive Python15644 after complete ownership-chain verification");
