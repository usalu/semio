/** 🩻️ The kernel validation errors of the multi-scene model of an asset with the solid each one points at: per error the code, the path, the solid id, its kind and its profile. invalid.ts <asset id | file.json>… */
import { readFileSync } from "node:fs";
import { validateModel } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { modelOfAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";

const entries = await assetPathsFromFiles();
for (const target of process.argv.slice(2)) {
  const asset = target.endsWith(".json") ? JSON.parse(readFileSync(target, "utf8")).asset : (await readStreamedAsset(entries.find((e) => e.folder === target)!)).asset;
  const document = await modelOfAsset(asset, { poses: "all", clips: [] });
  const errors = validateModel(document).filter((i) => i.severity === "error");
  console.log(`${asset.id}: ${errors.length} errors, ${document.solids.length} solids`);
  for (const issue of errors) {
    const solid = document.solids[Number(/^\/solids\/(\d+)/u.exec(issue.path)?.[1] ?? -1)] as Record<string, unknown> | undefined;
    console.log(`  ${issue.code} ${issue.path} ${solid?.id} ${solid?.kind} node ${solid?.node} ${JSON.stringify({ profile: solid?.profile, holes: solid?.holes, plane: solid?.plane, sweepRad: solid?.sweepRad }).slice(0, 3000)}`);
  }
}
