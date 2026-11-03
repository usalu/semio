import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { hrvFan } from "./hrvfan.mjs";

const root = process.argv[2];
const dir = join(root, "🔌️appliances", "decentralised-hrv-fan");
mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, "🔣️.json"), `${JSON.stringify(hrvFan(), null, 2)}\n`);
console.log("wrote decentralised-hrv-fan");
