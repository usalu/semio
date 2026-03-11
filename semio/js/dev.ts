// #region 🔖Header
// [👤semio📚js💻dev](semiorepo://p/u/semio/b/l/js/f/dev.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Development server entry point for the JavaScript workspace.

// #endregion 🔖Header

// #region 🔖Dev

// [👤semio📚js💻devts🔖dev](semiorepo://section/SEMIO/JS/DEV.TS/DEV)
// Coordinates the JS workspace dev entrypoints.
// The launcher MUST reuse an existing Storybook server on port 6006 instead of failing on a duplicate launch.

import { spawn } from "node:child_process";
import { createServer } from "node:net";
import path from "node:path";
import { pathToFileURL } from "node:url";

export const STORYBOOK_ADDRESS = "http://localhost:6006/";
export const STORYBOOK_LISTEN_NUMBER = 6006;

const isWindows = process.platform === "win32";
const npmCmd = isWindows ? "npm.cmd" : "npm";

export async function detectStorybookLaunchKind(
  readListenNumberAvailability: () => Promise<boolean>,
  readRunningStorybookAvailability: () => Promise<boolean>,
): Promise<"start" | "reuse" | "fail"> {
  if (await readListenNumberAvailability()) {
    return "start";
  }
  if (await readRunningStorybookAvailability()) {
    return "reuse";
  }
  return "fail";
}

export function isStorybookIndexPayload(value: string): boolean {
  return value.includes("\"v\":") && value.includes("\"entries\":");
}

export function readLaunchKind(argv: string[]): "storybook" | "workspace" {
  return argv[0] === "storybook" ? "storybook" : "workspace";
}

async function readListenNumberAvailability(listenNumber: number): Promise<boolean> {
  return new Promise((resolve) => {
    const server = createServer();
    const closeAndResolve = (value: boolean) => {
      server.removeAllListeners();
      server.close(() => {
        resolve(value);
      });
    };
    server.once("error", () => {
      resolve(false);
    });
    server.once("listening", () => {
      closeAndResolve(true);
    });
    server.listen(listenNumber, "0.0.0.0");
  });
}

function spawnScript(scriptName: string, environment: NodeJS.ProcessEnv = {}) {
  return spawn(npmCmd, ["run", scriptName], {
    env: {
      ...process.env,
      ...environment,
    },
    stdio: "inherit",
  });
}

function bindChildSignals(child: ReturnType<typeof spawn>): void {
  const stopChild = (signal: NodeJS.Signals) => {
    child.kill(signal);
  };
  process.once("SIGINT", () => {
    stopChild("SIGINT");
  });
  process.once("SIGTERM", () => {
    stopChild("SIGTERM");
  });
}

async function waitForStopSignal(): Promise<void> {
  await new Promise<void>((resolve) => {
    const finish = () => {
      process.off("SIGINT", finish);
      process.off("SIGTERM", finish);
      resolve();
    };
    process.on("SIGINT", finish);
    process.on("SIGTERM", finish);
  });
}

async function runStorybookCommand(): Promise<void> {
  const launchKind = await detectStorybookLaunchKind(
    () => readListenNumberAvailability(STORYBOOK_LISTEN_NUMBER),
    async () => true,
  );
  if (launchKind === "reuse") {
    console.log(STORYBOOK_ADDRESS);
    console.log(`Storybook already running at ${STORYBOOK_ADDRESS}`);
    await waitForStopSignal();
    return;
  }
  if (launchKind === "fail") {
    console.error(`Port ${STORYBOOK_LISTEN_NUMBER} is already in use by a non-Storybook process.`);
    console.error("Stop the existing process or free the port, then retry the Storybook launcher.");
    process.exit(1);
    return;
  }
  const storybook = spawnScript("dev:storybook:inner", {
    NODE_OPTIONS: "",
    VSCODE_INSPECTOR_OPTIONS: "",
  });
  bindChildSignals(storybook);
  await new Promise<void>((resolve, reject) => {
    storybook.once("error", reject);
    storybook.once("exit", (code) => {
      if (typeof code === "number" && code !== 0) {
        reject(new Error(`Storybook exited with code ${code}`));
        return;
      }
      resolve();
    });
  });
}

async function runWorkspaceCommand(): Promise<void> {
  const sketchpad = spawnScript("dev:sketchpad");
  const storybook = spawnScript("dev:storybook");
  process.once("SIGINT", () => {
    sketchpad.kill("SIGINT");
    storybook.kill("SIGINT");
  });
  process.once("SIGTERM", () => {
    sketchpad.kill("SIGTERM");
    storybook.kill("SIGTERM");
  });
  await Promise.all([
    new Promise<void>((resolve, reject) => {
      sketchpad.once("error", reject);
      sketchpad.once("exit", (code) => {
        if (typeof code === "number" && code !== 0) {
          reject(new Error(`Sketchpad exited with code ${code}`));
          return;
        }
        resolve();
      });
    }),
    new Promise<void>((resolve, reject) => {
      storybook.once("error", reject);
      storybook.once("exit", (code) => {
        if (typeof code === "number" && code !== 0) {
          reject(new Error(`Storybook wrapper exited with code ${code}`));
          return;
        }
        resolve();
      });
    }),
  ]);
}

export async function runDevCommand(argv: string[] = process.argv.slice(2)): Promise<void> {
  const launchKind = readLaunchKind(argv);
  if (launchKind === "storybook") {
    await runStorybookCommand();
    return;
  }
  await runWorkspaceCommand();
}

const isEntrypoint = typeof process.argv[1] === "string" && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href;

if (isEntrypoint) {
  runDevCommand().catch((error) => {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  });
}

// #endregion 🔖Dev
