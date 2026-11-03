// 🔬 Read-only check of the 1:500 band of named assets: status, denominator, reason and the three view geometries.
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const [catalogDir, ...slugs] = process.argv.slice(2);
const categories = ["components", "furniture", "objects", "appliances", "machinery", "vehicles", "humans", "animals", "vegetation"];
for (const slug of slugs) {
  let found = false;
  for (const cat of readdirSync(catalogDir)) {
    const dir = join(catalogDir, cat, slug);
    if (!statSync(join(catalogDir, cat)).isDirectory()) continue;
    try {
      if (!statSync(dir).isDirectory()) continue;
    } catch {
      continue;
    }
    const file = readdirSync(dir).find((n) => n.endsWith(".json"))!;
    const band = (JSON.parse(readFileSync(join(dir, file), "utf8")) as any).asset.representations["500"];
    console.log(`${slug} (${cat}) status=${band?.status} denominator=${band?.denominator} reason=${band?.reason ? "yes" : "no"} geometry=${Object.keys(band?.geometry ?? {}).sort().join(",") || "none"}`);
    found = true;
    break;
  }
  if (!found) console.log(`${slug} not found`);
}
void categories;
