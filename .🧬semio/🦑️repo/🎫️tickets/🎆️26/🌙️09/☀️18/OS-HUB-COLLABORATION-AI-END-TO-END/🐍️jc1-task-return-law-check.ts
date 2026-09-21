/** ⚖️ Runs the async task.return refusal against both real generated actors and the published closed bundle. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const { validateAsyncTaskReturnLift } = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts"));

const rows = process.argv.slice(2).map(path => {
  const source = readFileSync(path, "utf8");
  try {
    return { path, verdict: "admitted", taskReturnBindings: validateAsyncTaskReturnLift(source) };
  } catch (error) {
    return { path, verdict: "refused", reason: String((error as Error).message) };
  }
});
console.log(JSON.stringify(rows, null, 2));
