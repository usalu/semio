#!/usr/bin/env bun
/** 🧭 `@ui/styling` task router: `bun ./script.ts <generate|fonts>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../repo/lib/js/src/bundle-script.ts";
import { fetchElementsFonts, generateStylingArtifacts } from "../script.ts";

class GenerateScript extends BundleScript {
  run(): void {
    generateStylingArtifacts();
  }
}

class FontsScript extends BundleScript {
  async run(): Promise<void> {
    await fetchElementsFonts();
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("fonts", FontsScript);

await runBundleScriptMain(router, import.meta.url);
