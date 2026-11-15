import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { checkVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const slug = process.argv[2] ?? "agave";
const entry = (await assetPathsFromFiles()).find((e) => e.folder === slug)!;
const { asset } = await readStreamedAsset(entry);
const t = Date.now();
const findings = checkVolumes(asset as never, process.argv[3] ? { scale: Number(process.argv[3]) as never } : {});
console.log(slug, findings.length, "findings", Date.now() - t, "ms", [...new Set(findings.map((f) => f.rule))].join(","));
