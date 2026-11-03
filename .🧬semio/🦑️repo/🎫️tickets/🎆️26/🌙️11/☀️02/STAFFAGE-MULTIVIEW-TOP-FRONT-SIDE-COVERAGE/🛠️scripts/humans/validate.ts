import { readdirSync, readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { validateAsset } = await import(`${L}/🔨️modules/🗂️catalog/🟦️.ts`);
const only = process.argv[2];
const dir = `${L}/🗂️catalog/👤️humans`;
let total = 0;
for (const slug of readdirSync(dir).sort()) {
  if (only && only !== "all" && slug !== only) continue;
  const doc = JSON.parse(readFileSync(`${dir}/${slug}/🔣️.json`, "utf8"));
  const issues = validateAsset(doc.asset).filter((i: any) => !["view-twin-missing", "solid-on-elevation"].includes(i.code));
  total += issues.length;
  const by: Record<string, string[]> = {};
  for (const i of issues) (by[i.code] ??= []).push(`${i.path}: ${i.message?.en ?? i.messageEn ?? i.message ?? ""}`.slice(0, 220));
  console.log(slug, issues.length, Object.entries(by).map(([c, v]) => `${c}x${v.length}`).join(" "));
  if (only && only !== "all") for (const [c, v] of Object.entries(by)) for (const m of v) console.log("   ", c, m);
}
console.log("total", total);
