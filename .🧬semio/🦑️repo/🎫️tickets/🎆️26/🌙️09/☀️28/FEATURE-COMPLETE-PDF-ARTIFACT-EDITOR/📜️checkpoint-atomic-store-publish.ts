/** 📦 Publishes the trusted grouped Store replay after SPR exposes its genuine framing. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const output = join(import.meta.dir, "🗑️generated");
const paths = JSON.parse(readFileSync(join(output, "checkpoint-atomic-store-overlay.json"), "utf8")) as {
    preimages: Record<string, string | null>;
    candidate: Record<string, string>;
};
const spr = "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️.rs";
const framing = "🧰️framework/🔨️modules/📡️replication/🚪️io/💾️binary/🔗️causal/🏪️atomic-hub.rs";
const abi = readFileSync(spr, "utf8");
assert.ok(/pub use .*\batomic_hub\b/.test(abi), "SPR grouped replay ABI must be live first");
assert.ok(readFileSync(framing, "utf8").includes("pub fn decode_checkpoint_replay("));
const absolute = process.cwd() + "/";
const hash = (text: string | Buffer) => createHash("sha256").update(text).digest("hex");
const preimage = (file: string) => existsSync(file) ? hash(readFileSync(file)) : null;
const changes = Object.entries(paths.candidate).map(([path, body]) => {
    const file = resolve(path);
    assert.ok(file.startsWith(absolute) && file.length <= 256);
    assert.equal(preimage(file), paths.preimages[path], "Shared Store source changed: " + path);
    return { path, file, body, before: paths.preimages[path], after: hash(body) };
});
assert.equal(Object.keys(paths.preimages).length, changes.length);
const bounds = JSON.parse(readFileSync(join(output, "checkpoint-atomic-store-bounds.json"), "utf8"));
assert.equal(bounds.digest, hash(readFileSync(join(output, "checkpoint-atomic-store-overlay.json"))));
assert.ok(bounds.checks.length === 24 && bounds.checks.every((row: { admitted: boolean }) => row.admitted));

if (process.argv.includes("--apply")) {
    for (const [index, change] of changes.entries()) {
        const pending = join(output, "trusted-checkpoint-publish-" + index);
        writeFileSync(pending, change.body);
        assert.equal(preimage(change.file), change.before, "Shared Store source changed: " + change.path);
        renameSync(pending, change.file);
        assert.equal(preimage(change.file), change.after);
    }
    writeFileSync(join(output, "checkpoint-atomic-store-installed.json"), JSON.stringify(changes.map(({ path, before, after }) => ({ path, before, after })), null, 2));
}
console.log("[DEBUG] Trusted checkpoint replay verified", changes.length, "paths");
