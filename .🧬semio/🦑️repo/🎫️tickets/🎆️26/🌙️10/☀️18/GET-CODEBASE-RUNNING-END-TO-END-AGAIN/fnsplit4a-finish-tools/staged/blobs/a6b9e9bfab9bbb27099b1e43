/** 🟢️ The shared declarations of useAgentBridge. */
import { readPublishedPageOrigins } from "@semio-tech/framework/assets/published-page";
import { AGENT_BRIDGE_OFFER_ENDPOINT, BRIDGE_DISCOVERY_MIN_INTERVAL_MS, fetchAgentBridgeConfig, nextBridgeDiscoveryIntervalMs, sameAgentBridgeOffer, type AgentBridgeConfig, type AgentBridgeOfferScopeV1, type BridgeOfferFetch } from "../🛰️offer/🟦️.ts";
import { useEffect, useRef, useState } from "react";
import { reduce, type ReduceResult, type ShellCommand, type ShellState } from "@semio-tech/framework/shell";
import { type ApprovalDecision, type ApprovalWithdrawal, type BridgeFlags, type BridgeInstanceRef, type ShellKind, type ShellToGateway } from "@semio-tech/framework/mcp/bridge";
import { encodeShellAppFrame, shellAppFault, type ShellAppCommandV1, type ShellAppFrameV1 } from "@semio-tech/framework/mcp/channel";

export type UseDiscoveredAgentBridgeConfigOptions = {
  readonly enabled?: boolean;
  readonly endpoint?: string;
  readonly fetchImpl?: BridgeOfferFetch;
  /** 🎯️ Who this shell is right now, re-read before every poll — its hub origin, open space and the agents its human
   * delegated there (`AgentBridgeOfferScopeV1`); omitted, the shell is in no hub space and dials only local offers. */
  readonly offerScope?: (signal: AbortSignal) => Promise<AgentBridgeOfferScopeV1>;
};

/** 🛰️ The live supervisor offer, re-asked on a backoff. The returned object identity changes **only**
 * when `url` or `admissionProof` actually change, so an unchanged offer polled a hundred times never
 * re-runs {@link useAgentBridge}'s socket effect — the redial storm U1 already had to fix once. */
export function useDiscoveredAgentBridgeConfig(options: UseDiscoveredAgentBridgeConfigOptions = {}): AgentBridgeConfig | null {
  const { enabled = true, endpoint = AGENT_BRIDGE_OFFER_ENDPOINT, fetchImpl, offerScope } = options;
  const [config, setConfig] = useState<AgentBridgeConfig | null>(null);
  const fetchImplRef = useRef(fetchImpl);
  fetchImplRef.current = fetchImpl;
  const offerScopeRef = useRef(offerScope);
  offerScopeRef.current = offerScope;

  useEffect(() => {
    if (!enabled || Object.keys(readPublishedPageOrigins()).length > 0) {
      setConfig(null);
      return;
    }
    let disposed = false;
    let timer: ReturnType<typeof setTimeout> | null = null;
    let interval = BRIDGE_DISCOVERY_MIN_INTERVAL_MS;
    const abort = new AbortController();

    const poll = async (): Promise<void> => {
      const scope = await (offerScopeRef.current?.(abort.signal) ?? Promise.resolve(null)).catch(() => null);
      if (disposed) return;
      const discovered = await fetchAgentBridgeConfig(endpoint, fetchImplRef.current, abort.signal, scope);
      if (disposed) return;
      setConfig((current) => (sameAgentBridgeOffer(current, discovered) ? current : discovered));
      interval = nextBridgeDiscoveryIntervalMs(interval, discovered);
      if (!disposed) timer = setTimeout(() => void poll(), interval);
    };

    void poll();
    return () => {
      disposed = true;
      abort.abort();
      if (timer !== null) clearTimeout(timer);
    };
  }, [enabled, endpoint]);

  return config;
}

/** 📦️ Wire payload codec for the `bytes` fields inside `ShellState`/`ShellStatePatch`/
 * `ShellCommand`/`AppCommand` bridge frames. `📋️master.md` §2.2 names these `pack`, but that
 * binary format lives in `semio-framework-actor` — a peer ticket's exclusive, mid-rewrite
 * territory (`📌️important.md`'s collision matrix, packet A4) this packet must not depend on
 * (mirrors P1b's own §7.5 deviation for the frame envelope itself). Plain JSON is what the
 * gateway's own dispatch already expects on the other end (P9 report §8.4:
 * `serde_json::from_value::<ShellCommand>`), so it is used symmetrically here for both
 * directions. */
export function encodeJsonPayload(value: unknown): Uint8Array {
  return new TextEncoder().encode(JSON.stringify(value));
}

export function decodeJsonPayload<T>(bytes: Uint8Array): T {
  return JSON.parse(new TextDecoder("utf-8").decode(bytes)) as T;
}

/** 📤️ Builds the `shellState` frame publishing a full snapshot — sent on `welcome` and whenever
 * a `shellStatePatch`'s `baseRevision` would not match (this packet always sends full snapshots;
 * P11's real adoption packet can add patch diffing without touching this frame shape). */
export function buildShellStateFrame(state: ShellState): ShellToGateway {
  return { variant: "shellState", revision: BigInt(state.revision), state: encodeJsonPayload(state) };
}

export type ApplyInboundShellCommandOutcome = {
  /** 🎮️ `null` only when `commandBytes` failed to decode — no `ShellCommand` ever existed to reduce. */
  readonly command: ShellCommand | null;
  /** ❌️ `null` only when `commandBytes` failed to decode; otherwise the real `reduce()` outcome
   * (which itself may be `{ok:false}` for a command the reducer rejects — that is NOT a decode
   * failure and is reported via `result.error`, not `null`). */
  readonly result: ReduceResult | null;
  readonly resultFrame: ShellToGateway;
};

/** 🧮️ Applies one inbound `shellCommand` frame's JSON-encoded payload to `state` via the
 * `@semio-tech/framework-os-shell` reducer twin, returning the decoded command, the reducer's own
 * {@link ReduceResult}, and the `shellCommandResult` frame to send back — the exact seam
 * `useAgentBridge`'s socket handler and this file's own tests both call. */
export function applyInboundShellCommand(state: ShellState, seq: bigint, commandBytes: Uint8Array, nowMs: number): ApplyInboundShellCommandOutcome {
  let command: ShellCommand;
  try {
    command = decodeJsonPayload<ShellCommand>(commandBytes);
  } catch (error) {
    const fault = error instanceof Error ? error.message : "malformed ShellCommand payload";
    return { command: null, result: null, resultFrame: { variant: "shellCommandResult", inReplyTo: seq, ok: false, fault } };
  }
  const result = reduce(state, command, nowMs);
  const resultFrame: ShellToGateway = { variant: "shellCommandResult", inReplyTo: seq, ok: result.ok, fault: result.ok ? null : result.error.kind };
  return { command, result, resultFrame };
}

/** 🗿️ One inbound `appCommand` frame, decoded, with the correlation id its answer must carry.
 * `instanceId` is the shell's OWN plugin-instance id (the one this shell published in its
 * `instances` frame) — the gateway resolved it from the shell's census, never invented it. */
export type AgentAppCommandRequest = {
  readonly seq: bigint;
  readonly instanceId: string;
  readonly command: ShellAppCommandV1;
};

/** 🧑‍🔧️ What a host (`🏛️ShellHost`) supplies to make this shell the agent's document owner: it
 * executes `request` against the LIVE plugin instance — through the same action-dispatch lane the
 * UI uses, so the human sees the edit and can undo it — and answers the frames. Returning an
 * `error` frame is a first-class outcome; throwing is turned into one, never into silence. */
export type AgentAppCommandHandler = (request: AgentAppCommandRequest) => Promise<readonly ShellAppFrameV1[]> | readonly ShellAppFrameV1[];

/** 🚫️ The answer a shell with no artifact route mounted gives. It is a REFUSAL, not a timeout: the
 * agent learns immediately that this shell renders the agent chrome but does not own documents for
 * it, and its session can be re-resolved onto the headless channel instead of burning a wall
 * budget. */
/** 📇️ The stable empty census — a fresh `[]` per render would re-publish an `instances` frame on
 * every render, which is exactly the redial-storm class of defect one level down. */
export const EMPTY_INSTANCES: readonly BridgeInstanceRef[] = [];

export const NO_ARTIFACT_ROUTE_MESSAGE = "this shell has no live artifact route mounted — it renders the agent surface but does not execute artifact commands; resolve a headless context (`--folder`/`--hub`) for mutations";

/** 🔶️ Builds the `appFrames` answer for one request, never throwing: a handler that rejects or
 * throws becomes a single `channel.not-wired` error frame naming the failure. The correlation id
 * and the instance id are echoed verbatim — the gateway refuses a reply addressed elsewhere. */
export async function answerAgentAppCommand(request: AgentAppCommandRequest, handler: AgentAppCommandHandler | undefined): Promise<ShellToGateway> {
  let frames: readonly ShellAppFrameV1[];
  if (handler === undefined) {
    frames = [shellAppFault("plugin.unavailable", NO_ARTIFACT_ROUTE_MESSAGE)];
  } else {
    try {
      frames = await handler(request);
    } catch (error) {
      frames = [shellAppFault("channel.not-wired", error instanceof Error ? error.message : "the shell's artifact route failed without a message")];
    }
  }
  return { variant: "appFrames", inReplyTo: request.seq, instanceId: request.instanceId, frames: frames.map((frame) => encodeShellAppFrame(frame)) };
}

/** 🚦️ `unavailable` and `incompatible` are terminal for one offer: the gateway did not answer the
 * handshake within {@link BRIDGE_HANDSHAKE_DEADLINE_MS} {@link BRIDGE_UNANSWERED_ATTEMPTS} times, or
 * refused / welcomed with another bridge version. The shell then stops dialling that offer and waits
 * for a different one (a restarted gateway publishes a new url and proof). */
export type AgentBridgeStatus = "disabled" | "connecting" | "open" | "reconnecting" | "closed" | "unavailable" | "incompatible";

/** 🤝️ The bridge versions an `incompatible` status names, so the notice can say which side is older. */
export type AgentBridgeVersionMismatch = { readonly gateway: number; readonly shell: number };

export type AgentBridgePresence = { readonly active: boolean; readonly label: string; readonly invocationId: string | null };

export const IDLE_PRESENCE: AgentBridgePresence = { active: false, label: "", invocationId: null };

export type PendingAgentApproval = { readonly approvalId: string; readonly summary: string; readonly requestedAtMs: number };

/** 💬️ One entry of the live agent conversation, exactly as the bridge reported it. `userMessage` is
 * a turn this shell itself sent (echoed locally the moment the frame leaves, so the panel is never
 * behind the human's own typing); every other kind is a real `GatewayToShell` frame the gateway
 * emitted from its own `tools/call` dispatch or approval gate. Nothing here is synthesised from a
 * guess: a tool call with no result yet simply has `state: "running"`. */
export type AgentConversationEntry =
  | { readonly kind: "userMessage"; readonly id: string; readonly text: string; readonly atMs: number }
  /** 🛑️ `cancelling` is this shell's own optimistic state between `cancelToolCall` leaving and the
   * gateway's real `agentToolResult` landing: cancellation is cooperative, so the call may still
   * succeed, fail, or settle as cancelled — the panel says "asked to stop", never "stopped". */
  | { readonly kind: "toolCall"; readonly id: string; readonly toolName: string; readonly args: string; readonly state: "running" | "cancelling" | "ok" | "failed"; readonly summary: string | null; readonly atMs: number }
  /** ⛩️ `withdrawn` is the gateway taking the request back before anyone here decided it — the call was
   * cancelled, the countdown ran out, or a newer shell now carries it (`withdrawal` says which). */
  | { readonly kind: "approval"; readonly id: string; readonly summary: string; readonly state: "pending" | "resolved" | "withdrawn"; readonly decision: ApprovalDecision | null; readonly withdrawal: ApprovalWithdrawal | null; readonly atMs: number }
  /** 🗄️ The agent's own words, from a `GatewayToShell.agentReply` frame. `state` is `"streaming"`
   * until the chunk marked `complete` lands, so the panel can say a turn is still arriving without
   * guessing; `text` is the chunks concatenated in arrival order, never re-ordered or re-flowed. */
  | { readonly kind: "agentMessage"; readonly id: string; readonly text: string; readonly state: "streaming" | "complete"; readonly inReplyTo: string | null; readonly atMs: number };

/** ✂️ How many conversation entries the panel retains. The bridge is a live view, not an archive:
 * an agent running for hours must not grow this array without bound, and the oldest entries are the
 * least useful ones to keep. */
export const AGENT_CONVERSATION_MAX_ENTRIES = 200;

/** ➕️ Appends one entry and trims to {@link AGENT_CONVERSATION_MAX_ENTRIES}, oldest first. */
export function appendConversationEntry(current: readonly AgentConversationEntry[], entry: AgentConversationEntry): readonly AgentConversationEntry[] {
  const next = [...current, entry];
  return next.length > AGENT_CONVERSATION_MAX_ENTRIES ? next.slice(next.length - AGENT_CONVERSATION_MAX_ENTRIES) : next;
}

/** 🔁️ Replaces the entry with `id`, leaving the rest untouched — how a `toolCall` becomes its own
 * result and a `pending` approval becomes a resolved one, in place, rather than as a second row. */
export function updateConversationEntry(current: readonly AgentConversationEntry[], id: string, update: (entry: AgentConversationEntry) => AgentConversationEntry): readonly AgentConversationEntry[] {
  let found = false;
  const next = current.map((entry) => {
    if (entry.id !== id) return entry;
    found = true;
    return update(entry);
  });
  return found ? next : current;
}

export type UseAgentBridgeOptions = {
  /** 🪝️ Omit it (the product case) and the hook discovers the live gateway offer from the local
   * supervisor on {@link AGENT_BRIDGE_OFFER_ENDPOINT}; pass one (including `null`) and discovery is
   * off entirely, which is what every test and every embedder with its own supervisor does. */
  readonly config?: AgentBridgeConfig | null;
  /** 🔎️ Overrides for the discovery seam — the endpoint path and the `fetch` used to ask it. */
  readonly discoveryEndpoint?: string;
  readonly discoveryFetch?: BridgeOfferFetch;
  /** 🔷️ See {@link UseDiscoveredAgentBridgeConfigOptions.offerScope}. */
  readonly offerScope?: (signal: AbortSignal) => Promise<AgentBridgeOfferScopeV1>;
  readonly shellKind?: ShellKind;
  readonly shellSessionId?: string;
  readonly principalActor?: string;
  readonly flags?: BridgeFlags;
  readonly initialState?: ShellState;
  readonly onCommandApplied?: (command: ShellCommand | null, result: ReduceResult | null) => void;
  /** 🔸️ Makes this shell the agent's document owner. Omit it and every inbound `appCommand` is
   * answered with {@link NO_ARTIFACT_ROUTE_MESSAGE} — the shell still renders the agent surface,
   * it just does not claim to execute mutations. */
  readonly onAppCommand?: AgentAppCommandHandler;
  /** 📣️ The live plugin instances this shell owns, published as `instances` frames whenever the
   * array's identity changes. The gateway resolves an `AppCommand`'s target from this census, so a
   * shell that publishes nothing is a shell no artifact verb can address. */
  readonly instances?: readonly BridgeInstanceRef[];
};

export const RECONNECT_BASE_MS = 1000;

export const RECONNECT_MAX_MS = 30000;

export const PING_INTERVAL_MS = 20000;

/** ⏳️ How long a dialled gateway has to answer `hello` with `welcome` or `refused`. */
export const BRIDGE_HANDSHAKE_DEADLINE_MS = 8000;

/** 🔢️ How many dials in a row may go unanswered before the offer is `unavailable`. */
export const BRIDGE_UNANSWERED_ATTEMPTS = 3;
