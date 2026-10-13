/** 🧪️ Dry trusted-catalog receipt run over the three regenerated shipped components (bim, animate, architect): stages each
 * deliverable's component, committed semantic descriptor and `ComponentReceiptV1` into a scratch generation layout, builds the
 * schemaVersion-4 package records the publisher writes, and holds every record to the strict-Ajv `TrustedBundleReceiptV1`,
 * `TrustedBundleComponentV1`/`TrustedBundleFileV1` and `DocumentShippedComponentReceiptV1` exports, the first-party receipt law
 * (`componentReceiptFindingsV1`, `componentReceiptBindsV1`) and node + WebCrypto digests. It writes nothing outside `argv[2]`. */
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { hubSchemaExport } from "../../../../../../../../🌎️hub/🤝️integration-harness/🟦️.ts";
import { componentDeliverableV1, componentReceiptFindingsV1, readComponentReceiptV1 } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import { COMPONENT_RECEIPT_FILE, SHIPPED_COMPONENT_MAX_BYTES, SHIPPED_COMPONENT_PROFILE, componentReceiptBindsV1, encodeComponentReceiptV1 } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🟦️.ts";

const REPO = "/Users/ueli/Documents/semio";
const OUT = process.argv[2];
if (!OUT) throw new Error("usage: bun 🧪️dry-receipts.ts <empty scratch generation root>");
const COMPONENTS = [
  { pluginId: "bim", owner: "🌎️hub/🧩️compositions/🏙️bim", packageName: "semio-hub-bim" },
  { pluginId: "animate", owner: "🌎️hub/🧩️compositions/🎞️animate", packageName: "semio-hub-animate" },
  { pluginId: "architect", owner: "🌎️hub/🧩️compositions/🏛️architect", packageName: "semio-hub-architect" },
] as const;
const LEGACY_CATALOG_COMPONENT_MAX_BYTES = 64 * 1024 * 1024;
const exports = Object.fromEntries(
  ["TrustedBundleReceiptV1", "TrustedBundleComponentV1", "TrustedBundleFileV1"].map((name) => [name, hubSchemaExport(REPO, `schema://hub.artifact-authority.trusted-catalog/${name}`)]),
) as Record<string, (value: unknown) => boolean>;
const shipped = hubSchemaExport(REPO, "schema://os.directory/DocumentShippedComponentReceiptV1");
const webSha256 = async (bytes: Uint8Array): Promise<string> => Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex");

rmSync(OUT, { recursive: true, force: true });
mkdirSync(OUT, { recursive: true });
const records: Record<string, unknown>[] = [];
for (const [index, spec] of COMPONENTS.entries()) {
  const deliverable = componentDeliverableV1(join(REPO, spec.owner, "📦️packages/🦀️rust"), spec.packageName, SHIPPED_COMPONENT_PROFILE);
  const component = readFileSync(deliverable.wasm);
  const descriptor = readFileSync(join(REPO, spec.owner, "🧩️component/🛂️.descriptor.semio"));
  const receiptText = readFileSync(deliverable.receipt, "utf8");
  const receipt = readComponentReceiptV1(deliverable.receipt);
  const findings = componentReceiptFindingsV1(receipt, { pluginId: spec.pluginId, packageId: `semio:${spec.pluginId}`, profile: SHIPPED_COMPONENT_PROFILE }, component, descriptor);
  if (findings.length > 0) throw new Error(`${spec.pluginId}: ${findings.join("; ")}`);
  const componentSha256 = createHash("sha256").update(component).digest("hex");
  const descriptorSha256 = createHash("sha256").update(descriptor).digest("hex");
  if (componentSha256 !== (await webSha256(component)) || descriptorSha256 !== (await webSha256(descriptor))) throw new Error(`${spec.pluginId}: node and WebCrypto digests disagree`);
  if (encodeComponentReceiptV1(receipt) !== receiptText) throw new Error(`${spec.pluginId}: receipt file is not its canonical framing bytes`);
  if (!componentReceiptBindsV1(receipt, { pluginId: spec.pluginId, packageId: `semio:${spec.pluginId}`, wasmSha256: componentSha256, semanticDescriptorSha256: descriptorSha256 }) || receipt.wasmBytes !== component.byteLength) throw new Error(`${spec.pluginId}: receipt does not bind`);
  if (component.byteLength > SHIPPED_COMPONENT_MAX_BYTES) throw new Error(`${spec.pluginId}: component exceeds the shipped cap`);
  const stage = join(OUT, "packages", spec.pluginId);
  mkdirSync(stage, { recursive: true });
  writeFileSync(join(stage, "component.wasm"), component);
  writeFileSync(join(stage, "descriptor.semio"), descriptor);
  writeFileSync(join(stage, COMPONENT_RECEIPT_FILE), receiptText);
  const record = {
    pluginId: spec.pluginId,
    packageId: receipt.packageId,
    component: { path: `packages/${spec.pluginId}/component.wasm`, byteLength: component.byteLength, sha256: componentSha256, blake3: "b".repeat(64) },
    descriptor: { path: `packages/${spec.pluginId}/descriptor.semio`, byteLength: descriptor.byteLength, sha256: descriptorSha256 },
    receipt,
  };
  if (!exports.TrustedBundleReceiptV1!(record.receipt) || !shipped(record.receipt) || !exports.TrustedBundleComponentV1!(record.component) || !exports.TrustedBundleFileV1!(record.descriptor)) throw new Error(`${spec.pluginId}: strict Ajv refused the schemaVersion-4 record`);
  const foreign = { ...receipt, semanticDescriptorSha256: "0".repeat(63) + "1" };
  if (componentReceiptBindsV1(foreign, { pluginId: spec.pluginId, packageId: receipt.packageId, wasmSha256: componentSha256, semanticDescriptorSha256: descriptorSha256 }) || shipped({ ...receipt, profile: "wasm-iterate" })) throw new Error(`${spec.pluginId}: a foreign or iterate receipt was admitted`);
  records.push(record);
  console.log(`[DEBUG] dry-receipt ${index + 1}/${COMPONENTS.length} ${spec.pluginId}: component=${component.byteLength}B (${component.byteLength > LEGACY_CATALOG_COMPONENT_MAX_BYTES ? "above" : "within"} the former 64 MiB bound) descriptor=${descriptor.byteLength}B receipt=${receiptText.length}B binds=true ajv=4 webcrypto=agree`);
}
writeFileSync(join(OUT, "dry-records.json"), `${JSON.stringify({ schemaVersion: 4, packages: records }, null, 2)}\n`);
console.log(`dry-receipts: ${records.length} shipped components staged with binding receipts under ${OUT}`);
