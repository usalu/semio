import {readFileSync} from "node:fs";
const root=new URL("../../../../../../../",import.meta.url),schema=JSON.parse(readFileSync(new URL("linux-execution-schema.json",import.meta.url),"utf8")),{validateJsonSchemaSubset}=await import(new URL("🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts",root));
export function nextLinuxExecutionV1(facts){
 if(validateJsonSchemaSubset(schema,facts,schema).length)throw Error("Closed Linux execution facts required");
 if(facts.privilegedPending)return "wait-privileged";
 if(facts.reboot==="yes")return "reboot-required";
 if(facts.reboot==="unknown")return "inspect-reboot";
 if(facts.vmp==="unknown")return "inspect-vmp";
 if(facts.vmp==="disabled")return facts.elevation==="elevated"?"enable-vmp":"elevation-required";
 if(facts.wsl==="unknown")return "inspect-wsl";
 if(facts.wsl==="absent")return facts.elevation==="elevated"?"install-wsl":"elevation-required";
 if(facts.distribution==="collision")return "distribution-collision";
 if(facts.distribution==="unknown")return "inspect-distribution";
 if(facts.distribution==="absent")return "import-debian";
 if(facts.docker==="unknown")return "inspect-docker";
 return facts.docker==="absent"?"provision-docker":"ready";
}
