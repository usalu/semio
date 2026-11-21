/** 🧮 Validate the trusted grouped checkpoint sources under the repository crate caps. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const store = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
const scratch = join(import.meta.dir, "🗑️generated");
const raw = readFileSync(join(scratch, "checkpoint-atomic-store-overlay.json"));
const overlay = JSON.parse(raw.toString()) as { candidate: Record<string, string> };
const source = new Map(Object.entries(overlay.candidate).map(([name, body]) => [resolve(name), body]));
const { measureRustCrate } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));

const checks = [];
for (const path of ["📦️codec/🏭️factory", "⚙️engine", "🧪️tests/🌌️bin-04"]) {
    const manifest = resolve(store, path, "📦️packages/🦀️rust/Cargo.toml");
    const pkg = Bun.TOML.parse(readFileSync(manifest, "utf8")) as { package: { name: string }; lib: { path: string } };
    const folder = dirname(manifest);
    for (const os of ["macos", "linux", "windows", "wasi"]) {
        const cfg = [`target_os=${os}`, os === "wasi" ? "target_arch=wasm32" : os === "macos" ? "target_arch=aarch64" : "target_arch=x86_64",
            os === "wasi" ? "target_family=wasm" : os === "windows" ? "target_family=windows" : "target_family=unix"];
        for (const test of [false, true]) {
            const { files, lines, bytes } = await measureRustCrate(process.cwd(), resolve(folder, pkg.lib.path), test, source, folder, cfg);
            checks.push({ name: pkg.package.name, os, test, files, lines, bytes, admitted: files <= 32 && lines <= 4000 && bytes <= 262144 });
        }
    }
}
const digest = createHash("sha256").update(raw).digest("hex");
await Bun.write(join(scratch, "checkpoint-atomic-store-bounds.json"), JSON.stringify({ digest, checks }, null, 2) + "\n");
for (const check of checks.filter(row => !row.admitted)) console.error("[DEBUG] Trusted replay scope over cap", check);
assert.ok(checks.every(check => check.admitted));
console.log("[DEBUG] Checked trusted grouped replay sources", checks.length, digest);
