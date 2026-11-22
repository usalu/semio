/** 📐️ Measures selected Rust closures using the repository compiler’s production source reader. */
import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const workspace = process.cwd();
const { createRustSourceAnalysisV1, readRustSourceClosureV1 } = await import(resolve(workspace, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const analysis = createRustSourceAnalysisV1();
const owners = [
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✅validation",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧠️native-registry",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🏗️solid-topology",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧠️native-registry/🏗️solid-transaction",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine",
  "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session",
  "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep",
  "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/〰️analytic-curves",
  "🧰️framework/🔨️modules/🏗️mesh-engine",
  "🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping",
  "🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🗂️index",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🫧️shell-surfaces",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪝️shell-window-actions",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🖐️shell-pointer",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐙️shell-state",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔌️inputs",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-document",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/➡️vector",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📐️geometry",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/✏️drawing",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧮️geometry",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/🧱️records",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📤️export/📦️objects",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📤️export"
];
const { storage: { sourceBoundary } } = JSON.parse(await readFile(resolve(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8"));
const { cargoSourceBoundaryExcessV1 } = await import(resolve(workspace, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts"));
const targets = ["wasm32-wasip2", "aarch64-apple-darwin", "x86_64-pc-windows-msvc", "x86_64-unknown-linux-gnu"];
let excess = false;
for (const target of targets) {
  const command = spawnSync("repo", ["tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--print", "cfg", "--target", target], { encoding: "utf8" });
  if (command.status !== 0) throw Error(command.stderr);
  const cfg = command.stdout.trim().split("\n").map(line => line.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
  const closures = [
    ...owners.map(dir => ({ dir })),
    { dir: owners[0], test: "../../🧪️tests/🔬️unit/🦀️.rs" },
    { dir: owners.find(dir => dir.endsWith("/🚪️io/📐️geometry"))!, test: "../../🦀️.rs", libraryTests: true },
    { dir: owners[owners.length - 1], test: "🦀️.rs", libraryTests: true },
  ];
  for (const { dir, test, libraryTests } of closures) {
    const manifestDirectory = resolve(workspace, dir, "📦️packages/🦀️rust");
    const manifest = await readFile(join(manifestDirectory, "Cargo.toml"), "utf8");
    const name = /^name = "([^"]+)"/m.exec(manifest)?.[1];
    const library = /^path = "([^"]+[.]rs)"/m.exec(manifest)?.[1] ?? "src/lib.rs";
    const inputs = await readRustSourceClosureV1([resolve(manifestDirectory, test ?? library)], workspace, test ? [...cfg, "test"] : cfg,
      (file: string) => readFile(file).catch(error => { if (error.code === "ENOENT") return null; throw error; }),
      { manifestDirectory, generated: () => undefined }, analysis);
    const lines = inputs.reduce((sum: number, input: {activeLines: number}) => sum + input.activeLines, 0);
    const bytes = inputs.reduce((sum: number, input: {bytes: number}) => sum + input.bytes, 0);
    const stats = { files: inputs.length, lines, bytes };
    const limits = cargoSourceBoundaryExcessV1(sourceBoundary, stats);
    excess ||= limits.length > 0;
    console.log("[DEBUG] " + JSON.stringify({ target, name, unit: test ? (libraryTests ? "library tests" : "integration tests") : "library", ...stats, limits }));
  }
}
if (excess) process.exitCode = 1;
