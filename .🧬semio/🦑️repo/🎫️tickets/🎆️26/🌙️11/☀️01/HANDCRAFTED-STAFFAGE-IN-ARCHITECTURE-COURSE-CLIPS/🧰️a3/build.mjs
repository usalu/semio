import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { laptop } from "./laptop.mjs";
import { monitor } from "./monitor.mjs";
import { lamp } from "./lamp.mjs";

const root = process.argv[2];
const dir = join(root, "🔌️appliances");
const out = [
  ["laptop", laptop()],
  ["desktop-monitor", monitor()],
  ["pendant-lamp", lamp()],
];
for (const [id, doc] of out) {
  mkdirSync(join(dir, id), { recursive: true });
  writeFileSync(join(dir, id, "🔣️.json"), `${JSON.stringify(doc, null, 2)}\n`);
  console.log("wrote", id);
}
