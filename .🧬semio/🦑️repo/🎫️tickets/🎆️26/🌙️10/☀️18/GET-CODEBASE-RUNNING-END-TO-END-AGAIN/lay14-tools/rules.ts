import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
for (const x of r.summary.ruleSetUsed.forbidden) if (x.name.includes(process.argv[3] ?? "")) console.log(JSON.stringify(x, null, 1));
