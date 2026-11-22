import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = process.cwd();
const interpreter = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter");
const manifestDirectory = resolve(interpreter, "📦️packages/🦀️rust");
const targets = [
  ["macos", "aarch64", "unix"],
  ["linux", "x86_64", "unix"],
  ["windows", "x86_64", "windows"],
];

for (const [os, arch, family] of targets) {
  const cfg = ["debug_assertions", family, `target_os=${os}`, `target_arch=${arch}`, `target_family=${family}`, "target_pointer_width=64"];
  const inputs = await readRustSourceClosureV1([resolve(interpreter, "🦀️.rs")], root, cfg, async path => {
    try {
      return await readFile(path);
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
      throw error;
    }
  }, { manifestDirectory, generated: () => undefined });
  const bytes = inputs.reduce((sum, item) => sum + item.bytes, 0);
  const activeLines = inputs.reduce((sum, item) => sum + item.activeLines, 0);
  const diagnostics = inputs.some(item => item.path.includes("📊️trace"));
  console.log("[DEBUG]", JSON.stringify({ os, arch, bytes, files: inputs.length, activeLines, diagnostics }));
  if (bytes > 262_144 || inputs.length > 32 || diagnostics) process.exitCode = 1;
}
