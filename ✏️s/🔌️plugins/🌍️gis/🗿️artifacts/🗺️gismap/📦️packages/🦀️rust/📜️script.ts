#!/usr/bin/env bun
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import { runAdmittedExactCargoLawsV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️laws/🟦️.ts";
/** 📦️ gis-gismap Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runCmd } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runCargo } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runVitest } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
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
        const { runAdmittedExactCargoLawsV1 } = await import("../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts");
        await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 3_600_000, lawMilliseconds: 60_000 },cwd: this.repoRoot,groups: [{ package: "semio-s-artifact-gis-gismap", target: { kind: "lib" }, laws: ["inference_client::tests::production_manifest_installs_owner_transport_and_schema_vectors", "inference_client::tests::typed_owner_transport_preserves_proposal_binding_and_closed_geometry", "inference_client::tests::owner_reconcile_and_undo_preserve_exact_retry_identity_and_durable_tail"] }],progress(event) { console.log("GIS owner client " + event.stage + ": " + (event.law ?? "")); }});
      }
      return;
    }

if (segments[0] === "inference-mcp" || segments[0] === "inference-native-service") {
      const [cargoPackage, filter] = segments[0] === "inference-mcp" ? ["semio-s-artifact-gis-gismap-mcp", "tests"] : ["semio-s-artifact-gis-gismap", "inference_worker::tests"];
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", cargoPackage, "--lib", filter, "--", "--nocapture"], this.repoRoot);
      return;
    }
if (segments[0] === "gis-map-window-ownership") {
      const schemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config");
      const { testGisMapWindowOwnershipOracle } = await import(`${schemaRoot}/🧪️tests/🔬️window/🟦️.ts`);
      testGisMapWindowOwnershipOracle();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(schemaRoot, "🧬️schema/🟦️.ts"), join(schemaRoot, "🧪️tests/🔬️window/🟦️.ts")], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gismap", "--lib", "gis_map_window_ownership_", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
if (segments[0] === "map-document-contract") {
      await runRepositoryTestCommand(process.execPath, ["test", resolve(this.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts")], { cwd: this.repoRoot });
      const schemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gismap", "--lib", "map_document_contract", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

if (process.argv[2] === "test") process.env.RUST_MIN_STACK ??= "268435456";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-gis-gismap", { packages: ["semio-s-artifact-gis-gismap", "semio-s-artifact-gis-gismap-config", "semio-s-artifact-gis-gismap-mut-family", "semio-s-artifact-gis-gismap-demo-asset", "semio-s-artifact-gis-gismap-gis2d", "semio-s-artifact-gis-gismap-gismap", "semio-s-artifact-gis-gismap-inferences", "semio-s-artifact-gis-gismap-mut-any", "semio-s-artifact-gis-gismap-mutations", "semio-s-artifact-gis-gismap-schema", "semio-s-artifact-gis-gismap-snapshot", "semio-s-artifact-gis-gismap-terminology-tests", "semio-s-artifact-gis-gismap-tests", "semio-s-artifact-gis-gismap-tile-coordinates", "semio-s-artifact-gis-gismap-windows"], commands: { verify: OwnedVerifyScript }, snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
