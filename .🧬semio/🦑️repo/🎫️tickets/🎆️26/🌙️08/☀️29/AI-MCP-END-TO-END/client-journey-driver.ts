import { join } from "node:path";
import { buildComponentClosureV1 } from "@semio-tech/framework-plugin/build/driver-stages";
import { repositoryCargoNetworkV1 } from "@semio-tech/repo-lib/cargo";
import { createNativeBuildCallerCapabilitiesV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";
import { createIssuedResourceControlV1, createScriptProcessEnvelope, withScriptProcessEnvelope } from "@semio-tech/process/routing";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { runOsMcpClientJourney } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { withMcpClientWorkspaceV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🎫️session/📦️published/🟦️.ts";
import { SPECIFIC_MCP_CLIENT_E2E_PINS_V1 as pins } from "../../../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts";

const workspace = process.cwd();
const skipBuild = process.argv.includes("--skip-build");
const environment = { ...process.env, SEMIO_BUILD_MODE: "prototype", SEMIO_SCRIPT_PROGRESS: "human" };
const grant = createNativeBuildCallerCapabilitiesV1(workspace, ["run", "@semio-tech/s-dev-composition:mcp-client-e2e"], "nx", environment);
const envelope = createScriptProcessEnvelope({ version: 1, owner: "mcp:client-journey", maximumElapsedMilliseconds: 7_200_000 }, grant, Date.now());
await withScriptProcessEnvelope(envelope, async invocation => {
  if (!skipBuild) await buildComponentClosureV1(workspace, new Set(process.argv.includes("--note-only") ? [pins.pluginId] : [pins.pluginId, pins.inferencePluginId]), {
    invocation,
    control: scriptCompilerAdmissionControlV1(invocation, "mcp:client-e2e-models", 0),
    network: repositoryCargoNetworkV1(invocation, workspace),
    facet: "model",
    log: line => console.log(`[DEBUG] ${line}`),
  });
  const phase = createIssuedResourceControlV1(invocation, "mcp:client-e2e", 1_800_000);
  await withMcpClientWorkspaceV1(invocation, phase, async (folder, control) => {
    console.log(`[DEBUG] workspace folder ${folder}`);
    const steps = await runOsMcpClientJourney(workspace, folder, pins, invocation, control);
    for (const step of steps) console.log(`${step.ok ? "PASS" : "FAIL"}  ${step.step} — ${step.detail}`);
    console.log(`journey: ${steps.filter(s => s.ok).length}/${steps.length} green`);
  });
});
