/** 🔬️ The node ids over 64 characters of the multi-scene model of an asset (the JSON schema limit), and the longest of every scale: ids.ts <asset id>… */
import { readFileSync } from "node:fs";
import { assetFile, mod } from "../../🧰️linux/s4/lib.ts";
const { modelOfAsset } = await mod("🔨️modules/🧊️volume/🟦️.ts");
const { resolveScenes } = await mod("🔨️modules/📤️export/🗿️model/🧩️scenes/🟦️.ts");
const { qualityOf } = await mod("🔨️modules/🧊️volume/⏱️job/🟦️.ts");
for (const id of process.argv.slice(2)) {
  const asset = JSON.parse(readFileSync(assetFile(id), "utf8")).asset;
  const document = await modelOfAsset(asset, { poses: "all", clips: [] });
  const resolved = await resolveScenes(document, (scale: number) => qualityOf(asset, scale), { report: () => {}, cancelled: () => false });
  const ids: string[] = resolved.nodes.map((n: any) => n.id);
  const long = ids.filter((i) => i.length > 64);
  console.log(id, "nodes", ids.length, "over64", long.length, "longest", Math.max(...ids.map((i) => i.length)));
  for (const i of long) console.log("  ", i.length, i);
}
