#!/usr/bin/env bun
/** 📦️ json Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {prepareRepositoryCargoCommand,runCargoSystemOperation,cargoCommandOwner} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import { devToolingEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import { acquireCargoBuildLeaseV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import { repoCacheDirectory } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import { lstatSync, mkdirSync, realpathSync } from "node:fs";

/** 🧬️ Verifies the actual native codec graph under a caller-owned compiler cache. */
class NativeSchemaScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifacts || !isAbsolute(artifacts) || segments.length > 1) throw new Error("Native JSON schema proof requires an absolute caller artifact directory and at most one copied workspace");
    mkdirSync(artifacts, { recursive: true });
    const workspace = segments[0] ? realpathSync(segments[0]) : this.repoRoot;
    if (segments[0]) {
      const local = relative(realpathSync(artifacts), workspace);
      if (!local || isAbsolute(local) || local === ".." || local.startsWith(".." + sep) || workspace === this.repoRoot || lstatSync(segments[0]).isSymbolicLink()) throw new Error("Native JSON copied workspace must be contained in caller artifacts");
    }
    const manifest = resolve(workspace, relative(this.repoRoot, this.root), "Cargo.toml");
    const target = segments[0] ? join(workspace, "target") : join(artifacts, "json-native-target");
    const env = devToolingEnv({ NX_WORKSPACE_ROOT: workspace, CARGO_TARGET_DIR: target, CARGO_BUILD_BUILD_DIR: join(target, "intermediate") });
    const args = ["test", "--offline", "--manifest-path", manifest, "-p", "semio-s-artifact-stdio-json", "--lib"];
    const controller = new AbortController(), abort = () => controller.abort(), budget = buildBudgetMs() || 1_200_000, timer = setTimeout(abort, budget);
    process.once("SIGINT", abort); process.once("SIGTERM", abort);
    let lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined;
    try {
      await runCargoSystemOperation(workspace,budget,async()=>{const owner=cargoCommandOwner(),cancel=()=>owner.cancelDiscovery();controller.signal.addEventListener("abort",cancel,{once:true});if(controller.signal.aborted)cancel();try{await prepareRepositoryCargoCommand(workspace,args,workspace,env);}finally{controller.signal.removeEventListener("abort",cancel);}});
      lease = await acquireCargoBuildLeaseV1({ directory: repoCacheDirectory(workspace, "agents", "resource-leases"), buildDirectory: target, args, signal: controller.signal });
      for (const law of ["standards::v_rfc8259::subsets::base::schema::snapshot::component::sqlite_tests::sqlite_snapshot_json_owned_record_identity_matches_handwritten_definition"]) {
        let observed = false;
        await runOwnedCommand("cargo", [...args, law, "--", "--exact", "--nocapture"], workspace, "json-native-schema", budget, { env, signal: controller.signal, onLine: line => { if (line.includes(`test ${law} ... ok`)) observed = true; } });
        if (!observed) throw new Error(`Native JSON law did not execute: ${law}`);
      }
    } finally {
      clearTimeout(timer); process.off("SIGINT", abort); process.off("SIGTERM", abort); if (lease) await lease.release();
    }
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-json", { commands: { "native-schema-check": NativeSchemaScript }, testFeatures: ["component-app-assembly"], snapshotSqliteTests: ["../../🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🧬️schema/🧪️tests/🪶️sqlite/🟦️.ts", "../../🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🧪️tests/🪶️sqlite/🟦️.ts","../../🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
