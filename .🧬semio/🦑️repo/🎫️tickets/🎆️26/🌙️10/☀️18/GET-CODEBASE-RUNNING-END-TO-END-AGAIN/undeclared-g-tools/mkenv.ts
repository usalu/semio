// undeclared-g mkenv: prints, as one JSON object, the environment the Bazel executor gives the child of ONE task: births a root Original (no deadline) with the real build capabilities of "run <id>" and issues the task environment with the executor's own functions (issueBazelTaskEnvironmentV1 over bazelTaskBaseEnvironmentV1), so the action runs under the same variables as in an official build.
// usage: bun mkenv.ts <closure.jsonl> <task id> [<configuration>=prototype]
import {readFileSync} from "node:fs";
import {createScriptProcessEnvelope, readScriptPolicy, withScriptProcessEnvelope} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const [closureFile, id, configuration = "prototype"] = process.argv.slice(2);
const root = "/Users/ueli/Documents/semio", row = readFileSync(closureFile!, "utf8").split("\n").filter(Boolean).map((line) => JSON.parse(line)).find((entry) => entry.id === id);
if (!row) throw new Error("task not in closure file");
const before = {...process.env};
const env: NodeJS.ProcessEnv = {...process.env, SEMIO_BUILD_MODE: configuration, SEMIO_BUILD_TOOL: "bazel"};
const {issueRepositoryScriptProvidersV1} = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts");
issueRepositoryScriptProvidersV1(env);
const {createBuildBootstrapCapabilitiesV1} = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts");
const capabilities = await createBuildBootstrapCapabilitiesV1(root, ["run", id!], env);
const {issueBazelTaskEnvironmentV1, bazelTaskBaseEnvironmentV1} = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts");
const definition = {...row, ...{cache: row.cache}, execution: {...row.execution, env: {...(row.execution?.env ?? {}), ...(row.configurations?.[configuration]?.execution?.env ?? {})}}};
const task = {id: id + ":" + configuration, project: row.project, target: row.target, configuration, label: "t_undeclared", dependencies: [], args: [], definition, root: row.root};
const policy = readScriptPolicy({version: 1, owner: "undeclared-g", maximumElapsedMilliseconds: 0});
const out = await withScriptProcessEnvelope(createScriptProcessEnvelope(policy, capabilities, Date.now()), async (invocation) => issueBazelTaskEnvironmentV1(root, invocation, task as never, bazelTaskBaseEnvironmentV1(root, task as never, env)));
const added: Record<string, string> = {};
for (const [key, value] of Object.entries(out)) if (value !== undefined && before[key] !== value) added[key] = value;
process.stdout.write(JSON.stringify(added));
