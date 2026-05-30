#!/usr/bin/env bun
/** 🧭 Coordinator package router: `bun ./script.ts build`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../lib/js/src/index.ts";

class BuildScript extends BundleScript {
  run(): void {
    const ext = process.platform === "win32" ? ".exe" : "";
    execFileSync("go", ["build", "-o", `server${ext}`, "."], { cwd: this.root, stdio: "inherit" });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
