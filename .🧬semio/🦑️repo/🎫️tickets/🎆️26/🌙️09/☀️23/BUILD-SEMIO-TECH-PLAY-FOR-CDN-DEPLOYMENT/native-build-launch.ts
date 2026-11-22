import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { releaseUnitCacheDemotesV1, releaseUnitCacheModeV1 } from "./native-unit-cache-policy.ts";
import { issueRepositoryScriptProvidersV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import { createBuildBootstrapCapabilitiesV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts";
import { createScriptProcessEnvelope, readScriptPolicy, scriptProcessEnvironment } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🚀️ Launches one build target's command with the genuine deadline-free build-caller envelope the graph runner issues for `<project>:<target>`. Usage: native-build-launch.ts <project:target> <cwd> <command...> */
const root = process.env.SEMIO_WORKSPACE_ROOT!, [task, cwd, ...command] = process.argv.slice(2);
const env: NodeJS.ProcessEnv = { ...process.env, SEMIO_TASK_PROJECT: task!.slice(0, task!.lastIndexOf(":")), SEMIO_TASK_TARGET: task!.slice(task!.lastIndexOf(":") + 1) };
issueRepositoryScriptProvidersV1(env);
const envelope = createScriptProcessEnvelope(readScriptPolicy({ version: 1, owner: "repository:build", maximumElapsedMilliseconds: 0 }), await createBuildBootstrapCapabilitiesV1(root, ["run", task!], env), Date.now());
Object.assign(env, scriptProcessEnvironment(envelope, env));
const marker = join(process.env.NATIVE_PLAY_OUT ?? join(dirname(import.meta.path), "🗑️generated/native"), "unit-cache-demoted");
delete env.NATIVE_PLAY_OUT;
const run = (mode: "on" | "off") => { const started = Date.now(), result = spawnSync(command[0]!, command.slice(1), { cwd: join(root, cwd!), env: { ...env, SEMIO_UNIT_CACHE: mode }, stdio: "inherit" }); return { status: result.status ?? 1, elapsedMs: Date.now() - started }; };
const mode = releaseUnitCacheModeV1({ optOut: process.env.SEMIO_RELEASE_UNIT_CACHE, demoted: existsSync(marker) });
let outcome = run(mode);
if (releaseUnitCacheDemotesV1({ mode, status: outcome.status, elapsedMs: outcome.elapsedMs, earlyMs: 120_000 })) {
  mkdirSync(dirname(marker), { recursive: true }); writeFileSync(marker, `${new Date().toISOString()} unit cache failed after ${outcome.elapsedMs} ms with status ${outcome.status}\n`);
  console.error("[native-build-launch] unit cache attempt failed early; demoted, rerunning with SEMIO_UNIT_CACHE=off");
  outcome = run("off");
}
process.exit(outcome.status);
