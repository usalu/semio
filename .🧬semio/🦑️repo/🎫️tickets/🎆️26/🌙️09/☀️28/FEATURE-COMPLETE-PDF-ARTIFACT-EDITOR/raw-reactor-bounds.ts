/** 📐 Checks the proposed original-media reactor owners against the repository crate limits. */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const { measureRustCrate } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const { CRATE_HARD_BUDGET } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/💰️budget/🟦️.ts"));
const mounted = process.argv.includes("--mounted");
const issuer = mounted || process.argv.includes("--issuer");
const durable = process.argv.includes("--durable");
const consumers = process.argv.includes("--consumers");
const store = process.argv.includes("--store");
const scratch = join(import.meta.dirname, "🗑️generated");
const inputs = durable ? [consumers ? "raw-png-consumers-overlay.json" : "raw-png-graph-overlay.json", "raw-spr-group-overlay.json"] :
  [mounted ? "raw-picker-mounted-overlay.json" : issuer ? "raw-pdf-picker-overlay.json" : "raw-reactor-overlay.json"];
if (store) inputs.push("raw-store-atomic-overlay.json");
const stages = await Promise.all(inputs.map(input => Bun.file(join(scratch, input)).json() as Promise<{candidate: Record<string,string>}>));
const root = process.cwd();
const overlay = new Map(stages.flatMap(source => Object.entries(source.candidate).map(([path, body]) => [resolve(path), body])));
const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const reports = [];
const claimed = new Set<string>();
const paths = [
  ...["", "/📜️declaration/🎞️media", "/⚛️reactor/🔌️app", "/⚛️reactor/📨️pending", "/⚛️reactor/🔄️turn",
      "/⚛️reactor/🧬️wit-bridge", "/⚛️reactor/🏃️runtime/🧰️ops", "/🧵️instance",
      "/🖥️host/🔗️wit-bindings", "/🖥️host/🔁️poll"].map(path => plugin + path),
  "🧰️framework/🔨️modules/🎠️kernel",
  "🧰️framework/🔨️modules/🎠️kernel/📤️return/🔤️json",
  "🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress",
];
if (durable) {
  const subsets = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets";
  paths.splice(0, paths.length,
    "🧰️framework/🔨️modules/📡️replication/🔗️causal",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr",
    subsets + "/🧱️base/🧬️schema/💡️inferences",
    subsets + "/🧱️base/✏️editor/📄️document");
  if (consumers) paths.push(subsets + "/🧱️base", subsets + "/🧱️base/🧬️schema",
    subsets + "/🧱️base/🧬️schema/🧪️testkit", subsets + "/🧱️base/✏️editor/🖼️page", subsets + "/🧾️vt/🧬️schema");
}
if (store) {
  const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
  paths.push(root + "/⚙️engine/🧳️bundle", root + "/⚙️engine", root + "/🧵️operation-wire");
}
if (issuer) {
  const base = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets";
  paths.push(plugin + "/🧵️retained-command", base + "/🧱️base/✏️editor/📄️document", base + "/🧾️vt/🧬️schema");
}
if (mounted) {
  paths.push(plugin + "/📜️declaration/🧩️app", plugin + "/📜️declaration/🪟️surface",
    plugin + "/🧵️instance/👷️jobs", plugin + "/🧵️instance/🖼️render",
    plugin + "/🧵️instance/📨️dispatch", plugin + "/🧵️instance/🧾️typed-operations", plugin + "/🧵️instance/📄️docs", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base");
}
if (mounted) paths.push(...["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/💡️inferences","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io"]);
if (mounted) paths.push(...["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧪️testkit"]);
for (const path of paths) {
  const manifest = resolve(path, "📦️packages/🦀️rust/Cargo.toml");
  const pkg = Bun.TOML.parse(overlay.get(manifest) ?? readFileSync(manifest, "utf8")) as {
    package: { name: string; metadata?: { semio?: { "rust-source-boundary"?: Record<string,number> } } };
    lib: { path: string };
  };
  const location = dirname(manifest);
  const file = resolve(location, pkg.lib.path);
  const limits = pkg.package.metadata?.semio?.["rust-source-boundary"] ?? {};
  for (const os of ["linux", "macos", "windows", "wasi"]) {
    const cfg = [`target_os=${os}`, os === "wasi" ? "target_arch=wasm32" : os === "macos" ? "target_arch=aarch64" : "target_arch=x86_64",
      os === "wasi" ? "target_family=wasm" : os === "windows" ? "windows" : "unix"];
    if (os === "wasi") cfg.push("target_env=p2");
    for (const test of [false, true]) {
      const row = await measureRustCrate(root, file, test, overlay, location, cfg);
      for (const source of row.paths) claimed.add(resolve(source));
      const admitted = row.files <= Math.min(CRATE_HARD_BUDGET.files, limits.maxSourceFiles ?? Infinity)
        && row.lines <= Math.min(CRATE_HARD_BUDGET.lines, limits.maxActiveLines ?? Infinity)
        && row.bytes <= Math.min(CRATE_HARD_BUDGET.bytes, limits.maxSourceBytes ?? Infinity);
      const changed = row.paths.some((path: string) => overlay.has(resolve(path)));
      reports.push({ name: pkg.package.name, os, test, files: row.files, lines: row.lines, bytes: row.bytes, admitted, changed });
    }
  }
}
await Bun.write(join(scratch, mounted ? "raw-picker-mounted-bounds.json" : store ? "raw-store-bounds.json" : durable ? "raw-durable-bounds.json" : issuer ? "raw-issuer-bounds.json" : "raw-reactor-bounds.json"), JSON.stringify(reports));
for (const row of reports.filter(value => !value.admitted && value.changed)) console.error("[DEBUG] Raw reactor source limits", row);
console.log("[DEBUG]", reports.length, "raw reactor source limit checks");
if (reports.some(row => !row.admitted && row.changed)) process.exitCode = 1;
const unmeasured = [...overlay.keys()].filter(path => path.endsWith(".rs") && !claimed.has(path));
for (const path of unmeasured) console.error("[DEBUG] Unmeasured staged Rust source", path);
if (unmeasured.length) process.exitCode = 1;
if (mounted && !process.exitCode) {
  const source = createHash("sha256").update(readFileSync(join(scratch, inputs[0]))).digest("hex");
  await Bun.write(join(scratch, "raw-picker-mounted-measurement.json"), JSON.stringify({ source, checks: reports }, null, 2) + "\n");
}
