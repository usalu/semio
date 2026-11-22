import { buildComponentClosureV1 } from "@semio-tech/framework-plugin/build/driver-stages";
import { componentModelDirectoryKeyV1 } from "@semio-tech/framework-plugin/model/contract";
import { ensureDescriptorEmitterV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts";
import { repositoryCargoNetworkV1 } from "@semio-tech/repo-lib/cargo";
import { issueRepositoryScriptProvidersV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import { createNativeBuildCallerCapabilitiesV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";
import { SCRIPT_PROCESS_INVOCATION_ENV, createIssuedResourceControlV1, createScriptProcessEnvelope, receiveScriptProcessInvocation, scriptProcessEnvironment } from "@semio-tech/process/routing";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { withMcpComponentEnvironmentV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/📦️component-models/🟦️.ts";
import { buildMcpArtifactV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🏗️staging/🟦️.ts";
import { constants } from "node:fs";
import { copyFile, mkdir, mkdtemp, rm } from "node:fs/promises";
import { join, resolve } from "node:path";

const workspace = process.cwd();
const selected = process.argv.slice(2);
const environment = { ...process.env, SEMIO_BUILD_MODE: "prototype", SEMIO_SCRIPT_PROGRESS: "human" };
const gateway = selected.length === 1 && selected[0] === "--gateway";
if (!process.env[SCRIPT_PROCESS_INVOCATION_ENV]) {
  if (gateway) issueRepositoryScriptProvidersV1(environment);
  const command = gateway ? ["run", "@semio-tech/framework-os-mcp-rs:build"] : ["components", "describe", ...selected];
  const grant = createNativeBuildCallerCapabilitiesV1(workspace, command, "nx", environment);
  const envelope = createScriptProcessEnvelope({ version: 1, owner: "mcp-journey:build-models", maximumElapsedMilliseconds: 7_200_000 }, grant, Date.now());
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
    await withMcpComponentEnvironmentV1(workspace, invocation, control, { SEMIO_PLUGIN_CATALOG: catalog }, async (env, retained) => {
      const manifest = JSON.parse(env.SEMIO_MCP_COMPONENT_MODELS ?? '{"models":[]}') as { mode: string; models: { pluginId: string; cratePath: string; root: string; preparedActor?: { imageSha256: string } }[] };
      const rows = manifest.models;
      console.log(`[DEBUG] admitted MCP component models ${rows.map(row => row.pluginId).join(", ")}`);
      const copy = selected.find(argument => argument === "--copy-to-ticket" || argument.startsWith("--copy-to-ticket="));
      if (!copy) return;
      const owners = copy === "--copy-to-ticket" ? ["note", "wfc"] : [copy.slice("--copy-to-ticket=".length)];
      const ownerModels = owners.map(id => {
        if (id !== "note" && id !== "wfc") throw Error("Pinned MCP model required");
        const model = rows.find(row => row.pluginId === id);
        if (!model?.preparedActor) throw Error(`Missing retained prepared ${id} model`);
        return { ...model, preparedActor: model.preparedActor };
      });
      const root = await mkdtemp(join(import.meta.dirname, "🗑️generated", "m-managed-"));
      try {
        for (const model of ownerModels) {
          const target = join(root, "browser", manifest.mode, "components", componentModelDirectoryKeyV1(model.cratePath));
          await mkdir(target, { recursive: true });
          for (const file of ["📦️component.wasm", "🔣️.json", "🛂️.descriptor.semio", "🧾️receipt.json", "📦️actor.semio", "🌱️actor.json"]) {
            retained.signal.throwIfAborted();
            await copyFile(join(model.root, file), join(target, file), constants.COPYFILE_FICLONE | constants.COPYFILE_EXCL);
          }
          console.log(`[DEBUG] copied retained ${model.pluginId} actor ${model.preparedActor.imageSha256}`);
        }
        retained.signal.throwIfAborted();
        console.log(`[DEBUG] retained SDK model root ${root}`);
      } catch (error) {
        await rm(root, { recursive: true });
        throw error;
      }
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
