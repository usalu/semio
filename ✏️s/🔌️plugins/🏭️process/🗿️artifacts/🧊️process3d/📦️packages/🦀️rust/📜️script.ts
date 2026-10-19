#!/usr/bin/env bun
/** 📦️ process process3d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel, TEST_LEVEL_BUDGET_MS } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { resolve } from "node:path";
import { runSourceCheckScript, familyPackages } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🛂️ Check the actual persisted Source model and its owning consumers. */
class SourceCheckScript extends BundleScript {
  async run(): Promise<void> {
    await runSourceCheckScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}
/** 🪵️ Validate neutral timber replay with the independent Manifold geometry oracle. */
class ReplaySourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments, "quick");
    await runRepositoryTestCommand(process.execPath, ["test", resolve(this.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🟦️.ts"), ...rest], { cwd: this.repoRoot, budgetMs: TEST_LEVEL_BUDGET_MS[level] });
  }
}
const router = new ScriptRouter(import.meta.dir).register("check-snapshot-sqlite-source", SourceCheckScript).register("test-replay-source", ReplaySourceScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-process-process3d", { packages: familyPackages, commands: router, snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
