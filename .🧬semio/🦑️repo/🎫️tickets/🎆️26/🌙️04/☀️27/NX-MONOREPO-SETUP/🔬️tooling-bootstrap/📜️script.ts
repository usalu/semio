import assert from "node:assert/strict";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
const workspace = process.cwd(), ticket = dirname(import.meta.dirname), tooling = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools";
const root = mkdtempSync(join(ticket, "🗑️generated/tooling-bootstrap-"));
copyFileSync(join(workspace, tooling, "package.json"), join(root, "package.json"));
const manifest = JSON.parse(readFileSync(join(root, "package.json"), "utf8"));
for (const path of Object.values(manifest.patchedDependencies) as string[]) { mkdirSync(dirname(join(root, path)), { recursive: true }); copyFileSync(join(workspace, path), join(root, path)); }
const child = Bun.spawn([process.execPath, "install", "--ignore-scripts"], { cwd: root, stdout: "inherit", stderr: "inherit" });
assert.equal(await child.exited, 0);
copyFileSync(join(root, "bun.lock"), join(workspace, tooling, "bun.lock"));
console.log(`[DEBUG] Isolated pinned Nx tooling acquired in ${root}`);
