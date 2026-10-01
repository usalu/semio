#!/usr/bin/env bun
/** 📦️ Extension package router: `bun ./📜️script.ts <test|package>`. */
import { brepExtensionRetirementOracle } from "../../🧪️tests/🔬️extension-guest-standalone/🟦️.ts";
import { resolveTestLevel, runCargoTestBudgeted, runExactCargoLaws, runBun, runExtensionComponentPackage } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    console.log(`brep-extension-retirement-oracle cases=${brepExtensionRetirementOracle(import.meta.dir)}`);
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-flow-extension-brep"], this.repoRoot, rest);
  }
}

class MeshOracleScript extends BundleScript {
  async run(segments:string[]):Promise<void> {
    if (segments.length) throw new Error("test-mesh-oracle takes no arguments");
    runBun(["test","../../🥽️mesh/🧪️tests/🔬️unit/🟦️.ts"],import.meta.dir);
  }
}

class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("package writes the Nx-owned deliverable and accepts no output override");
    await runExtensionComponentPackage({ rsDir: import.meta.dir, repoRoot: this.repoRoot });
  }
}

class CanonicalArchitectureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some((segment) => segment !== "--oracle-only")) throw new Error("canonical-architecture accepts only --oracle-only");
    console.log(`brep-extension-retirement-oracle cases=${brepExtensionRetirementOracle(import.meta.dir)}`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
      nativeEnv: { RUST_MIN_STACK: "268435456" },
      groups: [{ package: "semio-s-plugin-flow-extension-brep", target: { kind: "lib" }, laws: [
        "bundle_identity_matches_catalogue_fixture",
        "extension_guest_retires_actual_session_geometry_and_inflight_tessellation",
        "extension_bundle_extends_flow_and_evaluates_box",
      ] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3600000),
      lawBudgetMs: 120000,
      progress(event) { console.log(`brep-extension-retirement ${event.stage}: ${event.law ?? ""}`); },
    });
    console.log(`brep-extension-retirement receipts=${receipts.length}`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("canonical-architecture", CanonicalArchitectureScript).register("test", TestScript).register("test-mesh-oracle", MeshOracleScript).register("package", PackageScript);
await runScriptMain(router, { defaultCommand: "test" });
