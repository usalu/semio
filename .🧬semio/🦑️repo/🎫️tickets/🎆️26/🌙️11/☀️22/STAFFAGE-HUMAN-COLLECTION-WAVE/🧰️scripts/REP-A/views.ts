/** 🧊️ The findings of the three-views guard (🧪️tests/🧊️three-views: the sweep over every pose and clip instant and the default-pose consistency) of asset files, per authored scale.  bun views.ts <asset.json>… */
import { authoredScales, consistency, sweep } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
for (const path of process.argv.slice(2)) {
  const asset = (await Bun.file(path).json()).asset;
  let total = 0;
  for (const scale of authoredScales(asset)) {
    const found = [...sweep(asset, scale), ...consistency(asset, scale)];
    total += found.length;
    const kinds = new Map<string, number>();
    for (const f of found) kinds.set(f.check, (kinds.get(f.check) ?? 0) + 1);
    if (found.length > 0) console.log(`  1:${scale}: ${[...kinds].map(([k, n]) => `${n} ${k}`).join(", ")} | ${found[0]!.message.slice(0, 230)}`);
  }
  console.log(`${asset.id} (${path.split("/").slice(-2).join("/")}): ${total} three-views findings`);
}
