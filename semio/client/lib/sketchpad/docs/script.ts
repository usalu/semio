#!/usr/bin/env bun
/** 🧭 Sketchpad docs router: `bun ./script.ts dev [vite args…]`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runViteBunxDevPlain } from "../../../../../repo/lib/js/src/bundle-script.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runViteBunxDevPlain(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
