/** 🧾️ The app channel client: the typed facade over one plugin instance's app channel, with its outcome pump, request correlation and document pack cache.
 *
 * 🧱️ The children own the rest: `🏷️codec-primitives`, `🎛️command-codec`, `🎞️frame-codec`, `📋️report-decoders`, `🔢️request-sequence` (channel version admission and request sequence) and `🗃️archive-load-host`.
 */

import type { ConflictResolution, MergePolicy, PluginDispatchHintV1, TurnOutcome } from "@semio-tech/framework";
import { conflictResolutionAsU8, mergePolicyAsU8 } from "@semio-tech/framework";
import type { ArtifactPresencePeer, LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryCommand, LocalInteractionQueryReply, MutationEnvelope } from "@semio-tech/framework-replication";
import { encodePresencePeer } from "@semio-tech/framework-replication";
import { decodePackValue, decodePackWire, encodePackValue, viewContextWireValue } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎁️packs/🟦️.ts";
import { type AppCommandValue, type AppDocumentIdentity, type AppFrameValue, type ChildHeadPackEntry, type DocumentArchiveLoadStatus, type DocumentArchivePack, type MediaExportHandle, type OperationCompletionV1, type WindowConfigPackEntry } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🪢️wire-contracts/🟦️.ts";
import { assertAppDocumentIdentity } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧾️app-channel/🏷️codec-primitives/🟦️.ts";
import { encodeAppCommand } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧾️app-channel/🎛️command-codec/🟦️.ts";
import { decodeAppFrame } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧾️app-channel/🎞️frame-codec/🟦️.ts";
import { decodeFaultFromWire, faultDisplayMessage } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧾️app-channel/📋️report-decoders/🟦️.ts";
import { appChannelFrameBelongsTo, type AppChannelHandle, appChannelReplySequence, AppChannelRequestSequence, appChannelTransactionReply, type AppChannelTransactionReply, type LocalInteractionClientQuery, sameLocalInteractionQuery } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧾️app-channel/🔢️request-sequence/🟦️.ts";
import { DOCUMENT_ARCHIVE_UNMERGEABLE_CODES, DocumentArchiveFaultError, type DocumentArchiveLoadAnswer, DocumentArchiveLoadHost, documentArchiveLoadReplyV1, type DocumentArchiveMergeV1, nextArchivePollTurnV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧾️app-channel/🗃️archive-load-host/🟦️.ts";

//#region 🔖️AppChannelClient
/** 🎞️ One unsolicited mid-operation `Invocation` frame as {@link AppChannelClient.onOperationProgress} hands it on: the
 * decoded `UiDirtyScope` and `HistoryPatch` it carries, each absent when the frame carried none. */
export type AppChannelOperationProgressV1 = { readonly uiScope?: unknown; readonly historyPatch?: unknown };

/**
 * 📡️ Typed facade over one plugin instance's app channel — encodes an {@link AppCommandValue}, queues
 * it via {@link PluginWasmHandle.enqueue}, and decodes every {@link AppFrameValue} the matching
 * {@link TurnOutcome} carries. This is the ONLY place `AppCommand`/`AppFrame` framing happens on the
 * host side; callers (a React renderer's dispatch/refresh loop, a headless workflow runner) work with
 * decoded frames and plain JS values, never raw bytes or wire tags. `seq` is a handle-owned monotonic
 * counter — the host has no other way to correlate a `Command`/`ConfigCommand`/
 * `ReadDocument`/`LoadConfig`/`ReadConfig` with the `Invocation`/`Document` frame(s) it produced
 * (`AppFrame.*.in_reply_to`). Channel v12 retired the `hello()`/`refreshUi()`/`attachBackbone()`/
 * `detachBackbone()`/`drain()` surface this class used to expose — the handshake, cache-probed UI
 * refresh, and empty-batch drain all disappeared with the reactor ABI (lifecycle now arrives via
 * `Event::InstanceOpen`/`InstanceClose`, UI updates are a `UiPatch` push, and guests wake on
 * events/timers/`next-wake` rather than a poll).
 *
 * `📌️important.md`'s "Replace, never wrap" list: the old handle's synchronous
 * per-call method used to hand this class its reply directly; the handle is now fire-and-forget
 * ({@link PluginWasmHandle.enqueue}) and replies arrive on the handle-wide {@link
 * PluginWasmHandle.outcomes} stream instead, so THIS class is what turns that stream back into the
 * one-reply-per-call shape every method below still returns. One background loop
 * ({@link pumpOutcomes}) owns the handle's async iterator for this instance's lifetime and routes
 * every explicit numeric receipt to its exact waiter. Empty outcomes and uncorrelated notifications
 * cannot complete a command. Transaction payloads accompany their numeric receipt and additionally
 * match the pending transaction identity. One outcome may complete several distinct commands.
 */
export class AppChannelClient {
  private localQuery: LocalInteractionClientQuery | null = null;
  private readonly completionListeners = new Set<(completion: OperationCompletionV1) => void>();
  private readonly progressListeners = new Set<(progress: AppChannelOperationProgressV1) => void>();
  private disposed = false;
  private retired = false;
  private readonly handle: AppChannelHandle;
  private readonly instanceId: number;
  private readonly appId: string;
  private readonly actor: string;
  private readonly sequenceOwner: AppChannelRequestSequence;
  private readonly outcomeIterator: AsyncIterator<TurnOutcome>;
  private readonly pending: { readonly seq: number; readonly queryReceipt: boolean; readonly transaction: AppChannelTransactionReply | null; readonly resolve: (frames: AppFrameValue[]) => void; readonly reject: (error: unknown) => void }[] = [];
  /** 📦️ Per-instance document-pack cache (ticket
   * 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS, scout-1 §4: "the
   * browser host keeps NO document pack per instance today"). Populated from BOTH directions —
   * {@link loadDocumentArchive}'s ready root pair and every
   * `AppFrame::Document` reply any sent command's outcome carries (`ReadDocument`, or any future command that
   * happens to include one) — so a transaction coordinator can
   * ask "what does this instance's document look like right now" without a dedicated round trip. */
  private cachedPack: Uint8Array | null = null;
  private cachedSpr: Uint8Array | null = null;
  /** 👥️ The last `AppFrame::Ephemeral` any outcome of this instance carried (every guest exchange appends one,
   * contract-freeze §C7.6) — the presence heartbeat's pack and interaction slice, read without a round trip (the
   * native twin: `ProgramBridge::observe_ephemeral`). */
  private lastEphemeral: Extract<AppFrameValue, { readonly Ephemeral: unknown }>["Ephemeral"] | null = null;

  constructor(handle: AppChannelHandle, sequenceOwner: AppChannelRequestSequence, instanceId: number, appId: string, actor: string = "local") {
    this.handle = handle;
    this.sequenceOwner = sequenceOwner;
    this.instanceId = instanceId;
    this.appId = appId;
    this.actor = actor;
    this.outcomeIterator = handle.outcomes[Symbol.asyncIterator]();
    void this.pumpOutcomes();
  }

  /** 🔁️ Owns this client's subscription against the handle-wide {@link PluginWasmHandle.outcomes}
   * stream for its whole lifetime — runs until {@link dispose} calls the iterator's own `return()`
   * (what breaks this loop) or the handle itself completes it. Every outcome for a DIFFERENT
   * `instanceId` is silently skipped (it belongs to a sibling `AppChannelClient` on the same handle);
   * stale or duplicate numeric receipts do not consume another command's waiter. Query pages retain
   * their separate ACK-owned lifecycle even when coalesced with ordinary replies or notifications. */
  private async pumpOutcomes(): Promise<void> {
    for (;;) {
      const step = await this.outcomeIterator.next();
      if (step.done) {
        this.cachedPack = null;
        this.cachedSpr = null;
        this.finishLocalInteractionQuery(new Error("local-interaction.channel-closed"));
        for (const waiter of this.pending.splice(0)) waiter.reject(new Error("app-channel.closed"));
        return;
      }
      const outcome = step.value;
      if (outcome.instanceId !== this.instanceId) continue;
      if ("error" in outcome) {
        this.finishLocalInteractionQuery(outcome.error);
        this.pending.shift()?.reject(outcome.error);
        continue;
      }
      const frames: AppFrameValue[] = [];
      for (const encoded of outcome.frames) {
        try { frames.push(decodeAppFrame(encoded)); }
        catch (error) {
          this.cancelLocalInteractionQuery(error);
          for (let index = this.pending.length - 1; index >= 0; index -= 1) {
            if (!this.pending[index]!.queryReceipt) this.pending.splice(index, 1)[0]!.reject(error);
          }
        }
      }
      const ordinary: AppFrameValue[] = [];
      for (const frame of frames) {
        if ("Ephemeral" in frame) this.lastEphemeral = frame.Ephemeral;
        if ("LocalInteractionQuery" in frame) this.receiveLocalInteractionQuery(frame.LocalInteractionQuery.reply);
        else if ("OperationCompleted" in frame) this.publishOperationCompletion(frame.OperationCompleted);
        else if ("Invocation" in frame && frame.Invocation.in_reply_to === 0 && (frame.Invocation.ui_scope.length > 0 || frame.Invocation.history_patch.length > 0)) this.publishOperationProgress(frame.Invocation);
        else ordinary.push(frame);
      }
      const correlated = new Set(ordinary.flatMap((frame) => { const sequence = appChannelReplySequence(frame); return sequence === null ? [] : [sequence]; }));
      for (let index = 0; index < this.pending.length;) {
        const waiter = this.pending[index]!;
        if (!correlated.has(waiter.seq)) { index += 1; continue; }
        this.pending.splice(index, 1);
        const reply = ordinary.filter((frame) => appChannelFrameBelongsTo(frame, waiter.seq, waiter.transaction));
        this.captureDocumentFrames(reply);
        waiter.resolve(reply);
      }
      this.finishDisposal();
    }
  }

  /** 👥️ This instance's last published presence pack, interaction slice and tool-run and history-edit summaries (each
   * summary empty while none is open), `null` until the guest published one. */
  ephemeral(): { readonly presence: readonly number[]; readonly presenceGeneration: number; readonly transientGeneration: number; readonly interaction: readonly number[]; readonly toolRun: readonly number[]; readonly historyEdit: readonly number[] } | null {
    const frame = this.lastEphemeral;
    return frame === null ? null : { presence: frame.presence, presenceGeneration: frame.presence_generation, transientGeneration: frame.transient_generation, interaction: frame.interaction, toolRun: frame.tool_run, historyEdit: frame.history_edit };
  }

  /** 🏁️ Subscribes to this instance's unsolicited typed-operation completions. A mounted operation
   * finishes turns after the command that started it already resolved, so its terminal
   * `AppFrame::OperationCompleted` matches no pending waiter and would otherwise be dropped — this is
   * the ONLY delivery path for it. Returns the unsubscribe. */
  onOperationCompleted(listener: (completion: OperationCompletionV1) => void): () => void {
    this.completionListeners.add(listener);
    return () => this.completionListeners.delete(listener);
  }

  /** 🎞️ Subscribes to this instance's unsolicited progress: the `UiDirtyScope` a running operation (a tool run's trace,
   * provisional pieces and progress) and the `HistoryPatch` a running history replay (its stage and `done/total`) hand
   * the shell mid-operation as an `Invocation` frame answering no command. It is best effort — the shell refreshes as
   * often as it keeps up with — and the ONLY delivery path for either once the operation runs on the background drain.
   * Returns the unsubscribe. */
  onOperationProgress(listener: (progress: AppChannelOperationProgressV1) => void): () => void {
    this.progressListeners.add(listener);
    return () => this.progressListeners.delete(listener);
  }

  private publishOperationProgress(frame: { readonly ui_scope: readonly number[]; readonly history_patch: readonly number[] }): void {
    if (this.disposed || this.progressListeners.size === 0) return;
    const progress: AppChannelOperationProgressV1 = {
      ...(frame.ui_scope.length > 0 ? { uiScope: decodePackWire(new Uint8Array(frame.ui_scope), "$.uiScope") } : {}),
      ...(frame.history_patch.length > 0 ? { historyPatch: decodePackWire(new Uint8Array(frame.history_patch), "$.historyPatch") } : {}),
    };
    for (const listener of [...this.progressListeners]) {
      try { listener(progress); }
      catch (error) { console.error("[TRACE] operation progress subscriber failed", error); }
    }
  }

  /** 🏁️ Decodes one completion frame's pack-encoded carriers and fans it out to every subscriber.
   * A throwing subscriber never starves its siblings nor the outcome pump. */
  private publishOperationCompletion(frame: Extract<AppFrameValue, { readonly OperationCompleted: unknown }>["OperationCompleted"]): void {
    if (this.disposed || this.completionListeners.size === 0) return;
    const completion: OperationCompletionV1 = {
      instanceId: this.instanceId,
      operation: frame.operation,
      revision: frame.revision,
      uiScope: frame.ui_scope.length ? decodePackWire(new Uint8Array(frame.ui_scope), "$.uiScope") : undefined,
      historyPatch: frame.history_patch.length ? decodePackWire(new Uint8Array(frame.history_patch), "$.historyPatch") : undefined,
    };
    for (const listener of [...this.completionListeners]) {
      try { listener(completion); }
      catch (error) { console.error("[TRACE] operation completion subscriber failed", error); }
    }
  }

  /** 🪦️ Closes this instance's COMMAND QUEUE and settles everything already in it, WITHOUT ending the
   * background {@link pumpOutcomes} subscription — the first half of `destroyApp`
   * (`PluginRuntime/🟦️.tsx`), run before the actor's close ladder rather than after it.
   *
   * The ladder runs on the actor's own lifecycle lease and never speaks through this channel, so while
   * it ran the queue stayed open: a gesture dispatched into a dying instance was admitted here and only
   * refused one layer down by `requireActorId`, as `no actor for instance N` plus a stack. Retiring the
   * queue first turns every such arrival into this channel's own typed terminal. The subscription is
   * deliberately NOT ended here — {@link dispose} still owns that, so a close the native side REFUSES
   * leaves the iterator exactly where it was and the retry can still retire it. */
  retire(): void {
    if (this.retired) return;
    this.retired = true;
    this.completionListeners.clear();
    this.cachedPack = null;
    this.cachedSpr = null;
    for (let index = this.pending.length - 1; index >= 0; index -= 1) {
      if (!this.pending[index]!.queryReceipt) this.pending.splice(index, 1)[0]!.reject(new Error("app-channel.disposed"));
    }
    if (this.localQuery) this.cancelLocalInteractionQuery(new Error("local-interaction.disposed"));
  }

  /** 🔌️ Ends this client's background {@link pumpOutcomes} subscription — call once from
   * `destroyApp` (`PluginRuntime/🟦️.tsx`) so a torn-down instance doesn't leak a live
   * subscriber against the handle-wide outcome stream for the rest of the handle's lifetime. */
  dispose(): void {
    this.retire();
    this.disposed = true;
    this.finishDisposal();
  }

  private finishDisposal(): void {
    if (this.disposed && this.localQuery === null && this.pending.length === 0) void this.outcomeIterator.return?.();
  }

  private nextSeq(): number {
    return this.sequenceOwner.nextSequence();
  }

  /** 📦️ Scans every frame one sent command's outcome carried for `AppFrame::Document` and refreshes
   * the pack cache — the "every `AppFrame::Document` reply" half of the cache-population contract. */
  private captureDocumentFrames(frames: readonly AppFrameValue[]): void {
    if (this.disposed || frames.some(frame => "Error" in frame)) return;
    for (const frame of frames) {
      if ("Document" in frame) {
        this.cachedPack = new Uint8Array(frame.Document.pack);
        this.cachedSpr = new Uint8Array(frame.Document.spr);
      }
    }
  }

  /** 📦️ The cached `{pack, spr}` for this instance's document, or `null` before any
   * ready {@link loadDocumentArchive} or `AppFrame::Document` reply has been observed. Surfaced to the
   * transaction coordinator through the `PluginWasmHandle` adapter's own `documentPack` accessor
   * (`PluginRuntime/🟦️.tsx`) — a contributor plan call needs the target's current snapshot
   * pack, and this is the only place that snapshot is retained host-side. */
  documentPack(): { readonly pack: Uint8Array; readonly spr: Uint8Array } | null {
    return this.cachedPack && this.cachedSpr ? { pack: this.cachedPack.slice(), spr: this.cachedSpr.slice() } : null;
  }

  /** 🔀️ Queues one encoded command and resolves with every frame its matching {@link TurnOutcome}
   * carries — see this class's own header doc for how the reply gets correlated back to this call.
   * `dispatch` ({@link PluginDispatchHintV1}) is forwarded verbatim to {@link PluginWasmHandle.enqueue};
   * only {@link command} ever passes one — `configure`/`readDocument`/… are maintenance and stay
   * unordered (arrival order). */
  private sendCommand(command: AppCommandValue, dispatch?: PluginDispatchHintV1): Promise<AppFrameValue[]> {
    if (this.disposed || this.retired) return Promise.reject(new Error("app-channel.disposed"));
    return new Promise<AppFrameValue[]>((resolve, reject) => {
      const wireSequence = Object.values(command)[0]!.seq;
      const seq = Number(wireSequence);
      if (!Number.isSafeInteger(seq) || seq < 0) { reject(new Error("app-channel.invalid-sequence-owner")); return; }
      const waiter = { seq, queryReceipt: false, transaction: appChannelTransactionReply(command), resolve, reject };
      this.pending.push(waiter);
      try { this.handle.enqueue(this.instanceId, [encodeAppCommand(command)], dispatch); }
      catch (error) {
        const index = this.pending.indexOf(waiter);
        if (index !== -1) this.pending.splice(index, 1);
        reject(error);
      }
    });
  }

  //#region 🏠️LocalInteractionQuery
  /** 📃️ Each page remains native-owned until its consumer resolves; completion waits for exact native root retirement. */
  readLocalInteractionPages(consume: (page: LocalInteractionPage) => Promise<void>, signal?: AbortSignal): Promise<LocalInteractionIdentity> {
    if (this.disposed || this.retired) return Promise.reject(new Error("app-channel.disposed"));
    if (this.localQuery) return Promise.reject(new Error("local-interaction.busy"));
    if (signal?.aborted) return Promise.reject(new Error("local-interaction.cancelled"));
    let admission: ReturnType<AppChannelRequestSequence["nextQuery"]>;
    try { admission = this.sequenceOwner.nextQuery(); }
    catch (error) { return Promise.reject(error); }
    const requestId = admission.request;
    return new Promise((resolve, reject) => {
      const abort = () => this.cancelLocalInteractionQuery(new Error("local-interaction.cancelled"));
      this.localQuery = { requestId, cancelSequence: admission.cancelSequence, consume, resolve, reject, signal, abort, token: null, nextOrdinal: 0n, consuming: false, cancelled: false, terminalConsumed: false, failure: null };
      signal?.addEventListener("abort", abort, { once: true });
      this.sendLocalInteractionQuery(admission.sequence, { kind: "read", requestId });
    });
  }

  private sendLocalInteractionQuery(seq: number, command: LocalInteractionQueryCommand): void {
    this.pending.push({ seq, queryReceipt: true, transaction: null, resolve: () => {}, reject: (error: unknown) => this.finishLocalInteractionQuery(error) });
    try { this.handle.enqueue(this.instanceId, [encodeAppCommand({ LocalInteractionQuery: { seq, command } })]); }
    catch (error) {
      const index = this.pending.findIndex((waiter) => waiter.seq === seq);
      if (index !== -1) this.pending.splice(index, 1);
      this.finishLocalInteractionQuery(error);
    }
  }

  private cancelLocalInteractionQuery(error: unknown): void {
    const query = this.localQuery;
    if (!query || query.cancelled) return;
    query.cancelled = true;
    query.failure = error;
    if (query.token) this.sendLocalInteractionQuery(query.cancelSequence, { kind: "cancel", token: query.token });
  }

  private finishLocalInteractionQuery(error: unknown = null): void {
    const query = this.localQuery;
    if (!query) return;
    this.localQuery = null;
    for (let index = this.pending.length - 1; index >= 0; index -= 1) {
      if (this.pending[index]!.queryReceipt) this.pending.splice(index, 1);
    }
    query.signal?.removeEventListener("abort", query.abort);
    const failure = query.failure ?? error;
    if (failure !== null) query.reject(failure);
    else if (query.token && query.terminalConsumed) query.resolve(query.token.identity);
    else query.reject(new Error("local-interaction.incomplete-close"));
    this.finishDisposal();
  }

  private receiveLocalInteractionQuery(reply: LocalInteractionQueryReply): void {
    const query = this.localQuery;
    if (!query) return;
    if (reply.kind === "rejected") {
      if (reply.requestId === query.requestId) this.finishLocalInteractionQuery(new Error(`local-interaction.${reply.code}`));
      return;
    }
    if (reply.kind === "started") {
      if (reply.token.requestId !== query.requestId || reply.token.identity.appInstanceId !== this.instanceId || query.token !== null) return;
      query.token = reply.token;
      if (query.cancelled) this.sendLocalInteractionQuery(query.cancelSequence, { kind: "cancel", token: query.token });
      return;
    }
    const token = reply.kind === "page" ? reply.page : reply.token;
    if (!query.token || !sameLocalInteractionQuery(query.token, token)) return;
    if (reply.kind === "closed") {
      if (!query.cancelled && (!query.terminalConsumed || token.ordinal !== query.token.ordinal)) return;
      this.finishLocalInteractionQuery(reply.cancelled ? new Error("local-interaction.cancelled") : null);
      return;
    }
    if (query.cancelled || query.consuming || token.ordinal !== query.nextOrdinal.toString()) return;
    query.token = { requestId: token.requestId, queryGeneration: token.queryGeneration, identity: token.identity, ordinal: token.ordinal };
    query.consuming = true;
    void Promise.resolve().then(() => query.consume(reply.page)).then(() => {
      if (this.localQuery !== query) return;
      query.consuming = false;
      if (query.cancelled) return;
      query.terminalConsumed = reply.page.terminal;
      query.nextOrdinal += 1n;
      let seq: number;
      try { seq = this.nextSeq(); }
      catch (error) { this.cancelLocalInteractionQuery(error); return; }
      this.sendLocalInteractionQuery(seq, { kind: "acknowledge", token: query.token! });
    }, (error: unknown) => {
      if (this.localQuery !== query) return;
      query.consuming = false;
      this.cancelLocalInteractionQuery(error);
    });
  }
  //#endregion 🏠️LocalInteractionQuery

  /** 🎛️ Forwards one opaque app-specific command (already encoded by the caller's own command
   * grammar) plus the current view state; may return several frames (`Invocation` + any dirtied
   * `UiPatch`es) — routing them is the caller's job. `dispatch` ({@link PluginDispatchHintV1}) carries
   * the input ledger's causal `order` for this one command; it steers only the command's position in
   * the actor's queue, never its content. */
  async command(commandBytes: Uint8Array, viewState: unknown, dispatch?: PluginDispatchHintV1): Promise<AppFrameValue[]> {
    return this.sendCommand({
      Command: { seq: this.nextSeq(), command: Array.from(commandBytes), view_state: Array.from(encodePackValue(viewContextWireValue(viewState))) },
    }, dispatch);
  }

  async configure(config: unknown): Promise<AppFrameValue[]> {
    return this.sendCommand({ ConfigCommand: { seq: this.nextSeq(), command: Array.from(encodePackValue(config)) } });
  }

  async readDocument(): Promise<AppFrameValue[]> {
    return this.sendCommand({ ReadDocument: { seq: this.nextSeq() } });
  }

  /** 🪪️ Reads the actor's trusted document envelope identity without exporting its bytes. */
  async readDocumentIdentity(): Promise<AppDocumentIdentity> {
    const seq = BigInt(this.nextSeq());
    const frames = await this.sendCommand({ ReadDocumentIdentity: { seq } });
    const fault = frames.find((frame) => "Error" in frame);
    if (fault && "Error" in fault) throw new Error(`document identity: ${faultDisplayMessage(fault.Error.fault, decodePackValue)}`);
    const replies = frames.filter((frame): frame is Extract<AppFrameValue, { readonly DocumentIdentity: unknown }> => "DocumentIdentity" in frame && frame.DocumentIdentity.in_reply_to === seq);
    if (replies.length !== 1) throw new Error("document identity: missing or ambiguous reply");
    const identity = replies[0]!.DocumentIdentity.identity;
    assertAppDocumentIdentity(identity);
    if (identity.app_instance_id !== this.instanceId) throw new Error("document identity: foreign instance");
    return identity;
  }

  /** 🪆️ Reads every owned child's CURRENT head snapshot pack (`AppCommand::ReadChildHeads` → `AppFrame::ChildHeads`), for a
   * reader that composes parent + children on read (design §20.15). A guest fault rejects with the guest's own fault text. */
  async readChildHeads(): Promise<readonly ChildHeadPackEntry[]> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({ ReadChildHeads: { seq } });
    const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (error) throw new Error(`AppChannelClient.readChildHeads(${this.appId}): ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
    const heads = frames.find((frame): frame is Extract<AppFrameValue, { readonly ChildHeads: unknown }> => "ChildHeads" in frame && frame.ChildHeads.in_reply_to === seq);
    if (!heads) throw new Error(`AppChannelClient.readChildHeads(${this.appId}): missing ChildHeads frame for seq ${seq}`);
    return heads.ChildHeads.entries;
  }

  /** 📤️ Produces one OUT port's current bytes (`AppCommand::MediaOut` → `AppFrame::Media`) — the
   * agent-facing export lane, which returns the guest's own bytes to the caller instead of handing
   * the human a download. Deliberately the SAME command the headless gateway drives, so an export
   * over the live-shell route and one over the headless route ask the guest the identical question. */
  async mediaOut(port: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ MediaOut: { seq: this.nextSeq(), port, request: [] } });
  }

  /** 📂️ Delivers one typed media artifact to a fresh or existing app instance. */
  async mediaIn(port: string, descriptor: unknown, data: Uint8Array): Promise<AppFrameValue[]> {
    return this.sendCommand({ MediaIn: { seq: this.nextSeq(), port, descriptor: Array.from(encodePackValue(descriptor)), data: Array.from(data) } });
  }

  /** 🎬️ Admits media work only against the caller's exact document and revision. */
  async submitMediaExport(port: string, parentDocumentId: string, revision: bigint): Promise<AppFrameValue[]> {
    return this.sendCommand({ SubmitMediaExport: { seq: BigInt(this.nextSeq()), port, expected_parent_document_id: parentDocumentId, expected_base_revision: revision } });
  }

  /** ⏱️ Advances one bounded media-export slice. */
  async pollMediaExport(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.sendCommand({ PollMediaExport: { seq: BigInt(this.nextSeq()), handle } });
  }

  /** 🛑️ Cancels exactly the submitted media generation. */
  async cancelMediaExport(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.sendCommand({ CancelMediaExport: { seq: BigInt(this.nextSeq()), handle } });
  }

  /** 📦️ Takes one bounded output chunk and observes terminal owner retirement. */
  async takeMediaExportChunk(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.sendCommand({ TakeMediaExportChunk: { seq: BigInt(this.nextSeq()), handle } });
  }

  /** 🗃️ Restores one root envelope and its complete recursive owned-member closure atomically, stepped through
   * {@link DocumentArchiveLoadHost}: `progress` hears every polled status, `signal` cancels. A cancel before admission sends
   * nothing; after it the guest restores the previous document, and the load rejects with the signal's reason — or an
   * `AbortError` when the guest cancelled on its own (a person's Cancel in the history body). A cancel that lost the race
   * resolves, because the document did load. A refusal or a guest fault rejects with the guest's own fault text. Driven by
   * {@link driveDocumentArchive}, like a merge. */
  async loadDocumentArchive(
    archive: DocumentArchivePack,
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<void> {
    const root = { pack: Uint8Array.from(archive.parent_pack), spr: Uint8Array.from(archive.parent_spr) };
    await this.driveDocumentArchive("loadDocumentArchive", new DocumentArchiveLoadHost(archive), [], signal, progress);
    this.cachedPack = root.pack;
    this.cachedSpr = root.spr;
  }

  /** 🔀️ Merges a read-back of the document this program already shows (design §22.22), stepped through
   * {@link DocumentArchiveLoadHost.merging}: the archive's events join the program's log, nothing is replaced, an open history
   * edit and the program's viewed alternative stay. Resolves `merged` with the events it took and the events the program is
   * `ahead` of the archive by — the cached root pair is dropped when it took any — or `unmergeable` with the guest's code
   * ({@link DOCUMENT_ARCHIVE_UNMERGEABLE_CODES}), which touched nothing. Cancels, refusals and every other fault reject like
   * {@link loadDocumentArchive}'s. */
  async mergeDocumentArchive(
    archive: DocumentArchivePack,
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<DocumentArchiveMergeV1> {
    const merge = DocumentArchiveLoadHost.merging(archive);
    const ended = await this.driveDocumentArchive("mergeDocumentArchive", merge, DOCUMENT_ARCHIVE_UNMERGEABLE_CODES, signal, progress);
    if ("code" in ended) return { kind: "unmergeable", code: ended.code };
    if (ended.completed > 0) {
      this.cachedPack = null;
      this.cachedSpr = null;
    }
    return { kind: "merged", merged: ended.completed, ahead: merge.ahead };
  }

  /** 🚶️ Drives one whole-document load or merge to its acknowledged end: resolves with the count its ready terminal completed,
   * or with the guest's fault code when it is one of `unmergeable`. A cancelled operation rejects with the signal's reason
   * (an `AbortError` when the guest cancelled on its own); a refusal and every other fault reject with a
   * {@link DocumentArchiveFaultError} carrying the guest's own fault, a reply that answers nothing with a plain error. */
  private async driveDocumentArchive(
    name: string,
    host: DocumentArchiveLoadHost,
    unmergeable: readonly string[],
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<{ readonly completed: number } | { readonly code: string }> {
    let completed = 0;
    for (;;) {
      if (signal?.aborted) host.requestCancel();
      const step = host.step(() => this.nextSeq());
      if (step.kind === "finished") {
        if (step.outcome.kind === "ready") return { completed };
        if (step.outcome.kind === "cancelled") throw signal?.reason ?? new DOMException(`AppChannelClient.${name}(${this.appId}): cancelled`, "AbortError");
        const code = decodeFaultFromWire(step.outcome.fault, decodePackValue)?.code;
        if (typeof code === "string" && unmergeable.includes(code)) return { code };
        throw new DocumentArchiveFaultError(`AppChannelClient.${name}(${this.appId}): ${faultDisplayMessage(step.outcome.fault, decodePackValue)}`, decodeFaultFromWire(step.outcome.fault, decodePackValue));
      }
      const seq = step.seq;
      const replyFrames = await this.sendCommand(step.command);
      const frame = documentArchiveLoadReplyV1(replyFrames, seq);
      const answer: DocumentArchiveLoadAnswer = frame ? host.answer(seq, frame) : { kind: "unanswered" };
      if (answer.kind === "refused") throw new DocumentArchiveFaultError(`AppChannelClient.${name}(${this.appId}): ${faultDisplayMessage(answer.fault, decodePackValue)}`, decodeFaultFromWire(answer.fault, decodePackValue));
      if (answer.kind === "unanswered") throw new Error(`AppChannelClient.${name}(${this.appId}): no answer to seq ${seq} of operation ${host.operation}`);
      if (answer.status) {
        completed = answer.status.completed;
        progress?.(answer.status);
      }
      if (answer.status ? answer.status.state === "pending" || answer.status.state === "running" : frame !== undefined && "Error" in frame) await nextArchivePollTurnV1();
    }
  }

  /** 🗃️ Reads one generation-fenced root envelope and its complete recursive owned-member closure. */
  async readDocumentArchive(): Promise<DocumentArchivePack> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({ ReadDocumentArchive: { seq } });
    const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (error) throw new Error(`AppChannelClient.readDocumentArchive(${this.appId}): ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
    const response = frames.find(
      (frame): frame is Extract<AppFrameValue, { readonly DocumentArchive: unknown }> =>
        "DocumentArchive" in frame && frame.DocumentArchive.in_reply_to === seq,
    );
    if (!response) throw new Error(`AppChannelClient.readDocumentArchive(${this.appId}): missing DocumentArchive frame for seq ${seq}`);
    this.cachedPack = Uint8Array.from(response.DocumentArchive.archive.parent_pack);
    this.cachedSpr = Uint8Array.from(response.DocumentArchive.archive.parent_spr);
    return {
      parent_pack: [...response.DocumentArchive.archive.parent_pack],
      parent_spr: [...response.DocumentArchive.archive.parent_spr],
      members: response.DocumentArchive.archive.members.map((entry) => ({
        ordinal: entry.ordinal,
        reference: { ...entry.reference },
        owner: { parent: { ...entry.owner.parent }, slot: entry.owner.slot, child_id: entry.owner.child_id },
        envelope_pack: [...entry.envelope_pack],
      })),
    };
  }

  /** 🪟️ Restores one exact concrete window's persisted-local config envelope. */
  async loadWindowConfig(entry: WindowConfigPackEntry): Promise<void> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({
      LoadWindowConfig: {
        seq,
        entry: { window_id: entry.window_id, window_kind_id: entry.window_kind_id, envelope_pack: Array.from(entry.envelope_pack) },
      },
    });
    const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (error) throw new Error(`AppChannelClient.loadWindowConfig(${this.appId}): ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
    if (!frames.some((frame) => "Done" in frame && frame.Done.in_reply_to === seq)) {
      throw new Error(`AppChannelClient.loadWindowConfig(${this.appId}): missing Done frame for seq ${seq}`);
    }
  }

  /** 🪟️ Reads every exact concrete window config envelope for persisted-local replay. */
  async readWindowConfigs(): Promise<readonly WindowConfigPackEntry[]> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({ ReadWindowConfigs: { seq } });
    const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (error) throw new Error(`AppChannelClient.readWindowConfigs(${this.appId}): ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
    const configs = frames.find(
      (frame): frame is Extract<AppFrameValue, { readonly WindowConfigs: unknown }> =>
        "WindowConfigs" in frame && frame.WindowConfigs.in_reply_to === seq,
    );
    if (!configs) throw new Error(`AppChannelClient.readWindowConfigs(${this.appId}): missing WindowConfigs frame for seq ${seq}`);
    return configs.WindowConfigs.entries.map((entry) => ({
      window_id: entry.window_id,
      window_kind_id: entry.window_kind_id,
      envelope_pack: Array.from(entry.envelope_pack),
    }));
  }

  /** 🧾️ Retrieves the complete history projection for initial load or cursor-gap recovery. */
  async readHistory(): Promise<AppFrameValue[]> {
    return this.sendCommand({ ReadHistory: { seq: this.nextSeq() } });
  }

  /** 📂️ Opens an artifact in its resolved (or explicitly named) viewer/editor surface —
   * `os.open-artifact` (contract-freeze §3 of
   * `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`). Empty
   * `pluginId`/`appId` means "resolve via the `OpeningResolver`". `role` is `0` Viewer, `1`
   * Editor — declaration order of `AppRole` (kernel `🔖️AppRouter` region). */
  async openArtifact(artifactRef: string, role: number, pluginId = "", appId = ""): Promise<AppFrameValue[]> {
    return this.sendCommand({ openArtifact: { seq: this.nextSeq(), artifact_ref: artifactRef, role, plugin_id: pluginId, app_id: appId } });
  }

  /** 🎚️ Pins a viewer/editor default for one `(artifactKind, standard, subset, role)` coordinate,
   * persisted event-sourced in the OS `os.config.opening` facet — `os.set-default-viewer`/
   * `os.set-default-editor`. */
  async setDefaultApp(artifactKind: string, standard: string, subset: string, role: number, pluginId: string, appId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ setDefaultApp: { seq: this.nextSeq(), artifact_kind: artifactKind, standard, subset, role, plugin_id: pluginId, app_id: appId } });
  }

  /** 🎚️ Clears a previously pinned default, falling back to the `OpeningResolver`'s owner/router
   * order — `os.clear-default-app`. */
  async clearDefaultApp(artifactKind: string, standard: string, subset: string, role: number): Promise<AppFrameValue[]> {
    return this.sendCommand({ clearDefaultApp: { seq: this.nextSeq(), artifact_kind: artifactKind, standard, subset, role } });
  }

  /** 🖱️ On-demand context menu — one `ContextMenu` reply whose `in_reply_to` matches this call's `seq`. */
  async contextMenu(request: unknown): Promise<unknown> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({
      ContextMenu: { seq, request: Array.from(encodePackValue(request)) },
    });
    const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (errorFrame) {
      throw new Error(`AppChannelClient.contextMenu(${this.appId}): ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    }
    const menuFrame = frames.find(
      (frame): frame is Extract<AppFrameValue, { readonly ContextMenu: { readonly in_reply_to: number; readonly items: readonly number[] } }> =>
        "ContextMenu" in frame && frame.ContextMenu.in_reply_to === seq,
    );
    if (!menuFrame) {
      throw new Error(`AppChannelClient.contextMenu(${this.appId}): missing ContextMenu frame for seq ${seq}`);
    }
    return decodePackValue(new Uint8Array(menuFrame.ContextMenu.items));
  }

  /** 📥️ Force-applies remote `MutationEnvelope`s through `AppCommand::ApplyEnvelopes`. */
  async applyEnvelopes(envelopes: readonly MutationEnvelope[]): Promise<AppFrameValue[]> {
    return this.sendCommand({ ApplyEnvelopes: { seq: this.nextSeq(), envelopes } });
  }

  //#region 🔖️Merge
  /** ⚖️ Sets this instance's local merge-policy authority (`os.set-merge-policy`, C6/C9) — a `Done`
   * reply, never a `MergeReport`/`Conflicts` (those only follow an ingest). */
  async setMergePolicy(policy: MergePolicy): Promise<AppFrameValue[]> {
    return this.sendCommand({ setMergePolicy: { seq: this.nextSeq(), policy: mergePolicyAsU8(policy) } });
  }

  /** ⚔️ Accepts or discards an `Open` {@link Conflict} (`os.resolve-conflict`) — replays it under
   * `LaissezFaire` on `accept` (Quarantined) or acks it in place (Degraded); `discard` on a
   * Quarantined conflict seeds the DAG as already-seen without ever relaying it, on a Degraded
   * conflict it is rejected (never rewrites shared history, C6 §`resolve_conflict`). Returns the
   * authoritative `MergeReport` + `Conflicts` frames. */
  async resolveConflict(conflictId: string, resolution: ConflictResolution): Promise<AppFrameValue[]> {
    return this.sendCommand({ resolveConflict: { seq: this.nextSeq(), conflict_id: conflictId, resolution: conflictResolutionAsU8(resolution) } });
  }

  /** 📖️ Reads the open-conflict projection (`os.read-conflicts`) — one `Conflicts` reply frame. */
  async readConflicts(): Promise<AppFrameValue[]> {
    return this.sendCommand({ readConflicts: { seq: this.nextSeq() } });
  }
  //#endregion 🔖️Merge

  //#region 🔖️Presence
  /** 👥️ Pushes the document-wide presence roster into this instance's plugin app — the ONLY plugin
   * ingress for peers (contract-freeze §C7.6). Encodes each {@link ArtifactPresencePeer} via
   * {@link encodePresencePeer}; the caller has already dropped its own actor from `peers` before
   * calling this (`ownColor` carries this actor's own hub-assigned palette index separately, `null`
   * for a folder-only session with no hub). A plain `Done` reply, never decoded further here. */
  async pushPresence(ownColor: number | null, peers: readonly ArtifactPresencePeer[]): Promise<AppFrameValue[]> {
    return this.sendCommand({ presence: { seq: this.nextSeq(), own_color: ownColor, peers: peers.map((peer) => encodePresencePeer(peer)) } });
  }
  //#endregion 🔖️Presence

  //#region 🔖️Transaction
  /** 🎫️ `TransactionPrepare`, owner-mutation wire form (contract freeze §2/§5.3): `mutationId` +
   * `payload` set, `preparedOps` empty. Sent when `ArtifactMutationRouter` resolves the mutation to
   * its OWNING plugin. */
  async transactionPrepareOwner(txnId: string, mutationId: string, payload: Uint8Array): Promise<AppFrameValue[]> {
    return this.sendCommand({
      transactionPrepare: { seq: this.nextSeq(), txn_id: txnId, mutation_id: mutationId, payload: Array.from(payload), prepared_ops: [], origin: [], prepared_child_ops: [] },
    });
  }

  /** 🎫️ `TransactionPrepare`, pre-planned wire form: `preparedOps`/`origin` set, `mutationId`
   * empty. Sent to a CONTRIBUTED-mutation target (after the host has already called the contributor's
   * `contributor.artifact-mutation-plan`) or to any member the coordinator is re-batching several
   * already-known ops onto in one call — see `PluginRuntime/🟦️.tsx`'s `TransactionCoordinator`. */
  async transactionPreparePlanned(txnId: string, preparedOps: readonly Uint8Array[], origin: Uint8Array): Promise<AppFrameValue[]> {
    return this.sendCommand({
      transactionPrepare: {
        seq: this.nextSeq(),
        txn_id: txnId,
        mutation_id: "",
        payload: [],
        prepared_ops: preparedOps.map((op) => Array.from(op)),
        origin: Array.from(origin),
        prepared_child_ops: [],
      },
    });
  }

  async transactionCommit(txnId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionCommit: { seq: this.nextSeq(), txn_id: txnId } });
  }

  async transactionRollback(txnId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionRollback: { seq: this.nextSeq(), txn_id: txnId } });
  }

  /** 🎁️ Group undo — fans out to every member of `groupId` (contract freeze §5.7); this call is one
   * member's half, the coordinator drives the fan-out. */
  async transactionUndo(groupId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionUndo: { seq: this.nextSeq(), group_id: groupId } });
  }

  async transactionRedo(groupId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionRedo: { seq: this.nextSeq(), group_id: groupId } });
  }
  //#endregion 🔖️Transaction

}
//#endregion 🔖️AppChannelClient
