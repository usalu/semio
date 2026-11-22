import { isDeepStrictEqual } from "node:util";
import { constants } from "node:fs";
import { copyFile, mkdir, mkdtemp, readFile, rm, stat, writeFile } from "node:fs/promises";
import { join, resolve, sep } from "node:path";
import { createNativeBuildCallerCapabilitiesV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";
import { withMcpClientSessionV1, driveOsMcpClientSessionV1, inferenceClientVerdictV1, verifyActiveInferenceCancellationV1, mcpServerEntries, takeMcpPluginCatalog, withMcpHostDriverPolicy, type McpClientDocumentPhaseV1, type McpClientSessionV1, type McpClientStep } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { encodeMcpComponentModelManifestV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/📦️component-models/🟦️.ts";
import { parseInferenceRunInput } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts";
import { readNativeCatalogV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📥️runtime/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
import { SPECIFIC_MCP_CLIENT_E2E_PINS_V1 as pins } from "../../../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts";
import initialFixture from "../../../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🧫️initial-artifact.json" with { type: "json" };
import { componentModelDirectoryKeyV1 } from "@semio-tech/framework-plugin/model/contract";
import { actorPreparationImageIdentityV1, actorPreparationPolicyV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🌱️actor-preparation/🟦️.ts";
import { createIssuedResourceControlV1, createScriptProcessEnvelope, withScriptProcessEnvelope } from "@semio-tech/process/routing";
import { stripCoreNamesV1 } from "./mcp-native-wasm-names.ts";

/** 🧾️ Reads the exact Pack and SPR bytes exposed by the artifact's public snapshot. */
async function snapshotPair(session: McpClientSessionV1, artifactId: string) {
  const snapshot = await session.call("artifact_snapshot", { artifactId });
  const content = (snapshot.structuredContent?.untrusted as { content?: { packBase64?: string; sprBase64?: string } } | undefined)?.content;
  return !snapshot.isError && content?.packBase64 && content.sprBase64 ? [content.packBase64, content.sprBase64] as const : undefined;
}

/** 💡️ Isolates the real WFC tool path when Note blocks the combined journey. */
async function probeWfc(session: McpClientSessionV1, checkInitial: boolean): Promise<readonly McpClientStep[]> {
  const steps: McpClientStep[] = [];
  const record = (step: string, ok: boolean, wire: unknown) => {
    const detail = JSON.stringify(wire)?.slice(0, 800) ?? "No response payload";
    steps.push({ step, ok, detail, wire });
    console.log(`${ok ? "PASS" : "FAIL"}  ${step} — ${detail}`);
    return ok;
  };
  const initialized = await session.request("initialize", {
    protocolVersion: "2025-06-18",
    capabilities: { roots: { listChanged: false } },
    clientInfo: { name: "semio-mcp-wfc-probe", version: "1" },
  });
  if (!record("wfc: initialize", !initialized.error, initialized.error ?? initialized.result)) return steps;
  await session.notify("notifications/initialized", {});
  const context = await session.call("context_resolve", { locale: pins.caller.locale });
  if (!record("wfc: headless context", !context.isError && context.structuredContent?.channel === "headless", context.structuredContent)) return steps;
  const listed = await session.call("inference_list", {});
  const declared = listed.structuredContent?.declared as readonly { artifactKind: string; inferenceSchema: string; payload?: { outputSchema?: string } }[] | undefined;
  const service = declared?.find(row => row.artifactKind === pins.inferenceArtifactKind && row.inferenceSchema === pins.inferenceSchema);
  const outputSchema = service?.payload?.outputSchema;
  if (!record("wfc: pinned inference discovery", !listed.isError && Boolean(outputSchema), service ?? listed.structuredContent)) return steps;
  let artifactId = `mcp-wfc-${Date.now().toString(36)}`;
  if (checkInitial) {
    const malformed = { ...initialFixture.solve, input: { ...initialFixture.solve.input, pixels: initialFixture.invalidPixels } };
    const rejected = await session.call("artifact_create", { artifactId, kind: pins.inferenceArtifactKind, initial: malformed });
    const missing = await session.call("artifact_snapshot", { artifactId });
    const ownerRefused = rejected.isError === true && rejected.structuredContent?.code === "SIDE_EFFECT_REJECTED" && String(rejected.structuredContent?.message).includes("Bitmap pixels require canonical base64");
    record("wfc: failed owner JSON import leaves the ID available", ownerRefused && missing.isError === true && missing.structuredContent?.code === "NOT_FOUND", { rejected: rejected.structuredContent, missing: missing.structuredContent });
    if (missing.isError !== true || missing.structuredContent?.code !== "NOT_FOUND") artifactId += "-fresh";
  }
  const binding = { artifactId, artifactKind: pins.inferenceArtifactKind, inferenceSchema: pins.inferenceSchema, pluginId: pins.inferencePluginId };
  const created = await session.call("artifact_create", { artifactId, kind: binding.artifactKind, initial: pins.inferenceInitial });
  if (!record("wfc: create pinned inference problem", !created.isError && created.structuredContent?.status === "SUCCEEDED" && created.structuredContent?.kind === binding.artifactKind, created.structuredContent)) return steps;
  if (checkInitial) {
    const before = await snapshotPair(session, artifactId);
    const duplicate = await session.call("artifact_create", { artifactId, kind: binding.artifactKind, initial: pins.inferenceCancellationInitial });
    const after = await snapshotPair(session, artifactId);
    if (!record("wfc: one successful create fixes the persisted owner pair", Boolean(before) && duplicate.isError === true && duplicate.structuredContent?.code === "PRECONDITION_FAILED" && isDeepStrictEqual(after, before), { stored: Boolean(before), duplicate: duplicate.structuredContent, unchanged: isDeepStrictEqual(after, before) })) return steps;
  }
  const request = parseInferenceRunInput({ ...binding, ...pins.inferenceLimits, revision: 0, documentGeneration: 1, cancellationId: `${artifactId}-solve` });
  const solution = await session.call("inference_run", request);
  const verdict = inferenceClientVerdictV1(solution, binding, JSON.parse(outputSchema!));
  const payload = solution.structuredContent?.payload as Record<string, unknown> | undefined;
  if (!record("wfc: solve pinned inference problem", verdict.ok && Object.entries(pins.inferenceExpectedOutput).every(([key, value]) => isDeepStrictEqual(payload?.[key], value)), solution.structuredContent)) return steps;
  await verifyActiveInferenceCancellationV1(session, pins, artifactId, step => {
    steps.push(step);
    console.log(`${step.ok ? "PASS" : "FAIL"}  ${step.step} — ${step.detail}`);
  });
  return steps;
}

const workspace = process.cwd();
const mode = "prototype";
const [executable, emitter, ...flags] = process.argv.slice(2);
if (!executable || !emitter) throw Error("Native MCP executable and real descriptor emitter required");
function diagnosticHostLimit(flag: string, maximum: number): number | undefined {
  const matches = flags.filter(row => row === flag || row.startsWith(flag + "="));
  const value = matches[0]?.slice(flag.length + 1);
  if (matches.length > 1 || (value !== undefined && (!/^[1-9][0-9]*$/u.test(value) || Number(value) > maximum))) {
    throw Error(`${flag}=<limit> requires one positive bound up to ${maximum}`);
  }
  return value === undefined ? undefined : Number(value);
}
const policyItems = diagnosticHostLimit("--host-driver-policy", 448_000_000);
const policyCopyBytes = diagnosticHostLimit("--host-driver-copy-bytes", 24_000_000_000);
const policyCapacityBytes = diagnosticHostLimit("--host-driver-capacity-bytes", 8_000_000_000);
const policyReleaseBytes = diagnosticHostLimit("--host-driver-release-bytes", 8_000_000_000);
const policyTurnItems = diagnosticHostLimit("--host-driver-turn-items", 4096);
const journeyMilliseconds = diagnosticHostLimit("--journey-ms", 900_000) ?? 360_000;
const publishedLaunch = flags.includes("--published-launch");
const generated = join(import.meta.dirname, "🗑️generated");
const usePrepared = flags.includes("--use-prepared-models");
await mkdir(generated, { recursive: true });
const selected = [pins.inferencePluginId, pins.pluginId];
const only = flags.find(row => row.startsWith("--emit-only="))?.slice("--emit-only=".length);
const probe = flags.find(row => row.startsWith("--probe-model="))?.slice("--probe-model=".length);
const noteReplay = flags.find(row => row.startsWith("--note-replay="))?.slice("--note-replay=".length);
const one = only ?? probe;
if (one && !selected.includes(one)) throw Error("A pinned plugin is required for a single-model run");
const reuse = flags.find(row => row.startsWith("--verified-models="))?.slice("--verified-models=".length);
const root = reuse ? resolve(reuse) : await mkdtemp(join(generated, "m-"));
if (!root.startsWith(generated + sep)) throw Error("A model root in the ticket generated directory is required");
await mkdir(root, { recursive: true });
const folder = await mkdtemp(join(root, "s-"));
const entry = mcpServerEntries(workspace).semio;
const catalog = takeMcpPluginCatalog(entry.args.slice(4), workspace);
if (!catalog.env.SEMIO_PLUGIN_CATALOG) throw Error("Explicit catalog required");
const { plugins } = readNativeCatalogV1(JSON.parse(await readFile(join(catalog.env.SEMIO_PLUGIN_CATALOG, "🦀️catalog.json"), "utf8")));
const nativeArgs = withMcpHostDriverPolicy(["stdio", ...catalog.args, "--folder", folder, "--no-bridge", "--auto-approve", "all"]);
const models = [];
let descriptor;
try {
  const diagnosticPolicy = {
    maximumItems: policyItems,
    maximumCopyBytes: policyCopyBytes,
    maximumCapacityBytes: policyCapacityBytes,
    maximumReleaseBytes: policyReleaseBytes,
  };
  const overrides = Object.entries(diagnosticPolicy).filter(([, value]) => value !== undefined);
  if (publishedLaunch && (overrides.length || policyTurnItems !== undefined)) {
    throw Error("The published launcher uses its configured host driver policy");
  }
  if ((overrides.length || policyTurnItems !== undefined) && !only) {
    const index = nativeArgs.indexOf("--host-driver-policy") + 1;
    const policy = JSON.parse(await readFile(nativeArgs[index], "utf8"));
    Object.assign(policy.operation, Object.fromEntries(overrides));
    if (policyTurnItems !== undefined) policy.drive.maximumItems = policyTurnItems;
    const diagnostic = join(folder, "host-driver-policy.json");
    await writeFile(diagnostic, JSON.stringify(policy));
    nativeArgs[index] = diagnostic;
    console.log(`[DEBUG] host driver operation item bound ${policy.operation.maximumItems}, copy bytes ${policy.operation.maximumCopyBytes}, capacity bytes ${policy.operation.maximumCapacityBytes}, release bytes ${policy.operation.maximumReleaseBytes}, drive item bound ${policy.drive.maximumItems}`);
  }
  for (const pluginId of one ? [one] : selected) {
    const plugin = plugins.find(row => row.pluginId === pluginId);
    if (!plugin) throw Error(`No catalog owner for ${pluginId}`);
    const modelRoot = join(root, "browser", mode, "components", componentModelDirectoryKeyV1(plugin.cratePath));
    await mkdir(modelRoot, { recursive: true });
    const component = join(modelRoot, "📦️component.wasm");
    if (!reuse || only) {
      const linked = join(generated, "native-guests", "wasm32-wasip2", "debug", plugin.wasmOut);
      const maximumBytes = 512 * 1024 ** 2;
      if ((await stat(linked)).size > maximumBytes) {
        const removed = await stripCoreNamesV1(linked, component, maximumBytes);
        console.log(`[DEBUG] removed ${removed} optional name-section bytes from ${pluginId} and verified every retained section`);
      } else {
        await copyFile(linked, component, constants.COPYFILE_FICLONE);
      }
      const emit = Bun.spawn([resolve(emitter), "describe", component, "--out", modelRoot, ...(usePrepared ? ["--prepare-actor"] : [])], { stdout: "inherit", stderr: "inherit" });
      if (await emit.exited) throw Error(`Descriptor emission failed for ${pluginId}`);
    }
    const json = await readFile(join(modelRoot, "🔣️.json"));
    const pack = await readFile(join(modelRoot, "🛂️.descriptor.semio"));
    const pair = verifySemanticDescriptorPairBytesV1(json, pack);
    if (pair.pluginId !== pluginId) throw Error(`Wrong emitted component identity for ${pluginId}`);
    console.log(`[DEBUG] verified ${pluginId} component descriptor ${pair.semanticSha256} at ${modelRoot}`);
    if (pluginId === pins.pluginId) descriptor = JSON.parse(json.toString());
    const preparedActor = usePrepared ? await actorPreparationImageIdentityV1(join(modelRoot, actorPreparationPolicyV1.image), new AbortController().signal) : undefined;
    models.push({ pluginId, cratePath: plugin.cratePath, root: modelRoot, ...(preparedActor ? { preparedActor } : {}) });
  }
  if (!only) {
    const environment: NodeJS.ProcessEnv = {
      ...process.env, ...catalog.env, ...entry.env,
      SEMIO_SCRIPT_PROGRESS: "human",
      SEMIO_REPO_ROOT: workspace,
      ...(flags.includes("--backtrace") ? { RUST_BACKTRACE: "1" } : {}),
    };
    const publishedArgs = [...entry.args];
    if (publishedLaunch) {
      delete environment.SEMIO_OS_MCP_BIN;
      delete environment.SEMIO_MCP_COMPONENT_MODELS;
      const folderIndex = publishedArgs.indexOf("--folder");
      if (folderIndex < 0) throw Error("The configured launcher requires a folder argument");
      publishedArgs[folderIndex + 1] = folder;
      publishedArgs.push("--no-bridge", "--auto-approve", "all");
    } else {
      environment.SEMIO_MCP_COMPONENT_MODELS = encodeMcpComponentModelManifestV1({ version: 1, mode, models });
    }
    const grant = createNativeBuildCallerCapabilitiesV1(workspace, ["run", "@semio-tech/s-dev-composition:mcp-client-e2e"], "nx", { ...environment, SEMIO_BUILD_MODE: mode });
    const envelope = createScriptProcessEnvelope({ version: 1, owner: "mcp:direct-journey", maximumElapsedMilliseconds: journeyMilliseconds }, grant, Date.now());
    await withScriptProcessEnvelope(envelope, async invocation => {
      const phase = createIssuedResourceControlV1(invocation, "mcp:direct-journey", journeyMilliseconds);
      const launch = publishedLaunch
        ? { command: entry.command, args: publishedArgs, cwd: workspace, environment }
        : { command: resolve(executable), args: nativeArgs, cwd: workspace, environment };
      let noteCapture: {
        artifactId: string;
        input: unknown;
        observations: Partial<Record<McpClientDocumentPhaseV1, { packBase64: string; sprBase64: string }>>;
      } | undefined;
      const observeNote = noteReplay ? (point: McpClientDocumentPhaseV1, artifactId: string, input: unknown, packAndSpr: string) => {
        const [packBase64, sprBase64] = packAndSpr.split(".");
        if (!packBase64 || !sprBase64) throw Error(`Typed Note ${point} supplied no exact Pack and SPR`);
        noteCapture ??= { artifactId, input, observations: {} };
        if (noteCapture.artifactId !== artifactId || noteCapture.observations[point]) throw Error("Typed Note capture changed its artifact or repeated a history step");
        noteCapture.observations[point] = { packBase64, sprBase64 };
      } : undefined;
      const { steps: os, note } = await withMcpClientSessionV1(invocation, phase, launch, async session => {
        const reported = new Set<string>();
        const flushStderr = (all: boolean) => {
          for (const line of session.stderrLines()) {
            if (!line || reported.has(line) || (!all && !line.startsWith("[DEBUG] "))) continue;
            reported.add(line);
            console.error(line.startsWith("[DEBUG] ") ? line : `[DEBUG] ${line}`);
          }
        };
        const trace = flags.includes("--backtrace") ? setInterval(() => flushStderr(false), 1_000).unref() : undefined;
        try {
          const steps = probe === pins.inferencePluginId
            ? await probeWfc(session, flags.includes("--check-initial"))
            : await driveOsMcpClientSessionV1(session, pins, async () => descriptor?.manifest.apps.find((app: { id: string }) => app.id === pins.caller.appId), observeNote);
          if (steps.some(step => !step.ok)) flushStderr(true);
          if (!steps.some(step => step.step === "os: history_redo (mutation restored)" && step.ok)) return { steps };
          const created = steps.find(step => step.step === "os: artifact_create (a real plugin artifact kind)" && step.ok);
          const artifactId = (created?.wire as { artifactId?: unknown } | undefined)?.artifactId;
          if (typeof artifactId !== "string") throw Error("The typed Note create response needs its artifact identity");
          const pair = await snapshotPair(session, artifactId);
          if (!pair) throw Error("Typed Note snapshot needs both persisted document bytes");
          return { steps, note: { artifactId, pair } };
        } catch (error) {
          flushStderr(true);
          throw error;
        } finally {
          clearInterval(trace);
        }
      });
      if (note) await withMcpClientSessionV1(invocation, phase, launch, async session => {
        const initialized = await session.request("initialize", {
          protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "semio-mcp-note-reopen", version: "1" },
        });
        if (initialized.error) throw Error(`Typed Note restart initialize failed: ${initialized.error.message}`);
        await session.notify("notifications/initialized", {});
        const reopened = await snapshotPair(session, note.artifactId);
        if (!isDeepStrictEqual(reopened, note.pair)) throw Error("Typed Note Pack and SPR changed across the gateway restart");
        console.log(`PASS  os: typed Note survives restart with identical Pack and SPR — artifactId=${note.artifactId}`);
      });
      if (note && noteReplay) {
        if (!noteCapture || Object.keys(noteCapture.observations).length !== 5) throw Error("Typed Note replay requires all five history snapshots");
        const capturePath = join(folder, "note-replay.json");
        await writeFile(capturePath, JSON.stringify(noteCapture));
        const verified = Bun.spawn([process.execPath, join(import.meta.dir, "note-mcp-replay-verify.ts"), resolve(noteReplay), capturePath], { stdout: "inherit", stderr: "inherit" });
        if (await verified.exited) throw Error("Typed Note history did not reconstruct the expected document content");
      }
      const failed = os.filter(step => !step.ok);
      for (const step of failed) console.error(`[DEBUG] ${step.step}: ${JSON.stringify(step.wire ?? step.detail).slice(0, 8192)}`);
      if (failed.length) process.exitCode = 1;
    });
  }
} finally {
  await rm(reuse || only || flags.includes("--keep-models") ? folder : root, { recursive: true, force: true });
}
