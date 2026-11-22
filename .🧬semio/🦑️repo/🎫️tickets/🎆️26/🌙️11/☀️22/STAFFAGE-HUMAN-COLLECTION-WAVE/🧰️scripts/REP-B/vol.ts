/** 🧊️ `checkVolumes` of people (every rule, or `--rules a,b`; `--no-silhouette` leaves `volumes-silhouette` out), the measured outer surface of every thermal region per scale with `--areas`, and the bounding box of the body against the declared dimensions.
 *  bun vol.ts <slug|file.json>… [--scales 50,100] [--rules …] [--no-silhouette] [--areas] */
import { parseArgs } from "node:util";
import { checkVolumes, measureVolumes, VOLUME_RULES } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { fileOf } from "./people.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string" }, rules: { type: "string" }, "no-silhouette": { type: "boolean" }, areas: { type: "boolean" } } });
for (const arg of positionals) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const scales = (values.scales ?? "500,200,100,50").split(",").map(Number).filter((d) => asset.volumes[String(d)]?.status === "authored");
  const rules = (values.rules?.split(",") ?? VOLUME_RULES.filter((r: string) => !(values["no-silhouette"] && r === "volumes-silhouette"))) as never;
  let bad = 0;
  for (const scale of scales) {
    if (values.areas) {
      const ms = measureVolumes(asset, { scale: scale as never, rules: ["volumes-thermal-coverage"] as never });
      const out: string[] = [];
      for (const region of asset.thermal.regions) {
        const m = ms.find((x: any) => x.name === `info:${region.areaBasis === "total" ? "totalAreaM2" : "outerAreaM2"}:${region.id}`);
        const dev = ms.find((x: any) => x.name === `areaDeviation:${region.id}`) as any;
        if (m !== undefined) out.push(`${region.id}=${(m as any).value.toFixed(4)}/${region.areaM2}${dev && !dev.ok ? "!" : ""}`);
      }
      console.log(`AREAS ${asset.id} 1:${scale} ${out.join(" ")}`);
      continue;
    }
    const findings = checkVolumes(asset, { scale: scale as never, rules });
    bad += findings.length;
    for (const f of findings) console.log(`   ${f.rule} ${f.path}: ${f.message.en}`.slice(0, Number(process.env.WIDTH ?? 700)));
  }
  if (!values.areas) console.log(`${asset.id}: ${bad === 0 ? "OK" : `${bad} findings`}`);
}
