import { SCRIPT_PROCESS_INVOCATION_ENV, createScriptProcessEnvelope, readScriptPolicy, scriptProcessEnvironment } from "/home/deck/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { SCRIPT_PROVIDERS_ENV, writeScriptProviderMapV1 } from "/home/deck/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔌️providers/🟦️.ts";
import { repositoryScriptProvidersV1 } from "/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
const R = "/home/deck/semio";
const policy = readScriptPolicy({ version: 1, owner: "workspace-root", maximumElapsedMilliseconds: 0 });
const env = scriptProcessEnvironment(createScriptProcessEnvelope(policy, { workspaceRoot: R, codeRoot: R, arguments: ["component"], buildMode: "prototype", selection: [], native: { artifactDirectory: R + "/.cache/b/p/m/browser/prototype", transport: { maximumBytes: 268435456, maximumLines: 4000000 }, child: { maximumElapsedMilliseconds: 14400000, idleMilliseconds: 1800000 }, network: { offline: true } } }, Date.now()), {});
process.stdout.write(JSON.stringify({ [SCRIPT_PROCESS_INVOCATION_ENV]: env[SCRIPT_PROCESS_INVOCATION_ENV], [SCRIPT_PROVIDERS_ENV]: writeScriptProviderMapV1(repositoryScriptProvidersV1()) }));
