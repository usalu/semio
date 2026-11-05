import { readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";

const base = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/";
const owners = [
  ["framework-headless", "🧰️framework/🔨️modules/🧵️job/🏃️headless/📦️packages/🦀️rust/🦀️.rs"],
  ["engine-job", "✏️s/🔌️plugins/🀄️wfc/⚙️engine/💼️job/📦️packages/🦀️rust/🦀️.rs"],
  ["inferences", base + "🧬️schema/💡️inferences/📦️packages/🦀️rust/🦀️.rs"],
  ["editor", base + "✏️editor/📦️packages/🦀️rust/🦀️.rs"],
  ["public-tests", base + "✏️editor/🧪️tests/🔬️unit/📦️packages/🦀️rust/🦀️.rs"],
  ["pin-solution", base + "✏️editor/🎮️commands/📌️pin-solution/📦️packages/🦀️rust/🦀️.rs"],
] as const;
for (const [owner, entry] of owners) {
  const inputs = await readRustSourceClosureV1([entry], process.cwd(), ["test"], async path => {
    try { return new Uint8Array(await readFile(path)); }
    catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
  }, { manifestDirectory: dirname(resolve(entry)), generated: () => undefined });
  console.log(JSON.stringify({ owner, cfg: ["test"], authoritativeNativeGate: false, upperBound: false, files: inputs.length, bytes: inputs.reduce((sum, input) => sum + input.bytes, 0), activeLines: inputs.reduce((sum, input) => sum + input.activeLines, 0), pinUnitMounted: inputs.some(input => input.path.endsWith("📌️pin-solution/🧪️tests/🔬️unit/🦀️.rs")) }));
}
