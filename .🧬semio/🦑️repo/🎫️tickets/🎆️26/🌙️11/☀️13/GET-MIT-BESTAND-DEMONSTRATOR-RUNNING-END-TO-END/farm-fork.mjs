import {fork} from "node:child_process";
const c=fork(import.meta.dir+"/farm-check.mjs",[],{stdio:["ignore","inherit","inherit","ipc"],execPath:process.execPath});
c.on("exit",code=>console.log("[DEBUG] child exit",code));
