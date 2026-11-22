import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { measureRustCrate, measureFits } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
import { CRATE_HARD_BUDGET } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/💰️budget/🟦️.ts";

const workspace = process.cwd();
const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const toolchain = /^channel\s*=\s*"([^"]+)"/m.exec(readFileSync("rust-toolchain.toml", "utf8"))?.[1];
const registry = Bun.spawnSync(["repo", "tools", "list", "--json"]);
assert.equal(registry.exitCode, 0);
const rustup = JSON.parse(registry.stdout.toString()).find((row: { id: string }) => row.id === "rustup")?.command;
assert(rustup && toolchain);
const crates = [
  "🧠️interpreter",
  "🧠️interpreter/🚪️io/📞️imports",
  "🧠️interpreter/🏃️runtime",
  "🖥️host/🔀️post-turn-relay",
];
let failed = false;
for (const target of ["aarch64-apple-darwin", "x86_64-pc-windows-msvc", "x86_64-unknown-linux-gnu"]) {
  const cfg = Bun.spawnSync([rustup, "run", toolchain, "rustc", "--print", "cfg", "--target", target]);
  assert.equal(cfg.exitCode, 0, cfg.stderr.toString());
  const shape = [...cfg.stdout.toString().trim().split("\n").map(row =>
    row.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted))
  ), "debug_assertions"];
  for (const crate of crates) {
    const directory = resolve(workspace, plugin, crate);
    const measure = await measureRustCrate(workspace, resolve(directory, "🦀️.rs"), false, new Map(), resolve(directory, "📦️packages", "🦀️rust"), shape);
    const fits = measureFits(measure, CRATE_HARD_BUDGET);
    console.log("[DEBUG] production scanner", JSON.stringify({ crate, target, files: measure.files, bytes: measure.bytes, lines: measure.lines, fits }));
    failed ||= !fits;
  }
}
if (failed) process.exitCode = 1;
