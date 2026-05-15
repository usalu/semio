#!/usr/bin/env bun
// #region DesktopTestRunner
// Launcher mirroring @vscode/test-electron `runTests`: spawns the desktop app with SEMIO_EXTENSION_TESTS_PATH set.
// Usage: from `semio/client/ui/desktop`: `bun ./test/test.script.ts`
// Specs: See https://code.visualstudio.com/api/working-with-extensions/testing-extension

import { spawn } from "node:child_process";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const defaultDesktopRoot = path.resolve(__dirname, "..");

/**
 * @param {{
 *   extensionDevelopmentPath?: string;
 *   extensionTestsPath?: string;
 *   workspaceFolder?: string;
 *   launchArgs?: string[];
 * }} [options] — same shape as `runTests` from @vscode/test-electron; `extensionTestsPath` overrides `.semio-test.mjs` `files`.
 */
export async function runTests(options: {
  extensionDevelopmentPath?: string;
  extensionTestsPath?: string;
  workspaceFolder?: string;
  launchArgs?: string[];
} = {}) {
  const extensionDevelopmentPath = path.resolve(options.extensionDevelopmentPath ?? defaultDesktopRoot);
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
    SEMIO_E2E_KIT_FOLDER: process.env.SEMIO_E2E_KIT_FOLDER ?? path.resolve(extensionDevelopmentPath, "../assets/semio/metabolism"),
    SEMIO_E2E_KIT_FILE: process.env.SEMIO_E2E_KIT_FILE ?? path.resolve(extensionDevelopmentPath, "../assets/semio/metabolism.kit.semio.json"),
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

const invokedDirectly =
  process.argv[1] &&
  pathToFileURL(path.resolve(process.argv[1])).href === pathToFileURL(fileURLToPath(import.meta.url)).href;
if (invokedDirectly) {
  runTests().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}

// #endregion DesktopTestRunner
