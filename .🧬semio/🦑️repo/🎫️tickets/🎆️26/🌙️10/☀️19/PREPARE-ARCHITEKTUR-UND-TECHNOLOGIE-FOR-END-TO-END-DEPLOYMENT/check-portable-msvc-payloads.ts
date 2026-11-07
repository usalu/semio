import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const generated = join(process.env.SEMIO_TICKET_DIR!, "🗑️generated");
const plan = JSON.parse(readFileSync(join(generated, "portable-msvc-plan.json"), "utf8"));
const observed = [28454839, 239072, 2057481, 51736641];
const hashes = ["03f56d24dd23314467144060c8ee70ca0fdd79a901b60e7d2f92f9cbe8d042cf", "40bc5dd011e16ee46698a8e36e71f4c85ee34dbe938368b295ed27690c569860", "428bcda4a867056ab696aeca95b1057f5282aa9a8121481e7787fae7548efde2", "254cbe1d5d5ac71ef7aebecba9980d530d3b3f63984d43bd5aca66b22d19ae30"];
for (let i = 0; i < hashes.length; i++) {
  const payload = plan.compiler[i].payloads[0];
  const url = new URL(payload.url);
  if (url.protocol !== "https:" || url.hostname !== "download.visualstudio.microsoft.com" || payload.sha256.toLowerCase() !== hashes[i]) throw Error("Payload plan refused");
  const response = await fetch(url, { signal: AbortSignal.timeout(120_000) });
  if (!response.ok || new URL(response.url).hostname !== url.hostname || Number(response.headers.get("content-length")) !== observed[i]) throw Error(`Payload length refused: ${i}`);
  const bytes = new Uint8Array(await response.arrayBuffer());
  const digest = createHash("sha256").update(bytes).digest("hex");
  if (bytes.length !== observed[i] || digest !== hashes[i]) throw Error(`Payload integrity refused: ${i}`);
  writeFileSync(join(generated, `msvc-${i}.vsix`), bytes, { flag: "wx" });
  console.log(JSON.stringify({ index: i, declaredSize: payload.size, actualSize: bytes.length, sha256: digest }));
}
