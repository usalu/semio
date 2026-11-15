import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const [slug, scales, rule] = [process.argv[2]!, process.argv[3] ?? "50", process.argv[4] ?? "volumes-thermal-coverage"];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === slug)!;
const { asset } = await readStreamedAsset(entry);
for (const dir of (process.env.DIRS ?? "🧊️volumes,🧊️volumes-next-a").split(",")) {
  const { measureVolumes } = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/${dir}/🟦️.ts`);
  for (const scale of scales.split(",").map(Number)) {
    const c = process.cpuUsage();
    const out = measureVolumes(asset as never, { scale, rules: [rule] } as never) as { name: string; value: number }[];
    const u = process.cpuUsage(c);
    console.log(dir, slug, scale, `cpu ${((u.user + u.system) / 1000).toFixed(0)} ms`, out.filter((m) => m.name.startsWith("areaDeviation")).map((m) => m.name.replace("areaDeviation","dev") + "=" + m.value.toFixed(3)).join(" "));
  }
}
