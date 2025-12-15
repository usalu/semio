#!/usr/bin/env tsx
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
      for (const part of value.split(",").map((item) => item.trim()).filter(Boolean)) {
        skip.add(part);
      }
      continue;
    }
    if (arg.startsWith("--skip=")) {
      for (const part of arg.slice("--skip=".length).split(",").map((item) => item.trim()).filter(Boolean)) {
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
function run(command: string, args: string[] = []): void {
  execFileSync(command, args, { stdio: "inherit", cwd: rootDir });
}
//#endregion

//#region Steps
function runFix(skip: Set<string>): void {
  if (skip.has("fix")) {
    return;
  }
  run("npx", ["tsx", "hooks/prettier.ts"]);
  run("npx", ["tsx", "hooks/ruff.ts"]);
}

function runAnalyze(skip: Set<string>, nxArgs: string[]): void {
  if (skip.has("analyze")) {
    return;
  }
  run("npx", ["tsx", "hooks/i18n.ts"]);
  run("npx", ["tsx", "hooks/typescript.ts"]);
  run("npx", ["tsx", "hooks/eslint.ts", ...nxArgs]);
}

function runPreflight(skip: Set<string>, nxArgs: string[]): void {
  if (skip.has("preflight")) {
    return;
  }
  runFix(skip);
  runAnalyze(skip, nxArgs);
}

function runNx(target: string, nxArgs: string[]): void {
  run("npx", ["nx", "run-many", "-t", target, ...nxArgs]);
}

function runTest(skip: Set<string>, nxArgs: string[]): void {
  if (!skip.has("preflight")) {
    runPreflight(skip, nxArgs);
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
  runFix(parsed.skip);
} else if (parsed.command === "analyze") {
  runAnalyze(parsed.skip, parsed.nxArgs);
} else if (parsed.command === "preflight") {
  runPreflight(parsed.skip, parsed.nxArgs);
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
