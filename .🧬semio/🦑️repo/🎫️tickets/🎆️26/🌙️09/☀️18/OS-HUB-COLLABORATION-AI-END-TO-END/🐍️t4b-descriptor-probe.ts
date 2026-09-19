/** 🧾️ Decodes the lease fixture's packed package descriptor and reports every field the worker's
 * `parseVerifiedPackageDescriptorV1` equality gate compares against the manifest's lease fields. */
import { readFileSync } from "node:fs";
import { decodePackValue } from "../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../../../../../../🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", import.meta.url), "utf8"));
const hex: string = fixture.descriptorHex;
const bytes = Uint8Array.from({ length: hex.length / 2 }, (_unused, index) => Number.parseInt(hex.slice(index * 2, index * 2 + 2), 16));
const descriptor = decodePackValue(bytes) as Record<string, unknown>;
const manifest = descriptor.manifest as Record<string, unknown>;
const fields = fixture.manifest;
const apps = (manifest.apps as Record<string, unknown>[]) ?? [];
const kinds = (manifest.artifactKinds as Record<string, unknown>[]) ?? [];
const app = apps.find((entry) => entry.id === fields.surface.appId);
console.log("descriptorVersion", descriptor.descriptorVersion, "packageId", descriptor.packageId, "vs", fields.package.packageId);
console.log("execution", descriptor.execution, "executionProtocol", JSON.stringify(descriptor.executionProtocol));
console.log("pluginId", manifest.pluginId, "vs", fields.package.pluginId, "| version", manifest.version, "vs", fields.package.version);
console.log("wasmSha256", (descriptor.hashes as Record<string, unknown>).wasmSha256, "vs", fields.component.sha256);
console.log("appIds", apps.map((entry) => entry.id), "want", fields.surface.appId, "surfaceId", fields.surface.surfaceId);
console.log("app.role", app?.role, "vs", fields.surface.role, "| dialect", JSON.stringify(app?.dialect), "vs", JSON.stringify(fields.parentDialect));
console.log("windowKinds", ((app?.windowKinds as Record<string, unknown>[]) ?? []).map((entry) => entry.id), "want", fields.surface.windowKindId);
console.log("artifactKinds", kinds.map((entry) => `${String(entry.id)}|${String(entry.schema)}`), "want", `${fields.artifact.kind}|${fields.artifact.schema}`);
