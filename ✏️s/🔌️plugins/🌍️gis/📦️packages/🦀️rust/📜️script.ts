#!/usr/bin/env bun
/** 🌍️ GIS plugin package command router. */
import { join } from "node:path";
import { registerPlaygroundSiteBuildCommands, BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd, devToolingEnv, buildBudgetMs } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts";
import { ComponentColdMapPatchCheckScript, ComponentColdMapPatchNativeCheckScript } from "../../🧪️tests/🌉️component-cold-map-patch/🟦️.ts";
import { DurableThreeStoreAssemblyCheckScript, DurableThreeStoreAssemblyNativeCheckScript } from "../../🧪️tests/🗄️durable-three-store-assembly/🟦️.ts";
import { MapCreateRegionGroupCheckScript, MapCreateRegionGroupNativeCheckScript } from "../../🧪️tests/🧩️map-create-region-group/🟦️.ts";
import { NativeCodecCheckScript, proveGisNativeCodecReceipts } from "../../🧪️tests/📇️native-codecs/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await proveGisNativeCodecReceipts(this.repoRoot);
    await runCargoTestBudgeted(["semio-s-plugin-gis"], this.repoRoot, rest);
  }
}

class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-gis", join(this.root, "..", "..")));
  }
}

/** 🧬️ Regenerates this plugin's committed native-codec projection `packSchemaHash` column from the live Rust
 * receipts (`os_pack::schema_hash`); the same test, run without the write mode, is the projection-equals-receipts law. */
class NativeCodecProjectionScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["test", "-p", "semio-s-plugin-gis", "--no-default-features", "--test", "native_codecs", "--", "native_codec_projection_pack_schema_hashes_equal_live_receipts", "--exact"], { cwd: this.repoRoot, env: devToolingEnv({ SEMIO_NATIVE_CODEC_PROJECTION: "write", CARGO_INCREMENTAL: "0" }), budgetMs: buildBudgetMs() });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("native-codec-projection", NativeCodecProjectionScript)
  .register("test", TestScript)
  .register("describe", DescribeScript)
  .register("native-codec-check", NativeCodecCheckScript)
  .register("map-create-region-group-check", MapCreateRegionGroupCheckScript)
  .register("map-create-region-group-native-check", MapCreateRegionGroupNativeCheckScript)
  .register("durable-three-store-assembly-check", DurableThreeStoreAssemblyCheckScript)
  .register("durable-three-store-assembly-native-check", DurableThreeStoreAssemblyNativeCheckScript)
  .register("component-cold-map-patch-check", ComponentColdMapPatchCheckScript)
  .register("component-cold-map-patch-native-check", ComponentColdMapPatchNativeCheckScript);
registerPlaygroundSiteBuildCommands(router);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
