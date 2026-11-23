/** 🚀️ Runs any script or executable inside an issued ScriptProcessEnvelope without waiting on the repo graph service. */
import { spawn } from "node:child_process";
import { mkdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { issueRepositoryScriptProvidersV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import { createScriptProcessEnvelope, scriptProcessEnvironment } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const [directory, selection, ...args] = process.argv.slice(2);
if (!directory || !selection || !args.length) {
  throw new Error("Usage: bun 🚀️launch-shooting-script.ts <cwd-relative-to-repo> <project:target> <executable> <args...>");
}
const workspaceRoot = resolve(import.meta.dirname, "../../../../../../..");
const artifactDirectory = join(import.meta.dirname, "🗑️generated", "artifacts");
mkdirSync(artifactDirectory, { recursive: true });
const buildMode = (process.env.SEMIO_BUILD_MODE ?? "prototype") as "prototype" | "release";
const capabilities = {
  tool: "nx" as const,
  workspaceRoot,
  codeRoot: workspaceRoot,
  buildMode,
  arguments: ["run", selection],
  selection: [selection],
  launches: [["run", selection]],
  native: {
    artifactDirectory,
    transport: { maximumBytes: 1 << 29, maximumLines: 10_000_000 },
    child: { maximumElapsedMilliseconds: 86_400_000 },
    network: { offline: false },
  },
};
issueRepositoryScriptProvidersV1(process.env);
const env = scriptProcessEnvironment(
  createScriptProcessEnvelope({ version: 1, owner: "shooting-e2e-native", maximumElapsedMilliseconds: 0 }, capabilities, Date.now()),
  {
    ...process.env,
    PATH: `${dirname(process.execPath)}:${join(workspaceRoot, ".cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin")}:${process.env.PATH ?? ""}`,
    SEMIO_LANGUAGE: process.env.SEMIO_LANGUAGE ?? "en",
    SEMIO_BUILD_MODE: buildMode,
    SEMIO_PLUGIN: process.env.SEMIO_PLUGIN ?? "shooting",
    SEMIO_RENDERER: process.env.SEMIO_RENDERER ?? "react",
    SEMIO_PLUGIN_CATALOG: join(workspaceRoot, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"),
  },
);
const [cmd, ...cmdArgs] = args;
const resolvedCmd = cmd === "bun" ? process.execPath : cmd!;
const child = spawn(resolvedCmd, cmdArgs, { cwd: resolve(workspaceRoot, directory), env, stdio: "inherit", windowsHide: true });
for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => child.kill(signal));
child.once("error", error => {
  console.error(error);
  process.exitCode = 1;
});
child.once("exit", code => {
  process.exitCode = code ?? 1;
});
