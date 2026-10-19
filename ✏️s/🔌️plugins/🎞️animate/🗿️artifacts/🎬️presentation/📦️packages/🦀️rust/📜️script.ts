#!/usr/bin/env bun
/** 📦️ animate presentation Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runCmd } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runCargo } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runVitest } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "presentation-document-contract") {
      const { runCargo } = await import("../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts");
      const { testPresentationDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
      testPresentationDocumentContractOracle();
      const schemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), ...["--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)]], { cwd: this.repoRoot });
      if (segments[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-animate-presentation", "--lib", "presentation_document_contract", "--", "--nocapture"], this.repoRoot);
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-animate-presentation", { packages: ["semio-s-artifact-animate-presentation", "semio-s-artifact-animate-presentation-animate", "semio-s-artifact-animate-presentation-animation", "semio-s-artifact-animate-presentation-examples", "semio-s-artifact-animate-presentation-host", "semio-s-artifact-animate-presentation-inferences", "semio-s-artifact-animate-presentation-mutations", "semio-s-artifact-animate-presentation-mut-family", "semio-s-artifact-animate-presentation-schema", "semio-s-artifact-animate-presentation-tests", "semio-s-artifact-animate-presentation-tile-grid", "semio-s-artifact-animate-presentation-v1", "semio-s-artifact-animate-presentation-video", "semio-s-artifact-animate-presentation-viewer"], commands: { verify: OwnedVerifyScript }, snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
