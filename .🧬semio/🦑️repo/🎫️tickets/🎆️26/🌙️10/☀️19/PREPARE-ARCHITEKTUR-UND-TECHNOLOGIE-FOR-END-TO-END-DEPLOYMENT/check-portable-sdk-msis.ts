import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const generated = join(process.env.SEMIO_TICKET_DIR!, "🗑️generated");
const source = readFileSync(join(generated, "bazel-msvc-manifest.json"));
if (createHash("sha256").update(source).digest("hex") !== "a8e854fa9f3fbaeac27d7304e297f1660d7137401389deb57cf7983f33c16fef") throw Error("Untrusted manifest");
const manifest = JSON.parse(source.toString("utf8"));
const sdk = manifest.packages.filter((entry: { id: string }) => entry.id === "Win11SDK_10.0.26100");
if (sdk.length !== 1 || sdk[0].version !== "10.0.26100.15") throw Error("Unexpected SDK version");
const names = ["Windows SDK for Windows Store Apps Tools-x86_en-us.msi", "Windows SDK for Windows Store Apps Headers-x86_en-us.msi", "Windows SDK for Windows Store Apps Headers OnecoreUap-x86_en-us.msi", "Windows SDK for Windows Store Apps Libs-x86_en-us.msi", "Universal CRT Headers Libraries and Sources-x86_en-us.msi", "Windows SDK Desktop Headers x64-x86_en-us.msi", "Windows SDK OnecoreUap Headers x64-x86_en-us.msi", "Windows SDK Desktop Libs x64-x86_en-us.msi"];
for (let i = 0; i < names.length; i++) {
  const entries = sdk[0].payloads.filter((payload: { fileName: string }) => payload.fileName.replaceAll("\\", "/") === `Installers/${names[i]}`);
  if (entries.length !== 1) throw Error(`SDK payload missing or ambiguous: ${names[i]}`);
  const payload = entries[0], url = new URL(payload.url);
  if (url.protocol !== "https:" || url.hostname !== "download.visualstudio.microsoft.com" || payload.size > 2_000_000 || !/^[a-f0-9]{64}$/i.test(payload.sha256)) throw Error("SDK payload plan refused");
  const response = await fetch(url, { signal: AbortSignal.timeout(60_000) });
  if (!response.ok || new URL(response.url).hostname !== url.hostname || Number(response.headers.get("content-length")) !== payload.size) throw Error(`SDK payload length refused: ${i}`);
  const bytes = new Uint8Array(await response.arrayBuffer());
  const digest = createHash("sha256").update(bytes).digest("hex");
  if (bytes.length !== payload.size || digest !== payload.sha256.toLowerCase()) throw Error(`SDK payload integrity refused: ${i}`);
  writeFileSync(join(generated, `sdk-${i}.msi`), bytes, { flag: "wx" });
  console.log(JSON.stringify({ name: names[i], size: bytes.length, sha256: digest }));
}
