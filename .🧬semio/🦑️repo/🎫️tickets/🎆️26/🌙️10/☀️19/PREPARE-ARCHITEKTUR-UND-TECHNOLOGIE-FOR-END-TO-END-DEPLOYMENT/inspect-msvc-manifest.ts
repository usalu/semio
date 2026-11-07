import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const directory = join(process.env.SEMIO_TICKET_DIR!, "🗑️generated");
async function read(url: string): Promise<Uint8Array> {
  const response = await fetch(url, { signal: AbortSignal.timeout(60_000) });
  if (!response.ok || new URL(response.url).protocol !== "https:") throw Error(`Metadata request failed: ${response.status}`);
  if (Number(response.headers.get("content-length")) > 32_000_000) throw Error("Metadata exceeds bounded size");
  const data = new Uint8Array(await response.arrayBuffer());
  if (data.length > 32_000_000) throw Error("Metadata exceeds bounded size");
  return data;
}
const major = process.argv[2] ?? "18";
if (major !== "17" && major !== "18") throw Error("Select Visual Studio channel 17 or 18");
const channel = await read(major === "17" ? "https://aka.ms/vs/17/release/channel" : "https://aka.ms/vs/18/stable/channel");
const channelJson = JSON.parse(new TextDecoder().decode(channel));
const item = channelJson.channelItems.find((entry: { id: string }) => entry.id === "Microsoft.VisualStudio.Manifests.VisualStudio");
if (!item || item.payloads.length !== 1) throw Error("Visual Studio manifest selection is ambiguous");
const manifest = await read(item.payloads[0].url);
const digest = createHash("sha256").update(manifest).digest("hex");
if (manifest.length !== item.payloads[0].size || digest !== item.payloads[0].sha256.toLowerCase()) throw Error(`Manifest integrity mismatch: ${manifest.length} bytes ${digest}`);
const parsed = JSON.parse(new TextDecoder().decode(manifest));
const ids = parsed.packages.map((entry: { id: string }) => entry.id).filter((id: string) => /^Microsoft\.VC\..*\.Tools\.HostX64\.TargetX64\.base$/i.test(id) || /^Microsoft\.VisualStudio\.Component\.Windows(?:10|11)SDK\.\d+$/i.test(id));
for (const [name, bytes] of [[`msvc-channel-${major}.json`, channel], [`msvc-manifest-${major}.json`, manifest]] as const) {
  writeFileSync(join(directory, name), bytes);
  console.log(name, bytes.length, createHash("sha256").update(bytes).digest("hex"));
}
console.log(JSON.stringify({ manifestPayload: item.payloads[0], versions: ids.sort() }));
