import type { ScriptInvocation } from "@semio-tech/process/routing";
import { scriptCompilerAdmissionControlV1 } from "@semio-tech/process/compiler-capacity";
import { frameworkOsViteArtifactInputsV1 } from "@semio-tech/framework-os-dev/builder/vite/artifacts";
import { repositoryCachePartitionRootV1 } from "@semio-tech/repo-lib/storage";
import { readBuildMode } from "@semio-tech/process/build-mode";
import { join, resolve } from "node:path";
import { buildViteArtifact } from "@semio-tech/repo-lib/dev";
import { prefetchDemonstratorMapTiles } from "./🗺️map-tiles/🟦️.ts";

/** 🏃️ Runs the `BuildScript` command under the caller's roots and invocation. */
export async function runBuildScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  if (args.length) throw new Error("The Demonstrator build accepts no compiler arguments");
  const root = resolve(context.root, "../.."), controller = new AbortController();
  const cancel = (): void => { controller.abort(); process.exitCode = 130; };
  process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
  try {
    await prefetchDemonstratorMapTiles(context.repoRoot);
    await buildViteArtifact({control:scriptCompilerAdmissionControlV1(context.invocation,"vite:entry",0), invocation: context.invocation, mode: readBuildMode(process.env.SEMIO_BUILD_MODE), artifactInputs: frameworkOsViteArtifactInputsV1(repositoryCachePartitionRootV1(context.repoRoot, "products"), {version:1,mode:readBuildMode(process.env.SEMIO_BUILD_MODE),renderer:"react",command:"build",engines:"play"}), root, workspace: context.repoRoot, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), configLoader: "native", owner: "demonstrator:site", signal: context.invocation.control.signal, environment: { SEMIO_BUILD_MODE: readBuildMode(process.env.SEMIO_BUILD_MODE), SEMIO_RENDERER: "react", GIS_MAP_TILE_SERVE_MODE: "bundle" } });
  } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
}
