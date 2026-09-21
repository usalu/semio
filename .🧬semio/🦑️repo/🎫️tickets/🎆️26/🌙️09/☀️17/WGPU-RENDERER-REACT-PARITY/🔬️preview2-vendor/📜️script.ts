/** 🗣️ Runs the installed Preview2 guest-log vendoring law through the workspace task runner. */
import { join } from "node:path";

const repoRoot = process.cwd();
const laws = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts"));
if (process.argv[2] === "full") await laws.testBrowserWasiActivation(repoRoot);
else await laws.testPreview2GuestLogVendoring(repoRoot);
