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
import { dirname, join } from "path";
import { fileURLToPath } from "url";

//#region Cli
type Command = "analyze" | "fix" | "preflight" | "test" | "build" | "publish:test" | "publish";
type ParsedArgs = { command: Command; skip: Set<string>; nxArgs: string[] };

function parseArgs(argv: string[]): ParsedArgs {
  if (argv[2] === "--help" || argv[2] === "-h") {
    console.log("Usage: npx tsx preflight.ts <command> [--skip=fix,analyze,preflight,test,build] [--nx <nxArgs...>]");
    process.exit(0);
  }
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
//#endregion Cli

//#region Exec
const __dirname = dirname(fileURLToPath(import.meta.url));
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
//#endregion Exec

//#region Steps
function runFix(skip: Set<string>): boolean {
  if (skip.has("fix")) {
    return true;
  }
  let ok = true;
  ok = runStep("hooks/code.tsx --fix", "npx", ["tsx", "hooks/code.tsx", "--fix"]).ok && ok;
  ok = runStep("hooks/prettier.tsx", "npx", ["tsx", "hooks/prettier.tsx"]).ok && ok;
  ok = runStep("hooks/ruff.tsx", "npx", ["tsx", "hooks/ruff.tsx"]).ok && ok;
  return ok;
}

function runAnalyze(skip: Set<string>, nxArgs: string[]): boolean {
  if (skip.has("analyze")) {
    return true;
  }
  let ok = true;
  ok = runStep("hooks/code.tsx", "npx", ["tsx", "hooks/code.tsx"]).ok && ok;
  ok = runStep("hooks/i18n.tsx", "npx", ["tsx", "hooks/i18n.tsx"]).ok && ok;
  ok = runStep("hooks/typescript.tsx", "npx", ["tsx", "hooks/typescript.tsx"]).ok && ok;
  ok = runStep("hooks/eslint.tsx", "npx", ["tsx", "hooks/eslint.tsx", ...nxArgs]).ok && ok;
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

function runPublishTest(skip: Set<string>, nxArgs: string[]): void {
  if (!skip.has("build")) {
    runBuild(skip, nxArgs);
  }
  runNx("publish:test", nxArgs);
}

function runPublish(skip: Set<string>, nxArgs: string[]): void {
  if (!skip.has("build")) {
    runBuild(skip, nxArgs);
  }
  runNx("publish", nxArgs);
}
//#endregion Steps

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
} else if (parsed.command === "publish:test") {
  runPublishTest(parsed.skip, parsed.nxArgs);
} else if (parsed.command === "publish") {
  runPublish(parsed.skip, parsed.nxArgs);
} else {
  throw new Error(`Unknown command: ${parsed.command}`);
}
//#endregion Main
