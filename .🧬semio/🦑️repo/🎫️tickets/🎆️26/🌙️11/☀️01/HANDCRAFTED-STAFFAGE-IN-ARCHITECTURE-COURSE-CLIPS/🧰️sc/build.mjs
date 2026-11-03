import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { solar } from "./solar.mjs";

const root = process.argv[2];
const dir = join(root, "🏗️machinery", "solar-thermal-collector");
mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, "🔣️.json"), `${JSON.stringify(solar(), null, 2)}\n`);
console.log("wrote solar-thermal-collector");
