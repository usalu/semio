import { join } from "node:path";
import { pathToFileURL } from "node:url";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const load = (path: string) => import(pathToFileURL(join(root, path)).href);
const admin = await load("🌎️hub/🔨️modules/🛡️admin/🧪️tests/🕸️build-graph/🟦️.ts");
let failures = 0;
for (const name of ["verifyAdminEntryGraph", "verifyAdminStylesheetGraph"]) { try { admin[name](join(root, "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript")); console.log(`[DEBUG] PASS ${name}`); } catch (error) { failures++; console.error(`[DEBUG] FAIL ${name}`, error); } }
try { const api = await load("📜️script.ts"); const tests = await load("🧪️tests/🦀️rust-warnings/🟦️.ts"); tests.rustWarningScopeChecks(root, api.rustWarningTargetScope); console.log("[DEBUG] PASS rustWarningScopeChecks"); } catch (error) { failures++; console.error("[DEBUG] FAIL rustWarningScopeChecks", error); }
process.exitCode = failures ? 1 : 0;
