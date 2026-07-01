#!/usr/bin/env bun
/** 🧭 `@semio-tech/cad-js-renderer` task router: `bun ./script.ts <dev|build|test|policy> [fixture <id>] [args…]`. */
import type { FileLinter } from "../../../repo/lib/js/index.ts";
import { dependencyBoundaryBreachesForFile } from "../../../repo/lib/js/index.ts";
import { getWorkspaceRoot } from "../../../repo/lib/js/index.ts";
import {
  BundleScript,
  ScriptRouter,
  consumePlaygroundFixtureArgv,
  playPollingEnv,
  playgroundDevPortString,
  playgroundPortEnv,
  runBun,
  runBundleScriptMain,
  runViteBuild,
  runViteDev,
  runVitest,
} from "../../../repo/lib/js/index.ts";
import { defineLint } from "../../../repo/lib/js/index.ts";
import { resolveCadPlayFixtureSlug } from "./play/fixture-slugs.ts";

export const policyFile = "index.tsx";

export const policy = defineLint("@semio-tech/cad-js-renderer-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

class DevScript extends BundleScript {
  run(segments: string[]): void {
    const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolveCadPlayFixtureSlug);
    Object.assign(process.env, fixtureEnv);
    runViteDev(this.root, viteSegments, {
      config: "play/vite.config.ts",
      portEnv: playgroundPortEnv("cad"),
      defaultPort: playgroundDevPortString("cad"),
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolveCadPlayFixtureSlug);
    runBun(["run", "vite", "build", "--config", "play/vite.config.ts", ...viteSegments], this.root, playPollingEnv(fixtureEnv));
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
