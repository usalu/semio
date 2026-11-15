import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { checkVolumes, VOLUME_RULES } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const [slug, scales] = [process.argv[2]!, process.argv[3] ?? "50,100,200,500"];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === slug)!;
const { asset } = await readStreamedAsset(entry);
for (const scale of scales.split(",").map(Number)) for (const rule of VOLUME_RULES) {
  const t = Date.now();
  checkVolumes(asset as never, { scale, rules: [rule] } as never);
  const ms = Date.now() - t;
  if (ms > 500) console.log(slug, scale, rule, `${ms} ms`);
}
