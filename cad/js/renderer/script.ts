#!/usr/bin/env bun
/** 🧭 `@cad/js-renderer-r3f` task router: `bun ./script.ts <dev|build|test|policy> [args…]`. */
import type { FileLinter } from "../../../repo/lib/js/src/linter.ts";
import { dependencyBoundaryBreachesForFile } from "../../../repo/lib/js/src/dependency-boundary.ts";
import { getWorkspaceRoot } from "../../../repo/lib/js/src/cli.ts";
import {
  BundleScript,
  ScriptRouter,
  runBundleScriptMain,
  runViteBuild,
  runViteDev,
  runVitest,
} from "../../../repo/lib/js/src/bundle-script.ts";
import { defineLint } from "../../../repo/lib/js/src/script.ts";

export const policyFile = "index.tsx";

export const policy = defineLint("@cad/js-renderer-r3f-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runViteDev(this.root, segments, {
      config: "play/vite.config.ts",
      portEnv: "SPATIAL_R3F_PLAY_PORT",
      defaultPort: "6020",
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runViteBuild(this.root, segments, "play/vite.config.ts");
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
