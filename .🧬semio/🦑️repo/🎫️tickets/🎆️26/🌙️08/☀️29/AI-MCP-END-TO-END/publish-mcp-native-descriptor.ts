import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { canonicalDescriptorDiffV1, componentStageTargetV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";

const owners = new Map([["dag", "🕸️dag"], ["reasoning", "💡️reasoning"], ["trinity", "🔱️trinity"]]);
const [owner, digest, baseline, ...remaining] = process.argv.slice(2);
const directory = owners.get(owner);
const isDigest = (value: string) => /^[a-f0-9]{64}$/u.test(value);
assert(directory && !remaining.length && (digest === undefined || (isDigest(digest) && isDigest(baseline))),
  "Pass one native owner, or the owner, verified semantic digest and original pair digest to publish");

const target = componentStageTargetV1(process.cwd(), `🌎️hub/🧩️compositions/${directory}/🧩️component/📦️packages/🦀️rust/Cargo.toml`);
const generated = join(import.meta.dirname, "🗑️generated", "descriptor-refresh", owner);
const names = ["🔣️.json", "🛂️.descriptor.semio"] as const;
const readPair = (root: string) => [readFileSync(join(root, names[0])), readFileSync(join(root, names[1]))] as const;
const original = readPair(target.ownerRoot);
const emitted = readPair(generated);
const verified = verifySemanticDescriptorPairBytesV1(...emitted);
const recorded = JSON.parse(original[0].toString("utf8"));
const expectedIdentity = [target.pluginId, target.packageId, target.role];
assert.deepEqual([recorded.manifest.pluginId, recorded.packageId, recorded.role], expectedIdentity);
assert.deepEqual([verified.pluginId, verified.packageId, verified.role], expectedIdentity);
const baselineDigest = createHash("sha256").update(original[0]).update(original[1]).digest("hex");
const changes = canonicalDescriptorDiffV1(recorded, JSON.parse(emitted[0].toString("utf8")), 300);
console.log("[DEBUG]", owner, { baselineDigest, semanticSha256: verified.semanticSha256, changes });

if (digest !== undefined) {
  assert.equal(verified.semanticSha256, digest);
  assert.equal(baselineDigest, baseline);
  const destinations = names.map(name => join(target.ownerRoot, name));
  const staging = mkdtempSync(join(generated, ".publish-"));
  const temporary = names.map(name => join(staging, name));
  const originalPack = join(staging, "original.pack");
  try {
    writeFileSync(temporary[1], emitted[1], { flag: "wx" });
    writeFileSync(temporary[0], emitted[0], { flag: "wx" });
    writeFileSync(originalPack, original[1], { flag: "wx" });
    assert.deepEqual(readPair(target.ownerRoot), original);
    renameSync(temporary[1], destinations[1]);
    try {
      renameSync(temporary[0], destinations[0]);
    } catch (error) {
      if (readFileSync(destinations[1]).equals(emitted[1])) renameSync(originalPack, destinations[1]);
      throw error;
    }
  } finally {
    rmSync(staging, { recursive: true, force: true });
  }
  const published = readPair(target.ownerRoot);
  assert.deepEqual(published, emitted);
  assert.equal(verifySemanticDescriptorPairBytesV1(...published).semanticSha256, digest);
  console.log("[DEBUG] Published and reverified exact native JSON and Pack", owner, digest);
}
