#!/usr/bin/env bun
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { buildViteArtifact } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";

/** 🎪️ Publishes the Demonstrator using prerequisites selected by the outer Nx graph. */
class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The Demonstrator build accepts no compiler arguments");
    const root = resolve(this.root, "../.."), controller = new AbortController();
    const cancel = (): void => { controller.abort(); process.exitCode = 130; };
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      await buildViteArtifact({ root, workspace: this.repoRoot, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), output: join(root, "dist/site"), owner: "demonstrator:site", signal: controller.signal, environment: { SEMIO_BUILD_MODE: "ship", SEMIO_RENDERER: "react" }, ...(process.env.SEMIO_TICKET_DIR ? { temporaryRoot: join(process.env.SEMIO_TICKET_DIR, "🗑️generated") } : {}) });
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript);
if (import.meta.main) await router.run(process.argv.slice(2));
