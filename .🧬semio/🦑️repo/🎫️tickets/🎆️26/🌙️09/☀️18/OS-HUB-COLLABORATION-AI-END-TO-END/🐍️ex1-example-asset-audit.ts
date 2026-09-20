/**
 * 🛡️ EX1 — exercises the updated `auditNavbarExampleArtifactPayload` over every committed
 * descriptor (it must stay silent: nothing is externalized yet), then over a synthetic descriptor
 * whose oversized example row carries an `AssetDeclaration` instead of an inline body (it must
 * accept that) and over the same row with neither (it must reject that).
 */
import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { auditNavbarExampleArtifactPayload } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts";
import { exampleBodyAssetName } from "../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";

const root = fileURLToPath(new URL("../../../../../../..", import.meta.url));
const pluginsRoot = join(root, "✏️s/🔌️plugins");

const problems: string[] = [];
let audited = 0;
for (const entry of readdirSync(pluginsRoot, { withFileTypes: true })) {
  if (!entry.isDirectory()) continue;
  const descriptorPath = join(pluginsRoot, entry.name, "🔣️.json");
  if (!existsSync(descriptorPath)) continue;
  const descriptor = JSON.parse(readFileSync(descriptorPath, "utf8")) as { manifest?: Parameters<typeof auditNavbarExampleArtifactPayload>[0]; assets?: { name?: string }[] };
  if (!descriptor.manifest?.pluginId) continue;
  audited += 1;
  problems.push(...auditNavbarExampleArtifactPayload(descriptor.manifest, descriptor.assets ?? []));
}
console.log(`committed descriptors audited: ${audited}; problems: ${problems.length}`);
for (const problem of problems) console.log(`  ${problem}`);

const dialect = { artifactKind: "s.puzzle.5d", standard: "1", subset: "*" };
const externalized = { pluginId: "puzzle", examples: [{ id: "capsule-dream", label: "Capsule Dream", dialect, artifactJson: "" }] } as unknown as Parameters<typeof auditNavbarExampleArtifactPayload>[0];
const assetName = exampleBodyAssetName({ id: "capsule-dream", dialect });
console.log(`asset name: ${assetName}`);
console.log(`externalized row WITH the declaration -> ${JSON.stringify(auditNavbarExampleArtifactPayload(externalized, [{ name: assetName }]))}`);
console.log(`externalized row WITHOUT any declaration -> ${JSON.stringify(auditNavbarExampleArtifactPayload(externalized, []))}`);
