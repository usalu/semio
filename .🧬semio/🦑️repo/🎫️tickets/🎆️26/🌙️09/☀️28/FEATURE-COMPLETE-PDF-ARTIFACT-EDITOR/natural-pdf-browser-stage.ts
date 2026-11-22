/** 🧰️ Stages the browser natural-source sender against the current revision-bound PNG sender. */
import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";

const root = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive";
const generated = join(import.meta.dir, "🗑️generated", "natural-source-browser");
await mkdir(generated, { recursive: true });
const sender = await Bun.file(join(root, "original-media-sender.ts")).text();
const wire = await Bun.file(join(root, "original-media-wire.ts")).text();
const exchange = await Bun.file(join(root, "original-media-exchange.ts")).text();
const pick = await Bun.file(join(root, "original-media-pick.ts")).text();
const source = await Bun.file(join(import.meta.dir, "natural-pdf-browser-source.ts")).text();
const archiveSource = await Bun.file(join(import.meta.dir, "natural-pdf-browser-archive.ts")).text();

function replaceOnce(text: string, before: string, after: string) {
  assert.equal(text.split(before).length, 2, "Shared worktree source changed near "+before.slice(0,100));
  return text.replace(before, after);
}

const oldType = "type OriginalMediaSink = (frame: OriginalMediaFrame, bytes?: Uint8Array) => Promise<void>;";
const newTypes = [
  "type OriginalMediaSink<Loan = OriginalMediaLoan> = (frame: OriginalMediaFrame<Loan>, bytes?: Uint8Array) => Promise<void>;",
  "export type OriginalMediaTransferControl = Readonly<{",
  "  signal: AbortSignal; current(): boolean; progress(completed: number, total: number): void;",
  "}>;",
].join("\n");
let next = replaceOnce(sender, "export type OriginalMediaFrame =", "export type OriginalMediaFrame<Loan = OriginalMediaLoan> =");
const first = next.indexOf("export type OriginalMediaFrame<"), last = next.indexOf("export type OriginalMediaSource");
assert(first >= 0 && last > first && next.slice(first,last).match(/loan: OriginalMediaLoan/g)?.length === 3);
next = next.slice(0,first)+next.slice(first,last).replaceAll("loan: OriginalMediaLoan", "loan: Loan")+next.slice(last);
next = replaceOnce(next, oldType, newTypes);
next = replaceOnce(next, "  control: {signal: AbortSignal; current: () => boolean; progress: (completed: number, total: number) => void},", "  control: OriginalMediaTransferControl,");
const begin = "  try {\n    await sink({kind: \"begin\", loan: captured, name, mediaType, byteLength: size});";
const shared = [
  "  return sendSelectedMedia(captured, source, mediaType, sink, control);",
  "}",
  "",
  "/** 📥️ Transfers bounded source slices and waits for each paid crossing before reading more. */",
  "export async function sendSelectedMedia<Loan>(",
  "  captured: Loan, source: OriginalMediaSource, mediaType: string,",
  "  sink: OriginalMediaSink<Loan>, control: OriginalMediaTransferControl,",
  "): Promise<void> {",
  "  const check = () => {",
  "    control.signal.throwIfAborted();",
  "    if (!control.current()) throw Error(\"The original media selection no longer belongs to its owner\");",
  "  };",
  "  const { size, name } = source;",
  "  check();",
  begin,
].join("\n");
next = replaceOnce(next, begin, shared);
await Promise.all([
  writeFile(join(generated, "original-media-sender.ts"), next),
  writeFile(join(generated, "original-media-wire.ts"), wire),
  writeFile(join(generated, "original-media-exchange.ts"), exchange),
  writeFile(join(generated, "original-media-pick.ts"), pick),
  writeFile(join(generated, "natural-source.ts"), source),
]);
console.log("[DEBUG] Staged shared selected-file pump and kind-241 source sender");

const runtime = join(root,"..");
const turnPath = join(root, "🟦️.ts");
const turnSource = await Bun.file(turnPath).text();
const imports = [
  'import { sendNaturalSource, type NaturalSourceActor, type NaturalSourceDeclaration, type NaturalSourceAcceptance } from "./natural-source.ts";',
  'import type { OriginalMediaSource, OriginalMediaTransferControl } from "./original-media-sender.ts";',
  'import { isNaturalArchiveReply, type NaturalArchiveExchange } from "./natural-source-archive.ts";',
].join("\n");
let turn = replaceOnce(turnSource, 'import { originalMediaRequest } from "./original-media-wire.ts";',
  'import { originalMediaRequest } from "./original-media-wire.ts";\n'+imports);
const mediaTail = turn.indexOf("  const openOriginalMediaIngress = ");
assert(mediaTail > 0);
const oldMediaHead = turn.slice(mediaTail,turn.indexOf("    const retired = () => {",mediaTail));
assert.equal(oldMediaHead.split("\n").filter(Boolean).length, 9);
const newMediaHead = [
  "  const nextMediaSequence = () => {",
  '    if (!Number.isSafeInteger(state.eventSeq + 1)) throw new Error("original-media.sequence");',
  "    return BigInt(++state.eventSeq);",
  "  };",
  '  const mediaIngressActor = (instanceId: number, seq: bigint, receiptTag: "original-media-ack" | "natural-source-ack", captureArchiveFrame?: (bytes: Uint8Array) => boolean): NaturalSourceActor & { current(): boolean } => {',
  "    const actorId = requireActorId(instanceId);",
  "    const activation = shardClient.captureActorActivation(actorId);",
  "    const binding = documentBindings.get(instanceId);",
  "",
].join("\n");
turn = replaceOnce(turn, oldMediaHead, newMediaHead);
const settleStart = turn.indexOf("    const settle = async () => {",mediaTail);
const settleEnd = turn.indexOf("    return createOriginalMediaExchange",settleStart);
assert(settleStart >= 0 && settleEnd > settleStart);
const settle = turn.slice(settleStart,settleEnd);
turn = turn.slice(0,settleStart) + replaceOnce(settle, "        if (frame) frames.push(frame);", "        if (frame && captureArchiveFrame?.(frame)) continue;\n        if (frame) frames.push(frame);") + turn.slice(settleEnd);
turn = replaceOnce(turn, "    return createOriginalMediaExchange(request, BigInt(instanceId), seq, {", "    return {");
turn = replaceOnce(turn, "value.commandIngress?.tag === \"original-media-ack\" || value.commandIngress?.tag === \"backpressure\"", "value.commandIngress?.tag === receiptTag || value.commandIngress?.tag === \"backpressure\"");
const oldReturn = "    });\n  };\n  return { runQueuedTurn, drainTypedOperations, openOriginalMediaIngress };";
const newReturn = [
  "    };",
  "  };",
  "  const openOriginalMediaIngress = (instanceId: number, issued: unknown) => {",
  "    const request = originalMediaRequest(issued, instanceId);",
  "    const scopedDocument = documentBindings.get(instanceId)?.port.owner.scope?.documentId;",
  '    if (scopedDocument !== undefined && scopedDocument !== request.loan.documentId) throw new Error("original-media.wrong-document");',
  "    const seq = nextMediaSequence();",
  '    return createOriginalMediaExchange(request, BigInt(instanceId), seq, mediaIngressActor(instanceId, seq, "original-media-ack"));',
  "  };",
  "  const importAppNaturalSource = (instanceId: number, declaration: NaturalSourceDeclaration, source: OriginalMediaSource, control: OriginalMediaTransferControl) => {",
  '    if (documentBindings.has(instanceId)) throw Error("natural-source.published-instance");',
  "    const seq = nextMediaSequence();",
  '    const actor = mediaIngressActor(instanceId, seq, "natural-source-ack");',
  "    return sendNaturalSource({ instanceId, seq, declaration, extent: source.size }, source, actor, { ...control,",
  "      current: () => actor.current() && control.current(),",
  "    });",
  "  };",
  "  const openAppNaturalArchiveExchange = (accepted: NaturalSourceAcceptance): NaturalArchiveExchange => {",
  "    const instanceId = accepted.loan.instanceId;",
  "    if (documentBindings.has(instanceId)) throw Error(\"natural-archive.published-instance\");",
  "    let reply: Uint8Array | undefined;",
  "    const actor = mediaIngressActor(instanceId, accepted.seq, \"natural-source-ack\", bytes => {",
  "      if (!isNaturalArchiveReply(bytes)) return false;",
  "      if (reply !== undefined) throw Error(\"natural-archive.multiple-receipts\");",
  "      reply = bytes;",
  "      return true;",
  "    });",
  "    return {",
  "      retired: actor.retired, yieldTurn: actor.yieldTurn, now: actor.now,",
  "      run: (packet, closing) => actor.serialize(async () => {",
  "        if (closing && actor.retired()) return undefined;",
  "        actor.assertOriginalActivation();",
  "        if (!closing && !actor.current()) throw Error(\"natural-archive.stale-owner\");",
  "        reply = undefined;",
  "        await actor.turn({ events: [{",
  "          kind: \"message\", payload: { source: { tag: \"shell\", val: instanceId }, payload: packet },",
  "        }] });",
  "        await actor.settle();",
  "        return reply;",
  "      }),",
  "    };",
  "  };",
  "  return { runQueuedTurn, drainTypedOperations, openOriginalMediaIngress, importAppNaturalSource, openAppNaturalArchiveExchange };",
].join("\n");
turn = replaceOnce(turn, oldReturn, newReturn);
const contractPath = join(runtime,"🧷️handle-contract/🟦️.ts");
const contract = await Bun.file(contractPath).text();
const contractEntry = "export type PluginWasmHandle = {";
const nextContract = replaceOnce(contract,contractEntry,[
  'import type { NaturalSourceAcceptance, NaturalSourceDeclaration } from "../🚚️handle-turn-drive/natural-source.ts";',
  'import type { NaturalArchiveExchange } from "../🚚️handle-turn-drive/natural-source-archive.ts";',
  'import type { OriginalMediaSource, OriginalMediaTransferControl } from "../🚚️handle-turn-drive/original-media-sender.ts";',
  "",
  contractEntry,
  "  readonly openAppNaturalArchiveExchange?: (accepted: NaturalSourceAcceptance) => NaturalArchiveExchange;",
  "  readonly importAppNaturalSource?: (instanceId: number, declaration: NaturalSourceDeclaration, source: OriginalMediaSource, control: OriginalMediaTransferControl) => Promise<NaturalSourceAcceptance>;",
].join("\n"));
const loaderPath = join(runtime,"🧩️module-loader/🟦️.ts");
const loader = await Bun.file(loaderPath).text();
const loaderExport = "openOriginalMediaIngress: turns.openOriginalMediaIngress";
const nextLoader = replaceOnce(loader,loaderExport,loaderExport+", importAppNaturalSource: turns.importAppNaturalSource, openAppNaturalArchiveExchange: turns.openAppNaturalArchiveExchange");
const bridgePath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts";
const bridge = await Bun.file(bridgePath).text();
const nextBridge = replaceOnce(bridge,"function normalizeCommandIngress(status) {",[
  "function normalizeCommandIngress(status) {",
  "  const naturalSourceAck = optionValue(status.naturalSourceAck);",
  "  if (naturalSourceAck !== undefined && naturalSourceAck !== null) return { tag: \"natural-source-ack\", val: {",
  "    ...naturalSourceAck, loan: optionValue(naturalSourceAck.loan), pageIndex: optionValue(naturalSourceAck.pageIndex),",
  "  } };",
].join("\n"));
const actorRoot = "🧰️framework/🔨️modules/🎭️actor/📮️shard-client";
const activationPath = join(actorRoot,"🔐️activation/🟦️.ts");
const activation = await Bun.file(activationPath).text();
const nextActivation = replaceOnce(activation,"export interface ShardInstanceOpenInput {", [
  "export interface ShardInstanceOpenInput {",
  "  readonly purpose?: { readonly tag: \"ordinary\" } | { readonly tag: \"natural-source\"; readonly val: Readonly<{",
  "    declaration: Readonly<{ formatKind: string; extension: string; mediaType: string; binary: boolean }>;",
  "    extent: number;",
  "  }> };",
].join("\n"));
const lifecyclePath = join(actorRoot,"🚪️instance-lifecycle/🟦️.ts");
const lifecycle = await Bun.file(lifecyclePath).text();
const nextLifecycle = replaceOnce(lifecycle, "capabilities: input.capabilities, quotas: input.quotas", 'capabilities: input.capabilities, quotas: input.quotas, purpose: input.purpose ?? { tag: "ordinary" }');
const channelPath = join(runtime,"🔗️channel-adapter/🟦️.ts");
const channel = await Bun.file(channelPath).text();
const narrowKernel = 'Omit<KernelPluginWasmHandle, "createApp"> & { readonly createApp: PluginWasmHandle["createApp"] }';
let nextChannel = replaceOnce(channel,"readonly handle: KernelPluginWasmHandle; readonly release:",`readonly handle: ${narrowKernel}; readonly release:`);
nextChannel = replaceOnce(nextChannel,"    createApp: async (appId) => {","    createApp: async (appId, purpose) => {");
nextChannel = replaceOnce(nextChannel,"await handle.createApp(appId)","await handle.createApp(appId, purpose)");
const typedContract = replaceOnce(nextContract,'import type { NaturalSourceAcceptance, NaturalSourceDeclaration }', [
  'import type { ShardInstanceOpenInput } from "@semio-tech/framework/actor/shard-client/activation";',
  'import type { NaturalSourceAcceptance, NaturalSourceDeclaration }',
].join("\n"));
const withOpenPurpose = replaceOnce(typedContract,"readonly createApp: (appId: string) => Promise<number>;",
  'readonly createApp: (appId: string, purpose?: ShardInstanceOpenInput["purpose"]) => Promise<number>;');
const scopePath = join(runtime,"../🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx");
const scope = await Bun.file(scopePath).text();
const shellRead = "        const bytes = await readBlobBytesBoundedV1(picked, {";
const shellSource = [
  "        const importSource = owner.handle.importAppNaturalSource;",
  "        if (picked.size > input.maximumBytes && importSource !== undefined && format.binary) {",
  "          return openNaturalFileOwnerV1({",
  "            signal: task.signal,",
  '            create: () => owner.handle.createApp(owner.app.id, { tag: "natural-source", val: { declaration: format, extent: picked.size } }),',
  "            importBytes: async (freshInstanceId, signal) => { await importSource(freshInstanceId, format, picked, {",
  "              signal, current: () => !core.pluginOpInFlightRef.current.has(owner.handle.pluginId), progress: task.progress,",
  "            }); },",
  "            retire: (freshInstanceId) => owner.handle.destroyApp(freshInstanceId),",
  "          });",
  "        }",
  shellRead,
].join("\n");
const scopeAnchor = "        const input = naturalFileMediaInputV1(format.formatKind);\n";
const nextScope = replaceOnce(scope,scopeAnchor+shellRead,scopeAnchor+shellSource);
let loaderPurpose = replaceOnce(nextLoader,"): KernelPluginWasmHandle {",`): ${narrowKernel} {`);
loaderPurpose = replaceOnce(loaderPurpose,"const handle: KernelPluginWasmHandle = {",`const handle: ${narrowKernel} = {`);
loaderPurpose = replaceOnce(loaderPurpose,"    createApp: (appId) => {", "    createApp: (appId, purpose) => {");
loaderPurpose = replaceOnce(loaderPurpose, "actor: currentPluginRuntimeActor, config: []", "actor: currentPluginRuntimeActor, purpose, config: []");

const workerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🧷️actor-reservation/🚀️guest-activation/🟦️.ts";
const worker = await Bun.file(workerPath).text();
const nextWorker = replaceOnce(worker,"                appId: fields.surface.appId,","                appId: fields.surface.appId,\n                purpose: { tag: \"ordinary\" },");
const edits: [string,string|null,string][] = [
  [join(root,"original-media-sender.ts"), sender, next],
  [join(root,"natural-source.ts"), null, source],
  [join(root,"natural-source-archive.ts"), null, archiveSource],
  [turnPath,turnSource,turn],
  [contractPath,contract,withOpenPurpose],
  [loaderPath,loader,loaderPurpose],
  [bridgePath,bridge,nextBridge],
  [activationPath,activation,nextActivation],
  [lifecyclePath,lifecycle,nextLifecycle],
  [channelPath,channel,nextChannel],
  [scopePath,scope,nextScope],
  [workerPath,worker,nextWorker],
];
const hash = (text: string) => new Bun.CryptoHasher("sha256").update(text).digest("hex");
const ts = new Bun.Transpiler({ loader: "ts" });
const tsx = new Bun.Transpiler({ loader: "tsx" });
for (const [path, before, after] of edits) {
  assert(Buffer.byteLength(path) <= 256 && before !== after, path);
  (path.endsWith(".tsx") ? tsx : ts).transformSync(after);
  if (before === null) assert(!await Bun.file(path).exists(), path);
}
const relativeEdits = edits.map(([path,before,after])=>[path,before === null ? null : hash(before),after]);
await writeFile(join(generated, "overlay.json"), JSON.stringify(relativeEdits));
const normalizeStart = nextBridge.indexOf("function normalizeCommandIngress(status)");
const normalizeEnd = nextBridge.indexOf("\nfunction uiPatchReceipt", normalizeStart);
assert(normalizeStart !== -1 && normalizeEnd > normalizeStart);
const normalizeCode = nextBridge.slice(normalizeStart,normalizeEnd).replaceAll("\\\x60", "\x60").replaceAll("\\${", "${");
await writeFile(join(generated,"normalize-command-ingress.js"),
  'const optionValue = value => value?.tag === "some" ? value.val : value?.tag === "none" ? undefined : value;\n' +
  'const commandIngressKinds = new Map([[0, "idle"]]);\n' + normalizeCode + "\nexport { normalizeCommandIngress };\n");
console.log("[DEBUG] Staged exact serialized actor driver and natural-source receipt transcoding");
