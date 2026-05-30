#!/usr/bin/env bun
/** 🧭 Sketchpad play router: `bun ./script.ts dev|build [vite args…]`. */
import {
  BundleScript,
  ScriptRouter,
  runBundleScriptMain,
  runViteBunxDevPlain,
  runViteBuild,
} from "../../../../../repo/lib/js/src/index.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runViteBunxDevPlain(this.root, segments);
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runViteBuild(this.root, segments, "vite.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
