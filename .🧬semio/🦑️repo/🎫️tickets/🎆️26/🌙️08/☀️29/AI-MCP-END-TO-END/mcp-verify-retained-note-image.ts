import assert from "node:assert/strict";
import { readFile, rm, writeFile } from "node:fs/promises";
import { join, resolve, sep } from "node:path";
import { actorPreparationImageIdentityV1, actorPreparationPolicyV1, readActorPreparationV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🌱️actor-preparation/🟦️.ts";

const [directory, ...extra] = process.argv.slice(2);
if (!directory || extra.length) throw Error("One known staged model directory is required");
const root = resolve(directory);
assert(root.startsWith(join(import.meta.dirname, "🗑️generated") + sep));
const bytes = Bun.file(join(root, "📦️component.wasm"));
const component = new Bun.CryptoHasher("sha256");
for await (const chunk of bytes.stream()) component.update(chunk);
const componentSha256 = component.digest("hex");
const image = join(root, actorPreparationPolicyV1.image);
const receiptPath = join(root, actorPreparationPolicyV1.receipt);
const signal = new AbortController().signal;
const { imageSha256, imageBytes } = await actorPreparationImageIdentityV1(image, signal);
const receipt = { version: 1, componentSha256, imageSha256, imageBytes };
const canonical = JSON.stringify(receipt) + "\n";
await writeFile(receiptPath, canonical, { flag: "wx" });
const duplicatePath = receiptPath + ".test";
try {
  await writeFile(duplicatePath, canonical.replace('{"version":1,', '{"version":1,"version":1,'), { flag: "wx" });
  await assert.rejects(() => readActorPreparationV1(image, duplicatePath, componentSha256, signal), /Canonical actor preparation receipt/);
  assert.deepEqual(await readActorPreparationV1(image, receiptPath, componentSha256, signal), receipt);
  assert.equal(await readFile(receiptPath, "utf8"), canonical);
  console.log("[DEBUG] exact model image and canonical receipt verified", { componentSha256, imageSha256, imageBytes });
} finally {
  await rm(duplicatePath, { force: true });
}
