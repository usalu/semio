#!/usr/bin/env bun
/** 📦️ process process3d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runCmd, runRepositoryTestCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { resolveTestLevel, TEST_LEVEL_BUDGET_MS } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { readdirSync } from "node:fs";
import { resolve } from "node:path";
/** 🛂️ Check the actual persisted Source model and its owning consumers. */
class SourceCheckScript extends BundleScript {
  run(): void {
    const roots: string[] = [];
    const visit = (dir: string): void => {
      for (const item of readdirSync(dir, { withFileTypes: true })) {
        const path = resolve(dir, item.name);
        if (item.isDirectory()) visit(path);
        else if (item.name.endsWith(".ts")) roots.push(path);
      }
    };
    visit(resolve(this.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"));
    runCmd(process.execPath, [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", ...roots], { cwd: this.repoRoot });
  }
}
/** 🪵️ Validate neutral timber replay with the independent Manifold geometry oracle. */
class ReplaySourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments, "quick");
    await runRepositoryTestCommand(process.execPath, ["test", resolve(this.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🟦️.ts"), ...rest], { cwd: this.repoRoot, budgetMs: TEST_LEVEL_BUDGET_MS[level] });
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-process-process3d", { commands: { "check-snapshot-sqlite-source": SourceCheckScript, "test-replay-source": ReplaySourceScript }, snapshotSqliteTestFeatures: [], snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
