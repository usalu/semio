/** 🚩 Lists the shapes the validator flags as `lateral-ambiguous` per bone, scale and view, with their kind and extent. bun humansb-B-flags.ts <asset> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const entry = (await assetPathsFromFiles()).find((e) => e.folder === process.argv[2])!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const a: any = asset;
for (const i of validateAsset(asset).filter((x: any) => x.code === "lateral-ambiguous") as any[]) {
  console.log(i.data.bone, i.data.thetaMaxDeg, i.data.bands.join(" | "));
  for (const sc of ["500", "200", "100", "50"]) for (const v of ["front", "plan", "elevation"]) for (const s of a.representations[sc].geometry[v]) if (s.bone === i.data.bone && i.data.shapes.includes(s.id) && s.solid === undefined) console.log("  ", sc, v, s.id, s.kind);
}
