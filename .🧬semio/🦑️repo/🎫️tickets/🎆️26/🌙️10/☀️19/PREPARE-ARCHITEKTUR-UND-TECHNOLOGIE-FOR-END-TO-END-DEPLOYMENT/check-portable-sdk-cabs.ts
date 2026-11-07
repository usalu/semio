import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(process.env.SEMIO_TICKET_DIR!, "🗑️generated");
const bytes = readFileSync(join(root, "bazel-msvc-manifest.json"));
const sha = (data: Uint8Array) => createHash("sha256").update(data).digest("hex");
if (sha(bytes) !== "a8e854fa9f3fbaeac27d7304e297f1660d7137401389deb57cf7983f33c16fef") throw Error("Untrusted manifest");
const manifest = JSON.parse(bytes.toString("utf8")), plan = JSON.parse(readFileSync(join(root, "portable-msvc-plan.json"), "utf8"));
const sdk = manifest.packages.filter((item: { id: string }) => item.id === "Win11SDK_10.0.26100");
if (sdk.length !== 1 || sdk[0].version !== "10.0.26100.15" || plan.sdk.payloads.length !== 8) throw Error("Unexpected SDK plan");
for (let i = 0; i < 8; i++) {
  const matches = sdk[0].payloads.filter((item: { fileName: string }) => item.fileName === plan.sdk.payloads[i].fileName);
  const data = readFileSync(join(root, `sdk-${i}.msi`));
  if (matches.length !== 1 || data.length !== matches[0].size || sha(data) !== matches[0].sha256.toLowerCase()) throw Error(`MSI integrity failed: ${i}`);
}
const cabinets = JSON.parse(readFileSync(join(root, "sdk-cabinets.json"), "utf8"));
if (!Array.isArray(cabinets) || cabinets.length !== 34 || new Set(cabinets.map(row => row.cabinet)).size !== 34) throw Error("Unexpected cabinet inventory");
for (const row of cabinets) {
  if (!Number.isInteger(row.index) || row.index < 0 || row.index >= 8 || !/^[a-f0-9]{32}\.cab$/u.test(row.cabinet)) throw Error("Cabinet name refused");
  const matches = sdk[0].payloads.filter((item: { fileName: string }) => item.fileName.replaceAll("\\", "/").toLowerCase() === `installers/${row.cabinet}`);
  if (matches.length !== 1) throw Error(`Ambiguous cabinet ${row.cabinet}`);
  const payload = matches[0], url = new URL(payload.url);
  if (url.protocol !== "https:" || url.hostname !== "download.visualstudio.microsoft.com" || payload.size > 60_000_000 || !/^[a-f0-9]{64}$/i.test(payload.sha256)) throw Error("Cabinet plan refused");
  const response = await fetch(url, { signal: AbortSignal.timeout(120_000) });
  if (!response.ok || new URL(response.url).hostname !== url.hostname || Number(response.headers.get("content-length")) !== payload.size) throw Error(`Cabinet length refused: ${row.cabinet}`);
  const data = new Uint8Array(await response.arrayBuffer());
  if (data.length !== payload.size || sha(data) !== payload.sha256.toLowerCase()) throw Error(`Cabinet integrity refused: ${row.cabinet}`);
  writeFileSync(join(root, row.cabinet), data, { flag: "wx" });
  console.log(`Verified ${row.cabinet} (${data.length} bytes)`);
}
