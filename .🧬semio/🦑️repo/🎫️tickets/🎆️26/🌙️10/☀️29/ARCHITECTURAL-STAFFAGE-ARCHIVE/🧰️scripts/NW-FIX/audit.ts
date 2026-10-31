/** [DEBUG] Audit validateAsset for the six staffage categories. */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const cats = ["vegetation", "furniture", "objects", "vehicles", "machinery", "components"];
const out: Record<string, string[]> = {};
for (const dir of await readdir(root, { withFileTypes: true })) {
  if (!dir.isDirectory()) continue;
  if (!cats.some((c) => dir.name.includes(c))) continue;
  const cat = resolve(root, dir.name);
  for (const folder of await readdir(cat, { withFileTypes: true })) {
    if (!folder.isDirectory()) continue;
    const path = resolve(cat, folder.name);
    const files = await readdir(path);
    const docName = files.find((f) => f.endsWith(".json"));
    if (!docName) continue;
    const doc = JSON.parse(await readFile(resolve(path, docName), "utf8"));
    const asset = doc.asset;
    const issues = validateAsset(asset);
    if (issues.length) out[asset.id] = issues.map((i) => `${i.code} ${i.path}: ${i.message.en}`);
  }
}
const dest = process.argv[2]!;
await import("node:fs/promises").then((fs) => fs.mkdir(dest.slice(0, dest.lastIndexOf("/")), { recursive: true }));
await writeFile(dest, JSON.stringify(out, null, 1));
console.log("[DEBUG] assets with issues", Object.keys(out).length);
for (const [id, issues] of Object.entries(out)) console.log(id, issues.length, issues[0]);
