/** 📐 Measure the composed segmented xref candidate through the repository's Cargo source reader. */
import assert from "node:assert/strict";
import { dirname, join, resolve } from "node:path";

const { measureRustCrate } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const { CRATE_HARD_BUDGET } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/💰️budget/🟦️.ts"));
const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules";
const generated = join(import.meta.dir, "🗑️generated");
const overlayName = process.argv[2] ?? "pdf-xref-byte-overlay.json";
const rows = await Bun.file(join(generated, overlayName)).json() as Array<[string, string, string]>;
const overlay = new Map<string, string>();
for (const [path, before, body] of rows) {
  const live = await Bun.file(path).text();
  assert.equal(new Bun.CryptoHasher("sha256").update(live).digest("hex"), before, path);
  overlay.set(resolve(path), body);
}

type Manifest = { package: { name: string; metadata?: { semio?: { "rust-source-boundary"?: Record<string, number> } } }; lib: { path: string }; features?: Record<string, string[]> };
const manifests = [
  join(root, "🔤️lexer/📦️packages/🦀️rust/Cargo.toml"),
  join(root, "🔗️xref/💧️stream/📦️packages/🦀️rust/Cargo.toml"),
  join(dirname(root), "🧬️schema/🔎️graph-projection/📦️packages/🦀️rust/Cargo.toml"),
];
if (overlay.has(resolve(root, "🔗️xref/🧪️tests/🔬️unit/🦀️.rs"))) {
  manifests.push(join(dirname(root), "🧬️schema/🏅️conformance-support/📦️packages/🦀️rust/Cargo.toml"));
}
const sourceContract = "🧰️framework/🔨️modules/🌱️value/📦️paged/📖️read/🎟️.rs";
if (overlay.has(resolve(sourceContract))) {
  manifests.push(
    "🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/📦️packages/🦀️rust/Cargo.toml",
    join(root, "🗜️filters/📦️packages/🦀️rust/Cargo.toml"),
    join(dirname(root), "🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml"),
    join(dirname(root), "🧬️schema/🧬️mutations/🪄️replace-page/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml"),
  );
}
const vtManifest = join(dirname(dirname(root)), "🧾️vt/🚪️io/📦️packages/🦀️rust/Cargo.toml");
if (overlay.has(resolve(dirname(vtManifest), "🦀️.rs"))) manifests.push(vtManifest);
const scopes = await Promise.all(manifests.map(async (path) => {
  const manifest = Bun.TOML.parse(await Bun.file(path).text()) as unknown as Manifest;
  return { dir: dirname(path), source: resolve(dirname(path), manifest.lib.path), manifest };
}));
const claimed = new Set<string>();
let outside = false;
for (const os of ["linux", "macos", "windows", "wasi"]) {
  const arch = os === "macos" ? "aarch64" : os === "wasi" ? "wasm32" : "x86_64";
  const family = os === "windows" ? "windows" : os === "wasi" ? "wasm" : "unix";
  const platform = [family, 'target_os="' + os + '"', 'target_arch="' + arch + '"', 'target_family="' + family + '"'];
  for (const { dir, source, manifest } of scopes) {
    const features = Object.keys(manifest.features ?? {}).filter(feature => feature !== "default");
    for (const enabled of features.length ? [[], features] : [[]]) {
      const cfg = [...platform, ...enabled.map(feature => 'feature="' + feature + '"')];
      for (const test of [false, true]) {
        const result = await measureRustCrate(process.cwd(), source, test, overlay, dir, cfg);
        const local = manifest.package.metadata?.semio?.["rust-source-boundary"] ?? {};
        const admitted = result.files <= Math.min(CRATE_HARD_BUDGET.files, local.maxSourceFiles ?? Infinity)
          && result.lines <= Math.min(CRATE_HARD_BUDGET.lines, local.maxActiveLines ?? Infinity)
          && result.bytes <= Math.min(CRATE_HARD_BUDGET.bytes, local.maxSourceBytes ?? Infinity);
        for (const path of result.paths) if (overlay.has(resolve(path))) claimed.add(resolve(path));
        console.log("[DEBUG]", os, manifest.package.name, test ? "test" : "lib", enabled.join(",") || "base", result.files, result.lines, result.bytes, admitted ? "admitted" : "over budget");
        if (!admitted) {
          const live = await measureRustCrate(process.cwd(), source, test, new Map(), dir, cfg);
          console.log("[DEBUG] Live baseline", live.files, live.lines, live.bytes);
          outside = true;
        }
      }
    }
  }
}
assert.equal(claimed.size, overlay.size, [...overlay.keys()].filter(path => !claimed.has(path)).join("\n"));
if (outside) process.exitCode = 1;

