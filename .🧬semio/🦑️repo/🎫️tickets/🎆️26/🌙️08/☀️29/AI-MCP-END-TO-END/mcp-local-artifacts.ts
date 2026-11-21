import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

if (process.argv.includes("--log")) {
  const log = readFileSync(join(import.meta.dir, "🗑️generated/native-retry.log"), "utf8");
  const lines = log.split("\n");
  for (const line of lines.filter(line => /^(?:\s+Compiling|\s+Finished)/u.test(line)).slice(-8)) console.log("[DEBUG]", line.trim());
  for (const [index, line] of lines.entries()) if (/^error(?:\[|:)/u.test(line)) console.log("[DEBUG]", lines.slice(index, index + 10).join("\n"));
  process.exit(0);
}

for (const root of [".cache/b/r/m/u/l", "target", "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist", "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"]) {
  const visit = (directory: string, depth: number) => {
    if (!existsSync(directory)) return;
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isFile() && /^(?:semio-os-mcp(?:\.exe)?|🦀️catalog\.json)$/.test(entry.name)) {
        const info = statSync(path);
        console.log("[DEBUG]", path, info.mtime.toISOString(), info.size);
      } else if (entry.isDirectory() && depth > 0 && !["deps", "build", "incremental", ".fingerprint", "debug", "release"].includes(entry.name)) visit(path, depth - 1);
    }
    for (const profile of ["build", "debug", "release"]) {
      const path = join(directory, profile, "semio-os-mcp");
      if (existsSync(path)) {
        const info = statSync(path);
        console.log("[DEBUG]", path, info.mtime.toISOString(), info.size);
      }
    }
  };
  visit(root, 3);
}
