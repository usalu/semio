/** 📤️ Prints the root Script process envelope wire (no deadline) so an owner script can run outside the blocked launcher. */
import { readScriptPolicy, } from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { SCRIPT_PROCESS_INVOCATION_ENV, createScriptProcessEnvelope, scriptProcessEnvironment } from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const policy = readScriptPolicy({ version: 1, owner: "workspace-root", maximumElapsedMilliseconds: 0 });
const env = scriptProcessEnvironment(createScriptProcessEnvelope(policy, {}, Date.now()), {});
process.stdout.write(env[SCRIPT_PROCESS_INVOCATION_ENV]!);
