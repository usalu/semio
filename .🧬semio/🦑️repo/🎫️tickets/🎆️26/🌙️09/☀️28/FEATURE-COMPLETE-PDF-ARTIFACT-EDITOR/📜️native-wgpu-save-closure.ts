/** 📏 Checks the mounted native Save postimages with the repository Rust source closure reader. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync, readFileSync} from "node:fs";
import {join, resolve} from "node:path";

const {measureRustCrate} = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const {stage, targets, live, kernel} = await Bun.stdin.json() as {stage: string; targets: Record<string, string[]>; live: boolean; kernel: boolean};
const root = process.cwd();
const engine = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu");
const packages = "📦️packages/🦀️rust";
const owners = (stage.endsWith("/original-pacing") || stage.endsWith("/extension-query")) ? ["📡️kernel-protocol", "🏊️kernel-pool"] : stage.endsWith("/original-picker") ? ["📡️kernel-protocol", "🏊️kernel-pool", "🪢️program-bridge", "📂️shell-doc-open", "🐙️shell-state", "🔀️shell-actions", "🪝️shell-window-actions", "🖐️shell-pointer"] : kernel ? ["📡️kernel-protocol", "🗃️kernel-retained", "🏊️kernel-pool"] : ["🪢️program-bridge", "📂️shell-doc-open", "🐙️shell-state", "🚶️shell-session", "🔀️shell-actions", "🪝️shell-window-actions"];
const receipt = await Bun.file(join(stage, "receipt.json")).json() as Array<{file: string; source: string; before: string | null; after: string}>;
const digest = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const overlay = new Map<string, string>();
for (const row of receipt) {
    const file = resolve(root, row.file);
    assert.equal(existsSync(file) ? digest(readFileSync(file)) : null, live ? row.after : row.before, "Drifted live source " + row.file);
    assert(file.length <= 256, "Path exceeds 256 UTF-16 code units " + row.file);
    const proposed = readFileSync(join(stage, row.source));
    assert.equal(digest(proposed), row.after, "Drifted postimage " + row.source);
    assert(!overlay.has(file), "Duplicate postimage " + row.file);
    overlay.set(file, proposed.toString("utf8"));
}
console.log("[DEBUG] Checked " + overlay.size + " shared preimages and staged digests");

const mounted = new Set<string>();
const overflow: string[] = [];
for (const target of Object.keys(targets)) {
    const bias = targets[target];
    for (const owner of owners) {
        const dir = join(engine, owner, packages);
        const spec = Bun.TOML.parse(readFileSync(join(dir, "Cargo.toml"), "utf8")) as {lib: {path: string}};
        const entry = resolve(dir, spec.lib.path);
        for (const test of [false, true]) {
            const check = await measureRustCrate(root, entry, test, overlay, dir, bias);
            for (const path of check.paths) mounted.add(resolve(root, path));
            const row = target + " " + owner + (test ? " test" : " library") + " " + check.files + " files / " + check.lines + " lines / " + check.bytes + " bytes";
            console.log("[DEBUG] " + row);
            if (check.files > 32 || check.lines > 4_000 || check.bytes > 262_144) {
                const base = await measureRustCrate(root, entry, test, new Map(), dir, bias);
                overflow.push(row + " (current source " + base.files + " files / " + base.lines + " lines / " + base.bytes + " bytes)");
            }
        }
    }
}
for (const file of overlay.keys()) assert(mounted.has(file), "Source reader missed " + file);
assert.equal(overflow.length, 0, overflow.join("\n"));
