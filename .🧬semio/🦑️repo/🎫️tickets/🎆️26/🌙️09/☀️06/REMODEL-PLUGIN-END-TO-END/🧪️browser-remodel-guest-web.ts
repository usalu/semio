import { rendererOriginalWorkStock } from "../../../../../../../🧰️framework/🔨️modules/🖼️canvas/🧵️renderer-work/🟦️.ts";
import { encodePackValue, decodePackValue, decodeInvocationResultPacks, viewContextWireValue } from "../../../../../../../🧰️framework/🔨️modules/🎁️packs/🟦️.ts";
import { actorInstanceLifetimeEquals, decodeActorInstanceLifecycle } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts";
import { decodeActorUiPatchReceipt } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts";
import { DEFAULT_SHARD_BUDGET } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts";
import { continueOpeningTurns } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🚚️more-work-drive/🟦️.ts";
import { createShardCommandIngressPages, type ShardCommandIngressPage } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts";
import { shellFrameBytes } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts";
import { encodeAppCommand } from "../../../../../../../🧰️framework/🔨️modules/🧾️app-channel/🎛️command-codec/🟦️.ts";
import { decodeAppFrame } from "../../../../../../../🧰️framework/🔨️modules/🧾️app-channel/🎞️frame-codec/🟦️.ts";
import { windowViewContext } from "../../../../../../../🧰️framework/🔨️modules/🛂️manifest/👁️view-state-and-public-invocation/🟦️.ts";

export async function run(frameData?: string, editorMode?: "editor" | "picker" | "read") {
  const turns: object[] = [];
  const frames: ReturnType<typeof decodeAppFrame>[] = [];
  let phase = "bootstrap";
  const progress = (value: string) => {
    phase = value;
    console.debug("[DEBUG] Guest probe " + value);
  };
  let receipts = 0;
  const bigint = (_key: string, value: unknown) => typeof value === "bigint" ? value.toString() : value;
  try {
    progress("bootstrap");
    const owner = rendererOriginalWorkStock();
    const admission = await owner.admit();
    const role = editorMode ? "editor" : "viewer";
    const returns: Promise<unknown>[] = [];
    const holds: unknown[] = [];
    progress("instantiate");
    const { createActorApi } = await import(new URL("./module/probe-bridge.mjs", import.meta.url).href);
    const guest = await createActorApi("remodel-isolated-" + role, 1n, admission, () => BigInt(Math.floor(performance.now() * 1000)), {
      receipt(value: Parameters<typeof owner.receive>[1]) {
        receipts++;
        returns.push(owner.receive(admission, value));
      },
      hold(value: Parameters<typeof owner.hold>[1]) {
        holds.push(value);
        returns.push(owner.hold(admission, value));
      },
    });
    const poll = async (label: string, event?: object, commandPage?: ShardCommandIngressPage) => {
      progress(label);
      const started = performance.now();
      let result;
      try { result = await guest.poll(event ? [event] : [], commandPage, undefined, DEFAULT_SHARD_BUDGET); }
      finally { await Promise.all(returns.splice(0)); }
      if (holds.length) throw Error("Guest held original work");
      turns.push({
        label, ms: Math.round(performance.now() - started), lifecycleBytes: result.lifecycleReceipt?.length ?? 0,
        patchReceiptBytes: result.uiPatchReceipt?.length ?? 0, patchCount: result.uiPatches.length,
        effects: result.effects, status: result.status, nextWake: result.nextWake, workUnits: result.retainedReceipt.workUnitsSpent, spent: result.retainedReceipt.spent, commandIngress: result.commandIngress.tag,
      });
      return result;
    };
    const instance = 1;
    const appId = "s.remodel.remodeling@1/*#" + role;
    let result = await poll(role + "-open", {
      kind: "instance-open",
      payload: { instance, activationGeneration: 1n, requestSequence: 1, appId, actor: "local", config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})) },
    });
    result = await continueOpeningTurns(result, () => !result.lifecycleReceipt, () => poll(role + "-open-continuation"));
    const captured = decodeActorInstanceLifecycle(result.lifecycleReceipt);
    if (captured.kind !== "captured" || captured.lifetime.instanceId !== instance || captured.requestSequence !== 1) throw Error("Expected exact instance capture receipt");
    await poll(role + "-capture-ack", { kind: "instance-lifecycle-ack", payload: { kind: "ack", receipt: captured } });
    const renderSurface = async (label: string, surfaceId: string, bodyKey: string, viewState: unknown) => {
      const surface = { instance, surface: surfaceId };
      let result = await poll(label + "-visible", { kind: "surface-visible", payload: { surface, bodyKey, viewState: encodePackValue(viewState) } });
      for (let attempt = 0; !result.uiPatches.length && result.status?.tag === "more-work" && attempt < 256; attempt++) result = await poll(label + "-continuation");
      const patchReceipt = decodeActorUiPatchReceipt(result.uiPatchReceipt);
      if (!actorInstanceLifetimeEquals(captured.lifetime, patchReceipt.lifetime)) throw Error("Patch belongs to a different guest lifetime");
      const patch = result.uiPatches.find((item: { surface: typeof surface }) => item.surface.instance === instance && item.surface.surface === surface.surface);
      if (!patch) throw Error("Missing actual surface patch");
      const nodes = patch.ops.filter((operation: { tag: string }) => operation.tag === "upsert").map((operation: { val: { node: Uint8Array } }) => decodePackValue(operation.val.node));
      await poll(label + "-patch-ack", { kind: "patch-ack", payload: { surface, revision: patch.revision, receipt: patchReceipt } });
      return { patchReceipt, patch, nodes };
    };
    if (editorMode) {
      let seq = 0;
      const command = async (label: string, make: (seq: number) => Parameters<typeof encodeAppCommand>[0]) => {
        seq++;
        const bytes = encodeAppCommand(make(seq));
        const pages = createShardCommandIngressPages({ owner: BigInt(instance), generation: 1n, commandIndex: 0, commandCount: 1, instance, seq: BigInt(seq), command: bytes });
        const accept = (turn: Awaited<ReturnType<typeof poll>>) => {
          for (const effect of turn.effects) {
            const payload = shellFrameBytes(effect, instance);
            if (payload) frames.push(decodeAppFrame(payload));
          }
          if (turn.commandIngress.tag === "fault") throw Error("Guest command ingress fault: " + JSON.stringify(turn.commandIngress, bigint));
          return turn.commandIngress.tag;
        };
        let status = "";
        for (const page of pages) status = accept(await poll(label + "-page", undefined, page));
        for (let attempt = 0; status !== "command-complete" && attempt < 1024; attempt++) status = accept(await poll(label + "-continuation"));
        if (status !== "command-complete") throw Error("Guest command ingress did not complete");
        return seq;
      };
      if (editorMode === "read") {
        const readSeq = await command("editor-read-identity", (seq) => ({ ReadDocumentIdentity: { seq: BigInt(seq) } }));
        const identity = frames.find((frame) => "DocumentIdentity" in frame && frame.DocumentIdentity.in_reply_to === BigInt(readSeq));
        if (!identity) throw Error("Guest emitted no editor identity response");
        return JSON.parse(JSON.stringify({ ok: true, appId, captured, receipts, turns, frames, identity }, bigint));
      }
      const picker = editorMode === "picker";
      const windowId = "remodeling-frames";
      const windowInstances = ["remodeling-main", windowId].map((id) => ({ id, windowKindId: id }));
      const viewState = viewContextWireValue(windowViewContext({ locale: "en", terminology: "native", activeModeId: "model", focusedWindowId: windowId, windowInstances }, windowId));
      await renderSurface("editor-frames", windowId, "remodeling.play.frames", viewState);
      const args = { payload: frameData, name: "two-by-two.png", index: 0, total: 1, windowId };
      const actionId = picker ? "importFrames" : "importFramePayload";
      const invocation = { address: { pluginId: "remodel", appId, modeId: "model", windowKindId: windowId, windowInstanceId: windowId, actionId }, arguments: picker ? { windowId } : args };
      const invocationSeq = await command(picker ? "editor-request-picker" : "editor-import-frame", (seq) => ({
        Command: { seq, command: Array.from(encodePackValue(invocation)), view_state: Array.from(encodePackValue(viewState)) },
      }));
      if (picker) {
        const request = turns.flatMap((turn) => (turn as { effects: { tag: string; val?: { params?: { importAction?: string } } }[] }).effects).find((effect) => effect.tag === "request-file-open" && effect.val?.params?.importAction === "importFramePayload");
        if (!request) throw Error("Guest emitted no frame-import picker request");
        return JSON.parse(JSON.stringify({ ok: true, appId, captured, receipts, turns, frames, request }, bigint));
      }
      const readSeq = await command("editor-read-document", (seq) => ({ ReadDocument: { seq } }));
      const reply = frames.find((frame) => "Invocation" in frame && frame.Invocation.in_reply_to === invocationSeq);
      const invocationResult = reply && "Invocation" in reply ? decodeInvocationResultPacks(reply.Invocation) : null;
      const snapshot = frames.find((frame) => "Document" in frame && frame.Document.in_reply_to === readSeq);
      const ops = snapshot && "Document" in snapshot ? snapshot.Document.ops : "";
      if (!ops.includes(args.name) || !ops.includes(frameData!.split(",")[1])) throw Error("Guest document did not contain the submitted frame");
      return JSON.parse(JSON.stringify({ ok: true, appId, captured, receipts, turns, frames, invocationResult, ops }, bigint));
    }
    const { patchReceipt, patch, nodes } = await renderSurface("viewer-model", "remodeling.view.scene3d/model", "remodeling.view.model", { locale: "en", terminology: "native" });
    if (!JSON.stringify(nodes, bigint).includes("remodeling-result")) throw Error("Expected Remodel viewer model node: " + JSON.stringify(nodes, bigint).slice(0, 4500));
    return JSON.parse(JSON.stringify({ ok: true, appId, captured, patchReceipt, patch, nodes, receipts, turns }, bigint));
  } catch (error) {
    return JSON.parse(JSON.stringify({ ok: false, phase, message: error instanceof Error ? error.stack : String(error), detail: error instanceof Error ? error.cause ?? (error as Error & { originalCall?: unknown }).originalCall : error, receipts, turns, frames }, bigint));
  }
}
