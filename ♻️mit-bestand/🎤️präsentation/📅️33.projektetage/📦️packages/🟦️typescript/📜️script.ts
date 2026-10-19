#!/usr/bin/env bun
import {scriptCompilerAdmissionControlV1} from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import {resolveTestLevel} from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";

/** 🧭️ `@semio-tech/mit-bestand-praesentation-projektetage` task router: `bun ./📜️script.ts <dev|build|test> [args…]`. */
import {BundleScript, ScriptRouter} from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import { playgroundDevPortString, playgroundPortEnv, runViteBunxDev, runViteBuild } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";

import {runVitest} from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runViteBunxDev(this.invocation, scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), this.root, segments, {
      runtimeChannels: [],
      config: "../../🏗️builder/🌐️vite/🟦️.ts",
      portEnv: playgroundPortEnv("projektetage"),
      defaultPort: playgroundDevPortString("projektetage"),
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some((arg) => /^(?:--outDir|--config|--root)(?:=|$)/.test(arg))) throw new Error("Build output and configuration are owned by this build target");
    await runViteBuild(this.invocation, scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), this.root, segments, "../../🏗️builder/🌐️vite/🟦️.ts");
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
