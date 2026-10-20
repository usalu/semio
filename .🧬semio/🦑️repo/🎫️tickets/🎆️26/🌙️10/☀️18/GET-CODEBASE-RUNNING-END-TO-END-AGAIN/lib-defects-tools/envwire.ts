import { SCRIPT_PROCESS_INVOCATION_ENV, createScriptProcessEnvelope, readScriptPolicy, scriptProcessEnvironment } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { SCRIPT_PROVIDERS_ENV, writeScriptProviderMapV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔌️providers/🟦️.ts";
import { repositoryScriptProvidersV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import { createNativeBuildCallerCapabilitiesV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";

/** 🎟️ Mints the launcher envelope of `bun repo run <project>:test` with the configured native caller grant; usage: bun envwire.ts [root] */
const root = process.argv[2] ?? process.env.SEMIO_REPO_ROOT ?? process.cwd();
const policy = readScriptPolicy({ version: 1, owner: "lib-defects", maximumElapsedMilliseconds: 0 });
const capabilities = createNativeBuildCallerCapabilitiesV1(root, ["run", "lib-defects:test"], "nx", { SEMIO_BUILD_MODE: "prototype" });
const env = scriptProcessEnvironment(createScriptProcessEnvelope(policy, capabilities, Date.now()), {});
process.stdout.write(JSON.stringify({ [SCRIPT_PROCESS_INVOCATION_ENV]: env[SCRIPT_PROCESS_INVOCATION_ENV], [SCRIPT_PROVIDERS_ENV]: writeScriptProviderMapV1(repositoryScriptProvidersV1()) }));
