import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdir } from "node:fs/promises";
import { join } from "node:path";

const output = join(import.meta.dir, "🗑️generated/print2");
const extra = join(output, "article-tex");
const lock = await Bun.file("🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🔒️dependencies.json").json();
const index = Bun.file(join(output, "upstream-index.gz"));
if (!(await index.exists())) {
  const response = await fetch(lock.url + ".index.gz");
  assert.equal(response.status, 200);
  await Bun.write(index, await response.arrayBuffer());
}
const rows = new Map(new TextDecoder().decode(Bun.gunzipSync(await index.arrayBuffer())).trimEnd().split("\n").map(line => {
  const [name, offset, bytes] = line.split(" ");
  return [name, { name, offset: Number(offset), bytes: Number(bytes) }];
}));
async function range(name: string): Promise<Uint8Array> {
  const row = rows.get(name);
  assert.ok(row, name);
  const span = row.offset + "-" + (row.offset + row.bytes - 1);
  const response = await fetch(lock.url, { headers: { Range: "bytes=" + span }, signal: AbortSignal.timeout(30000) });
  assert.equal(response.status, 206, name);
  assert.equal(response.headers.get("Content-Range"), "bytes " + span + "/" + lock.archiveBytes, name);
  const bytes = new Uint8Array(await response.arrayBuffer());
  assert.equal(bytes.length, row.bytes, name);
  return bytes;
}
assert.equal(new TextDecoder().decode(await range("SHA256SUM")).trim(), lock.upstreamIdentity);
await mkdir(extra, { recursive: true });
const receipt = Bun.file(join(output, "article-ranges.json"));
const entries = new Map((await receipt.exists() ? await receipt.json() : []).map(row => [row.name, row]));
const candidates = process.argv.slice(2).flatMap(argument => {
  if (argument === "--latin-modern") return [...rows.keys()].filter(name => /^(?:(?:ec|rm|ts1)-lm|lm(?:r|bx|ri|tt|mi|sy|ex|csc)).*\.(?:tfm|pfb|enc)$/.test(name));
  if (argument === "--ams-fonts") return [...rows.keys()].filter(name => /^(?:amsfonts\.sty|amssymb\.sty|ums[ab]\.fd|ms[ab]m\d+\.(?:pfb|tfm))$/.test(name));
  return [argument];
});
const missing = candidates.filter(name => !lock.files.some(row => row.name === name) && !entries.has(name));
for (let index = 0; index < missing.length; index += 8) {
  await Promise.all(missing.slice(index, index + 8).map(async name => {
    const bytes = await range(name);
    const sha256 = createHash("sha256").update(bytes).digest("hex");
    await Bun.write(join(extra, name), bytes);
    entries.set(name, { ...rows.get(name), sha256 });
    console.log("[DEBUG]", name, sha256);
  }));
}
await Bun.write(receipt, JSON.stringify([...entries.values()], null, 2) + "\n");
