/** 🧷️ Stages the browser raw picker on the real serialized actor turn and selection-only effect. */
import { join } from "node:path";

const ticket = import.meta.dirname;
const runtime = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime";
const elements = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements";
const owner = join(runtime, "🚚️handle-turn-drive");
const files: [path: string, before: string | null, after: string][] = [];
const hash = (value: string) => new Bun.CryptoHasher("sha256").update(value).digest("hex");
function change(body: string, anchor: string, replacement: string): string {
  if (body.split(anchor).length !== 2) throw new Error("raw picker source drift: " + anchor);
  return body.replace(anchor, replacement);
}
async function guard(path: string, edit: (body: string) => string): Promise<void> {
  const body = await Bun.file(path).text();
  const after = edit(body);
  if (body === after) throw new Error("raw picker empty change: " + path);
  files.push([path, hash(body), after]);
}

const components = {
  "original-media-wire.ts": "original-media-react-wire.ts",
  "original-media-exchange.ts": "original-media-react-exchange.ts",
  "original-media-pick.ts": "original-media-react-pick.ts",
  "original-media-sender.ts": "📜️original-media-ingress.ts",
};
for (const [name, source] of Object.entries(components)) {
  const path = join(owner, name);
  if (await Bun.file(path).exists()) throw new Error("raw picker target already present: " + path);
  files.push([path, null, await Bun.file(join(ticket, source)).text()]);
}

await guard(join(owner, "🟦️.ts"), body => {
  const text = change(body, "import { typedOperationAcknowledgements, withTypedOperationCall }",
    "import { createOriginalMediaExchange } from \"./original-media-exchange.ts\";\n" +
    "import { originalMediaRequest } from \"./original-media-wire.ts\";\n" +
    "import { typedOperationAcknowledgements, withTypedOperationCall, type TypedOperationCall }");
  return change(text, "  return { runQueuedTurn, drainTypedOperations };", `
  const openOriginalMediaIngress = (instanceId: number, issued: unknown) => {
    const request = originalMediaRequest(issued, instanceId);
    const actorId = requireActorId(instanceId);
    const activation = shardClient.captureActorActivation(actorId);
    const binding = documentBindings.get(instanceId);
    const scopedDocument = binding?.port.owner.scope?.documentId;
    if (scopedDocument !== undefined && scopedDocument !== request.loan.documentId) throw new Error("original-media.wrong-document");
    if (!Number.isSafeInteger(state.eventSeq + 1)) throw new Error("original-media.sequence");
    const seq = BigInt(++state.eventSeq);
    const retired = () => {
      if (state.disposing || closingInstances.has(instanceId) || actorIdByInstance.get(instanceId) !== actorId) return true;
      try { activation.assertActive(); return false; } catch { return true; }
    };
    const current = () => !retired() && documentBindings.get(instanceId) === binding && !binding?.port.closing;
    let call: TypedOperationCall | undefined;
    const results: WireTurnResult[] = [];
    let acknowledgements: readonly ShardEventEnvelope[] = [];
    const settle = async () => {
      if (results.length === 0) return;
      const turns = results.splice(0);
      const acks = acknowledgements;
      acknowledgements = [];
      const settled = await settleAcknowledgedPluginTurns(actorId, turns, acks, (turn) => acceptUiPatches(instanceId, turn), activation, call);
      await commitReservedToolSpawnsWhileSerialized(instanceId, actorId, settled.effects);
      const frames: Uint8Array[] = [];
      const pending: WireVariant[] = [...pendingTurnEffects.get(instanceId) ?? []];
      for (const effect of routeHostEffects(instanceId, settled.effects, binding?.port)) {
        const frame = shellFrameBytes(effect, instanceId);
        if (frame) frames.push(frame);
        else pending.push(effect);
      }
      if (pending.length > PLUGIN_OPERATION_EFFECT_CAPACITY) throw new Error("original-media.effect-capacity");
      pendingTurnEffects.set(instanceId, promoteShellSendMessages(instanceId, pending, frames));
      if (frames.length > 0) turnOutcomes.push({ instanceId, frames });
      if (wireTurnStatusTag(settled.status) === "more-work") void drainTypedOperations(instanceId);
    };
    return createOriginalMediaExchange(request, BigInt(instanceId), seq, {
      current,
      retired,
      assertOriginalActivation: () => activation.assertActive(),
      serialize: (run) => withTypedOperationCall(actorId, \`original-media#\${instanceId}:\${seq}\`, owned =>
        serializeCommandIngressForActor(actorId, async () => {
          call = owned;
          try { return await run(); }
          finally {
            try { if (!retired()) await settle(); }
            finally { call = undefined; }
          }
        })).then(async (answer) => { await flushReservedJobDrives(); return answer; }),
      turn: async (crossing) => {
        const turn = await submitTurn(actorId, [...acknowledgements, ...(crossing?.events ?? [])], { commandPage: crossing?.page, activation });
        const accepted = await acceptUiPatches(instanceId, turn);
        const observations = [turn, ...accepted.turns];
        const failure = observations.find((value) => value.commandIngress?.tag === "fault");
        const ingress = observations.findLast((value) => value.commandIngress?.tag === "original-media-ack" || value.commandIngress?.tag === "backpressure")?.commandIngress;
        const collected = results.length;
        results.push(...observations);
        compactAcknowledgedTurns(results, collected);
        acknowledgements = [...accepted.acknowledgements, ...observations.flatMap(typedOperationAcknowledgements)];
        if (failure) throw new Error(commandIngressFaultDisplay(failure.commandIngress));
        return { ingress, deferredEvents: turn.deferredEvents };
      },
      settle,
      yieldTurn: yieldPluginUiContinuation,
      now: () => performance.now(),
    });
  };
  return { runQueuedTurn, drainTypedOperations, openOriginalMediaIngress };`);
});

await guard(join(runtime, "🧷️handle-contract", "🟦️.ts"), body => {
  const declaration = "export type PluginWasmHandle = {";
  return change(body, declaration,
    "import type { OriginalMediaExchange } from \"../🚚️handle-turn-drive/original-media-wire.ts\";\n\n" +
    declaration + "\n  readonly openOriginalMediaIngress?: (instanceId: number, issued: unknown) => OriginalMediaExchange;");
});

await guard(join(runtime, "🧩️module-loader", "🟦️.ts"), body =>
  change(body, "return { ...richHandle, refreshUi,", "return { ...richHandle, openOriginalMediaIngress: turns.openOriginalMediaIngress, refreshUi,"),
);

await guard(join(elements, "🏛️ShellHost/⚡️host-effects/📥️document-load-effects/🟦️.ts"), body => {
  const root = "../../../🔌️PluginRuntime/🚚️handle-turn-drive/";
  const text = change(body, "import { coerceWireBytes }", "import { pickOriginalMedia } from \"" + root + "original-media-pick.ts\";\n" +
    "import { originalMediaRequest } from \"" + root + "original-media-wire.ts\";\nimport { coerceWireBytes }");
  return change(text, "  const selected = maxFileBytes === undefined", `  if (typeof args === "object" && args !== null && "originalMedia" in args) {
    const issued = args.originalMedia;
    const current = () => isCurrentEffectOwner(effectOwner);
    const host = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId)?.handle;
    if (!host?.openOriginalMediaIngress) throw new Error("original-media.host-unavailable");
    if (multiple) throw new Error("original-media.selection-count");
    const request = originalMediaRequest(issued, baseSession.instanceId);
    const exchange = host.openOriginalMediaIngress(baseSession.instanceId, request);
    const transfers = core.documentTransferRef.current;
    let pickedName = "";
    try {
      const status = await pickOriginalMedia(request, exchange, () => requestFileSelectionV1(filter), current, (file) => {
        pickedName = file.name;
        return transfers.trackDocumentTransfer(file.name, baseSession.pluginId);
      });
      if (status === "cancelled" && pickedName) transfers.notifyDocumentTransfer("import-cancelled", pickedName, "info");
    } catch (error) {
      if (!pickedName) throw error;
      const code = error instanceof Error && error.message === "The selected media exceeds its admitted source extent" ? "import-size-limit" : "import-failed";
      transfers.notifyDocumentTransfer(code, pickedName, "error");
    }
    return;
  }
  const selected = maxFileBytes === undefined`);
});

await guard("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts", body => change(body, "function normalizeCommandIngress(status) {", `function normalizeCommandIngress(status) {
  const originalMediaAck = optionValue(status.originalMediaAck);
  if (originalMediaAck !== undefined && originalMediaAck !== null) return { tag: "original-media-ack", val: originalMediaAck };
`));

for (const [path, before] of files) {
  if (path.length > 256) throw new Error("raw picker path exceeds repository limit: " + path);
  if (before && hash(await Bun.file(path).text()) !== before) throw new Error("raw picker premount source changed: " + path);
}
const output = join(ticket, "🗑️generated", "original-media-react-overlay.json");
await Bun.write(output, JSON.stringify(files));
console.log("[DEBUG] Guarded " + files.length + " browser raw picker paths");
