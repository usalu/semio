import { buildComponentClosureV1 } from "@semio-tech/framework-plugin/build/driver-stages";
import { ensureDescriptorEmitterV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts";
import { repositoryCargoNetworkV1 } from "@semio-tech/repo-lib/cargo";
import { issueRepositoryScriptProvidersV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import { createNativeBuildCallerCapabilitiesV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";
import { SCRIPT_PROCESS_INVOCATION_ENV, createIssuedResourceControlV1, createScriptProcessEnvelope, receiveScriptProcessInvocation, scriptProcessEnvironment } from "@semio-tech/process/routing";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { withMcpComponentEnvironmentV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/📦️component-models/🟦️.ts";
import { buildMcpArtifactV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🏗️staging/🟦️.ts";
import { resolve } from "node:path";

const workspace = process.cwd();
const selected = process.argv.slice(2);
const environment = { ...process.env, SEMIO_BUILD_MODE: "prototype", SEMIO_SCRIPT_PROGRESS: "human" };
const gateway = selected.length === 1 && selected[0] === "--gateway";
if (!process.env[SCRIPT_PROCESS_INVOCATION_ENV]) {
  if (gateway) issueRepositoryScriptProvidersV1(environment);
  const command = gateway ? ["run", "@semio-tech/framework-os-mcp-rs:build"] : ["components", "describe", ...selected];
  const grant = createNativeBuildCallerCapabilitiesV1(workspace, command, "nx", environment);
  const envelope = createScriptProcessEnvelope({ version: 1, owner: "mcp-journey:build-models", maximumElapsedMilliseconds: 2_700_000 }, grant, Date.now());
  const child = Bun.spawn([process.execPath, import.meta.path, ...selected], {
    env: scriptProcessEnvironment(envelope, environment), stdin: "inherit", stdout: "inherit", stderr: "inherit",
  });
  for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => child.kill(signal));
  process.exitCode = await child.exited;
} else await receiveScriptProcessInvocation(process.env, async invocation => {
  if (gateway) {
    await buildMcpArtifactV1(workspace, invocation, scriptCompilerAdmissionControlV1(invocation, "cargo:build", 0));
    console.log("[DEBUG] Published the managed native MCP gateway");
    return;
  }
  if (selected[0] === "--inspect") {
    const catalog = resolve("✏️s/🧑‍💻dev/🧩️catalog/🤖️generated");
    const control = createIssuedResourceControlV1(invocation, "mcp-journey:inspect-models", 120_000);
    await withMcpComponentEnvironmentV1(workspace, invocation, control, { SEMIO_PLUGIN_CATALOG: catalog }, async env => {
      const rows = JSON.parse(env.SEMIO_MCP_COMPONENT_MODELS ?? '{"models":[]}').models;
      console.log(`[DEBUG] admitted MCP component models ${rows.map(row => row.pluginId).join(", ")}`);
    });
    return;
  }
  const control = scriptCompilerAdmissionControlV1(invocation, "mcp-journey:build-models", 0);
  await ensureDescriptorEmitterV1(workspace, invocation, control);
  const outcome = await buildComponentClosureV1(workspace, new Set(selected), {
    facet: "model",
    network: repositoryCargoNetworkV1(invocation, workspace),
    control,
    log: line => console.log(`[DEBUG] ${line}`),
    invocation,
  });
  console.log(`[DEBUG] Built ${outcome.staged.length} MCP models`);
});
