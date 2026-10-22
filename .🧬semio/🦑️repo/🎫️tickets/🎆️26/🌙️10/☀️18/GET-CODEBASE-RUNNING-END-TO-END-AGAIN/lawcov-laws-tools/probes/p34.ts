import { mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { interactivityAuditRun } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts";
const root = mkdtempSync(join(realpathSync(tmpdir()), "p34-"));
writeFileSync(join(root, "README.md"), "x");
const lanes = new Proxy({}, { get: () => async () => 0 }) as any;
const t = Date.now();
try { const r = interactivityAuditRun(root, lanes); console.log(Date.now() - t, "ms", r.findings.length, JSON.stringify(r.byCategory), r.blockingBridgeUnlisted.length, r.staleAllowlistEntries.length, r.expectedNeverToMatchEntries.length, r.preDeclaredOutOfScope.length); console.log(JSON.stringify(r.findings.slice(0, 3))); } catch (e) { console.log("ERR", String(e).slice(0, 500)); }
rmSync(root, { recursive: true, force: true });
