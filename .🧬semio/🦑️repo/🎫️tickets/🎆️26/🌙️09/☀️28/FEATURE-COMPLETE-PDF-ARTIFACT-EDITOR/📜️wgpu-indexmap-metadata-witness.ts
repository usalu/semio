/** 🔎 Checks the exact retained Rust crate pair from the WGPU metadata failure. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const toolRoot = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🗃️store";
const { casRootV1 } = await import(resolve(toolRoot, "🟦️.ts"));
const { readCasObjectSyncV1 } = await import(resolve(toolRoot, "🔁️sync/🟦️.ts"));
const cache = resolve(".cache/b/r/m/u");
const out = join(import.meta.dir, "🗑️generated", "native-wgpu-save", "metadata-witness");
mkdirSync(out, { recursive: true });
const names = new Map([
    ["indexmap", "libindexmap-b714a93ca0d43eaf.rmeta"],
    ["hashbrown", "libhashbrown-1ddd3887212c7adf.rmeta"],
    ["equivalent", "libequivalent-5d2f3db634be715e.rmeta"],
    ["foldhash", "libfoldhash-23473c34dfe3a4b7.rmeta"],
    ["serde_core", "libserde_core-d89b06ca726e49e5.rmeta"],
]);
const events = readFileSync(join(cache, "x", "p83c054.jsonl"), "utf8").trim().split("\n").map(line => JSON.parse(line));
const paths = new Map<string, string>();
for (const [crate, name] of names) {
    const keys = new Set<string>(events.filter(event => event.crate === crate && typeof event.key === "string").map(event => event.key));
    for (const key of keys) {
        const directory = join(cache, "e", key.slice(0, 2), key);
        if (!existsSync(directory)) continue;
        for (const record of readdirSync(directory).filter(file => file.endsWith(".json")).sort().reverse()) {
            const entry = JSON.parse(readFileSync(join(directory, record), "utf8"));
            const output = entry.outputs.find((candidate: { name: string }) => candidate.name === name);
            if (!output) continue;
            const bytes = readCasObjectSyncV1(casRootV1(), output.sha256);
            if (!bytes) continue;
            assert.equal(createHash("sha256").update(bytes).digest("hex"), output.sha256);
            const path = join(out, crate, name);
            mkdirSync(dirname(path), { recursive: true });
            writeFileSync(path, bytes);
            paths.set(crate, path);
            console.log("[DEBUG] Checked", crate, output.sha256);
            break;
        }
        if (paths.has(crate)) break;
    }
}
for (const crate of names.keys()) assert.ok(paths.has(crate), "Missing verified compiler metadata for " + crate);
const rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"];
const input = join(out, "metadata.rs");
writeFileSync(input, "extern crate indexmap;\npub fn index() -> indexmap::IndexMap<u32, u32> { indexmap::IndexMap::new() }\n");
const shared = [...rustc, "--crate-type=lib", "--emit=metadata", "--edition=2021", input, "-o", join(out, "probe.rmeta"),
    "--extern", "indexmap=" + paths.get("indexmap"),
    ...[...paths].filter(([name]) => name !== "hashbrown").flatMap(([, file]) => ["-L", "dependency=" + dirname(file)])];
const absent = Bun.spawnSync(shared, { stdout: "pipe", stderr: "pipe" });
const full = Bun.spawnSync([...shared, "-L", "dependency=" + dirname(paths.get("hashbrown")!)], { stdout: "pipe", stderr: "pipe" });
writeFileSync(join(out, "without-hashbrown.log"), absent.stderr);
writeFileSync(join(out, "with-hashbrown.log"), full.stderr);
assert.ok(absent.exitCode !== 0 && absent.stderr.toString().includes("E0460"), absent.stderr.toString());
assert.equal(full.exitCode, 0, full.stderr.toString());
console.log("[DEBUG] Reproduced E0460 without the exact retained hashbrown metadata and cleared it by restoring the matching file");
