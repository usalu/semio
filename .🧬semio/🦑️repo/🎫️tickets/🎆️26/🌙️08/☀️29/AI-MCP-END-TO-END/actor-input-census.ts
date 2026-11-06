import {doctorPortsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts";
const ports=doctorPortsV1(process.cwd(),{signal:new AbortController().signal});
const all=await ports.processes();
const selected=all.filter(row=>JSON.stringify(row).includes("borrowed_actor_input_keeps_identity_and_returns_wire_only_after_source_close"));
console.log(JSON.stringify({at:Date.now(),total:all.length,selected}));
