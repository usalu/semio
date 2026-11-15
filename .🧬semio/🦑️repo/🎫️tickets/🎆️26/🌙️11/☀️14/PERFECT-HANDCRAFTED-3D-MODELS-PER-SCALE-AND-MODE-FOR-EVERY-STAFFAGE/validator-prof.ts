import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const dir = process.env.VALIDATOR_MODULE ?? "🧊️volumes";
const { measureVolumes } = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/${dir}/🟦️.ts`);
const slug = process.argv[2]!;
const entry = (await assetPathsFromFiles()).find((e) => e.folder === slug)!;
const { asset } = await readStreamedAsset(entry);
const rules = (process.argv[4] ?? "volumes-silhouette,volumes-thermal-coverage,volumes-structure-coverage,volumes-cross-scale,volumes-monotonic,volumes-views-disagree").split(",");
const scales = (process.argv[3] ?? "50,100,200,500").split(",").map(Number);
for (const scale of scales) for (const rule of rules) {
  const c = process.cpuUsage();
  const t = Date.now();
  measureVolumes(asset as never, { scale, rules: [rule] } as never);
  const u = process.cpuUsage(c);
  console.log(slug, scale, rule, `wall ${Date.now() - t} ms cpu ${((u.user + u.system) / 1000).toFixed(0)} ms`);
}
