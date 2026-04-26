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
export type StorybookKitGraphqlHandle = { execute(requestJson: string): Promise<string> };

function storybookKitGraphqlData(response: unknown): Record<string, unknown> {
  if (response == null || typeof response !== "object") throw new Error("kitGraphql: response is not an object");
  const r = response as { data?: Record<string, unknown> | null; errors?: readonly { message?: string }[] };
  if (Array.isArray(r.errors) && r.errors.length > 0) throw new Error(r.errors[0]?.message ?? "GraphQL error");
  if (r.data != null && typeof r.data === "object") return r.data as Record<string, unknown>;
  throw new Error("kitGraphql: no data in response");
}

/** @emoji 🌐 Runs one GraphQL document over a handle; resolves with the **complete JSON** response. */
export async function storybookKitGraphqlRun(
  handle: StorybookKitGraphqlHandle,
  body: { query: string; variables?: Record<string, unknown>; operationName?: string },
): Promise<unknown> {
  const json = await handle.execute(JSON.stringify(body));
  return JSON.parse(json) as unknown;
}

export type StorybookKitStoreExecuteResult =
  | { ok: true; result: unknown }
  | { ok: false; error: { kind: string; message: string } };

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
    let q: { query: string; variables?: Record<string, unknown> };
    switch (tag) {
      case "newSession":
        q = { query: `mutation { newSession }` };
        break;
      case "endSession": {
        const idv = (value as { id?: string } | null)?.id;
        if (typeof idv !== "string") throw new Error("endSession id");
        q = { query: `mutation($id: String!) { endSession(id: $id) }`, variables: { id: idv } };
        break;
      }
      case "newAlternative": {
        const v = value as { fromCheckpoint?: string | null; name: string } | null;
        if (v == null || typeof v.name !== "string") throw new Error("newAlternative");
        q = {
          query: `mutation($fromCheckpoint: String, $name: String!) { newAlternative(fromCheckpoint: $fromCheckpoint, name: $name) }`,
          variables: { fromCheckpoint: v.fromCheckpoint ?? null, name: v.name },
        };
        break;
      }
      case "batch": {
        const cmds = (value as { commands?: unknown[] } | null)?.commands;
        if (!Array.isArray(cmds)) throw new Error("batch.commands");
        q = {
          query: `mutation($input: KitStoreBatchInput!) { kitStore { batch(input: $input) { clientMutationId results { kind } } } }`,
          variables: { input: { commands: cmds } },
        };
        break;
      }
      default:
        throw new Error(`executeStoreCommand: unhandled ${tag}`);
    }
    const result = storybookKitGraphqlData(await storybookKitGraphqlRun(handle, q));
    return { ok: true, result };
  } catch (e) {
    return { ok: false, error: { kind: "Internal", message: String(e) } };
  }
}
// #endregion 🧰StorybookGraphqlWire
