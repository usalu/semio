#!/usr/bin/env repo
import { resolve } from "node:path";
import { spawn } from "node:child_process";
const root = resolve(import.meta.dir, "../../../../../../..");
const { createScriptProcessEnvelope, readScriptPolicy, scriptProcessEnvironment } = await import(resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
/** 🪂️ Runs `<cwd> <script> args…` under a freshly issued original script envelope, so target commands run without loading the project graph. */
const [cwd, ...argv] = process.argv.slice(2);
const { createBuildBootstrapCapabilitiesV1 } = await import(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts"));
const environment = { SEMIO_BUILD_MODE: "prototype", ...process.env } as NodeJS.ProcessEnv;
const { issueRepositoryScriptProvidersV1 } = await import(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts"));
issueRepositoryScriptProvidersV1(environment);
const envelope = createScriptProcessEnvelope(readScriptPolicy({ version: 1, owner: "native-exec", maximumElapsedMilliseconds: 0 }), await createBuildBootstrapCapabilitiesV1(root, ["run", process.env.NATIVE_EXEC_TARGET ?? "@semio-tech/framework-os-dev:activate-din4108-react-prototype"], environment), Date.now());
const child = spawn("repo", argv, { cwd: resolve(root, cwd), stdio: "inherit", env: scriptProcessEnvironment(envelope, environment) as NodeJS.ProcessEnv });
child.on("exit", (code, signal) => process.exit(code ?? (signal ? 1 : 0)));
