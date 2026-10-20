// ✅ Validates a rules document with the staged engine: bun validate.ts <rules.json>
import {readInputPrecisionV1} from "/private/tmp/ip/run/p/🟦️.ts";
import {readFileSync} from "node:fs";
const doc=readInputPrecisionV1(JSON.parse(readFileSync(process.argv[2]!,"utf8")));
console.log("valid",doc.rules.length,"rules",doc.rules.filter(r=>r.evidence==="assumed").length,"assumed");
