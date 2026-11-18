/** 🧾 Turns an in-memory patch module into an ops file for humansb-B-apply.py. bun humansb-B2-ops.ts <asset> <patch.mjs> <out.json> */
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const [name, patch, out] = process.argv.slice(2) as [string, string, string];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const load = async () => ((await readStreamedAsset(entry)) as { asset: any }).asset;
const before = await load(), after = await load();
(await import(patch)).default(after);
const ops: unknown[] = [];
for (const sc of Object.keys(after.representations)) for (const v of Object.keys(after.representations[sc].geometry)) {
  const old = before.representations[sc].geometry[v];
  after.representations[sc].geometry[v].forEach((s: any, i: number) => { if (JSON.stringify(s) !== JSON.stringify(old[i])) ops.push({ op: "shape", scale: sc, view: v, id: s.id, shape: s }); });
}
for (const sc of Object.keys(after.volumes)) {
  const old = before.volumes[sc].body;
  after.volumes[sc].body.forEach((s: any, i: number) => { if (JSON.stringify(s) !== JSON.stringify(old[i])) ops.push({ op: "solid-set", scale: sc, id: s.id, set: { sections: s.sections } }); });
}
await Bun.write(out, JSON.stringify(ops));
console.log(ops.length, "ops");
