import { SCRIPT_PROCESS_INVOCATION_ENV, createScriptProcessEnvelope, readScriptPolicy, scriptProcessEnvironment } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const policy = readScriptPolicy({ version: 1, owner: "law1", maximumElapsedMilliseconds: Number(process.env.LAW1_MAX_MS ?? 0) });
const env = scriptProcessEnvironment(createScriptProcessEnvelope(policy, JSON.parse(process.env.LAW1_CAPS ?? "{}"), Date.now()), {});
process.stdout.write(env[SCRIPT_PROCESS_INVOCATION_ENV]!);
