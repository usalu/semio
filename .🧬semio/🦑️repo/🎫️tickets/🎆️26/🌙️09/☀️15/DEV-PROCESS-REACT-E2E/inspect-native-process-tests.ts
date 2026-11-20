import { existsSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const root = ".cache/b/r/m/u/l";
const testName = /process3d|process3d_bearbeiten|stdio_step|3d_brep_analysis/;
function scan(dir: string, depth: number) {
  if (!existsSync(dir)) return;
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory() && depth > 0 && !["build", "incremental", ".fingerprint"].includes(entry.name)) {
      scan(path, depth - 1);
    } else if (entry.isFile() && testName.test(entry.name) && /^[^.]+(?:\.exe)?$/.test(entry.name)) {
      const stat = statSync(path);
      if (stat.mode & 0o111 || entry.name.endsWith(".exe")) console.log("[DEBUG]", path, stat.mtime.toISOString());
    }
  }
}
scan(root, 5);
