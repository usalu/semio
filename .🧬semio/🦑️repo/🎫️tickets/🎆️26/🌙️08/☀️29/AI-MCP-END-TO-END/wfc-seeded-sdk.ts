import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { join, resolve, sep } from "node:path";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { CallToolResultSchema, type Progress } from "@modelcontextprotocol/sdk/types.js";
import Ajv from "ajv";
import { componentModelDirectoryKeyV1 } from "@semio-tech/framework-plugin/model/contract";
import { inferenceCancellationVerdictV1, inferenceClientVerdictV1, mcpServerEntries, takeMcpPluginCatalog, withMcpHostDriverPolicy } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { parseInferenceRunInput, parseInferenceSubmitInput } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts";
import { encodeMcpComponentModelManifestV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/📦️component-models/🟦️.ts";
import { readNativeCatalogV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📥️runtime/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
import { actorPreparationImageIdentityV1, actorPreparationPolicyV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🌱️actor-preparation/🟦️.ts";
import { SPECIFIC_MCP_CLIENT_E2E_PINS_V1 as pins } from "../../../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts";
import activationCancel from "./wfc-activation-cancel.json";

const workspace = process.cwd();
const generated = join(import.meta.dirname, "🗑️generated");
const options = process.argv.slice(2);
const mode = "prototype";
const entry = mcpServerEntries(workspace).semio;
const catalog = takeMcpPluginCatalog(entry.args.slice(4), workspace);
assert(catalog.env.SEMIO_PLUGIN_CATALOG);
const { plugins } = readNativeCatalogV1(JSON.parse(await readFile(join(catalog.env.SEMIO_PLUGIN_CATALOG, "🦀️catalog.json"), "utf8")));
const pluginId = pins.inferencePluginId;
const plugin = plugins.find(row => row.pluginId === pluginId);
assert(plugin);
const modelsRoot = resolve(options.find(option => option.startsWith("--models-root="))?.slice("--models-root=".length) ?? join(generated, "m-canon"));
assert(modelsRoot.startsWith(generated + sep), "Use a retained model diagnostic directory in the ticket");
const root = join(modelsRoot, "browser", mode, "components", componentModelDirectoryKeyV1(plugin.cratePath));
const pair = verifySemanticDescriptorPairBytesV1(await readFile(join(root, "🔣️.json")), await readFile(join(root, "🛂️.descriptor.semio")));
assert.equal(pair.pluginId, pluginId);
const preparedActor = await actorPreparationImageIdentityV1(join(root, actorPreparationPolicyV1.image), AbortSignal.timeout(30_000));
console.log("[DEBUG] model", pluginId, pair.semanticSha256);
const owner = await readFile("✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🔨️modules/🏠️host/💡️inferences/🧪️tests/🔬️unit/🦀️.rs", "utf8");
const hash = (bytes: Buffer) => createHash("sha256").update(bytes).digest("hex");
const work = await mkdtemp(join(generated, "wfc-seeded-"));
const folder = join(work, "workspace");
const fixture = await Promise.all(["PACK", "SPR"].map(async key => {
  const bytes = owner.match(new RegExp(`const GENESIS_${key}_BASE64: &str = "([^"]+)"`))?.[1];
  assert(bytes, `Missing owner ${key} constant`);
  const decoded = Buffer.from(bytes, "base64");
  assert.equal(decoded.toString("base64"), bytes);
  const path = join(work, key);
  await writeFile(path, decoded, { flag: "wx" });
  const sha256 = hash(decoded);
  console.log("[DEBUG] fixture", key, decoded.length, sha256);
  return { key, path, bytes: decoded.length, sha256 };
}));
await mkdir(folder);
const native = (stem: string) => process.platform === "win32" ? `${stem}.exe` : stem;
const useFolder = (operation: string, id: string) => assert.equal(Bun.spawnSync([join(generated, native("wfc-seeded-folder")), operation, folder, id, ...fixture.map(row => row.path)], {
  stdout: "inherit",
  stderr: "inherit",
}).exitCode, 0);
const tasks = options.filter(value => !value.startsWith("--"));
const modes = ["solve", "cancel"].filter(task => tasks.length === 0 || tasks.includes(task));
assert(modes.length > 0);
const env = Object.fromEntries(Object.entries({
  ...process.env, ...catalog.env, ...entry.env,
  SEMIO_REPO_ROOT: workspace,
  SEMIO_BUILD_MODE: mode,
  SEMIO_SCRIPT_PROGRESS: "human",
  SEMIO_MCP_COMPONENT_MODELS: encodeMcpComponentModelManifestV1({ version: 1, mode, models: [{ pluginId, cratePath: plugin.cratePath, root, preparedActor }] }),
}).filter((row): row is [string, string] => typeof row[1] === "string"));
const output: Record<string, unknown> = { fixture: fixture.map(({ key, bytes, sha256 }) => ({ key, bytes, sha256 })), plugin: { id: pluginId, descriptorSha256: pair.semanticSha256 } };
const stackProbe = options.includes("--stack-probe");
const openForSubmit = options.includes("--open-for-submit");
const schemaProbe = options.includes("--schema-probe");
const cancelDuringHash = options.includes("--cancel-during-hash");
const sampleRetirement = options.includes("--sample-retirement");
const gateway = options.find(option => option.startsWith("--gateway="))?.slice("--gateway=".length);
const label = options.find(option => option.startsWith("--label="))?.slice("--label=".length);
const observationMilliseconds = Number(options.find(option => option.startsWith("--observe-ms="))?.slice("--observe-ms=".length) ?? pins.inferenceLimits.maximumElapsedMilliseconds);
const solveCallMilliseconds = Number(options.find(option => option.startsWith("--solve-timeout-ms="))?.slice("--solve-timeout-ms=".length) ?? 50_000);
assert(!label || /^[a-z0-9-]+$/u.test(label));
assert(Number.isSafeInteger(observationMilliseconds) && observationMilliseconds > 0 && observationMilliseconds <= 180_000);
assert(Number.isSafeInteger(solveCallMilliseconds) && solveCallMilliseconds > 0 && solveCallMilliseconds <= 180_000);
const registry = stackProbe ? Bun.spawnSync(["repo", "tools", "list", "--json"]) : undefined;
const python = registry?.exitCode === 0 ? JSON.parse(registry.stdout.toString()).find((tool: { id: string }) => tool.id === "python")?.command : undefined;
assert(!stackProbe || python, "Pinned Python required for the stack diagnostic");
assert(!sampleRetirement || process.platform === "darwin" && !stackProbe && label && modes.length === 1 && modes[0] === "solve");
try {
  for (const task of modes) {
    const artifactId = `wfc-seeded-${task}`;
    useFolder("seed", artifactId);
    const executable = gateway ? resolve(gateway) : join(generated, "native-host", "debug", native("semio-os-mcp"));
    const cli = withMcpHostDriverPolicy(["stdio", ...catalog.args, "--folder", folder, "--no-bridge", "--auto-approve", "all"]);
    const transport = new StdioClientTransport({
      command: stackProbe ? python : executable,
      args: stackProbe ? [join(import.meta.dirname, "wfc-seeded-stack-launch.py"), executable, ...cli] : cli,
      cwd: workspace, env, stderr: "pipe",
    });
    const client = new Client({ name: "semio-mcp-sdk-seeded-wfc", version: "1" });
    const start = performance.now();
    const progress: unknown[] = [];
    let stderr = "";
    let sample: Promise<number> | undefined;
    try {
      const connect = client.connect(transport, { signal: AbortSignal.timeout(45_000), timeout: 45_000 });
      transport.stderr?.setEncoding("utf8").on("data", (chunk: string) => {
        stderr = (stderr + chunk).slice(-65_536);
        if (!sampleRetirement || sample || !stderr.includes("[DEBUG] relay retirement body:") || !transport.pid) return;
        const path = join(generated, `wfc-${label}-sample.txt`);
        console.log("[DEBUG] sampling exact gateway after body refusal", transport.pid, Math.round(performance.now() - start), path);
        sample = Bun.spawn(["repo", "capacity", "run", "--memory=32", "--", "/usr/bin/sample", String(transport.pid), "2", "1", "-mayDie", "-file", path], {
          cwd: workspace,
          env: { ...process.env, SEMIO_HOST_CAPACITY: "off" },
          stdout: "inherit",
          stderr: "inherit",
        }).exited;
      });
      await connect;
      assert.equal(client.getServerVersion()?.name, "semio");
      console.log("[DEBUG] gateway PID", transport.pid);
      const call = async (name: string, args: Record<string, unknown> = {}, onprogress?: (progress: Progress) => void) => {
        const result = await client.callTool({ name, arguments: args }, CallToolResultSchema, { timeout: name === "inference_run" ? solveCallMilliseconds : 50_000, onprogress });
        console.log("[DEBUG] tool", name, Math.round(performance.now() - start), JSON.stringify(result.structuredContent).slice(0, name === "inference_list" ? 300 : 2200));
        return result;
      };
      const context = await call("context_resolve", { locale: pins.caller.locale });
      assert.equal(context.structuredContent?.channel, "headless");
      if (schemaProbe) {
        try {
          const resource = await client.readResource({ uri: `semio://artifact/${artifactId}/schema` }, { timeout: 10_000 });
          output[`${task}-schema`] = resource;
          const content = resource.contents[0];
          assert(content && typeof content.text === "string");
          const identity = JSON.parse(content.text);
          assert.equal(identity.artifactId, artifactId);
          assert.equal(identity.artifactKind ?? identity.schema, pins.inferenceArtifactKind);
          console.log("[DEBUG] schema", JSON.stringify(identity));
        } catch (error) {
          output[`${task}-schema-failure`] = String(error);
          console.error("[DEBUG] schema", String(error));
        }
      }
      const listed = await call("inference_list");
      assert.notEqual(listed.isError, true);
      const service = (listed.structuredContent?.declared as Record<string, unknown>[]).find(row =>
        row.artifactKind === pins.inferenceArtifactKind && row.inferenceSchema === pins.inferenceSchema);
      const contract = service?.payload as Record<string, unknown> | undefined;
      assert.equal((contract?.artifactBinding as Record<string, string>)?.field, "document");
      assert(typeof contract?.outputSchema === "string");
      const outputSchema = JSON.parse(contract.outputSchema);
      output[`${task}-service`] = service;
      const binding = { artifactId, artifactKind: pins.inferenceArtifactKind, inferenceSchema: pins.inferenceSchema, pluginId };
      const args = { inferenceSchema: binding.inferenceSchema, pluginId, revision: 0, documentGeneration: 1, cancellationId: artifactId, ...pins.inferenceLimits };
      if (task === "solve") {
        const result = await call("inference_run", parseInferenceRunInput({ ...args, ...binding }), row => {
          progress.push({ milliseconds: Math.round(performance.now() - start), ...row });
          console.log("[DEBUG] solve progress", JSON.stringify(progress.at(-1)));
        });
        output.solve = { result, progress };
        const verdict = inferenceClientVerdictV1(result, binding, outputSchema);
        assert(verdict.ok, verdict.detail);
        const valid = new Ajv({ strict: false }).addFormat("base64", value =>
          Buffer.from(value, "base64").toString("base64") === value).compile(outputSchema);
        const payload = result.structuredContent?.payload as Record<string, unknown>;
        assert(valid(payload), JSON.stringify(valid.errors));
        assert.equal(payload.contradiction, false);
        assert.equal(typeof payload.pixels, "string");
        const pixels = Buffer.from(payload.pixels as string, "base64");
        assert.equal(pixels.length, 24 * 24);
        assert.equal(pixels.toString("base64"), payload.pixels);
        assert(pixels.every(pixel => pixel < 3));
        if ("width" in payload) assert.equal(payload.width, 24);
        if ("height" in payload) assert.equal(payload.height, 24);
        if ("palette" in payload) assert.equal((payload.palette as unknown[]).length, 3);
        if ("entropy" in payload) assert.equal((payload.entropy as unknown[]).length, 24 * 24);
        console.log("[DEBUG] output semantics", JSON.stringify({ cells: pixels.length, paletteIndicesUsed: [...new Set(pixels)].sort(), pixelDigest: hash(pixels), progress: progress.length }));
      } else {
        if (schemaProbe) {
          try {
            const absentId = `${artifactId}-absent`;
            const absent = await call("inference_submit", parseInferenceSubmitInput({ ...args, documentId: absentId, cancellationId: absentId }));
            output["cancel-absent-document"] = absent;
            assert.equal(absent.isError, true);
            assert.equal(absent.structuredContent?.code, "PLUGIN_UNAVAILABLE");
            assert.equal(absent.structuredContent?.retryable, true);
          } catch (error) {
            output["cancel-absent-document-failure"] = String(error);
            console.error("[DEBUG] missing-folder-document", String(error));
          }
        }
        if (openForSubmit) {
          const opened = await call("artifact_open", { artifactId });
          output["cancel-open"] = opened;
          assert.notEqual(opened.isError, true);
          assert.equal(opened.structuredContent?.artifactKind ?? opened.structuredContent?.kind, pins.inferenceArtifactKind);
        }
        const submitted = await call("inference_submit", parseInferenceSubmitInput({ ...args, documentId: artifactId }));
        output.cancel = { submitted };
        if (submitted.isError) {
          await client.ping({ timeout: 10_000 });
          output["cancel-server-alive-after-refusal"] = true;
          throw Error(JSON.stringify(submitted.structuredContent));
        }
        const jobHandle = submitted.structuredContent?.jobHandle;
        assert.equal(typeof jobHandle, "string");
        const job = (page: typeof submitted) => page.structuredContent?.job as Record<string, unknown> | undefined;
        const active = (row: ReturnType<typeof job>) => ["ACCEPTED", "RUNNING"].includes(String(row?.state).toUpperCase());
        const progressCount = (row: ReturnType<typeof job>) => Array.isArray(row?.progress) ? row.progress.length : 0;
        const observe = async (page: typeof submitted, waiting: (row: ReturnType<typeof job>) => boolean) => {
          const deadline = performance.now() + observationMilliseconds;
          for (let poll = 0; !page.isError && waiting(job(page)) && performance.now() < deadline; poll++) {
            await Bun.sleep(Math.min(10 * 2 ** poll, 500));
            page = await call("inference_events", { jobHandle });
          }
          return page;
        };
        const milestone = cancelDuringHash ? activationCancel.report : "running in the plugin's guest";
        const dispatched = (row: ReturnType<typeof job>) => {
          if (!Array.isArray(row?.progress)) return false;
          if (!cancelDuringHash) return row.progress.some(progress => progress.message === milestone);
          const progress = row.progress.at(-1);
          return progress?.message === milestone && progress.fraction > activationCancel.startsAt && progress.fraction < activationCancel.finishesAt;
        };
        const observed = await observe(submitted, row => active(row) && (!progressCount(row) || !dispatched(row)));
        assert(dispatched(job(observed)), `No active inference progress was observed at ${milestone}`);
        const cancelled = await call("inference_cancel", { jobHandle });
        const terminal = await observe(cancelled, active);
        useFolder("verify", artifactId);
        output.cancel = { submitted, observed, cancelled, terminal, persistedPairIdentical: true };
        await client.ping({ timeout: 10_000 });
        output["cancel-server-alive-after-terminal"] = true;
        assert.notEqual(terminal.isError, true);
        assert(inferenceCancellationVerdictV1(job(observed), job(terminal), true), "The server did not confirm cancellation of an observed progressing job without committed effects");
        if (cancelDuringHash) {
          const valid = new Ajv({ strict: true }).compile({
            type: "object", required: ["before", "after", "proposalState", "persistedPairIdentical"], additionalProperties: false,
            properties: {
              before: { const: activationCancel.before }, after: { const: activationCancel.after },
              proposalState: { const: activationCancel.proposalState }, persistedPairIdentical: { const: activationCancel.persistedPairIdentical },
            },
          });
          assert(valid({ before: job(observed)?.state, after: job(terminal)?.state, proposalState: job(terminal)?.proposalState, persistedPairIdentical: true }), JSON.stringify(valid.errors));
        }
      }
    } catch (error) {
      output[`${task}-failure`] = String(error);
      output[`${task}-stderr-tail`] = stderr;
      output[`${task}-progress`] = progress;
      console.error("[DEBUG]", task, String(error), stderr);
    } finally {
      try {
        if (sample) output[`${task}-retirement-sample-exit`] = await sample;
        await client.close();
      } finally {
        useFolder("verify", artifactId);
        output[`${task}-persisted-pair-identical`] = true;
      }
    }
  }
} finally {
  await writeFile(join(generated, `wfc-seeded-${modes.join("-")}${stackProbe ? "-stack" : ""}${openForSubmit ? "-opened" : ""}${label ? `-${label}` : ""}.json`), JSON.stringify(output, null, 2));
  await rm(work, { recursive: true });
}
if (modes.some(task => output[`${task}-failure`] || output[`${task}-schema-failure`]) || output["cancel-absent-document-failure"]) process.exitCode = 1;
