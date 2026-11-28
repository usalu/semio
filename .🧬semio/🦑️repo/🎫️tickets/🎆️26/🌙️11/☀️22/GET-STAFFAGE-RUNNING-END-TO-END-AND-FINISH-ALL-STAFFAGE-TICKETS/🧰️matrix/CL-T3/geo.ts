/** 🧿️ The geometry findings of `🧊️model-catalog` for assets without the file audit, and where the triangles of every scene sit: per asset the findings of `auditGeometry` (or the error it throws), then per scene the triangles and, with `--solids`, every solid with its kind, tags and triangles. geo.ts [--solids] [--scene <substring>] <asset id | file.json>… */
import { readFileSync } from "node:fs";
import { scenesOf, tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { modelOfAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import { qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { auditGeometry } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts";

const args = process.argv.slice(2);
const solids = args.includes("--solids");
const only = args.includes("--scene") ? args[args.indexOf("--scene") + 1] : undefined;
const targets = args.filter((a, i) => !a.startsWith("--") && args[i - 1] !== "--scene");
const entries = await assetPathsFromFiles();
for (const target of targets) {
  const asset = target.endsWith(".json") ? JSON.parse(readFileSync(target, "utf8")).asset : (await readStreamedAsset(entries.find((e) => e.folder === target)!)).asset;
  try {
    const audit = await auditGeometry(asset);
    console.log(`${asset.id} findings ${audit.findings.length}: ${audit.findings.map((f) => `${f.check} ${f.item}: ${f.message}`).join(" | ")}`);
    console.log(`${asset.id} scenes ${audit.facts.scenes.map((s) => `${s.scene}=${s.triangles}`).join(" ")}`);
    if (!solids) continue;
    const document = await modelOfAsset(asset, { poses: "all", clips: [] });
    for (const scene of scenesOf(document)) {
      if (only !== undefined && !scene.id.includes(only)) continue;
      const quality = qualityOf(asset, Number(/^s(\d+)-/u.exec(scene.id)![1]));
      const members = new Set(scene.nodes);
      for (const solid of document.solids.filter((s) => members.has(s.node))) console.log(`  ${scene.id} ${solid.id} ${solid.kind} [${(solid.tags ?? []).join(",")}] ${tessellateSolid(solid, quality).indices.length / 3}`);
    }
  } catch (error) {
    console.log(`${asset.id} ERROR ${String(error).slice(0, 600)}`);
  }
}
