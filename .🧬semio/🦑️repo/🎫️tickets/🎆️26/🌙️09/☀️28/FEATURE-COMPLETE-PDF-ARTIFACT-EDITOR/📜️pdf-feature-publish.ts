/** 🔏 Publish one source-reviewed PDF editor candidate using the exact live preimages. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

type Row = [string, string | null, string];
const output = join(import.meta.dir, "🗑️generated");
const roster = join(output, "pdf-registered-feature-overlay.json");
const bytes = readFileSync(roster);
const rows = JSON.parse(bytes.toString()) as Row[];
const sha = (body: string | Buffer) => createHash("sha256").update(body).digest("hex");
const { overlay, checks: bounds } = JSON.parse(readFileSync(join(output, "pdf-feature-source-bounds.json"), "utf8"));
assert.equal(overlay, sha(bytes));
assert.ok(bounds.length > 0 && bounds.every((row: {admitted:boolean}) => row.admitted));
assert.equal(new Set(bounds.map((row: {os:string}) => row.os)).size, 4);
assert.equal(new Set(rows.map(([path]) => path)).size, rows.length);

const observe = (path: string) => existsSync(path) ? sha(readFileSync(path)) : null;
for (const [path, before, body] of rows) {
    assert.ok(resolve(path).normalize("NFD").length <= 256, path);
    const current = observe(path);
    assert.ok(current === before || current === sha(body), `Source preimage moved: ${path}`);
}
if (process.argv[2] !== "--apply") {
    console.log(`[DEBUG] ${rows.length} candidates verified at ${sha(bytes)}`);
    process.exit();
}

const step = ([path, before]: Row) => before === null ? 0 : path.endsWith("/Cargo.lock") ? 4
    : path.endsWith("/Cargo.toml") ? 3 : path.endsWith("/✏️editor/🦀️.rs") ? 2 : 1;
const scratch = join(output, "pdf-live-publication");
mkdirSync(scratch,{recursive:true});
const completed = [];
let i = 0;
for (const [path, before, body] of rows.sort((a,b) => step(a) - step(b))) {
    const after = sha(body);
    if (observe(path) !== after) {
        mkdirSync(dirname(path),{recursive:true});
        const ready = join(scratch, `write-${process.pid}-${i++}`);
        const mode = existsSync(path) ? statSync(path).mode & 0o777 : 0o644;
        writeFileSync(ready, body, {flag:"wx",mode});
        assert.equal(observe(path), before, `Source preimage moved: ${path}`);
        renameSync(ready,path);
        assert.equal(observe(path),after,path);
    }
    completed.push({path,before,after});
}
writeFileSync(join(output,"pdf-registered-feature-installed.json"),JSON.stringify({overlay:sha(bytes),files:completed},null,2)+"\n");
console.log(`[DEBUG] Published ${completed.length} exact PDF editor source paths`);
