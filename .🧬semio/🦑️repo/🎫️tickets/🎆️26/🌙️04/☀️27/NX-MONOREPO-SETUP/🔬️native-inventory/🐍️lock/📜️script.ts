import { mkdirSync, mkdtempSync, copyFileSync } from "node:fs";
import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!, output = process.env.SEMIO_TEST_ARTIFACT_DIR!;
const source = join(root, "🧰️framework/🔨️modules/🖱️ui/🎨️styling", "📦️packages/🐍️python"), temporary = mkdtempSync(join(output, "styling-python-lock-"));
copyFileSync(join(source, "pyproject.toml"), join(temporary, "pyproject.toml"));
const child = Bun.spawn(["uv", "lock", "--project", temporary], { stdout: "inherit", stderr: "inherit" });
if (await child.exited !== 0) throw new Error("Lock generation failed");
copyFileSync(join(temporary, "uv.lock"), join(source, "uv.lock"));
console.log("[DEBUG] Locked Python build tools without installing a shared environment");
