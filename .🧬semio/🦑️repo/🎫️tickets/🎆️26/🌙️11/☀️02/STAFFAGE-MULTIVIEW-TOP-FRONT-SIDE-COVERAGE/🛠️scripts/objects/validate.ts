const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { validateAsset } = await import(`${L}/🔨️modules/🗂️catalog/🟦️.ts`);
const { readdirSync, readFileSync } = await import("node:fs");
const only = process.argv.slice(2);
let total = 0;
for (const cat of ["🪑️objects", "🛋️furniture"]) for (const slug of readdirSync(`${L}/🗂️catalog/${cat}`)) {
  if (only.length && !only.includes(slug)) continue;
  const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
  const issues = validateAsset(a, {});
  const by: Record<string, number> = {};
  for (const i of issues) by[i.code] = (by[i.code] ?? 0) + 1;
  total += issues.length;
  console.log(slug.padEnd(24), issues.length, JSON.stringify(by));
  if (only.length) for (const i of issues.slice(0, 40)) console.log("   ", i.code, i.path, String(i.message?.en ?? i.message).slice(0, 200));
}
console.log("TOTAL", total);
