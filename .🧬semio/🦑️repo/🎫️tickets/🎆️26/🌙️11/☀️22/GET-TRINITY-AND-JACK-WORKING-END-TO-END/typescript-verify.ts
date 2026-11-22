import { readdirSync, writeFileSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const artifact = process.argv[2]!;
const workspace = "C:/semio";
const root = join(workspace, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts", artifact, "🏅️standards");
const files: string[] = [];
const pending = [root];
while (pending.length) {
  const directory = pending.pop()!;
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory() && entry.name !== "🧪️tests" && entry.name !== "node_modules") pending.push(path);
    else if (entry.isFile() && entry.name.endsWith(".ts")) files.push(path.replaceAll("\\", "/"));
  }
}
const config = join(workspace, `tsconfig.[DEBUG]-verify-${artifact}.json`);
writeFileSync(config, JSON.stringify({ compilerOptions: { noEmit: true, strict: true, target: "ESNext", module: "ESNext", moduleResolution: "bundler", resolveJsonModule: true, allowImportingTsExtensions: true, esModuleInterop: true, skipLibCheck: true }, files }));
const result = spawnSync(process.execPath, ["x", "tsc", "-p", config], { cwd: workspace, stdio: "inherit" });
rmSync(config);
console.log(`checked=${files.length} status=${result.status}`);
process.exit(result.status ?? 1);
