/** 🔎️ S15 — replays the browser worker's descriptor admission (`parseVerifiedPackageDescriptorV1`'s canonical + protocol
 * gates) against every package descriptor of the hub's trusted catalog generation, with the CURRENT tree's pack codec. */
import { readFileSync, readdirSync } from "node:fs";
import { decodePackValue, encodePackValue } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
const root = process.argv[2]!;
for (const pkg of readdirSync(`${root}/packages`).sort()) {
  const bytes = new Uint8Array(readFileSync(`${root}/packages/${pkg}/descriptor.semio`));
  try {
    const decoded = decodePackValue(bytes) as Record<string, any>;
    const canonical = encodePackValue(decoded as never);
    const same = canonical.length === bytes.length && canonical.every((byte: number, index: number) => byte === bytes[index]);
    const channel = decoded.executionProtocol?.appChannelVersion;
    console.log(`${pkg}: bytes=${bytes.length} canonical=${same} descriptorVersion=${JSON.stringify(decoded.descriptorVersion, (_k, v) => typeof v === "bigint" ? String(v) : v)} execution=${decoded.execution} appChannelVersion=${JSON.stringify(channel, (_k, v) => typeof v === "bigint" ? String(v) : v)} (browser expects ${DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1}) protocolKeys=${Object.keys(decoded.executionProtocol ?? {}).join(",")}`);
  } catch (error) {
    console.log(`${pkg}: decode failed ${String(error).slice(0, 200)}`);
  }
}
