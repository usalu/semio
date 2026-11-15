import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const [slug, scales, rule, filter] = [process.argv[2]!, process.argv[3] ?? "100", process.argv[4] ?? "volumes-cross-scale", process.argv[5] ?? ""];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === slug)!;
const { asset } = await readStreamedAsset(entry);
for (const dir of ["🧊️volumes", "🧊️volumes-next"]) {
  const { measureVolumes } = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/${dir}/🟦️.ts`);
  for (const scale of scales.split(",").map(Number)) {
    const c = process.cpuUsage();
    const out = measureVolumes(asset as never, { scale, rules: [rule] } as never) as { name: string; value: number; limit: number; ok: boolean }[];
    const u = process.cpuUsage(c);
    console.log(dir, slug, scale, `cpu ${((u.user + u.system) / 1000).toFixed(0)} ms`, out.filter((m) => m.name.includes(filter)).map((m) => `${m.name}=${+m.value.toFixed(4)}${m.ok ? "" : "!"}`).join(" "));
  }
}
