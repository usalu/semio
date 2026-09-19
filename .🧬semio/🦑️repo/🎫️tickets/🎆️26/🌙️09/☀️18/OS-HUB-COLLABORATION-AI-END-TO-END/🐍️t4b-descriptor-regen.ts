/** 🔁️ Re-seals the lease fixture's packed package descriptor at the current app-channel protocol
 * version and rewrites every hash the corpus derives from those bytes. Round-trips the existing
 * bytes first, so an encoding that is not already canonical refuses instead of silently reshaping. */
import { readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { decodePackValue, encodePackValue } from "../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";

const path = new URL("../../../../../../../🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", import.meta.url);
const text = readFileSync(path, "utf8");
const fixture = JSON.parse(text);
const hex: string = fixture.descriptorHex;
const bytes = Uint8Array.from({ length: hex.length / 2 }, (_unused, index) => Number.parseInt(hex.slice(index * 2, index * 2 + 2), 16));
const decoded = decodePackValue(bytes) as Record<string, unknown>;
const roundTrip = encodePackValue(decoded);
if (roundTrip.length !== bytes.length || roundTrip.some((byte, index) => byte !== bytes[index])) throw new Error("fixture descriptor bytes are not canonical");

const protocol = decoded.executionProtocol as Record<string, unknown>;
console.log("carrier", typeof protocol.appChannelVersion, JSON.stringify(protocol.appChannelVersion));
protocol.appChannelVersion = DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1;
const sealed = encodePackValue(decoded);
const sealedHex = [...sealed].map((byte) => byte.toString(16).padStart(2, "0")).join("");
const sha256 = createHash("sha256").update(sealed).digest("hex");
console.log("byteLength", bytes.length, "→", sealed.length, "| sha256", fixture.manifest.descriptor.sha256, "→", sha256);

if (process.argv.includes("--apply")) {
  const next = text
    .replaceAll(hex, sealedHex)
    .replaceAll(`"byteLength": ${fixture.manifest.descriptor.byteLength}`, `"byteLength": ${sealed.length}`)
    .replaceAll(fixture.manifest.descriptor.sha256, sha256);
  writeFileSync(path, next);
  console.log("applied");
}
