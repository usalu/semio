#!/usr/bin/env bun
import { buildModeEnvironment, readBuildMode } from "../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { buildViteArtifact } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { prefetchDemonstratorMapTiles } from "./🗺️map-tiles/🟦️.ts";

/** 🎪️ Publishes the Demonstrator using prerequisites selected by the outer Nx graph. */
class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The Demonstrator build accepts no compiler arguments");
    const root = resolve(this.root, "../.."), controller = new AbortController();
    const cancel = (): void => { controller.abort(); process.exitCode = 130; };
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      await prefetchDemonstratorMapTiles(this.repoRoot);
      await buildViteArtifact({ root, workspace: this.repoRoot, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), output: join(root, "dist", readBuildMode(process.env.SEMIO_BUILD_MODE), "site"), owner: "demonstrator:site", signal: controller.signal, environment: { SEMIO_BUILD_MODE: readBuildMode(process.env.SEMIO_BUILD_MODE), SEMIO_RENDERER: "react", GIS_MAP_TILE_SERVE_MODE: "bundle" }, ...(process.env.SEMIO_TICKET_DIR ? { temporaryRoot: join(process.env.SEMIO_TICKET_DIR, "🗑️generated") } : {}) });
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
