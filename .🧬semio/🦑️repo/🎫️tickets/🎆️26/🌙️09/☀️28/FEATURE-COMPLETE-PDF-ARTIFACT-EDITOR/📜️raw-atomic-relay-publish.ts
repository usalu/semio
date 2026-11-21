/** 🧷 Verifies and installs the joint authored-media relay source cut. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, resolve, sep } from "node:path";

const generated = join(import.meta.dir, "🗑️generated");
const names = [
    "raw-spr-transport-overlay.json",
    "raw-store-inbox-overlay.json",
    "raw-hub-socket-overlay.json",
    "raw-hub-host-overlay.json",
    "raw-reactor-inbound-overlay.json",
    "checkpoint-atomic-store-overlay.json",
];
const worker = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/";
const joinedWorkerPaths = new Set([
    worker + "♻️artifact-lifecycle/🟦️.ts",
    worker + "🧷️actor-reservation/📬️document-backbone/🟦️.ts",
    worker + "🛑️session-teardown/🟦️.ts",
]);
const hash = (body: string | Buffer) => createHash("sha256").update(body).digest("hex");
const observed = (file: string) => existsSync(file) ? hash(readFileSync(file)) : null;
const changes = new Map<string, {owner: string; file: string; before: string | null; body: string; after: string}>();
const digests: Record<string, string> = {};
const root = process.cwd() + sep;

for (const name of names) {
    const bytes = readFileSync(join(generated, name));
    digests[name] = hash(bytes);
    const overlay = JSON.parse(bytes.toString("utf8")) as {
        preimages: Record<string, string | null>;
        candidate: Record<string, string>;
    };
    assert.equal(Object.keys(overlay.preimages).length, Object.keys(overlay.candidate).length, name);
    for (const [path, body] of Object.entries(overlay.candidate)) {
        const file = resolve(path);
        const before = overlay.preimages[path];
        const previous = changes.get(path);
        assert.ok(file.startsWith(root) && file.length <= 256, path);
        assert.ok(before === null || /^[0-9a-f]{64}$/u.test(before), path);
        if (previous) {
            assert.equal(previous.before, before, path);
            if (previous.body !== body) {
                assert.equal(previous.owner, "raw-spr-transport-overlay.json", path);
                assert.equal(name, "raw-hub-host-overlay.json", path);
                assert.ok(joinedWorkerPaths.delete(path), path);
            }
        }
        changes.set(path, {owner: name, file, before, body, after: hash(body)});
    }
}

assert.equal(joinedWorkerPaths.size, 0);
assert.equal(changes.size, 58);
const bounds = await Bun.file(join(generated, "raw-atomic-baseline-source-bounds.json")).json();
assert.deepEqual(bounds.digests, digests);
assert.ok(bounds.results.length > 0 && bounds.results.every((row: {admitted: boolean}) => row.admitted));
assert.deepEqual(bounds.missed, []);
assert.equal(bounds.outside.length, 1);
assert.ok(bounds.outside[0].endsWith("📮️document-codec-replay-and-projection-gate-laws/🦀️.rs"));
const checkpoint = await Bun.file(join(generated, "checkpoint-atomic-store-bounds.json")).json();
assert.equal(checkpoint.digest, digests["checkpoint-atomic-store-overlay.json"]);
assert.ok(checkpoint.checks.length === 24 && checkpoint.checks.every((row: {admitted: boolean}) => row.admitted));

for (const [path, change] of changes) assert.equal(observed(change.file), change.before, path);
if (process.argv.includes("--apply")) {
    for (const [index, [path, change]] of [...changes].entries()) {
        const pending = join(generated, "atomic-relay-publish-" + index);
        writeFileSync(pending, change.body);
        assert.equal(observed(change.file), change.before, path);
        mkdirSync(dirname(change.file), {recursive: true});
        renameSync(pending, change.file);
        assert.equal(observed(change.file), change.after, path);
    }
    const record = [...changes].map(([path, {before, after}]) => ({path, before, after}));
    writeFileSync(join(generated, "atomic-relay-installed.json"), JSON.stringify(record, null, 2));
}
console.log("[DEBUG]", changes.size, "authored-media relay paths and", bounds.results.length, "platform scopes verified");
