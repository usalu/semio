import { resolve } from "node:path";

const workspace = process.cwd();
const routing = await import(resolve(workspace, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const { issueRepositoryScriptProvidersV1 } = await import(resolve(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts"));
const policy = routing.readScriptPolicy({ version: 1, owner: "workspace-root", maximumElapsedMilliseconds: 0 });
const envelope = routing.createScriptProcessEnvelope(policy, { workspaceRoot: workspace }, Date.now());
const script = resolve(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts");
const args = process.argv.slice(2);
const environment = { ...process.env };
issueRepositoryScriptProvidersV1(environment);
if (args[0] === "vitest") {
  const config = resolve(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🎚️config/🟦️.ts");
  process.exit(await Bun.spawn(["node", "node_modules/vitest/vitest.mjs", "run", "--config", config, ...args.slice(1)], { cwd: workspace, env: environment, stdout: "inherit", stderr: "inherit" }).exited);
}

process.exitCode = await routing.withScriptProcessEnvelope(envelope, async invocation => {
  const child = Bun.spawn([process.execPath, script, ...(args.length ? args : ["test-stdio-source"])], {
    cwd: workspace,
    env: routing.scriptInvocationEnvironment(invocation, environment),
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  });
  return child.exited;
});
