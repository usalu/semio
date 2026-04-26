// #region 🧲Header
// Storybook: lazy wasm init for @semio/rs-wasm
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion

import initSemio, { boot, generateId, KitStoreHandle } from "@semio/rs-wasm";

// Bundle `semio.js` in Storybook, the default `new URL("semio_bg.wasm", import.meta.url)` is often wrong;
// point at the pkg explicitly so `fetch` loads the file.
// Path: .storybook/stories/kit-store/ → parent×4 = semio/semio → sibling rs/pkg
const semioWasmUrl = new URL("../../../../rs/pkg/semio_bg.wasm", import.meta.url);

let initPromise: Promise<void> | null = null;

/** Single-flight wasm init; safe to call from multiple components. */
export function ensureSemioWasm(): Promise<void> {
  if (typeof window === "undefined") {
    return Promise.resolve();
  }
  if (!initPromise) {
    initPromise = (async () => {
      try {
        await initSemio(semioWasmUrl);
        boot();
      } catch (e) {
        initPromise = null;
        throw e;
      }
    })();
  }
  return initPromise;
}

export { boot, generateId, initSemio, KitStoreHandle };

// #region 🧰StorybookGraphqlWire
/** @emoji 🔌 Storybook-only GraphQL execute boundary (same shape as WASM `KitStoreHandle.execute`).
 * Returns the **complete JSON** GraphQL response document — no NDJSON / line-of-json. */
export type StorybookKitGraphqlHandle = Pick<KitStoreHandle, "execute" | "subscribe">;

function storybookKitGraphqlData(response: unknown): Record<string, unknown> {
  if (response == null || typeof response !== "object") throw new Error("kitGraphql: response is not an object");
  const r = response as { data?: Record<string, unknown> | null; errors?: readonly { message?: string }[] };
  if (Array.isArray(r.errors) && r.errors.length > 0) throw new Error(r.errors[0]?.message ?? "GraphQL error");
  if (r.data != null && typeof r.data === "object") return r.data as Record<string, unknown>;
  throw new Error("kitGraphql: no data in response");
}

/** @emoji 🌐 Runs one GraphQL document over a handle; resolves with the **complete JSON** response. */
export async function storybookKitGraphqlRun(
  handle: Pick<KitStoreHandle, "execute">,
  body: { query: string; variables?: Record<string, unknown>; operationName?: string },
): Promise<unknown> {
  const json = await handle.execute(JSON.stringify(body));
  return JSON.parse(json) as unknown;
}

export type StorybookKitStoreExecuteResult =
  | { ok: true; result: unknown }
  | { ok: false; error: { kind: string; message: string } };

function parseSemioKitCommandRow(eventJson: string): { requestId: string; phase: string; result?: unknown; error?: unknown } | undefined {
  try {
    const msg = JSON.parse(eventJson) as { data?: { eventStream?: unknown } | null };
    const raw = msg.data?.eventStream;
    if (raw == null || typeof raw !== "object") return undefined;
    const top = raw as Record<string, unknown>;
    const inner = (top.SemioKitCommand ?? top.semioKitCommand) as Record<string, unknown> | undefined;
    if (!inner || typeof inner.requestId !== "string" || typeof inner.phase !== "string") return undefined;
    return {
      requestId: inner.requestId,
      phase: inner.phase,
      result: inner.result,
      error: inner.error,
    };
  } catch {
    return undefined;
  }
}

/** Shell dispatch wraps payloads as `{ data: { <field>: … } }` — unwrap to the GraphQL `data` object Storybook helpers used before. */
function kitShellEventResultAsGraphqlData(raw: unknown): Record<string, unknown> {
  if (raw != null && typeof raw === "object" && "data" in raw) {
    const d = (raw as { data: unknown }).data;
    if (d != null && typeof d === "object") return d as Record<string, unknown>;
  }
  return (raw as Record<string, unknown>) ?? {};
}

/** @emoji 🧭 `submitKitCommand` + wait for matching `SemioKitCommand` succeeded on `eventStream`. */
async function storybookSubmitKitCommandAwaitData(
  handle: StorybookKitGraphqlHandle,
  commandKind: string,
  shellVariables: Record<string, unknown>,
  timeoutMs = 30_000,
): Promise<Record<string, unknown>> {
  const lifecycleByRid = new Map<string, Array<{ phase: string; result?: unknown; error?: unknown }>>();
  const record = (row: { requestId: string; phase: string; result?: unknown; error?: unknown }) => {
    const arr = lifecycleByRid.get(row.requestId) ?? [];
    arr.push({ phase: row.phase, result: row.result, error: row.error });
    lifecycleByRid.set(row.requestId, arr);
  };

  void handle.subscribe(JSON.stringify({ query: "subscription { eventStream }" }), (eventJson: string) => {
    const row = parseSemioKitCommandRow(eventJson);
    if (row) record(row);
  });

  await new Promise((r) => setTimeout(r, 20));

  const mutBody = {
    query: `mutation($input: KitCommandShellInput!) { submitKitCommand(input: $input) { requestId commandKind accepted } }`,
    variables: { input: { commandKind, request: { variables: shellVariables } } },
  };
  const mutJson = await handle.execute(JSON.stringify(mutBody));
  const resp = JSON.parse(mutJson) as { data?: { submitKitCommand?: { requestId?: string } }; errors?: { message?: string }[] };
  if (resp.errors?.length) throw new Error(resp.errors[0]?.message ?? "GraphQL error");
  const rid = resp.data?.submitKitCommand?.requestId;
  if (!rid) throw new Error("submitKitCommand: no requestId");

  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const arr = lifecycleByRid.get(rid);
    const failed = arr?.find((x) => x.phase === "failed");
    if (failed) {
      const em = failed.error as { message?: string } | undefined;
      throw new Error(em?.message ?? "kit command failed");
    }
    const succ = arr?.find((x) => x.phase === "succeeded");
    if (succ) return kitShellEventResultAsGraphqlData(succ.result ?? null);
    await new Promise((r) => setTimeout(r, 8));
  }
  throw new Error(`submitKitCommand: timeout waiting for ${rid}`);
}

/** @emoji 🧾 Executes one tagged `KitStoreExecuteCommand` variant (session / batch) over GraphQL (Storybook). */
export async function storybookKitGraphqlExecuteStoreCommand(
  handle: StorybookKitGraphqlHandle,
  cmd: unknown,
): Promise<StorybookKitStoreExecuteResult> {
  try {
    if (cmd == null || typeof cmd !== "object" || Array.isArray(cmd)) throw new Error("command object expected");
    const o = cmd as Record<string, unknown>;
    const keys = Object.keys(o);
    if (keys.length !== 1) throw new Error("single tagged variant expected");
    const tag = keys[0]!;
    const value = o[tag];
    let data: Record<string, unknown>;
    switch (tag) {
      case "newSession":
        data = await storybookSubmitKitCommandAwaitData(handle, "newSession", {});
        break;
      case "endSession": {
        const idv = (value as { id?: string } | null)?.id;
        if (typeof idv !== "string") throw new Error("endSession id");
        data = await storybookSubmitKitCommandAwaitData(handle, "endSession", { id: idv });
        break;
      }
      case "newAlternative": {
        const v = value as { fromCheckpoint?: string | null; name: string } | null;
        if (v == null || typeof v.name !== "string") throw new Error("newAlternative");
        data = await storybookSubmitKitCommandAwaitData(handle, "newAlternative", {
          fromCheckpoint: v.fromCheckpoint ?? null,
          name: v.name,
        });
        break;
      }
      case "batch": {
        const cmds = (value as { commands?: unknown[] } | null)?.commands;
        if (!Array.isArray(cmds)) throw new Error("batch.commands");
        data = await storybookSubmitKitCommandAwaitData(handle, "batch", { input: { commands: cmds } });
        break;
      }
      default:
        throw new Error(`executeStoreCommand: unhandled ${tag}`);
    }
    return { ok: true, result: data };
  } catch (e) {
    return { ok: false, error: { kind: "Internal", message: String(e) } };
  }
}
// #endregion 🧰StorybookGraphqlWire
