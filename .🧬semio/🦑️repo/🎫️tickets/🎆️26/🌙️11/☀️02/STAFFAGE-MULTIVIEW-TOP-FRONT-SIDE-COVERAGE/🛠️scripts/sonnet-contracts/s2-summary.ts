import { readFileSync } from "node:fs";
const f = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
for (const [k, v] of Object.entries<any>(f.assets)) {
  console.log(k, JSON.stringify(v.codes));
  if (v.codes["evaluation-failed"] || v.codes["unavailable-but-drawable"]) for (const i of v.issues.filter((i: any) => i.code !== "solid-sweep" && i.code !== "lateral-ambiguous")) console.log("   ", i.code, i.path, i.message.slice(0, 300));
}
