import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync, copyFileSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const root = join(ticket, "../../../../../../..");
const storage = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage";
const work = join(ticket, "🗑️generated/mutation"), tests = join(work, "🧪️tests/🆔️identity");
rmSync(work, { recursive: true, force: true });
mkdirSync(tests, { recursive: true });
writeFileSync(join(work, "🟦️.ts"), execFileSync("git", ["show", "HEAD:" + storage + "/🟦️.ts"], { cwd: root, maxBuffer: 1 << 26 }));
for (const name of ["🟦️.ts", "🔣️.json"]) copyFileSync(join(root, storage, "🧪️tests/🆔️identity", name), join(tests, name));
try { console.log("[DEBUG]", execFileSync("bun", ["test", "./🧪️tests/🆔️identity/🟦️.ts"], { cwd: work, encoding: "utf8", stdio: "pipe" })); }
catch (error) { console.log("[DEBUG] original fails as expected:", String((error as { stderr?: string }).stderr).match(/"line": \d+/g)); }
