import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🦀️rust");

const familyRoot = resolve(SCRIPT_DIR, "../..");
export const familyPackages = (Bun.TOML.parse(readFileSync(join(familyRoot, "Cargo.toml"), "utf8")) as { workspace: { members: string[] } }).workspace.members.map((member) => (Bun.TOML.parse(readFileSync(join(familyRoot, member, "Cargo.toml"), "utf8")) as { package: { name: string } }).package.name);

/** 🏃️ Runs the `SourceCheckScript` command under the caller's roots and invocation. */
export function runSourceCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): void {
  const roots: string[] = [];
  const visit = (dir: string): void => {
    for (const item of readdirSync(dir, { withFileTypes: true })) {
      const path = resolve(dir, item.name);
      if (item.isDirectory()) visit(path);
      else if (item.name.endsWith(".ts")) roots.push(path);
    }
  };
  visit(resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"));
  runCmd(process.execPath, [resolve(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", ...roots], { cwd: context.repoRoot });
}
