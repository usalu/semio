import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const [slug, scale, rule] = [process.argv[2]!, Number(process.argv[3]), process.argv[4]!];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === slug)!;
const { asset } = await readStreamedAsset(entry);
const { checkVolumes } = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/${process.env.VALIDATOR_MODULE ?? "🧊️volumes"}/🟦️.ts`);
for (const f of checkVolumes(asset as never, { scale, rules: [rule] } as never) as { message: { en: string } }[]) console.log(f.message.en);
