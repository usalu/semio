import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { rack } from "./rack.mjs";

const dir = join(process.argv[2], "🔌️appliances", "server-rack");
mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, "🔣️.json"), `${JSON.stringify(rack(), null, 2)}\n`);
console.log("wrote server-rack");
