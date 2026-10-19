import { readFileSync } from "node:fs";
const rows = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as any[];
const OS = "🧰️framework/🛍️products/💻️os/";
const skip = process.argv[3];
for (const r of rows.filter(r => r.to?.startsWith(OS) && !r.sdk && !(skip && r.spec.includes(skip)))) console.log(r.from.replace(/✏️s\/🔌️plugins\//u, "").slice(-90).padEnd(92), "->", r.to.slice(OS.length).replace("🔨️modules/","M/").replace("📺️renderer/🧑‍🎨engine/","R/"), "|", r.spec.startsWith(".")?"":r.spec);
