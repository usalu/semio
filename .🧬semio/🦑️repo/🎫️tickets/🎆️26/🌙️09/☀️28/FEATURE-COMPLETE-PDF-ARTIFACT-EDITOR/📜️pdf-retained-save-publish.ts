/** 💾 Install verified retained Save proof and page-graft source together. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const generated = join(import.meta.dir, "🗑️generated");
const root = join(process.argv[2], "✏️editor", "💾️native-save");
const pkg = resolve(root, "📦️packages", "🦀️rust");
const sha = (data: Buffer | string) => createHash("sha256").update(data).digest("hex");
const evidence = ["pdf-save-visible-graph", "pdf-save-box-graft"].map(name =>
    readFileSync(join(generated, name + "-overlay.json")));
const rows = evidence.flatMap(record => JSON.parse(record.toString()) as [string, string, string][])
    .map(([file, before, body]) => ({ file, before, body, after: sha(body) }));
assert.deepEqual(rows.map(row => row.file), [join(root, "🔏️proof.rs"), join(root, "🪡️graft.rs")]);
for (const { file, before, after } of rows) {
    assert.ok(resolve(file).normalize("NFD").length <= 256, file);
    const current = sha(readFileSync(file));
    assert.ok(current === before || current === after, file);
}

const { measureRustCrate } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const overlay = new Map(rows.map(row => [resolve(row.file), row.body]));
const targets = [["linux", "x86_64", "unix"], ["macos", "aarch64", "unix"], ["windows", "x86_64", "windows"], ["wasi", "wasm32", "wasm"]];
const closure = [];
for (const [os, arch, family] of targets) {
    const cfg = [`target_os="${os}"`, `target_arch="${arch}"`, `target_family="${family}"`];
    const { files, lines, bytes } = await measureRustCrate(process.cwd(), resolve(pkg, "🦀️.rs"), false, overlay, pkg, cfg);
    assert.ok(files <= 32 && lines <= 4000 && bytes <= 262144, os);
    closure.push({ os, files, lines, bytes });
}
const receipt = {
    stages: evidence.map(sha), closure,
    files: rows.map(({ file, before, after }) => ({ file, before, after })),
};
console.log("[DEBUG] Retained PDF Save closure", JSON.stringify(receipt));
if (process.argv.includes("--apply")) {
    const landing = join(generated, "retained-save-install");
    mkdirSync(landing, { recursive: true });
    for (const [index, { file, before, body, after }] of rows.entries()) {
        if (sha(readFileSync(file)) === after) continue;
        const next = join(landing, `postimage-${process.pid}-${index}.rs`);
        writeFileSync(next, body, { flag: "wx", mode: statSync(file).mode & 0o777 });
        assert.equal(sha(readFileSync(file)), before, file);
        renameSync(next, file);
        assert.equal(sha(readFileSync(file)), after, file);
    }
    writeFileSync(join(generated, "pdf-retained-save-installed.json"), JSON.stringify(receipt, null, 2) + "\n");
}
