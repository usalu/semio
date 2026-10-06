#!/usr/bin/env bun
/** 📦️ gis-gismap Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "inference-client") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("Expected verify inference-client [native]");
      const { proveGisMapInferenceClientV1 } = await import("../../💡️inference/🔌️client/🧪️tests/🟦️.ts");
      proveGisMapInferenceClientV1();
      if (segments[1] === "native") {
        const { runExactCargoLaws } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runExactCargoLaws({ cwd: this.repoRoot, artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
          groups: [{ package: "semio-s-artifact-gis-gismap", target: { kind: "lib" }, laws: ["inference_client::tests::production_manifest_installs_owner_transport_and_schema_vectors", "inference_client::tests::typed_owner_transport_preserves_proposal_binding_and_closed_geometry", "inference_client::tests::owner_reconcile_and_undo_preserve_exact_retry_identity_and_durable_tail"] }],
          buildBudgetMs: 3_600_000, lawBudgetMs: 60_000, progress(event) { console.log("GIS owner client " + event.stage + ": " + (event.law ?? "")); } });
      }
      return;
    }

if (segments[0] === "inference-mcp" || segments[0] === "inference-native-service") {
      const feature = segments[0] === "inference-mcp" ? ["--features", "mcp-service"] : [];
      const filter = segments[0] === "inference-mcp" ? "inference_mcp::tests" : "inference_worker::tests";
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gismap", ...feature, "--lib", filter, "--", "--nocapture"], this.repoRoot);
      return;
    }
if (segments[0] === "gis-map-window-ownership") {
      const schemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config");
      const { testGisMapWindowOwnershipOracle } = await import(`${schemaRoot}/🧪️tests/🔬️window/🟦️.ts`);
      testGisMapWindowOwnershipOracle();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(schemaRoot, "🧬️schema/🟦️.ts"), join(schemaRoot, "🧪️tests/🔬️window/🟦️.ts")], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gismap", "--features", "component-app-assembly", "--lib", "gis_map_window_ownership_", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
if (segments[0] === "map-document-contract") {
      const { testMapDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
      testMapDocumentContractOracle();
      const schemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gismap", "--lib", "map_document_contract", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

if (process.argv[2] === "test") process.env.RUST_MIN_STACK ??= "268435456";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-gis-gismap", { commands: { verify: OwnedVerifyScript }, snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
