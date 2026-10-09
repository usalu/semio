#!/usr/bin/env bun
/** 🏢️ BIM model owning SQLite and Rust package commands, plus the repository test-platform roles (`test oracle|subject|parity [level]`). */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {runCmd} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import {BundleScript} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {resolve} from "node:path";
import { runBudgetedTestCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";

const PLATFORM = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts";
const OWNER = "🏙️bim";

/** 📏️ Checks the defining Source schema and owning provider without publishing artifacts. */
class SqliteVerify extends BundleScript { run(args:string[]):void{if(args.length!==1||args[0]!=="source")throw Error("verify-snapshot-sqlite source");const owner=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any");runCmd("bun",[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--skipLibCheck",resolve(owner,"🧬️schema/🟦️.ts"),resolve(owner,"🧬️schema/📸️snapshot/🟦️.ts"),resolve(owner,"🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts"),resolve(owner,"🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts")],{cwd:this.repoRoot});}}

/** 🧪️ Runs one platform role over every case the BIM plugin owns; `--case <slug>` narrows to one. @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts */
function platformRole(role: "oracle" | "subject" | "parity"): new (root: string, repoRoot: string) => BundleScript {
  return class extends BundleScript {
    async run(args: string[]): Promise<void> {
      const { level, rest } = resolveTestLevel(args);
      await runBudgetedTestCommand(process.execPath, [PLATFORM, role, level, "--owner", OWNER, ...rest], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
    }
  };
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-bim-model", {
  commands: { "verify-snapshot-sqlite": SqliteVerify },
  snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],
  testCommands: { oracle: platformRole("oracle"), subject: platformRole("subject"), parity: platformRole("parity") },
});
