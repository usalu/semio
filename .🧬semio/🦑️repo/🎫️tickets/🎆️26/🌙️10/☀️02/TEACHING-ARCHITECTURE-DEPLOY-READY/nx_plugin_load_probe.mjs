// Loads the repo's Nx inference plugin the way Nx's `handleImport` does (`require` first; a module with top-level await
// falls back to `import(<absolute path>)`, which Node refuses on Windows) and reads one command's import closure.
import { createRequire } from "node:module";
import { resolve } from "node:path";

const require = createRequire(import.meta.url);
const plugin = resolve(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs");
try {
  const loaded = require(plugin);
  console.log("[DEBUG] required", Object.keys(loaded));
  const entry = resolve(process.cwd(), "🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📜️script.ts");
  console.log("[DEBUG] sync closure", loaded.cacheInternals.relativeScriptInputs([entry], process.cwd()).length);
  await loaded.libraryBootstrap;
  console.log("[DEBUG] bootstrapped closure", loaded.cacheInternals.relativeScriptInputs([entry], process.cwd()).length);
} catch (error) {
  console.log("[DEBUG] failed", error?.code, error?.stack ?? error);
  process.exitCode = 1;
}
