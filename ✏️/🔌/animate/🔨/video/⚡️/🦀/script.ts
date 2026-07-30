#!/usr/bin/env bun
/** 🎥 `@semio-tech/animate-video-rs` router: `bun ./script.ts test|render|preview|flush-cache`. */
import { existsSync, rmSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runCmd } from "../../../../../../../🧰/🛍️/🦑/🔨/lib/⚡️/🟦/📦.ts";

const TICKET_TARGET_REL = ".repo/🎫/26/07/18/ANIMATE-VIDEO-CLI-PRESENT-BRIDGE/target";

export type RenderCliOptions = {
  quality: string;
  scene?: string;
  preview: boolean;
  flushCache: boolean;
};

/** 🎛 Parses animate video CLI flags from argv segments. */
export class RenderCliScript extends BundleScript {
  parse(segments: string[]): RenderCliOptions {
    const options: RenderCliOptions = {
      quality: "high",
      preview: false,
      flushCache: false,
    };
    for (let index = 0; index < segments.length; index += 1) {
      const token = segments[index];
      if (token === "--quality") {
        options.quality = segments[index + 1] ?? options.quality;
        index += 1;
        continue;
      }
      if (token === "--scene") {
        options.scene = segments[index + 1];
        index += 1;
        continue;
      }
      if (token === "--preview") {
        options.preview = true;
        continue;
      }
      if (token === "--flush-cache") {
        options.flushCache = true;
      }
    }
    return options;
  }

  cargoEnv(): NodeJS.ProcessEnv {
    return { ...process.env, CARGO_TARGET_DIR: join(this.repoRoot, TICKET_TARGET_REL) };
  }

  qualityPreset(quality: string): string {
    switch (quality.toLowerCase()) {
      case "low":
      case "l":
        return "low";
      case "medium":
      case "m":
        return "medium";
      case "fourk":
      case "4k":
        return "four_k";
      case "production":
      case "p":
        return "production";
      default:
        return "high";
    }
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["animate_core", "animate_video"], this.repoRoot, segments, this.cargoEnv());
  }

  cargoEnv(): NodeJS.ProcessEnv {
    return { ...process.env, CARGO_TARGET_DIR: join(this.repoRoot, TICKET_TARGET_REL) };
  }
}

class RenderScript extends RenderCliScript {
  run(segments: string[]): void {
    const options = this.parse(segments);
    const extra = segments.filter((segment) => !segment.startsWith("--") && segment !== options.quality && segment !== options.scene);
    if (options.flushCache) {
      runCmd("bun", ["./script.ts", "flush-cache"], { cwd: this.root, env: this.cargoEnv() });
    }
    if (options.preview) {
      runCmd("bun", ["./script.ts", "preview", "--quality", options.quality, ...(options.scene ? ["--scene", options.scene] : [])], {
        cwd: this.root,
        env: this.cargoEnv(),
      });
      return;
    }
    const testName = options.scene ? `render_scene_for_hash_${options.scene}` : "render_scene_writes_last_frame";
    runCargoTestBudgeted(
      ["animate_video"],
      this.repoRoot,
      [testName, "--nocapture", "--", `--quality=${options.quality}`, ...extra],
      this.cargoEnv(),
    );
  }
}

class PreviewScript extends RenderCliScript {
  run(segments: string[]): void {
    const options = this.parse(segments);
    const testName = options.scene ? `preview_scene_for_hash_${options.scene}` : "preview_scene_window_metadata_runs";
    runCargoTestBudgeted(
      ["animate_video"],
      this.repoRoot,
      [testName, "--nocapture", "--", `--quality=${options.quality}`],
      this.cargoEnv(),
    );
  }
}

class FlushCacheScript extends RenderCliScript {
  run(): void {
    const partialRoot = join(this.root, "partial_movie_files");
    if (existsSync(partialRoot)) {
      rmSync(partialRoot, { recursive: true, force: true });
    }
    runCargoTestBudgeted(["animate_video"], this.repoRoot, ["lru_evicts_oldest_entry", "--nocapture"], this.cargoEnv());
    console.log(`[animate-video] flushed cache at ${partialRoot}`);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("test", TestScript)
  .register("render", RenderScript)
  .register("preview", PreviewScript)
  .register("flush-cache", FlushCacheScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
