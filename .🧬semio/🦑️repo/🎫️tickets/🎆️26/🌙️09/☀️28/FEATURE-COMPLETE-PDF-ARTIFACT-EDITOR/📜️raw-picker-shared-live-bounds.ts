/** 🧭 Checks the joined picker and its maintenance owners in all supported source configurations. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const { measureRustCrate } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const rows = [];
for (const path of ["⚛️reactor/📨️pending", "🧵️instance/🖼️render", "⚛️reactor/🔌️app", "🧵️instance", "🧵️instance/📄️docs", "⚛️reactor/🏃️runtime/🧰️ops", "⚛️reactor/🔄️turn", "🪜️maintenance-ladder"]) {
    const manifest = resolve(root, path, "📦️packages/🦀️rust/Cargo.toml");
    const folder = dirname(manifest);
    const pkg = Bun.TOML.parse(readFileSync(manifest, "utf8")) as { package: { name: string }; lib: { path: string } };
    for (const os of ["macos", "linux", "windows", "wasi"]) {
        const cfg = ["target_os=" + os, "target_arch=" + (os === "wasi" ? "wasm32" : os === "macos" ? "aarch64" : "x86_64"),
            "target_family=" + (os === "wasi" ? "wasm" : os === "windows" ? "windows" : "unix")];
        for (const test of [false, true]) {
            const result = await measureRustCrate(process.cwd(), resolve(folder, pkg.lib.path), test, new Map(), folder, cfg);
            const { files, lines, bytes } = result;
            rows.push({ package: pkg.package.name, os, test, files, lines, bytes, admitted: files <= 32 && lines <= 4000 && bytes <= 262144 });
        }
    }
}
await Bun.write(join(import.meta.dir, "🗑️generated", "raw-picker-shared-live-bounds.json"), JSON.stringify(rows, null, 2) + "\n");
for (const row of rows.filter(row => row.os === "macos" || !row.admitted)) console.log("[DEBUG]", JSON.stringify(row));
assert.ok(rows.every(row => row.admitted));
