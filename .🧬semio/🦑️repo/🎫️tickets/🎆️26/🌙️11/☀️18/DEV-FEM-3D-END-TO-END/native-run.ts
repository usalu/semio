import {spawn} from "node:child_process";
import {mkdirSync} from "node:fs";
import {join, resolve} from "node:path";
import {issueRepositoryScriptProvidersV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import {createScriptProcessEnvelope, scriptProcessEnvironment} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";

/** 🚦️ Runs one project Script natively under a freshly issued original build-orchestration envelope, without reading the build graph: `bun native-run.ts <cwd> <selection> <script args…>`. */
const [cwd, selection, ...args] = process.argv.slice(2);
if (!cwd || !selection || !args.length) throw Error("native-run <cwd> <project:target> <script args…>");
const workspaceRoot = resolve(import.meta.dirname, "../../../../../../.."), buildMode = process.env.SEMIO_BUILD_MODE === "release" ? "release" : "prototype";
const artifactDirectory = join(import.meta.dirname, "🗑️generated", "artifacts");
mkdirSync(artifactDirectory, {recursive: true});
const capabilities = {tool: "nx", workspaceRoot, codeRoot: workspaceRoot, buildMode, arguments: ["run", selection], selection: [selection], launches: [["run", selection]], native: {artifactDirectory, transport: {maximumBytes: 1 << 30, maximumLines: 10_000_000}, child: {maximumElapsedMilliseconds: 86_400_000}, network: {offline: false}}};
issueRepositoryScriptProvidersV1(process.env);
const env = scriptProcessEnvironment(createScriptProcessEnvelope({version: 1, owner: "dev-fem3d-native", maximumElapsedMilliseconds: 0}, capabilities, Date.now()), {...process.env, SEMIO_BUILD_MODE: buildMode});
const child = spawn("repo", args, {cwd, env, stdio: "inherit", shell: process.platform === "win32"});
for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => child.kill(signal));
child.on("exit", code => process.exit(code ?? 1));
