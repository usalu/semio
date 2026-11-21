/** 🔏 Installs the checked picker work under exact shared source custody. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const generated = join(import.meta.dir, "🗑️generated");
const source = readFileSync(join(generated, "raw-picker-mounted-overlay.json"));
const sha = (bytes: Buffer | string) => createHash("sha256").update(bytes).digest("hex");
const { preimages, candidate } = JSON.parse(source.toString()) as {
    preimages: Record<string, string | null>;
    candidate: Record<string, string>;
};
const proof = JSON.parse(readFileSync(join(generated, "raw-picker-mounted-measurement.json"), "utf8")) as {
    source: string;
    checks: { os: string; changed: boolean; admitted: boolean }[];
};
assert.equal(proof.source, sha(source));
assert.equal(new Set(proof.checks.map(row => row.os)).size, 4);
assert.ok(proof.checks.some(row => row.changed) && proof.checks.every(row => !row.changed || row.admitted));
const rows = Object.entries(candidate).map(([path, body]) => ({ path, body, before: preimages[path], after: sha(body) }));
const live = (path: string) => existsSync(path) ? sha(readFileSync(path)) : null;
for (const row of rows) {
    assert.ok(resolve(row.path).normalize("NFD").length <= 256, row.path);
    assert.ok(live(row.path) === row.before || live(row.path) === row.after, row.path);
}
if (process.argv.includes("--apply")) {
    const scratch = join(generated, "picker-install");
    mkdirSync(scratch, { recursive: true });
    for (const [i, row] of rows.sort((a, b) => Number(a.before !== null) - Number(b.before !== null)).entries()) {
        if (live(row.path) === row.after) continue;
        const staging = join(scratch, `source-${process.pid}-${i}`);
        writeFileSync(staging, row.body, { flag: "wx", mode: existsSync(row.path) ? statSync(row.path).mode & 0o777 : 0o644 });
        assert.equal(live(row.path), row.before, row.path);
        mkdirSync(dirname(row.path), { recursive: true });
        renameSync(staging, row.path);
        assert.equal(live(row.path), row.after, row.path);
    }
    const files = rows.map(({ path, before, after }) => ({ path, before, after }));
    writeFileSync(join(generated, "raw-picker-mounted-installed.json"), JSON.stringify({ digest: proof.source, files }, null, 2) + "\n");
}
console.log("[DEBUG] Verified", rows.length, "guarded mounted picker sources", proof.source);
