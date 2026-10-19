#!/usr/bin/env bun
import {scriptCompilerAdmissionControlV1} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {frameworkOsViteArtifactInputsV1} from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🏗️builder/🌐️vite/📥️artifacts/🟦️.ts";
import { repositoryCachePartitionRootV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import { buildModeEnvironment, readBuildMode } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { buildViteArtifact } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";
import { prefetchDemonstratorMapTiles } from "./🗺️map-tiles/🟦️.ts";

/** 🎪️ Publishes the Demonstrator using prerequisites selected by the outer build graph. */
class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The Demonstrator build accepts no compiler arguments");
    const root = resolve(this.root, "../.."), controller = new AbortController();
    const cancel = (): void => { controller.abort(); process.exitCode = 130; };
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      await prefetchDemonstratorMapTiles(this.repoRoot);
      await buildViteArtifact({control:scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), invocation: this.invocation, mode: readBuildMode(process.env.SEMIO_BUILD_MODE), artifactInputs: frameworkOsViteArtifactInputsV1(repositoryCachePartitionRootV1(this.repoRoot, "products"), {version:1,mode:readBuildMode(process.env.SEMIO_BUILD_MODE),renderer:"react",command:"build",engines:"play"}), root, workspace: this.repoRoot, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), owner: "demonstrator:site", signal: this.invocation.control.signal, environment: { SEMIO_BUILD_MODE: readBuildMode(process.env.SEMIO_BUILD_MODE), SEMIO_RENDERER: "react", GIS_MAP_TILE_SERVE_MODE: "bundle" } });
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
