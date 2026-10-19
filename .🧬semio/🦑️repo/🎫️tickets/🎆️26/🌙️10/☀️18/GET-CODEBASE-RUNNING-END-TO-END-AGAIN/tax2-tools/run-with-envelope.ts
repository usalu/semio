// 🧪 Runs a script with a freshly issued Original process envelope in its environment. usage: bun run-with-envelope.ts <script.ts> [args...]
import {spawnSync} from "node:child_process";
import {createScriptProcessEnvelope, scriptProcessEnvironment} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
const [script, ...args] = process.argv.slice(2);
const envelope = createScriptProcessEnvelope({version: 1, owner: "tax2:probe", maximumElapsedMilliseconds: 120000} as any, {command: "tax2-probe"}, Date.now());
const env = {...(scriptProcessEnvironment(envelope, process.env) as NodeJS.ProcessEnv), SEMIO_SCRIPT_PROVIDERS: JSON.stringify({version: 1, providers: {"repository-tooling": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🛠️repository-tooling/🔌️provider/🟦️.ts", "artifact-package": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧩️artifact-package/🔌️provider/🟦️.ts"}})};
const r = spawnSync("bun", [script!, ...args], {env, stdio: "inherit"});
process.exit(r.status ?? 1);
