#!/usr/bin/env bun
/** 📦️ architect program Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "architect-window-ownership") {
      const windowsRoot = join(this.repoRoot, "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows");
      const configRoots = ["📋️register", "↔️adjacency", "🕸️graph", "📓️report"].map((window) => join(windowsRoot, window, "🎚️config"));
      const oracle = join(configRoots[0]!, "🧪️tests/🔬️window-ownership/🟦️.ts");
      const { testArchitectWindowOwnershipOracle } = await import(oracle);
      testArchitectWindowOwnershipOracle();
      runCmd(
        "bun",
        [
          join(this.repoRoot, "node_modules/typescript/bin/tsc"),
          "--noEmit",
          "--strict",
          "--target",
          "ESNext",
          "--module",
          "ESNext",
          "--moduleResolution",
          "bundler",
          "--resolveJsonModule",
          "--allowImportingTsExtensions",
          "--esModuleInterop",
          "--skipLibCheck",
          ...configRoots.map((root) => join(root, "🧬️schema/🟦️.ts")),
          oracle,
        ],
        { cwd: this.repoRoot },
      );
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-architect-program", "--lib", "architect_window_ownership_", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
if (segments[0] === "program-document-contract") {
      const { testProgramDocumentContract } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️document-contract/🟦️.ts");
      testProgramDocumentContract();
      const schemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-architect-program", "--lib", "program_document_contract", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-architect-program", { commands: { verify: OwnedVerifyScript } });
