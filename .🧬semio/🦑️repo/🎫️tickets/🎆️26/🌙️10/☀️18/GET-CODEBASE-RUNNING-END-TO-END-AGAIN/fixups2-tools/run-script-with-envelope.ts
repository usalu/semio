import { createScriptProcessEnvelope, SCRIPT_PROCESS_INVOCATION_ENV } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const [cwd, ...command] = process.argv.slice(2);
const wire = JSON.stringify(createScriptProcessEnvelope({ version: 1, owner: "fixups2-probe", maximumElapsedMilliseconds: 0 }, { exact: "original" }, Date.now()));
const child = Bun.spawnSync(["bun", "./📜️script.ts", ...command], { cwd, env: { ...process.env, [SCRIPT_PROCESS_INVOCATION_ENV]: wire }, stdout: "inherit", stderr: "inherit" });
process.exit(child.exitCode ?? 1);
