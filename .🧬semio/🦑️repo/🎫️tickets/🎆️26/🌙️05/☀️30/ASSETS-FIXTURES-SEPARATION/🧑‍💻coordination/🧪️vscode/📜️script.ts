import { mkdirSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, generated = process.env.SEMIO_FIXTURE_GENERATED!;
mkdirSync(generated, { recursive: true });
let failed = 0;
for (const [test, name] of [["🧩️host-build", "testExtensionHostBuild"], ["📦️package", "testExtensionPackage"]]) {
  try {
    const api = await import(pathToFileURL(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests", test!, "🟦️.ts")).href);
    await api[name!](generated);
    console.log(`[DEBUG] PASS ${name}`);
  } catch (error) { failed++; console.error(`[DEBUG] FAIL ${name}`, error); }
}
process.exitCode = failed ? 1 : 0;
