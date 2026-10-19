
/** 📦️ gis-gismap Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

if (process.argv[2] === "test") process.env.RUST_MIN_STACK ??= "268435456";
const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-gis-gismap", { packages: ["semio-s-artifact-gis-gismap", "semio-s-artifact-gis-gismap-config", "semio-s-artifact-gis-gismap-mut-family", "semio-s-artifact-gis-gismap-demo-asset", "semio-s-artifact-gis-gismap-gis2d", "semio-s-artifact-gis-gismap-gismap", "semio-s-artifact-gis-gismap-inferences", "semio-s-artifact-gis-gismap-mut-any", "semio-s-artifact-gis-gismap-mutations", "semio-s-artifact-gis-gismap-schema", "semio-s-artifact-gis-gismap-snapshot", "semio-s-artifact-gis-gismap-terminology-tests", "semio-s-artifact-gis-gismap-tests", "semio-s-artifact-gis-gismap-tile-coordinates", "semio-s-artifact-gis-gismap-windows"], commands: router, snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
