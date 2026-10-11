#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ `@semio-tech/mit-bestand-praesentation-projektetage` task router: `bun ./📜️script.ts <dev|build|test> [args…]`. */
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { playgroundDevPortString, playgroundPortEnv } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🟦️.ts";
import { runViteBunxDev, runViteBuild, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runViteBunxDev(this.root, segments, {
      config: "../../🏗️builder/🌐️vite/🟦️.ts",
      portEnv: playgroundPortEnv("projektetage"),
      defaultPort: playgroundDevPortString("projektetage"),
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.some((arg) => /^(?:--outDir|--config|--root)(?:=|$)/.test(arg))) throw new Error("Build output and configuration are owned by this Nx target");
    runViteBuild(this.root, segments, "../../🏗️builder/🌐️vite/🟦️.ts");
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => router.run(process.argv.slice(2), original));
