#!/usr/bin/env bun
/** 📦️ sequence sequence Rust artifact package router. */
import { resolve } from "node:path";
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { sequenceSnapshotFixtureAssetSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️snapshot-fixture-asset/🟦️.ts";
import { testArtifactIoDescriptorParity } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️descriptor-parity/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "sequence-window-ownership") {
      const windowsRoot = join(this.repoRoot, "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows");
      const configRoot = join(windowsRoot, "📽️main/🎚️config");
      const { testSequenceWindowOwnershipOracle } = await import(`${configRoot}/🧪️tests/🔬️window/🟦️.ts`);
      testSequenceWindowOwnershipOracle();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(configRoot, "🧬️schema/🟦️.ts"), join(windowsRoot, "📜️script/🫧️transient/🧬️schema/🟦️.ts"), join(configRoot, "🧪️tests/🔬️window/🟦️.ts")], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-sequence-sequence", "--lib", "sequence_window_ownership_", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

const segments = process.argv.slice(2);
if (segments[0] === "test" && segments[1] === "io-descriptor-parity") {
  testArtifactIoDescriptorParity();
  if (segments[2] !== "oracle") {
    const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-sequence-sequence", "--lib", "artifact_io_", "--", "--nocapture"], resolve(import.meta.dir, "../../../../../../.."));
  }
} else await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-sequence-sequence", { ...{
  twins: [{ name: "sequence-snapshot-fixture-asset", run: sequenceSnapshotFixtureAssetSelfTests }, { name: "artifact-io-descriptor-parity", run: testArtifactIoDescriptorParity }],
}, commands: { verify: OwnedVerifyScript } });
