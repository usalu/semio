import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";

/** 🛂️ Regenerates the committed `stdio-pdf` semantic descriptor pair from a built component with the published descriptor emitter (`semio-plugin-describe describe`, the executable `components describe` runs) and settles it in `write` mode, exactly as `repo run workspace:components -- describe stdio-pdf` does after its own cargo build, which this ticket's memory gate forbids.
 * Usage: `bun regen-pdf-descriptor.ts <component.wasm>` from the workspace root.
 * @see ./🌐️pdf-browser-journey.md */

const root = process.cwd();
const [component] = process.argv.slice(2) as [string];
const describe = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe");
const { componentStageTargetV1, settleComponentDescriptorV1 } = await import(join(describe, "🧩️component-stage/🟦️.ts"));
const { verifySemanticDescriptorPairBytesV1 } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts"));
const emitter = join(root, ".cache/b/p/m/browser/prototype/tools/descriptor-emitter/darwin/semio-plugin-describe");
const target = componentStageTargetV1(root, "🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/📦️packages/🦀️rust/Cargo.toml");
const out = mkdtempSync(join(import.meta.dir, "🗑️generated", "describe-"));
try {
  const run = Bun.spawnSync([emitter, "describe", component, "--out", out], { cwd: root, stdout: "inherit", stderr: "inherit" });
  if (run.exitCode !== 0) throw new Error(`descriptor emitter failed with ${run.exitCode}`);
  const jsonBytes = readFileSync(join(out, "🔣️.json")), packBytes = readFileSync(join(out, "🛂️.descriptor.semio"));
  const pair = verifySemanticDescriptorPairBytesV1(jsonBytes, packBytes);
  if (pair.pluginId !== target.pluginId) throw new Error(`the component describes ${pair.pluginId}, not ${target.pluginId}`);
  const drift = settleComponentDescriptorV1(root, target, { profile: "prototype", directory: out }, { jsonBytes, packBytes }, "write");
  console.log(drift.length ? `rewrote ${target.ownerRoot}:\n  ${drift.join("\n  ")}` : "committed descriptor already current");
} finally {
  rmSync(out, { recursive: true, force: true });
}
