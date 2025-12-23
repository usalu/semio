#!/usr/bin/env tsx
// #region Header

// preflight.ts

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

// #endregion Header

import { execFileSync } from "child_process";
import { join } from "path";

//#region Cli
type Command = "analyze" | "fix" | "preflight" | "test" | "build" | "prepublish" | "publish";
type ParsedArgs = { command: Command; skip: Set<string>; nxArgs: string[] };

function parseArgs(argv: string[]): ParsedArgs {
  const command = (argv[2] ?? "preflight") as Command;
  const skip = new Set<string>();
  const nxArgs: string[] = [];
  for (let i = 3; i < argv.length; i++) {
    const arg = argv[i] ?? "";
    if (arg === "--nx") {
      nxArgs.push(...argv.slice(i + 1));
      break;
    }
    if (arg === "--skip") {
      const value = argv[i + 1] ?? "";
      i++;
      for (const part of value
        .split(",")
        .map((item) => item.trim())
        .filter(Boolean)) {
        skip.add(part);
      }
      continue;
    }
    if (arg.startsWith("--skip=")) {
      for (const part of arg
        .slice("--skip=".length)
        .split(",")
        .map((item) => item.trim())
        .filter(Boolean)) {
        skip.add(part);
      }
      continue;
    }
    if (arg === "--help" || arg === "-h") {
      console.log("Usage: npx tsx preflight.ts <command> [--skip=fix,analyze,preflight,test,build] [--nx <nxArgs...>]");
      process.exit(0);
    }
    throw new Error(`Unknown argument: ${arg}`);
  }
  return { command, skip, nxArgs };
}
//#endregion

//#region Exec
const rootDir = join(__dirname);
type StepResult = { ok: boolean; label: string };
function run(command: string, args: string[] = []): void {
  execFileSync(command, args, { stdio: "inherit", cwd: rootDir, shell: true });
}
function runStep(label: string, command: string, args: string[] = []): StepResult {
  try {
    run(command, args);
    return { ok: true, label };
  } catch {
    return { ok: false, label };
  }
}
//#endregion

//#region Steps
function runFix(skip: Set<string>): boolean {
  if (skip.has("fix")) {
    return true;
  }
  let ok = true;
  ok = runStep("hooks/code.ts --fix", "npx", ["tsx", "hooks/code.ts", "--fix"]).ok && ok;
  ok = runStep("hooks/prettier.ts", "npx", ["tsx", "hooks/prettier.ts"]).ok && ok;
  ok = runStep("hooks/ruff.ts", "npx", ["tsx", "hooks/ruff.ts"]).ok && ok;
  return ok;
}

function runAnalyze(skip: Set<string>, nxArgs: string[]): boolean {
  if (skip.has("analyze")) {
    return true;
  }
  let ok = true;
  ok = runStep("hooks/code.ts", "npx", ["tsx", "hooks/code.ts"]).ok && ok;
  ok = runStep("hooks/i18n.ts", "npx", ["tsx", "hooks/i18n.ts"]).ok && ok;
  ok = runStep("hooks/typescript.ts", "npx", ["tsx", "hooks/typescript.ts"]).ok && ok;
  ok = runStep("hooks/eslint.ts", "npx", ["tsx", "hooks/eslint.ts", ...nxArgs]).ok && ok;
  return ok;
}

function runPreflight(skip: Set<string>, nxArgs: string[]): boolean {
  if (skip.has("preflight")) {
    return true;
  }
  let ok = true;
  ok = runFix(skip) && ok;
  ok = runAnalyze(skip, nxArgs) && ok;
  return ok;
}

function runNx(target: string, nxArgs: string[]): void {
  run("npx", ["nx", "run-many", "-t", target, ...nxArgs]);
}

function runTest(skip: Set<string>, nxArgs: string[]): void {
  if (!skip.has("preflight") && !runPreflight(skip, nxArgs)) {
    process.exit(1);
  }
  runNx("test", nxArgs);
}

function runBuild(skip: Set<string>, nxArgs: string[]): void {
  if (!skip.has("test")) {
    runTest(skip, nxArgs);
  }
  runNx("build", nxArgs);
}

function runPrepublish(skip: Set<string>, nxArgs: string[]): void {
  if (!skip.has("build")) {
    runBuild(skip, nxArgs);
  }
  runNx("prepublish", nxArgs);
}

function runPublish(skip: Set<string>, nxArgs: string[]): void {
  if (!skip.has("build")) {
    runBuild(skip, nxArgs);
  }
  runNx("publish", nxArgs);
}
//#endregion

//#region Main
const parsed = parseArgs(process.argv);
if (parsed.command === "fix") {
  if (!runFix(parsed.skip)) {
    process.exit(1);
  }
} else if (parsed.command === "analyze") {
  if (!runAnalyze(parsed.skip, parsed.nxArgs)) {
    process.exit(1);
  }
} else if (parsed.command === "preflight") {
  if (!runPreflight(parsed.skip, parsed.nxArgs)) {
    process.exit(1);
  }
} else if (parsed.command === "test") {
  runTest(parsed.skip, parsed.nxArgs);
} else if (parsed.command === "build") {
  runBuild(parsed.skip, parsed.nxArgs);
} else if (parsed.command === "prepublish") {
  runPrepublish(parsed.skip, parsed.nxArgs);
} else if (parsed.command === "publish") {
  runPublish(parsed.skip, parsed.nxArgs);
} else {
  throw new Error(`Unknown command: ${parsed.command}`);
}
//#endregion
