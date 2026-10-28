import {join} from "node:path";
const R="/Users/ueli/Documents/semio";
const {readLeaseReportV1}=await import(R+"/🧰️framework/🔨️modules/🏃️process/🔒️leases/📋️status/🟦️.ts");
const {readLeaseAdmissionV1}=await import(R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔒️leases/⌨️command/🟦️.ts");
const rep=readLeaseReportV1(join(R,".cache/b/l/m/data"),process.env,Date.now(),10);
const adm=await readLeaseAdmissionV1(process.env,"en");
await Bun.write(process.argv[2],JSON.stringify({...rep,admission:adm},null,1));
