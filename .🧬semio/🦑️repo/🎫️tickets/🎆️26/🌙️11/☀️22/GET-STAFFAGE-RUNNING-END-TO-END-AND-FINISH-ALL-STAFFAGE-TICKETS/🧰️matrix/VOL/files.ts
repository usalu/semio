/** 🔬️ The file half of the 🧊️model-catalog case in one process (no vitest) and the "four scales" case for every id: files.ts <asset id>… */
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { assetFile, mod } from "../../🧰️linux/s4/lib.ts";
const { exportModels, plannedNames } = await mod("🔨️modules/📤️export/📦️batch/🟦️.ts");
const { auditFiles, auditGeometry } = await mod("🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts");
const { authoredScales } = await mod("🔨️modules/🧊️volume/🟦️.ts");
const FBX = ["light-solid", "thermal", "forces"];
for (const id of process.argv.slice(2)) {
  const asset = JSON.parse(readFileSync(assetFile(id), "utf8")).asset;
  console.log(id, "scales", JSON.stringify(authoredScales(asset.volumes)));
  const geometry = await auditGeometry(asset);
  const directory = mkdtempSync(join(tmpdir(), "vol-files-"));
  try {
    await exportModels({ out: directory, assets: [asset.id], views: FBX });
    const manifest = JSON.parse(readFileSync(join(directory, "manifest.json"), "utf8"));
    const files = await auditFiles(asset, manifest.files, (f: string) => new Uint8Array(readFileSync(join(directory, f))), plannedNames(asset, { views: FBX }));
    const all = [...geometry.findings, ...files.findings];
    console.log(id, "geometry", geometry.findings.length, "files", files.findings.length, "bounds", files.bounds.ok, files.bounds.pose);
    for (const f of all.slice(0, 40)) console.log(" ", f.check, f.item, String(f.message).slice(0, 300));
  } finally { rmSync(directory, { recursive: true, force: true }); }
}
