/** 📥️ Brings the manifest row files `K/📥️manifest-rows/<id>.json` of people up to the evaluated asset (height, width, power by `totalPowerW`, named poses, clips, representation states), rewrites only a row whose value changed and validates every row against its asset with `validateManifest`.
 *  bun rows.ts <slug>… */
import { totalPowerW, validateManifest, type Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { catalogFile, K } from "./people.ts";

let failing = 0;
const changed: string[] = [];
for (const slug of process.argv.slice(2)) {
  const a: Asset = (await Bun.file(catalogFile(slug)).json()).asset;
  const file = `${K}/📥️manifest-rows/${slug}.json`;
  const row = await Bun.file(file).json();
  const next = { ...row, heightM: a.dimensions.heightM, widthM: a.dimensions.widthM, powerW: Math.round(totalPowerW(a) * 10) / 10, poses: a.poses.filter((p) => !(p as { key?: boolean }).key).map((p) => p.id), clips: a.clips.map((c) => c.id), representations: Object.fromEntries(["50", "100", "200", "500"].map((d) => [d, (a.representations as any)[d].status])) };
  const differs = Object.keys(next).filter((k) => JSON.stringify((next as any)[k]) !== JSON.stringify(row[k]));
  if (differs.length > 0) { await Bun.write(file, JSON.stringify(next, null, 2) + "\n"); changed.push(slug); }
  const issues = validateManifest({ categories: [], collections: [], entries: [next] } as never, [a]).filter((i) => !/category-missing|manifest-count/.test(i.code));
  if (issues.length > 0) failing++;
  console.log(`${slug}: ${differs.length > 0 ? `row changed (${differs.map((k) => `${k} ${JSON.stringify(row[k])} -> ${JSON.stringify((next as any)[k])}`).join(", ")})` : "row unchanged"}; ${issues.length === 0 ? "manifest ok" : issues.map((i) => `${i.code}: ${i.message.en}`.slice(0, 160)).join(" | ")}`);
}
console.log(`rows changed: ${changed.join(" ") || "none"}; failing ${failing}`);
