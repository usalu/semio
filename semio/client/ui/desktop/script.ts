#!/usr/bin/env bun
/** 🧭 Desktop app router: `bun ./script.ts test` (integration test runner). */
import { spawn } from "node:child_process";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../repo/lib/js/src/index.ts";

export async function runTests(options: {
  extensionDevelopmentPath?: string;
  extensionTestsPath?: string;
  workspaceFolder?: string;
  launchArgs?: string[];
} = {}) {
  const extensionDevelopmentPath = path.resolve(options.extensionDevelopmentPath ?? import.meta.dir);
  let extensionTestsPath = options.extensionTestsPath;
  if (!extensionTestsPath) {
    const cfgHref = pathToFileURL(path.join(extensionDevelopmentPath, ".semio-test.mjs")).href;
    const cfg = (await import(cfgHref)).default as { files: string };
    extensionTestsPath = path.resolve(extensionDevelopmentPath, cfg.files);
  } else {
    extensionTestsPath = path.resolve(extensionTestsPath);
  }

  const env: NodeJS.ProcessEnv = {
    ...process.env,
    SEMIO_EXTENSION_TESTS_PATH: extensionTestsPath,
    SEMIO_E2E_KIT_FOLDER: process.env.SEMIO_E2E_KIT_FOLDER ?? path.resolve(extensionDevelopmentPath, "../../fixtures/kit/dev/metabolism"),
    SEMIO_E2E_KIT_FILE: process.env.SEMIO_E2E_KIT_FILE ?? path.resolve(extensionDevelopmentPath, "../../fixtures/kit/dev/metabolism/wip/initialKit/kit.semio.json"),
    ELECTRON_DISABLE_SANDBOX: "1",
  };
  if (options.workspaceFolder) {
    env.SEMIO_DESKTOP_WORKSPACE_FOLDER = path.resolve(extensionDevelopmentPath, options.workspaceFolder);
  }

  const extra = options.launchArgs?.length ? options.launchArgs : [];

  await new Promise<void>((resolve, reject) => {
    const child = spawn("bunx", ["electron-forge", "start", ...extra], {
      cwd: extensionDevelopmentPath,
      env,
      stdio: "inherit",
      shell: true,
    });
    child.on("error", reject);
    child.on("exit", (code) => {
      if (code === 0) resolve();
      else reject(new Error(`Desktop integration tests exited with code ${code}`));
    });
  });
}

class TestScript extends BundleScript {
  async run(): Promise<void> {
    await runTests();
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
