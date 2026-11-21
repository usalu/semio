/** 📏 Measures the composed repository source closures. */
import {readFileSync} from "node:fs";
import {join, resolve} from "node:path";
const {measureRustCrate} = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const [base, generated, additional] = process.argv.slice(2);
const owner = {
  lift: "🔨️modules/⬇️lift",
  fonts: "🔨️modules/🔤️fonts",
  inferences: "🧬️schema/💡️inferences",
  content: "🔨️modules/🖋️content",
};
const stages = JSON.parse(readFileSync(join(generated, "font-release", "stages.json"), "utf8"));
if (additional) {
  const changes = JSON.parse(readFileSync(join(generated, additional, "stage.json"), "utf8")) as Record<string, {staged: string}>;
  for (const [path, entry] of Object.entries(changes)) stages[path] = entry.staged;
}
const files = new Map(Object.entries(stages).map(([file, source]) => [resolve(base, file), readFileSync(String(source), "utf8")]));
for (const target of ["macos", "linux", "windows", "wasi"]) {
  const arch = target === "wasi" ? "wasm32" : "x86_64";
  const cfg = [`target_os="${target}"`, `target_arch="${arch}"`, target === "windows" ? "windows" : target === "wasi" ? `target_family="wasm"` : "unix"];
  for (const [name, source] of Object.entries(owner)) {
    const dir = resolve(base, source, "📦️packages", "🦀️rust");
    for (const test of [false, true]) {
      const m = await measureRustCrate(process.cwd(), join(dir, "🦀️.rs"), test, files, dir, cfg);
      if (m.files > 32 || m.lines > 4000 || m.bytes > 262_144) throw Error(name + " exceeds current source cap");
      console.log("[DEBUG]", target, name, test ? "test" : "lib", m.files, m.lines, m.bytes);
    }
  }
}
