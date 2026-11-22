import { join } from "node:path";

const root = process.cwd();
const library = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const routing = await import(join(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const { createBuildBootstrapCapabilitiesV1 } = await import(join(library, "🏗️build/📥️caller/📤️child/🟦️.ts"));
const { issueRepositoryScriptProvidersV1 } = await import(join(library, "🏃️process/🔌️providers/🟦️.ts"));
const { runTimeTravelCli } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts"));
issueRepositoryScriptProvidersV1(process.env);
const capabilities = await createBuildBootstrapCapabilitiesV1(root, ["tools", "setup", "all"], process.env);
const envelope = routing.createScriptProcessEnvelope({ version: 1, owner: "repository:editor-lifecycle", maximumElapsedMilliseconds: 1800000 }, capabilities, Date.now());
await routing.withScriptProcessEnvelope(envelope, invocation => runTimeTravelCli(invocation, root, join(root, process.argv[2]), ["--scenario", "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🖐️5d-lifecycle/🟦️.ts", "--renderer", "react", "--out", join(root, process.argv[2])]));
