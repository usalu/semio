/** 📞️ App channel client: local interaction queries, turn outcomes, conflicts, merge and dispatch reports, handshake and request sequences.
 *
 * @see ../🛠️backbone-wire-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe, expect, it } from "vitest";
import { AppChannelClient, AppChannelRequestSequence, decodeAppCommand, decodeAppFrame, decodeConflictsFromWire, decodeDispatchReportFromWire, decodeMergeReportFromWire, encodeAppCommand, encodeAppFrame, faultMessages } from "@semio-tech/framework/app-channel";
import { decodePackValue, encodePackValue, packUInt } from "@semio-tech/framework/packs";
import { createTurnOutcomeBroadcast } from "@semio-tech/framework";
import { source, documentArchive, type AppChannelHandle, type AppCommandValue, type AppFrameValue, type TurnOutcome } from "../🛠️backbone-wire-helpers/🟦️.ts";

const commandSeq = (command: AppCommandValue): number => {
  const sequence = Object.values(command)[0]!.seq;
  if (typeof sequence === "bigint" && (sequence < 0n || sequence > BigInt(Number.MAX_SAFE_INTEGER))) throw new RangeError("Fixture receipt exceeds the exact Done sequence range");
  return Number(sequence);
};

type Conflict = import("@semio-tech/framework-kernel/merge-outcome").Conflict;

type DispatchReport = import("@semio-tech/framework-kernel/merge-outcome").DispatchReport;

type LocalInteractionIdentity = import("@semio-tech/framework-replication/wire/local-interaction").LocalInteractionIdentity;

type LocalInteractionQueryReply = import("@semio-tech/framework-replication/wire/local-interaction/transport").LocalInteractionQueryReply;

type LocalInteractionQueryToken = import("@semio-tech/framework-replication/wire/local-interaction").LocalInteractionQueryToken;

type MergeReport = import("@semio-tech/framework-kernel/merge-outcome").MergeReport;

describe("@semio-tech/framework-os AppChannelClient", () => {
  it("local interaction outer wire matches strict fixtures and the independent LEB128 oracle", async () => {
    const { readFileSync } = await import("node:fs");
    const oracleModule = "@webassemblyjs/leb128/lib/leb.js";
    const imported: unknown = await import(oracleModule);
    if (!imported || typeof imported !== "object") throw new Error("invalid LEB128 oracle module");
    const oracle: unknown = Reflect.get(imported, "default");
    if (!oracle || typeof oracle !== "object") throw new Error("invalid LEB128 oracle interface");
    const encodeUnsigned: unknown = Reflect.get(oracle, "encodeUIntBuffer");
    if (typeof encodeUnsigned !== "function") throw new Error("missing LEB128 oracle encoder");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8"));
    const read: AppCommandValue = { LocalInteractionQuery: { seq: 9, command: { kind: "read", requestId: "13" } } };
    const rejected: AppFrameValue = { LocalInteractionQuery: { reply: { kind: "rejected", requestId: "13", code: "busy" } } };
    const u64 = (value: bigint): number[] => {
      const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(value);
      const encoded: unknown = encodeUnsigned(bytes);
      if (!(encoded instanceof Uint8Array)) throw new Error("invalid LEB128 oracle bytes");
      return Array.from(encoded);
    };
    expect([...encodeAppCommand(read)]).toEqual([29, ...u64(9n), ...u64(2n), 0, ...u64(13n)]);
    expect(Buffer.from(encodeAppCommand(read)).toString("hex")).toBe(fixture.outerReadHex);
    expect(Buffer.from(encodeAppFrame(rejected)).toString("hex")).toBe(fixture.outerRejectedHex);
    const receipt: AppFrameValue = { Done: { in_reply_to: 2 } };
    expect([...encodeAppFrame(receipt)]).toEqual([0, ...u64(2n)]);
    expect(Buffer.from(encodeAppFrame(receipt)).toString("hex")).toBe(fixture.receiptHex);
    for (const row of fixture.sequenceCases) {
      if (row.result === null) continue;
      const sequence = row.result.sequence;
      expect([...encodeAppCommand({ ReadDocument: { seq: sequence } })]).toEqual([7, ...u64(BigInt(sequence))]);
      if (row.result.request !== null) {
        const inner = [0, ...u64(BigInt(row.result.request))];
        expect([...encodeAppCommand({ LocalInteractionQuery: { seq: sequence, command: { kind: "read", requestId: row.result.request } } })]).toEqual([29, ...u64(BigInt(sequence)), ...u64(BigInt(inner.length)), ...inner]);
      }
    }
    expect(decodeAppCommand(encodeAppCommand(read))).toEqual(read);
    expect(decodeAppFrame(encodeAppFrame(rejected))).toEqual(rejected);
    expect(() => decodeAppCommand(Uint8Array.from([...encodeAppCommand(read), 0]))).toThrow();
    expect(() => decodeAppFrame(Uint8Array.from([...encodeAppFrame(rejected), 0]))).toThrow();
  });

  it("keeps each instance's last published Ephemeral frame as its presence snapshot, from any outcome", async () => {
    const { decodePresenceInteraction, encodePresenceInteraction } = await import("@semio-tech/framework-replication");
    const presence = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/👕️canvas-presence/🔣️.json", source.url), "utf8")) as { readonly paint: readonly { readonly roster: readonly { readonly actor: string; readonly interaction?: Parameters<typeof encodePresenceInteraction>[0] }[] }[] };
    const selecting = presence.paint[0]!.roster.find((row) => row.actor === "peer-a")!.interaction!;
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    const handle: AppChannelHandle = { enqueue: () => undefined, outcomes: broadcast.stream };
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 7, "fixture");
    const flush = async () => { for (let index = 0; index < 12; index += 1) await Promise.resolve(); };
    expect(client.ephemeral()).toBeNull();
    broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Ephemeral: { presence: [1], presence_generation: 1, transient_generation: 1, interaction: [], tool_run: [], history_edit: [] } })] });
    await flush();
    expect(client.ephemeral()).toEqual({ presence: [1], presenceGeneration: 1, transientGeneration: 1, interaction: [], toolRun: [], historyEdit: [] });
    const { encodePresenceHistoryEdit } = await import("@semio-tech/framework-replication");
    const editing = encodePresenceHistoryEdit({ mutationId: "m-2", stage: "reviewing", drafts: 1 });
    broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 0 } }), encodeAppFrame({ Ephemeral: { presence: [2, 3], presence_generation: 2, transient_generation: 5, interaction: encodePresenceInteraction(selecting), tool_run: [], history_edit: editing } })] });
    broadcast.push({ instanceId: 8, frames: [encodeAppFrame({ Ephemeral: { presence: [9], presence_generation: 9, transient_generation: 9, interaction: [], tool_run: [], history_edit: [] } })] });
    await flush();
    const snapshot = client.ephemeral();
    expect(snapshot?.presence).toEqual([2, 3]);
    expect([snapshot?.presenceGeneration, snapshot?.transientGeneration]).toEqual([2, 5]);
    expect(decodePresenceInteraction(Uint8Array.from(snapshot!.interaction), [0])).toEqual(selecting);
    expect([snapshot?.toolRun, snapshot?.historyEdit]).toEqual([[], editing]);
    client.dispose();
  });

  it("local interaction client fixture lifecycles preserve ACK ownership and ordinary replies", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8"));
    for (const row of fixture.lifecycles) {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const sent: AppCommandValue[] = [];
      const handle: AppChannelHandle = { enqueue: (_id, events) => {
        for (const command of events.map(decodeAppCommand)) {
          sent.push(command);
          if ("LocalInteractionQuery" in command && !(["disposal-before-read-receipt", "coalesced-query-and-ordinary-receipts"].includes(row.id) && command.LocalInteractionQuery.command.kind === "read")) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: command.LocalInteractionQuery.seq } })] });
        }
      }, outcomes: broadcast.stream };
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 7, "fixture");
      const abort = new AbortController();
      let consume!: () => void;
      const consumed = new Promise<void>((resolve) => { consume = resolve; });
      let complete = false;
      let queryResult: Promise<LocalInteractionIdentity | unknown> | undefined;
      let ordinary: Promise<AppFrameValue[]> | undefined;
      let ordinaryComplete = false;
      const token: LocalInteractionQueryToken = { requestId: "1", queryGeneration: "41", identity: { appInstanceId: 7, generation: "9007199254740993", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
      const push = (reply: LocalInteractionQueryReply) => broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } })] });
      const flush = async () => { for (let index = 0; index < 12; index += 1) await Promise.resolve(); };
      for (const event of row.events) {
        if (event === "read") queryResult = client.readLocalInteractionPages(() => {
          if (row.id === "synchronous-consumer-failure") throw new Error("synchronous consumer fixture");
          if (row.id === "consumer-failure") return Promise.reject(new Error("consumer fixture"));
          return consumed;
        }, abort.signal).then((identity) => { complete = true; return identity; }, (error: unknown) => { complete = true; return error; });
        else if (event === "ordinary") ordinary = client.readDocument().then((frames) => { ordinaryComplete = true; return frames; });
        else if (event === "started") push({ kind: "started", token });
        else if (event === "abort") abort.abort();
        else if (event === "dispose") client.dispose();
        else if (event === "malformed") broadcast.push({ instanceId: 7, frames: [Uint8Array.from(Buffer.from(fixture.malformedFrameHex, "hex"))] });
        else if (event === "readReceipt") broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 1 } })] });
        else if (event === "wrongReceipt" || event === "duplicateReadReceipt") {
          broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: event === "wrongReceipt" ? 999 : 1 } })] });
          await flush();
          expect(ordinaryComplete).toBe(false);
        }
        else if (event === "ordinaryDone") { broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 3 } })] }); expect(await ordinary).toEqual([{ Done: { in_reply_to: 3 } }]); }
        else if (event === "ordinaryEmpty") { broadcast.push({ instanceId: 7, frames: [] }); await flush(); expect(ordinaryComplete).toBe(false); }
        else if (event === "mixedReceipts") {
          broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 1 } }), encodeAppFrame({ Done: { in_reply_to: 3 } })] });
          await flush(); expect(ordinaryComplete).toBe(true);
          expect(await ordinary).toEqual([{ Done: { in_reply_to: 3 } }]);
        }
        else if (event === "startedWithNotice" || event === "pageWithNotice") {
          const reply: LocalInteractionQueryReply = event === "startedWithNotice" ? { kind: "started", token } : { kind: "page", page: { ...token, terminal: true, bytes: [0xe2, 0x9c, 0x93] } };
          broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } }), encodeAppFrame({ Ephemeral: { presence: [], presence_generation: 0, transient_generation: 0, interaction: [], tool_run: [], history_edit: [] } }), encodeAppFrame({ UiPatch: { in_reply_to: null, surface: "fixture", kind: "graph", revision: 1, base_revision: 0, ops: [] } })] });
          await flush(); expect(ordinaryComplete).toBe(false);
        }
        else if (event === "page") push({ kind: "page", page: { ...token, terminal: true, bytes: [0xe2, 0x9c, 0x93] } });
        else if (event === "consume") consume();
        else if (event === "consumeError") await flush();
        else if (event === "ack" || event === "cancel") {
          await flush();
          const last = sent.at(-1);
          expect(last && "LocalInteractionQuery" in last ? last.LocalInteractionQuery.command : null).toEqual({ kind: event === "ack" ? "acknowledge" : "cancel", token });
        } else if (event === "closed") {
          expect(complete).toBe(false);
          push({ kind: "closed", token, cancelled: row.cancelled });
        }
        await flush();
      }
      const result = await queryResult;
      expect(complete).toBe(true);
      if (!row.cancelled) expect(result).toEqual(token.identity);
      else expect(result).toBeInstanceOf(Error);
      client.dispose();
    }
  });
  it("local interaction sequence admission matches the checked shared-owner fixture", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8"));
    for (const row of fixture.sequenceCases) {
      const owner = new AppChannelRequestSequence(row.sequence, BigInt(row.request));
      const allocate = () => row.operation === "query" ? owner.nextQuery() : { sequence: owner.nextSequence(), cancelSequence: null, request: null };
      if (row.result === null) expect(allocate).toThrow();
      else expect(allocate()).toEqual(row.result);
      expect(owner.checkpoint()).toEqual(row.after);
    }
    for (const invalid of [-1, Number.MAX_SAFE_INTEGER + 1, NaN, 0.5]) expect(() => new AppChannelRequestSequence(invalid)).toThrow();
    for (const invalid of [-1n, 0x1_0000_0000_0000_0000n]) expect(() => new AppChannelRequestSequence(0, invalid)).toThrow();
  });

  it("local interaction reopened clients reject delayed Started pages and ordinary receipts", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8")).reopen;
    const owner = new AppChannelRequestSequence();
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    const sent: AppCommandValue[] = [];
    const handle: AppChannelHandle = { enqueue: (_instance, bytes) => {
      for (const command of bytes.map(decodeAppCommand)) {
        sent.push(command);
        if ("LocalInteractionQuery" in command) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: command.LocalInteractionQuery.seq } })] });
      }
    }, outcomes: broadcast.stream };
    const flush = async () => { for (let index = 0; index < 16; index += 1) await Promise.resolve(); };
    const push = (reply: LocalInteractionQueryReply) => broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } })] });
    const oldToken: LocalInteractionQueryToken = { requestId: fixture.requests[0], queryGeneration: "41", identity: { appInstanceId: 7, generation: "7", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
    const newToken: LocalInteractionQueryToken = { ...oldToken, requestId: fixture.requests[1], queryGeneration: "42" };
    const first = new AppChannelClient(handle, owner, 7, "fixture");
    const firstResult = first.readLocalInteractionPages(async () => {}).catch((error: unknown) => error);
    first.dispose();
    await flush();
    push({ kind: "started", token: oldToken });
    await flush();
    push({ kind: "closed", token: oldToken, cancelled: true });
    expect(await firstResult).toBeInstanceOf(Error);
    const second = new AppChannelClient(handle, owner, 7, "fixture");
    let consumed = 0;
    let ordinaryComplete = false;
    const secondResult = second.readLocalInteractionPages(async () => { consumed += 1; });
    const ordinary = second.readDocument().then((frames) => { ordinaryComplete = true; return frames; });
    for (const sequence of [...fixture.readSequences.slice(0, 1), ...fixture.cancelSequences.slice(0, 1), 999]) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: sequence } })] });
    push({ kind: "started", token: oldToken });
    push({ kind: "page", page: { ...oldToken, terminal: true, bytes: [1] } });
    push({ kind: "closed", token: oldToken, cancelled: false });
    await flush();
    expect(consumed).toBe(0);
    expect(ordinaryComplete).toBe(false);
    push({ kind: "started", token: newToken });
    push({ kind: "page", page: { ...newToken, terminal: true, bytes: [2] } });
    await flush();
    expect(consumed).toBe(1);
    expect(sent.at(-1)).toEqual({ LocalInteractionQuery: { seq: fixture.ackSequence, command: { kind: "acknowledge", token: newToken } } });
    broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: fixture.ordinarySequence } })] });
    expect(await ordinary).toEqual([{ Done: { in_reply_to: fixture.ordinarySequence } }]);
    push({ kind: "closed", token: newToken, cancelled: false });
    expect(await secondResult).toEqual(newToken.identity);
    expect(sent.filter((command) => "LocalInteractionQuery" in command && command.LocalInteractionQuery.command.kind === "read").map((command) => "LocalInteractionQuery" in command ? command.LocalInteractionQuery.seq : 0)).toEqual(fixture.readSequences);
    second.dispose();
  });

  it("local interaction exhausted admission leaves no query slot and retains a cancellation sequence", async () => {
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    const sent: AppCommandValue[] = [];
    const handle: AppChannelHandle = { enqueue: (_instance, bytes) => {
      for (const command of bytes.map(decodeAppCommand)) {
        sent.push(command);
        broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: commandSeq(command) } })] });
      }
    }, outcomes: broadcast.stream };
    const exhausted = new AppChannelClient(handle, new AppChannelRequestSequence(Number.MAX_SAFE_INTEGER - 1), 7, "fixture");
    await expect(exhausted.readLocalInteractionPages(async () => {})).rejects.toThrow("sequence-exhausted");
    await expect(exhausted.readLocalInteractionPages(async () => {})).rejects.toThrow("sequence-exhausted");
    expect(sent).toHaveLength(0);
    await exhausted.readDocument();
    expect(sent).toEqual([{ ReadDocument: { seq: Number.MAX_SAFE_INTEGER } }]);
    exhausted.dispose();
    sent.length = 0;
    const last = new AppChannelClient(handle, new AppChannelRequestSequence(Number.MAX_SAFE_INTEGER - 2), 7, "fixture");
    const result = last.readLocalInteractionPages(async () => {}).catch((error: unknown) => error);
    const token: LocalInteractionQueryToken = { requestId: "1", queryGeneration: "43", identity: { appInstanceId: 7, generation: "7", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
    broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "started", token } } }), encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "page", page: { ...token, terminal: true, bytes: [1] } } } })] });
    for (let index = 0; index < 16; index += 1) await Promise.resolve();
    expect(sent.at(-1)).toEqual({ LocalInteractionQuery: { seq: Number.MAX_SAFE_INTEGER, command: { kind: "cancel", token } } });
    broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "closed", token, cancelled: true } } })] });
    expect(await result).toBeInstanceOf(Error);
    last.dispose();
  });
  /** 🧪️ A fake handle that decodes whatever {@link AppChannelClient} `enqueue`d and pushes
   * caller-supplied frames back as this SAME instance's next outcome — enough to assert the client
   * frames/unframes correctly (and correlates replies through the real `outcomes` broadcast) without
   * a real plugin instance. */
  function fakeHandle(reply: (instanceId: number, commands: AppCommandValue[]) => AppFrameValue[]): AppChannelHandle {
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    return {
      enqueue: (instanceId, events) => {
        const commands = events.map(decodeAppCommand);
        const frames = reply(instanceId, commands).map(encodeAppFrame);
        broadcast.push({ instanceId, frames });
      },
      outcomes: broadcast.stream,
    };
  }

  /**
   * 🏁️ The defect this frame exists for: a retained typed operation reaches its terminal result on a
   * continuation turn no host call is awaiting. The old wire published that completion as an
   * `Invocation` with `in_reply_to: 0`, which correlates with NO pending waiter, so the shell simply
   * dropped it — the outliner, the inspection panel and the History list all kept showing the
   * previous document after an example was selected.
   */
  it("delivers an unsolicited typed-operation completion to its subscriber exactly once, never to a pending command waiter", async () => {
    const completions: unknown[] = [];
    const handle = fakeHandle((_instanceId, commands) => [
      {
        OperationCompleted: {
          operation: 9n,
          revision: 4n,
          ui_scope: Array.from(encodePackValue("full")),
          history_patch: Array.from(encodePackValue({ cursor: packUInt(7n), upserts: [{ seq: packUInt(41n), label: "Set Active Example", count: packUInt(1n) }], canUndo: true, canRedo: false, commandFilter: "all" })),
        },
      },
      { Invocation: { in_reply_to: Object.values(commands[0]!)[0]!.seq, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } },
    ]);
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const unsubscribe = client.onOperationCompleted((completion) => completions.push(completion));
    const frames = await client.command(new Uint8Array([1]), {});
    expect(frames).toHaveLength(1);
    expect(frames.every((frame) => "Invocation" in frame)).toBe(true);
    expect(completions).toHaveLength(1);
    const completion = completions[0] as { readonly instanceId: number; readonly operation: bigint; readonly revision: bigint; readonly uiScope: unknown; readonly historyPatch: { readonly cursor: number; readonly upserts: readonly { readonly seq: number }[] } };
    expect(completion.instanceId).toBe(1);
    expect(completion.operation).toBe(9n);
    expect(completion.revision).toBe(4n);
    expect(completion.uiScope).toBe("full");
    // 📌️ The `HistoryPatch` carrier bug: `seq`/`cursor` cross as pack integer carriers, and a row id
    // built from an unprojected one renders `framework.history.entry.[object Object]`.
    expect(completion.historyPatch.cursor).toBe(7);
    expect(completion.historyPatch.upserts[0]!.seq).toBe(41);
    expect(`framework.history.entry.${completion.historyPatch.upserts[0]!.seq}`).toBe("framework.history.entry.41");
    unsubscribe();
    await client.command(new Uint8Array([2]), {});
    expect(completions).toHaveLength(1);
  });

  /**
   * 🎞️ A running operation (a tool run) hands the shell its dirty scope mid-operation as an `Invocation` answering no
   * command. Once the operation runs on the background drain that frame correlates with no waiter, so without this
   * subscription the renderer saw nothing until the run ended and then everything at once.
   */
  it("delivers an unsolicited operation progress scope and history patch to its subscriber, never to a pending command waiter", async () => {
    const progress: unknown[] = [];
    const replay = { cursor: 9, timeTravel: { sessionId: "1", generation: 4, stage: "replaying", done: 3, total: 8, blocking: false, acceptedCount: 1 } };
    const handle = fakeHandle((_instanceId, commands) => [
      { Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: Array.from(encodePackValue({ kind: "partial", panelBodies: ["framework.body.toolRun"] })), history_patch: [], messages: [], mutations: [], inverse_group: [] } },
      { Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: [], history_patch: Array.from(encodePackValue(replay)), messages: [], mutations: [], inverse_group: [] } },
      { Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } },
      { Invocation: { in_reply_to: Object.values(commands[0]!)[0]!.seq, output: [], diagnostics: [], ui_scope: Array.from(encodePackValue("full")), history_patch: [], messages: [], mutations: [], inverse_group: [] } },
    ]);
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const unsubscribe = client.onOperationProgress((uiScope) => progress.push(uiScope));
    const frames = await client.command(new Uint8Array([1]), {});
    expect(frames).toHaveLength(1);
    expect(progress).toEqual([{ uiScope: { kind: "partial", panelBodies: ["framework.body.toolRun"] } }, { historyPatch: replay }]);
    unsubscribe();
    await client.command(new Uint8Array([2]), {});
    expect(progress).toHaveLength(2);
  });

  it("matches the shared cross-language typed-operation completion fixture vector, byte-for-byte", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const vectors = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "🧪️tests/🧫️fixtures", "📡️channel", "🏁️app-frame-operation-completed.json"), "utf8")) as Record<string, string>;
    for (const [key, operation, revision] of [["OperationCompleted", 7n, 5n], ["OperationCompletedWideRevision", 7n, 0xfedc_ba98_7654_3210n], ["OperationCompletedWideOperation", 0xffff_ffff_ffff_ffffn, 5n]] as const) {
      const frame: AppFrameValue = { OperationCompleted: { operation, revision, ui_scope: [1], history_patch: [2] } };
      const hex = Array.from(encodeAppFrame(frame), (byte) => byte.toString(16).padStart(2, "0")).join("");
      expect(hex).toBe(vectors[key]);
      expect(decodeAppFrame(new Uint8Array(Buffer.from(vectors[key]!, "hex")))).toEqual(frame);
    }
  });

  it("matches the shared cross-language document-archive merge vectors, byte-for-byte", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const vectors = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "🧪️tests/🧫️fixtures", "📡️channel", "🧲️document-archive-merge.json"), "utf8")) as { seq: number; archive: { parent_pack: number[]; parent_spr: number[]; members: [] }; LoadDocumentArchive: string; MergeDocumentArchive: string; status: { in_reply_to: number; operation: number; state: string; completed: number; total: number; ahead: number; fault: number[] }; DocumentArchiveLoad: string };
    const hex = (bytes: Uint8Array) => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
    const load = { LoadDocumentArchive: { seq: vectors.seq, archive: vectors.archive } };
    const merge = { MergeDocumentArchive: { seq: vectors.seq, archive: vectors.archive } };
    expect(hex(encodeAppCommand(load))).toBe(vectors.LoadDocumentArchive);
    expect(hex(encodeAppCommand(merge))).toBe(vectors.MergeDocumentArchive);
    expect(decodeAppCommand(new Uint8Array(Buffer.from(vectors.LoadDocumentArchive, "hex")))).toEqual(load);
    expect(decodeAppCommand(new Uint8Array(Buffer.from(vectors.MergeDocumentArchive, "hex")))).toEqual(merge);
    expect(vectors.status.state).toBe("ready");
    const frame: AppFrameValue = { DocumentArchiveLoad: { in_reply_to: vectors.status.in_reply_to, status: { operation: vectors.status.operation, state: "ready", completed: vectors.status.completed, total: vectors.status.total, ahead: vectors.status.ahead, fault: vectors.status.fault } } };
    expect(hex(encodeAppFrame(frame))).toBe(vectors.DocumentArchiveLoad);
    expect(decodeAppFrame(new Uint8Array(Buffer.from(vectors.DocumentArchiveLoad, "hex")))).toEqual(frame);
  });

  it("command() allocates an incrementing seq and returns every frame the batch produced", async () => {
    const seqsSeen: number[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      const cmd = commands[0];
      if (cmd && "Command" in cmd) seqsSeen.push(cmd.Command.seq);
      return [
        { Invocation: { in_reply_to: seqsSeen.at(-1) ?? 0, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } },
        { UiPatch: { in_reply_to: seqsSeen.at(-1) ?? 0, surface: "1:body", kind: "window", revision: 1, base_revision: 0, ops: [] } },
      ];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const first = await client.command(new Uint8Array([1, 2]), { cursor: 0 });
    const second = await client.command(new Uint8Array([3]), { cursor: 1 });
    expect(seqsSeen).toEqual([1, 2]);
    expect(first).toHaveLength(2);
    expect(second).toHaveLength(2);
  });

  it("readChildHeads() sends ReadChildHeads and answers the ChildHeads entries of its own sequence, owning their bytes", async () => {
    const seen: AppCommandValue[] = [];
    const heads = [{ slot: "mesh", child_id: "child-1", dialect: "s.stdio.mesh@1/*", head_pack: [7, 8], owner: "content/forms-1" }];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      const command = commands[0]!;
      return "ReadChildHeads" in command ? [{ ChildHeads: { in_reply_to: command.ReadChildHeads.seq, entries: heads } }] : [{ Done: { in_reply_to: commandSeq(command) } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const entries = await client.readChildHeads();
    heads[0]!.head_pack.fill(0);
    expect([seen, entries]).toEqual([[{ ReadChildHeads: { seq: 1 } }], [{ slot: "mesh", child_id: "child-1", dialect: "s.stdio.mesh@1/*", head_pack: [7, 8], owner: "content/forms-1" }]]);
    const refusing = new AppChannelClient(fakeHandle((_instanceId, commands) => [{ Done: { in_reply_to: commandSeq(commands[0]!) } }]), new AppChannelRequestSequence(), 1, "app.demo");
    await expect(refusing.readChildHeads()).rejects.toThrow("missing ChildHeads frame for seq 1");
  });

  it("configure()/readDocument() frame the right AppCommand variant", async () => {
    const seen: AppCommandValue[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    await client.configure({ locale: "en" });
    await client.readDocument();
    expect(seen).toEqual([{ ConfigCommand: { seq: 1, command: Array.from(encodePackValue({ locale: "en" })) } }, { ReadDocument: { seq: 2 } }]);
  });

  it("loadDocumentArchive()/readDocumentArchive() preserve the complete recursive closure and root cache", async () => {
    const seen: AppCommandValue[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      const command = commands[0]!;
      if ("ReadDocumentArchive" in command) return [{ DocumentArchive: { in_reply_to: command.ReadDocumentArchive.seq, archive: documentArchive } }];
      if ("PollDocumentArchiveLoad" in command) return [{ DocumentArchiveLoad: { in_reply_to: command.PollDocumentArchiveLoad.seq, status: { operation: command.PollDocumentArchiveLoad.operation, state: "ready", completed: 3, total: 3, ahead: 0, fault: [] } } }];
      return [{ Done: { in_reply_to: commandSeq(command) } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    await client.loadDocumentArchive(documentArchive);
    const actual = await client.readDocumentArchive();
    expect(seen).toEqual([
      { LoadDocumentArchive: { seq: 1, archive: documentArchive } },
      { PollDocumentArchiveLoad: { seq: 2, operation: 1 } },
      { AcknowledgeDocumentArchiveLoad: { seq: 3, operation: 1 } },
      { ReadDocumentArchive: { seq: 4 } },
    ]);
    expect(actual).toEqual(documentArchive);
    expect(client.documentPack()).toEqual({ pack: new Uint8Array([1, 2]), spr: new Uint8Array([3]) });
  });

  it("loadDocumentArchive() sends nothing for a cancel before admission, and cancels, polls and acknowledges the exact retained operation after it", async () => {
    const seen: AppCommandValue[] = [];
    const controller = new AbortController();
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      const command = commands[0]!;
      if ("LoadDocumentArchive" in command) controller.abort(new Error("caller cancelled archive"));
      if ("PollDocumentArchiveLoad" in command) {
        return [{ DocumentArchiveLoad: { in_reply_to: command.PollDocumentArchiveLoad.seq, status: { operation: command.PollDocumentArchiveLoad.operation, state: "cancelled", completed: 0, total: 3, ahead: 0, fault: [] } } }];
      }
      return [{ Done: { in_reply_to: commandSeq(command) } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const early = new AbortController();
    early.abort(new Error("caller cancelled before admission"));
    await expect(client.loadDocumentArchive(documentArchive, early.signal)).rejects.toThrow("caller cancelled before admission");
    expect(seen).toEqual([]);
    await expect(client.loadDocumentArchive(documentArchive, controller.signal)).rejects.toThrow("caller cancelled archive");
    expect(seen).toEqual([
      { LoadDocumentArchive: { seq: 1, archive: documentArchive } },
      { CancelDocumentArchiveLoad: { seq: 2, operation: 1 } },
      { PollDocumentArchiveLoad: { seq: 3, operation: 1 } },
      { AcknowledgeDocumentArchiveLoad: { seq: 4, operation: 1 } },
    ]);
  });

  it("loadDocumentArchive() reports a cancel the guest decided on its own (a person's Cancel in the history body) as an AbortError, after its progress and acknowledgement", async () => {
    const seen: AppCommandValue[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      const command = commands[0]!;
      if ("PollDocumentArchiveLoad" in command) {
        const state = seen.filter((sent) => "PollDocumentArchiveLoad" in sent).length === 1 ? "running" : "cancelled";
        return [{ DocumentArchiveLoad: { in_reply_to: command.PollDocumentArchiveLoad.seq, status: { operation: command.PollDocumentArchiveLoad.operation, state, completed: 1, total: 3, ahead: 0, fault: [] } } }];
      }
      return [{ Done: { in_reply_to: commandSeq(command) } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const progress: string[] = [];
    const outcome = await client.loadDocumentArchive(documentArchive, undefined, (status) => progress.push(`${status.state} ${status.completed}/${status.total}`)).then(() => null, (error: unknown) => error);
    expect([outcome instanceof DOMException && outcome.name === "AbortError", progress, seen.map((sent) => Object.keys(sent)[0]), client.documentPack()]).toEqual([true, ["running 1/3", "cancelled 1/3"], ["LoadDocumentArchive", "PollDocumentArchiveLoad", "PollDocumentArchiveLoad", "AcknowledgeDocumentArchiveLoad"], null]);
  });

  it("loads and reads exact persisted-local window config envelopes", async () => {
    const seen: AppCommandValue[] = [];
    const expected = [
      { window_id: "graph-a", window_kind_id: "graph", envelope_pack: [1, 2] },
      { window_id: "graph-b", window_kind_id: "graph", envelope_pack: [3] },
    ];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      const command = commands[0]!;
      if ("ReadWindowConfigs" in command) return [{ WindowConfigs: { in_reply_to: command.ReadWindowConfigs.seq, entries: expected } }];
      return [{ Done: { in_reply_to: commandSeq(command) } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const source = { window_id: "graph-a", window_kind_id: "graph", envelope_pack: [1, 2] };
    await client.loadWindowConfig(source);
    const actual = await client.readWindowConfigs();
    source.envelope_pack.fill(255);
    expect(seen).toEqual([
      { LoadWindowConfig: { seq: 1, entry: { window_id: "graph-a", window_kind_id: "graph", envelope_pack: [1, 2] } } },
      { ReadWindowConfigs: { seq: 2 } },
    ]);
    expect(actual).toEqual(expected);
  });

  it("publishes only accepted document cache candidates and owns both byte arrays", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/📦️document-cache/🔣️.json", source.url), "utf8"));
    const pair = (value: { pack: number[]; spr: number[] }) => ({ pack: Uint8Array.from(value.pack), spr: Uint8Array.from(value.spr) });
    const archiveOf = (value: { pack: number[]; spr: number[] }) => ({ parent_pack: [...value.pack], parent_spr: [...value.spr], members: [] });
    for (const row of fixture.cases) {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      let admission = "done";
      const client = new AppChannelClient({
        outcomes: broadcast.stream,
        enqueue: (_instanceId: number, commands: readonly Uint8Array[]) => {
          const command = decodeAppCommand(commands[0]!);
          if ("LoadDocumentArchive" in command && admission === "enqueue") throw new Error("document-cache.enqueue");
          const seq = commandSeq(command);
          const document = { Document: { in_reply_to: seq, pack: fixture.reply.pack, spr: fixture.reply.spr, ops: "" } };
          const error = { Error: { in_reply_to: seq, fault: [99], report: [] } };
          const reply: AppFrameValue[] | "transport" =
            "PollDocumentArchiveLoad" in command ? [{ DocumentArchiveLoad: { in_reply_to: seq, status: { operation: command.PollDocumentArchiveLoad.operation, state: "ready", completed: 1, total: 1, ahead: 0, fault: [] } } }] :
            !("LoadDocumentArchive" in command) || admission === "done" ? [{ Done: { in_reply_to: seq } }] :
            admission === "transport" ? "transport" :
            admission === "document" ? [document] :
            admission === "error-document" ? [document, error] : [error];
          queueMicrotask(() => broadcast.push(reply === "transport" ? { instanceId: 1, error: new Error("document-cache.transport") } : { instanceId: 1, frames: reply.map(encodeAppFrame) }));
        },
      }, new AppChannelRequestSequence(), 1, "cache");
      try {
        await client.loadDocumentArchive(archiveOf(fixture.initial));
        expect(client.documentPack()).toEqual(pair(fixture.initial));
        admission = row.outcome;
        const candidate = archiveOf(fixture.candidate);
        const settled = client.loadDocumentArchive(candidate).then(() => "resolved", () => "rejected");
        candidate.parent_pack.fill(255);
        candidate.parent_spr.fill(255);
        expect(client.documentPack()).toEqual(pair(fixture.initial));
        expect(await settled).toBe(row.outcome === "done" ? "resolved" : "rejected");
        expect(client.documentPack()).toEqual(pair(fixture[row.expected]));
        const exposed = client.documentPack()!;
        exposed.pack.fill(254);
        exposed.spr.fill(254);
        expect(client.documentPack()).toEqual(pair(fixture[row.expected]));
        client.dispose();
        expect(client.documentPack()).toBeNull();
      } finally {
        client.dispose();
        broadcast.complete();
      }
    }
  });

  it("caches the document pack from every AppFrame::Document reply, most recent wins", async () => {
    const handle = fakeHandle((_instanceId, commands) => {
      const cmd = commands[0];
      if (cmd && "ReadDocument" in cmd) {
        return [{ Document: { in_reply_to: cmd.ReadDocument.seq, pack: [9, 9], spr: [8], ops: "" } }];
      }
      return [{ Done: { in_reply_to: 1 } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    await client.readDocument();
    expect(client.documentPack()).toEqual({ pack: new Uint8Array([9, 9]), spr: new Uint8Array([8]) });
  });

  it("transactionPrepareOwner()/transactionPreparePlanned()/transactionCommit()/transactionRollback()/transactionUndo()/transactionRedo() frame the right AppCommand variant", async () => {
    const seen: AppCommandValue[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    await client.transactionPrepareOwner("txn-1", "s.doc#kind", new Uint8Array([1]));
    await client.transactionPreparePlanned("txn-1", [new Uint8Array([2]), new Uint8Array([3])], new Uint8Array([4]));
    await client.transactionCommit("txn-1");
    await client.transactionRollback("txn-1");
    await client.transactionUndo("grp-1");
    await client.transactionRedo("grp-1");
    expect(seen[0]).toEqual({ transactionPrepare: { seq: 1, txn_id: "txn-1", mutation_id: "s.doc#kind", payload: [1], prepared_ops: [], origin: [], prepared_child_ops: [] } });
    expect(seen[1]).toEqual({ transactionPrepare: { seq: 2, txn_id: "txn-1", mutation_id: "", payload: [], prepared_ops: [[2], [3]], origin: [4], prepared_child_ops: [] } });
    expect(seen[2]).toEqual({ transactionCommit: { seq: 3, txn_id: "txn-1" } });
    expect(seen[3]).toEqual({ transactionRollback: { seq: 4, txn_id: "txn-1" } });
    expect(seen[4]).toEqual({ transactionUndo: { seq: 5, group_id: "grp-1" } });
    expect(seen[5]).toEqual({ transactionRedo: { seq: 6, group_id: "grp-1" } });
  });

  /**
   * 🔗️ Rust↔TS parity for the C8 merge/conflict surface, driven through {@link AppChannelClient}'s
   * OWN public methods rather than the raw `encodeAppCommand`/`decodeAppFrame` codec functions the
   * `AppChannelCodec` suite already asserts against the same two files — this is the "does the
   * CLIENT layer itself send/surface the new commands and frames correctly" half of the parity
   * story (contract-freeze §C8/§C9, ticket
   * `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`). Four throwaway
   * `configure({})` calls burn seq 1-4 so `setMergePolicy`/`resolveConflict`/`readConflicts` land on
   * the exact seq (5/6/7) the golden vectors were baked against.
   */
  it("setMergePolicy()/resolveConflict()/readConflicts() match the shared cross-language merge command vectors, byte-for-byte", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const commandVectors = JSON.parse(readFileSync(join(here, "🧪️tests/🧫️fixtures", "📡️channel", "🔀️app-command-merge.json"), "utf8")) as Record<string, string>;
    const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

    const seen: AppCommandValue[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    await client.configure({});
    await client.configure({});
    await client.configure({});
    await client.configure({});
    await client.setMergePolicy("Normal");
    await client.resolveConflict("conflict-1", "accept");
    await client.readConflicts();

    expect(seen[4]).toEqual({ setMergePolicy: { seq: 5, policy: 1 } });
    expect(seen[5]).toEqual({ resolveConflict: { seq: 6, conflict_id: "conflict-1", resolution: 0 } });
    expect(seen[6]).toEqual({ readConflicts: { seq: 7 } });
    expect(hex(encodeAppCommand(seen[4]!)), "AppCommand::SetMergePolicy").toBe(commandVectors.SetMergePolicy);
    expect(hex(encodeAppCommand(seen[5]!)), "AppCommand::ResolveConflict").toBe(commandVectors.ResolveConflict);
    expect(hex(encodeAppCommand(seen[6]!)), "AppCommand::ReadConflicts").toBe(commandVectors.ReadConflicts);
  });

  it("command() surfaces unsolicited MergeReport/Conflicts frames and the extended Invocation.messages/Error.report fields verbatim", async () => {
    const handle = fakeHandle(() => [
      { Invocation: { in_reply_to: 1, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [9], mutations: [], inverse_group: [] } },
      { MergeReport: { in_reply_to: null, report: [1] } },
      { Conflicts: { in_reply_to: null, conflicts: [2] } },
    ]);
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const frames = await client.command(new Uint8Array([1]), {});
    expect(frames).toHaveLength(3);
    const invocation = frames.find((frame): frame is Extract<AppFrameValue, { readonly Invocation: unknown }> => "Invocation" in frame);
    const mergeReport = frames.find((frame): frame is Extract<AppFrameValue, { readonly MergeReport: unknown }> => "MergeReport" in frame);
    const conflicts = frames.find((frame): frame is Extract<AppFrameValue, { readonly Conflicts: unknown }> => "Conflicts" in frame);
    expect(invocation?.Invocation.messages).toEqual([9]);
    expect(mergeReport?.MergeReport.report).toEqual([1]);
    expect(conflicts?.Conflicts.conflicts).toEqual([2]);
  });

  /**
   * 🔗️ Round-trips the frozen TS shapes ({@link DispatchReport}/{@link MergeReport}/{@link
   * Conflict}) through {@link encodePackValue}/the new `decode*FromWire` helpers — proving the
   * field-name mapping this lane derived from Rust's `#[serde(rename_all = "camelCase")]`
   * (`policy`/`worst`/`messages`, `insertionIndex`, `editId`→`edit_id` NOT renamed inside
   * `ConflictKind`'s struct variants, `MergePolicy`'s bare un-camelCased variant names) actually
   * decodes the way the wire's `store::pack_rt::encode_wire_value`-backed `encode_wire_serialized`
   * would produce it, since no live Rust `DispatchReport`/`MergeReport`/`Conflict` pack bytes are
   * checked into a fixture yet (only the outer `AppFrame` framing is, in `📢️app-frame-merge.json`).
   */
  it("faultMessages()/decodeDispatchReportFromWire()/decodeMergeReportFromWire()/decodeConflictsFromWire() decode the frozen TS report shapes", () => {
    const dispatchReport: DispatchReport = {
      policy: "Vigilant",
      worst: "warning",
      messages: [{ level: "warning", code: "mutation.clamped", message: "value clamped to range" }],
    };
    const reportBytes = Array.from(encodePackValue(dispatchReport));
    expect(decodeDispatchReportFromWire(reportBytes, decodePackValue)).toEqual(dispatchReport);
    expect(faultMessages(reportBytes, decodePackValue)).toEqual(dispatchReport.messages);
    expect(faultMessages([], decodePackValue)).toEqual([]);

    const mergeReport: MergeReport = {
      policy: "Normal",
      accepted: true,
      insertionIndex: 3,
      replayed: [{ edit_id: "e1", messages: [{ level: "info", code: "mutation.cascade", message: "cascaded" }] }],
      worst: "info",
      conflict: null,
    };
    expect(decodeMergeReportFromWire(Array.from(encodePackValue(mergeReport)), decodePackValue)).toEqual(mergeReport);
    expect(decodeMergeReportFromWire([], decodePackValue)).toBeNull();

    const conflicts: readonly Conflict[] = [
      {
        id: "conflict-abc",
        kind: { kind: "degraded", edit_ids: ["e1"] },
        status: "open",
        messages: [{ level: "error", code: "mutation.target-missing", message: "target missing" }],
        actors: ["actor-1"],
        timestamp: { actor: 1, physical_ms: 100, logical: 0 },
      },
    ];
    expect(decodeConflictsFromWire(Array.from(encodePackValue(conflicts)), decodePackValue)).toEqual(conflicts);
    expect(decodeConflictsFromWire([], decodePackValue)).toEqual([]);
  });

  /** 🔌️ A handle whose replies the test answers by hand and whose outcome subscription counts how often it was asked to end. */
  function scriptedHandle(): { readonly handle: AppChannelHandle; readonly broadcast: ReturnType<typeof createTurnOutcomeBroadcast<TurnOutcome>>; readonly sent: AppCommandValue[]; readonly ended: { count: number } } {
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    const sent: AppCommandValue[] = [];
    const ended = { count: 0 };
    const outcomes: AsyncIterable<TurnOutcome> = {
      [Symbol.asyncIterator]: () => {
        const inner = broadcast.stream[Symbol.asyncIterator]();
        return { next: () => inner.next(), return: () => { ended.count += 1; return inner.return!(); } };
      },
    };
    return { handle: { enqueue: (_instanceId, events) => { sent.push(...events.map(decodeAppCommand)); }, outcomes }, broadcast, sent, ended };
  }

  const settle = async () => { for (let index = 0; index < 12; index += 1) await Promise.resolve(); };

  it("wires the composer to the full public surface of the client and to nothing else", () => {
    const surface = ["applyEnvelopes", "cancelMediaExport", "clearDefaultApp", "command", "configure", "contextMenu", "dispose", "documentPack", "ephemeral", "loadDocumentArchive", "loadWindowConfig", "mediaIn", "mediaOut", "mergeDocumentArchive", "onOperationCompleted", "onOperationProgress", "openArtifact", "pollMediaExport", "pushPresence", "readChildHeads", "readConflicts", "readDocument", "readDocumentArchive", "readDocumentIdentity", "readHistory", "readLocalInteractionPages", "readWindowConfigs", "resolveConflict", "retire", "setDefaultApp", "setMergePolicy", "submitMediaExport", "takeMediaExportChunk", "transactionCommit", "transactionPrepareOwner", "transactionPreparePlanned", "transactionRedo", "transactionRollback", "transactionUndo"];
    const { handle } = scriptedHandle();
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    for (const name of surface) expect(typeof (client as unknown as Record<string, unknown>)[name], name).toBe("function");
    expect(Object.getOwnPropertyNames(AppChannelClient.prototype).filter((name) => name !== "constructor").sort()).toEqual(surface);
    client.dispose();
  });

  it("retire() fails the queued commands, silences completion subscribers only and forgets the pack; dispose() then ends the subscription", async () => {
    const { handle, broadcast, sent, ended } = scriptedHandle();
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const first = client.readDocument();
    broadcast.push({ instanceId: 1, frames: [encodeAppFrame({ Document: { in_reply_to: 1, pack: [9, 9], spr: [8], ops: "" } })] });
    await first;
    expect(client.documentPack()).toEqual({ pack: new Uint8Array([9, 9]), spr: new Uint8Array([8]) });
    const completions: unknown[] = [];
    const progress: unknown[] = [];
    client.onOperationCompleted((completion) => completions.push(completion));
    client.onOperationProgress((update) => progress.push(update));
    const queued = client.configure({ locale: "en" });
    const queuedFailure = expect(queued).rejects.toThrow("app-channel.disposed");
    expect(sent.at(-1)).toEqual({ ConfigCommand: { seq: 2, command: Array.from(encodePackValue({ locale: "en" })) } });
    client.retire();
    await queuedFailure;
    expect(client.documentPack()).toBeNull();
    await expect(client.readDocument()).rejects.toThrow("app-channel.disposed");
    await expect(client.readLocalInteractionPages(async () => undefined)).rejects.toThrow("app-channel.disposed");
    const completed = encodeAppFrame({ OperationCompleted: { operation: 9n, revision: 4n, ui_scope: Array.from(encodePackValue("full")), history_patch: [] } });
    const progressing = encodeAppFrame({ Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: Array.from(encodePackValue("full")), history_patch: [], messages: [], mutations: [], inverse_group: [] } });
    broadcast.push({ instanceId: 1, frames: [completed, progressing] });
    await settle();
    expect([completions.length, progress.length, ended.count]).toEqual([0, 1, 0]);
    client.retire();
    expect(ended.count).toBe(0);
    client.dispose();
    expect(ended.count).toBe(1);
    broadcast.push({ instanceId: 1, frames: [progressing] });
    await settle();
    expect(progress).toHaveLength(1);
  });

  it("an ended outcome stream rejects every waiter and the running local interaction query and forgets the pack", async () => {
    const { handle, broadcast } = scriptedHandle();
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const first = client.readDocument();
    broadcast.push({ instanceId: 1, frames: [encodeAppFrame({ Document: { in_reply_to: 1, pack: [5], spr: [6], ops: "" } })] });
    await first;
    const waiting = client.readDocument();
    const waitingFailure = expect(waiting).rejects.toThrow("app-channel.closed");
    const query = client.readLocalInteractionPages(async () => undefined);
    const queryFailure = expect(query).rejects.toThrow("local-interaction.channel-closed");
    broadcast.complete();
    await Promise.all([waitingFailure, queryFailure]);
    expect(client.documentPack()).toBeNull();
  });

  it("an error outcome fails only the oldest waiter, and a sibling instance's outcome settles nobody", async () => {
    const { handle, broadcast } = scriptedHandle();
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const outcomes: string[] = [];
    const oldest = client.readDocument().then(() => outcomes.push("oldest resolved"), (error: unknown) => outcomes.push(`oldest rejected ${(error as Error).message}`));
    const newest = client.readHistory().then(() => outcomes.push("newest resolved"), (error: unknown) => outcomes.push(`newest rejected ${(error as Error).message}`));
    broadcast.push({ instanceId: 2, frames: [encodeAppFrame({ Done: { in_reply_to: 1 } }), encodeAppFrame({ Done: { in_reply_to: 2 } })] });
    broadcast.push({ instanceId: 2, error: new Error("sibling trapped") });
    await settle();
    expect(outcomes).toEqual([]);
    broadcast.push({ instanceId: 1, error: new Error("trapped") });
    await oldest;
    expect(outcomes).toEqual(["oldest rejected trapped"]);
    broadcast.push({ instanceId: 1, frames: [encodeAppFrame({ Done: { in_reply_to: 2 } })] });
    await newest;
    expect(outcomes).toEqual(["oldest rejected trapped", "newest resolved"]);
    client.dispose();
  });

  it("an undecodable frame fails every waiting command and leaves the channel usable", async () => {
    const { handle, broadcast } = scriptedHandle();
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const first = expect(client.readDocument()).rejects.toThrow();
    const second = expect(client.readHistory()).rejects.toThrow();
    broadcast.push({ instanceId: 1, frames: [Uint8Array.of(255, 255, 255, 255)] });
    await Promise.all([first, second]);
    const third = client.readHistory();
    broadcast.push({ instanceId: 1, frames: [encodeAppFrame({ Done: { in_reply_to: 3 } })] });
    expect(await third).toEqual([{ Done: { in_reply_to: 3 } }]);
    client.dispose();
  });
});
