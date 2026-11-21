/** 🪟 Install the verified browser bridge for the gated Original media picker. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const generated = join(import.meta.dir, "🗑️generated");
const overlay = readFileSync(join(generated, "original-media-react-overlay.json"));
const sha = (body: Buffer | string) => createHash("sha256").update(body).digest("hex");
const live = (file: string) => existsSync(file) ? sha(readFileSync(file)) : null;
const rows = (JSON.parse(overlay.toString()) as [string, string | null, string][]).map(([file, before, body]) =>
    ({ file, before, body, after: sha(body) }));
assert.equal(rows.length, 9);
for (const row of rows) {
    assert.ok(resolve(row.file).normalize("NFD").length <= 256, row.file);
    assert.ok(live(row.file) === row.before || live(row.file) === row.after, row.file);
}
if (process.argv.includes("--apply")) {
    const scratch = join(generated, "picker-browser-install");
    mkdirSync(scratch, { recursive: true });
    for (const [i, row] of rows.sort((a, b) => Number(a.before !== null) - Number(b.before !== null)).entries()) {
        if (live(row.file) === row.after) continue;
        const staging = join(scratch, `source-${process.pid}-${i}.ts`);
        writeFileSync(staging, row.body, { flag: "wx", mode: existsSync(row.file) ? statSync(row.file).mode & 0o777 : 0o644 });
        assert.equal(live(row.file), row.before, row.file);
        mkdirSync(dirname(row.file), { recursive: true });
        renameSync(staging, row.file);
        assert.equal(live(row.file), row.after, row.file);
    }
    const files = rows.map(({ file, before, after }) => ({ file, before, after }));
    writeFileSync(join(generated, "raw-browser-picker-installed.json"), JSON.stringify({ digest: sha(overlay), files }, null, 2) + "\n");
}
console.log("[DEBUG] Checked browser picker source receipt", sha(overlay), rows.length);
