import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const src = loadSource(entry)!;
const big = merged(asset, src);
big.thermal = { ...big.thermal, regions: big.thermal.regions.map((r: any) => ({ ...r, areaM2: 1e6 })) };
const want = process.argv.slice(3);
for (const scale of [50, 100, 200, 500]) {
  const ms = measureVolumes(big as never, { scale: scale as never, rules: ["volumes-thermal-coverage"] as never });
  const line: string[] = [];
  for (const x of ms) if (x.name.startsWith("areaDeviation")) {
    const r = asset.thermal.regions.find((q: any) => "areaDeviation:" + q.id === x.name); if (!r) continue;
    const a = (1 - x.value) * 1e6;
    const lim = scale === 50 ? 0.15 : 0.3;
    line.push(`${r.id} ${a.toFixed(4)}/${r.areaM2} (${(((a / r.areaM2) - 1) * 100).toFixed(0)}%${Math.abs(a / r.areaM2 - 1) <= lim ? "" : " !"})`);
  }
  console.log("1:" + scale, line.join("  "));
}
