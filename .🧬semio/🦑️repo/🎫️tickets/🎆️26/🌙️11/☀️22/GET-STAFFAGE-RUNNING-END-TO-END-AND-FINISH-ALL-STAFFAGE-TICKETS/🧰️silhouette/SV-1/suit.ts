/** 🖨️ The print suitability of a plant or an asset file at every authored scale under the default print profile, as the product resolves it before it draws: the kind of the resolution and every violation.
 *  bun suit.ts <id|file.json>… */
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { fileOf } from "./plants.ts";

for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  let bad = 0;
  for (const d of [50, 100, 200, 500]) {
    if (asset.representations[String(d)]?.status !== "authored") continue;
    const r: any = resolveRepresentation(asset, { denominator: d } as never, DEFAULT_PRINT_PROFILE);
    const violations = r.suitability?.violations ?? r.violations ?? [];
    if (r.kind !== "authored") bad++;
    console.log(`${asset.id} 1:${d}: ${r.kind}${violations.length > 0 ? `, ${violations.length} violations` : ""}`);
    for (const v of violations.slice(0, Number(process.env.N ?? 8))) console.log(`   ${JSON.stringify(v).slice(0, 330)}`);
  }
  console.log(`${asset.id}: ${bad === 0 ? "prints" : `${bad} scales unsuitable`}`);
}
