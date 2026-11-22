import { spawn } from "node:child_process";
import { mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { issueRepositoryScriptProvidersV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import { createScriptProcessEnvelope, scriptProcessEnvironment } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const [directory, selection, ...args] = process.argv.slice(2);
if (!directory || !selection || !args.length) throw Error("launch-native <cwd> <project:target> <script args…>");
const workspaceRoot = resolve(import.meta.dirname, "../../../../../../..");
const artifactDirectory = join(import.meta.dirname, "🗑️generated", "artifacts");
mkdirSync(artifactDirectory, { recursive: true });
const buildMode = "prototype";
const capabilities = {
  tool: "nx", workspaceRoot, codeRoot: workspaceRoot, buildMode,
  arguments: ["run", selection], selection: [selection], launches: [["run", selection]],
  native: { artifactDirectory, transport: { maximumBytes: 1 << 30, maximumLines: 10_000_000 }, child: { maximumElapsedMilliseconds: 86_400_000 }, network: { offline: false } },
};
issueRepositoryScriptProvidersV1(process.env);
const env = scriptProcessEnvironment(createScriptProcessEnvelope({ version: 1, owner: "dev-trinity-native", maximumElapsedMilliseconds: 0 }, capabilities, Date.now()), {
  ...process.env, SEMIO_BUILD_MODE: buildMode, SEMIO_PLUGIN: process.env.TRINITY_VARIANT ?? "trinity-jack", SEMIO_RENDERER: "react",
  SEMIO_PLUGIN_CATALOG: join(workspaceRoot, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"),
});
const child = spawn("repo", args, { cwd: resolve(workspaceRoot, directory), env, stdio: "inherit", windowsHide: true });
for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => child.kill(signal));
child.once("error", error => { console.error(error); process.exitCode = 1; });
child.once("exit", code => { process.exitCode = code ?? 1; });
