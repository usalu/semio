#!/usr/bin/env bun
/** 🧩️ Puzzle playground relayer ticket router. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../repo/lib/js/index.ts";

const tasks: Record<string, string> = {
  "rename-scopes": "./rename-scopes.ts",
  "clean-react-imports": "./clean-react-imports.ts",
  "complete-relayer": "./complete-relayer.ts",
  "extract-play-hosts": "./extract-play-hosts.ts",
  "fix-2d-merge": "./fix-2d-merge.ts",
  "fix-3d-merge": "./fix-3d-merge.ts",
  "fix-host-imports": "./fix-host-imports.ts",
  "merge-host-back": "./merge-host-back.ts",
  "move-hosts-to-renderer": "./move-hosts-to-renderer.ts",
  "split-play-host": "./split-play-host.ts",
};

const router = new ScriptRouter(import.meta.dir);
for (const [name, modulePath] of Object.entries(tasks)) {
  router.register(
    name,
    class extends BundleScript {
      async run(): Promise<void> {
        await import(modulePath);
      }
    },
  );
}

await runBundleScriptMain(router, import.meta.url);
