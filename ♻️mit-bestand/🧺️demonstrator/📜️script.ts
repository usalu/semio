#!/usr/bin/env bun
/** 🧭️ `@semio-tech/mit-bestand-demonstrator` task router: `bun ./📜️script.ts <test> [args…]`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { demonstratorRuntimeBuildVariants } from "./🔨️modules/🧩️runtime/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "./🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url);

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️demonstratorruntimebuildvariants/🟦️.ts");
  await registerTests1(import.meta.vitest, { demonstratorRuntimeBuildVariants, join }, { directory: import.meta.dir, url: import.meta.url });
  const { DEMONSTRATOR_PANES, demonstratorPaneDescriptionParagraphs } = await import("./🪧️brand.ts");
  const { registerTests1: registerDescriptionTests } = await import("./🧪️tests/🧪️demonstratorpanedescription/🟦️.ts");
  await registerDescriptionTests(import.meta.vitest, { demonstratorPaneDescriptionParagraphs, DEMONSTRATOR_PANES }, { directory: import.meta.dir, url: import.meta.url });
  const { demonstratorGisMapTileServeMode, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR } = await import("./🔨️modules/📦️site/🗺️tile-serve-mode/🟦️.ts");
  const { registerTests1: registerMapTileTests } = await import("./🧪️tests/🧪️demonstratormaptiles/🟦️.ts");
  await registerMapTileTests(import.meta.vitest, { demonstratorGisMapTileServeMode, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR });
  const { registerTests1: registerCompileClosureTests } = await import("./🧪️tests/🧪️demonstratorcompileclosure/🟦️.ts");
  await registerCompileClosureTests(import.meta.vitest);
}
