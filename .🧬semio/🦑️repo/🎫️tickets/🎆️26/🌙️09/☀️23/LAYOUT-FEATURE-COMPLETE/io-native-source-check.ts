import { readFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";

const root = process.cwd();
const io = resolve(process.argv[2]);
const mode = process.argv[3];
const { createRustSourceAnalysisV1, readRustSourceClosureV1 } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const routes = {
  library: "🦀️.rs",
  birth: "🔀️interchange/🪆️drawing/🌱️birth/🧪️tests/🦀️.rs",
  interchange: "🔀️interchange/🧪️tests/🦀️.rs",
};
const groups = [
  ["drawing_lift", "🔀️interchange/🪆️drawing/🧪️tests/🦀️.rs"],
  ["source_plan", "🔀️interchange/🧭️source-plan/🧪️tests/🦀️.rs"],
  ["vector_scene", "📤️export/📐️vector-scene/🧪️tests/🦀️.rs"],
  ["json_export", "📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🧪️tests/🔬️unit/🦀️.rs"],
];
const moved = new Set([routes.interchange, ...groups.map(([, path]) => path), "🔀️interchange/🧪️tests/🔮️oracle/🦀️.rs"]);
const analysis = createRustSourceAnalysisV1();
const owner = { manifestDirectory: join(io, "📦️packages/🦀️rust"), generated: () => undefined };
const overlay = new Map();
if (mode === "draft") {
  const patch = await readFile(join(dirname(process.argv[1]), "🗑️generated", "io-native-test-admission.patch"), "utf8");
  for (const group of patch.slice(6).split("\n--- a/")) {
    const [file, destination, ...chunks] = group.replace(/\n$/, "").split("\n");
    if (destination !== "+++ b/" + file) throw new Error(`Unexpected draft destination for ${file}`);
    const path = join(root, file);
    const before = (await readFile(path, "utf8")).split("\n");
    let index = 0;
    const after = [];
    for (const [position, line] of chunks.entries()) {
      if (line.startsWith("@@")) {
        const match = /^@@ -(\d+)(?:,(\d+))? \+\d+(?:,\d+)? @@/.exec(line);
        if (!match) throw new Error(`Invalid draft hunk header for ${file}`);
        let start = Number(match[1]) - (match[2] === "0" ? 0 : 1);
        const context = [];
        for (const candidate of chunks.slice(position + 1)) {
          if (candidate.startsWith("@@")) break;
          if (!candidate.startsWith("+")) context.push(candidate.slice(1));
        }
        const matches = (at) => at >= index && at + context.length <= before.length
          && context.every((text, offset) => before[at + offset] === text);
        if (!matches(start)) {
          const candidates = [];
          for (let at = index; at <= before.length - context.length; at += 1) {
            if (matches(at)) candidates.push(at);
          }
          if (candidates.length !== 1) throw new Error(`Draft hunk for ${file} has ${candidates.length} matches in live source`);
          const shifted = candidates[0];
          console.log(`[DEBUG] ${file}: draft hunk shifted by ${shifted - start} lines`);
          start = shifted;
        }
        after.push(...before.slice(index, start));
        index = start;
      } else {
        if (line[0] !== "+" && before[index++] !== line.slice(1)) throw new Error(`Shared source changed under the draft for ${file}`);
        if (line[0] !== "-") after.push(line.slice(1));
      }
    }
    after.push(...before.slice(index));
    overlay.set(path, Buffer.from(after.join("\n")));
  }
}
async function read(path, draft) {
  if (draft && overlay.has(path)) return overlay.get(path);
  return readFile(path).catch(error => { if (error.code === "ENOENT") return null; throw error; });
}
if (mode === "draft" || mode === "verify") {
  const draft = mode === "draft";
  const manifest = Bun.TOML.parse((await read(join(owner.manifestDirectory, "Cargo.toml"), draft)).toString("utf8"));
  const lock = Bun.TOML.parse((await read(join(owner.manifestDirectory, manifest.package.workspace, "Cargo.lock"), draft)).toString("utf8"));
  const reader = "semio-framework-test-fixtures";
  const member = lock.package.find(row => row.name === manifest.package.name);
  const dependency = manifest["dev-dependencies"][reader];
  if (!dependency || manifest.dependencies[reader] || !member.dependencies.includes(reader) || !lock.package.some(row => row.name === reader)) {
    throw new Error("Layout IO fixture reader is missing a test-only locked dependency edge");
  }
  const testTargets = new Map(manifest.test.map(target => [target.name, resolve(owner.manifestDirectory, target.path)]));
  if (resolve(owner.manifestDirectory, manifest.lib.path) !== join(io, routes.library)
    || testTargets.get("drawing-birth") !== join(io, routes.birth)
    || testTargets.get("interchange") !== join(io, routes.interchange)) {
    throw new Error("The scanner roots differ from Cargo's declared source paths");
  }
  console.log("[DEBUG] Cargo target paths and locked test-only fixture dependency validated by Bun TOML");
}
async function inspect(name, entry, test, draft) {
  const cfg = process.argv[4] === "windows" ? ['target_arch="x86_64"', 'target_os="windows"', 'target_env="msvc"', 'target_family="windows"', 'target_vendor="pc"', 'target_pointer_width="64"', 'target_endian="little"', "windows", "debug_assertions"] : ['target_arch="aarch64"', 'target_os="macos"', 'target_env=""', 'target_family="unix"', 'target_vendor="apple"', 'target_pointer_width="64"', 'target_endian="little"', "unix", "debug_assertions"];
  if (test) cfg.push("test");
  const sources = await readRustSourceClosureV1([join(io, entry)], root, cfg, path => read(path, draft), owner, analysis);
  const bytes = sources.reduce((sum, file) => sum + file.bytes, 0);
  const lines = sources.reduce((sum, file) => sum + file.activeLines, 0);
  console.log(`[DEBUG] ${name}: ${sources.length} paths, ${bytes} bytes, ${lines} active lines`);
  for (const source of sources.filter(source => source.kind === "embedded" || source.path.includes("🧪️tests"))) console.log(`[DEBUG]   ${source.kind} ${source.bytes} ${source.path.split("🚪️io/").at(-1)}`);
  if (draft || mode === "verify") {
    if (sources.length > 32 || bytes > 262_144 || lines > 4_000) throw new Error(`${name} exceeds its native source boundary`);
    const actual = sources.map(source => source.path.split("🚪️io/").at(-1)).filter(path => moved.has(path));
    if (actual.length !== (name === "Interchange" ? moved.size : 0)) throw new Error(`${name} owns ${actual.length} of ${moved.size} moved sources`);
  }
}
if (mode !== "verify") await inspect("Existing test library", routes.library, true, false);
const draft = mode === "draft";
await inspect("Production library", routes.library, false, draft);
await inspect("Test library", routes.library, true, draft);
await inspect("Drawing birth", routes.birth, true, draft);
await inspect("Interchange", routes.interchange, true, draft);
