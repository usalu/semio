import {doctorPortsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts";
import {writeFileSync} from "node:fs";
const ports=doctorPortsV1(process.cwd(),{signal:new AbortController().signal});
const all=await ports.processes();
const targets=["@semio-tech/s-dev-composition:generate","guest-host-test.ts","guest-native-parent-test.ts","guest-source-encoding-test.ts","original_host_factory","original-native-parent-retirement","original-source-native-encoding","framework-os-dev:plugin"];
const selected=all.filter(row=>targets.some(target=>JSON.stringify(row).includes(target))||row.pid===28012);
const report={at:Date.now(),totalProcesses:all.length,selected};
writeFileSync(import.meta.dirname+"/🗑️generated/guest-catalog-full-census.json",JSON.stringify({report,all}));
console.log(JSON.stringify({...report,selected:selected.map(row=>Object.fromEntries(Object.entries(row).map(([key,value])=>[key,typeof value==="string"&&value.length>420?value.slice(0,420):value])))}));
