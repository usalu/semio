import { readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { workspace, authoredReleaseDigestV1 } from "./authored-release-descriptors.ts";
import { canonicalDescriptorDiffV1, componentStageTargetsV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts";
import { normalizeCatalogDescriptorEnums } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🛡️guards/🟦️.ts";
import { decodePackValue, packValueToExactJson } from "../../../../../../../🧰️framework/🔨️modules/🎁️packs/🟦️.ts";

const cases = JSON.parse(readFileSync(join(workspace, "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json"), "utf8"));
const roster = new Set([...cases.pluginIds, ...cases.extensionIds]);
for (const owner of componentStageTargetsV1(workspace)) {
  if (!roster.has(owner.pluginId)) continue;
  const json = readFileSync(join(owner.ownerRoot, "🔣️.json"));
  const pack = readFileSync(join(owner.ownerRoot, "🛂️.descriptor.semio"));
  const identity = {id: owner.pluginId, root: relative(workspace, owner.ownerRoot), manifest: relative(workspace, owner.manifestPath)};
  try {
    console.log(JSON.stringify({...identity, digest: authoredReleaseDigestV1(owner, json, pack)}));
  } catch (error) {
    const authored = normalizeCatalogDescriptorEnums(JSON.parse(json.toString("utf8")));
    const packed = normalizeCatalogDescriptorEnums(packValueToExactJson(decodePackValue(pack)));
    const diff = canonicalDescriptorDiffV1(authored, packed, 256);
    const embedded = (descriptor: unknown) => (descriptor as {contributions: {topicContributions: readonly {payload: {manifestJson?: string}}[]}}).contributions.topicContributions.map(row => row.payload.manifestJson).filter((value): value is string => Boolean(value)).map(value => JSON.parse(value));
    const prepare = (values: unknown[]) => values.map(value => {
      const manifest = value as {contributes: {operators: readonly {id: string}[]}};
      return {...manifest, contributes: {...manifest.contributes, operators: Object.fromEntries(manifest.contributes.operators.map(operator => [operator.id, operator]))}};
    });
    const embeddedDiff = canonicalDescriptorDiffV1(prepare(embedded(authored)), prepare(embedded(packed)), 256);
    console.log(JSON.stringify({...identity, error: String(error), diff, embeddedDiff}));
    process.exitCode = 1;
  }
}
