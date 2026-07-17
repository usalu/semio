#!/usr/bin/env bun
/** 🎥 `@semio-tech/animate-video-rs` router: `bun ./script.ts test|render`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["animate_core", "animate_video"], this.repoRoot, segments);
  }
}

class RenderScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["animate_video"], this.repoRoot, ["render_scene_writes_last_frame", "--nocapture", ...segments]);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("render", RenderScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
