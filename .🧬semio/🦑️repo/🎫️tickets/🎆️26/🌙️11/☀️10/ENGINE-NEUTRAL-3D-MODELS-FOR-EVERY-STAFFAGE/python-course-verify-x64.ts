import { createHash } from "node:crypto";
import { mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { fetchVerified, readManifest } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/🧱️native/🟦️.ts";

const [directory, host] = process.argv.slice(2);
mkdirSync(directory, { recursive: true });
const manifest = readManifest();
let count = 0;
for (const row of manifest.packages.filter(row => row.host === host)) {
  const file = join(directory, row.archive);
  await fetchVerified(row.url, file, row.sha256, row.bytes);
  const md5 = createHash("md5").update(readFileSync(file)).digest("hex");
  if (md5 !== row.md5) throw new Error(`md5 mismatch ${row.archive}`);
  if (![host, "noarch"].includes(row.subdir) || !row.url.includes(`/${row.subdir}/`)) throw new Error(`subdir mismatch ${row.archive}`);
  count++;
}
const asset = manifest.micromamba.assets.find(asset => asset.subdir === host)!;
await fetchVerified(`${manifest.micromamba.release}/${asset.name}`, join(directory, asset.name), asset.sha256, asset.bytes);
console.log(`[DEBUG] ${host}: ${count} packages and ${asset.name} verified (sha256, size, md5)`);
