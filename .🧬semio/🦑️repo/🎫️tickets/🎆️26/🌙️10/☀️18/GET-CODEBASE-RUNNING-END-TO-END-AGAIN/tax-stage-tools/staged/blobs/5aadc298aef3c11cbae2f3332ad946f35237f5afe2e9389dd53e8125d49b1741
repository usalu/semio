#!/usr/bin/env bun
import { InactiveCargoRetirementScript } from "./🧹️pruning/🏪️budget/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { AuditScript, PolicyScript } from "./📇️inventory/📋️orchestration/🟦️.ts";
import { GraphScript } from "./🕸️graph/✅️verification/🟦️.ts";
import { DiskScript } from "./💾️storage/📊️report/🟦️.ts";
import { DoctorScript } from "./🩺️environment/📋️inspection/🟦️.ts";
import { CacheVerifyScript } from "./🔁️verification/📋️orchestration/🟦️.ts";
import { CachePruneScript, CacheReportScript } from "./🧹️pruning/📋️orchestration/🟦️.ts";
import { CargoProvenanceScript } from "./🦀️cargo/🧾️provenance/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { SCRIPT_ROOT, TestScript, LeaseScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";


/** 🧪️ Loads the artifact package proof only for its explicit verification command. */
class ArtifactPackageVerificationScript extends BundleScript {
  async run(): Promise<void> {
    const { ArtifactPackageContractScript } = await import("./🧪️tests/📦️artifact-packages/🟦️.ts");
    await new ArtifactPackageContractScript(this.root, this.repoRoot, this.invocation).run();
  }
}

const router = new ScriptRouter(SCRIPT_ROOT)
  .register("test", TestScript)
  .register("lease", LeaseScript)
  .register("audit", AuditScript)
  .register("policy-check", PolicyScript)
  .register("artifact-check", PolicyScript)
  .register("artifact-package-contract", ArtifactPackageVerificationScript)
  .register("graph-check", GraphScript)
  .register("doctor", DoctorScript)
  .register("disk-report", DiskScript)
  .register("cache-verify", CacheVerifyScript)
  .register("cache-report", CacheReportScript)
  .register("cache-prune", CachePruneScript)
  .register("cache-retire-inactive", InactiveCargoRetirementScript)
  .register("cargo-provenance", CargoProvenanceScript);

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));