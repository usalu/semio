import { mkdtemp, readFile, rm } from "node:fs/promises";
import { join, resolve } from "node:path";
import { parseArgs } from "node:util";
import { createIssuedResourceControlV1, createScriptProcessEnvelope, withScriptProcessEnvelope } from "@semio-tech/process/routing";
import { createNativeBuildCallerCapabilitiesV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";
import { driveOsMcpClientSessionV1, mcpServerEntries, takeMcpPluginCatalog, withMcpHostDriverPolicy } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { withMcpClientSessionV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🎫️session/🟦️.ts";
import { encodeMcpComponentModelManifestV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/📦️component-models/🟦️.ts";
import { actorPreparationImageIdentityV1, actorPreparationPolicyV1, assertActorPreparationBindingV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🌱️actor-preparation/🟦️.ts";
import { fileDigest, readArtifactPrefixV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import { SPECIFIC_MCP_CLIENT_E2E_PINS_V1 as pins } from "../../../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts";

import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";

const { values, positionals } = parseArgs({ args: process.argv.slice(2), allowPositionals: true, options: { driver: { type: "string" } } });
const [binary, operationPolicy, ...modelPaths] = positionals;
const cwd = process.cwd();
const { args, env } = takeMcpPluginCatalog(mcpServerEntries(cwd).semio.args.slice(4), cwd);
const catalog = JSON.parse(await readFile(join(env.SEMIO_PLUGIN_CATALOG!, "🦀️catalog.json"), "utf8"));
const models = await Promise.all(modelPaths.map(async path => {
  const root = resolve(path);
  const pair = verifySemanticDescriptorPairBytesV1(await readFile(join(root, "🔣️.json")), await readFile(join(root, "🛂️.descriptor.semio")));
  const plugin = catalog.plugins.find(row => row.pluginId === pair.pluginId);
  if (!plugin) throw Error("Installed catalog does not declare " + pair.pluginId);
  const image = join(root, actorPreparationPolicyV1.image);
  const signal = AbortSignal.timeout(30000);
  const preparedActor = await actorPreparationImageIdentityV1(image, signal);
  assertActorPreparationBindingV1(await readArtifactPrefixV1(image, 64, actorPreparationPolicyV1.maximumBytes, signal), await fileDigest(join(root, "📦️component.wasm"), signal));
  return { root, pluginId: plugin.pluginId, cratePath: plugin.cratePath, preparedActor };
}));
const environment = { ...process.env, ...env, SEMIO_BUILD_MODE: "prototype", CLAUDE_CODE_SESSION_ID: "mcp-client-e2e", CLAUDE_CODE_MESSAGING_TOKEN: "mcp-client-e2e", SEMIO_MCP_COMPONENT_MODELS: encodeMcpComponentModelManifestV1({ version: 1, mode: "prototype", models }) };
const authority = createNativeBuildCallerCapabilitiesV1(cwd, ["run", "@semio-tech/s-dev-composition:mcp-client-e2e"], "nx", environment);
const envelope = createScriptProcessEnvelope({ version: 1, owner: "mcp:direct-client-journey", maximumElapsedMilliseconds: 3_600_000 }, authority, Date.now());
const folder = await mkdtemp(join(import.meta.dir, "🗑️generated", "direct-journey-"));
try {
  args[args.indexOf("--folder") + 1] = folder;
  if (operationPolicy && operationPolicy !== "-") args[args.indexOf("--operation-policy") + 1] = resolve(operationPolicy);
  if (values.driver) args.push("--host-driver-policy", resolve(values.driver));
  await withScriptProcessEnvelope(envelope, async invocation => {
    const phase = createIssuedResourceControlV1(invocation, "mcp:direct-client-journey", 3_000_000);
    const launch = { command: resolve(binary), cwd, environment, args: withMcpHostDriverPolicy(["stdio", ...args, "--no-bridge", "--auto-approve", "all"]) };
    const steps = await withMcpClientSessionV1(invocation, phase, launch, session => driveOsMcpClientSessionV1(session, pins, async () => {
      const model = models.find(row => row.pluginId === pins.pluginId);
      if (!model) return undefined;
      const descriptor = JSON.parse(await readFile(join(model.root, "🔣️.json"), "utf8"));
      return descriptor.manifest.apps.find(row => row.id === pins.caller.appId);
    }));
    for (const step of steps) console.log(step.ok ? "PASS" : "FAIL", step.step, step.detail, ...(step.ok ? [] : [JSON.stringify(step.wire)]));
    if (steps.some(step => !step.ok)) process.exitCode = 1;
  });
} finally {
  await rm(folder, {recursive: true, force: true});
}
