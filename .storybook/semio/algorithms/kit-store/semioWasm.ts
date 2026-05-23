// #region 🧲Header
// Storybook: lazy wasm init; GraphQL-only rs/js wire via `dev://empty` + `installProjection`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion

import initSemio, { boot, KitStoreHandle } from "@semio/rs-wasm";

/** @emoji 🧪 Empty WASM store URI — must match {@link RS_WASM_EMPTY_STORE_URI} in `@semio/js`. */
export const RS_WASM_EMPTY_STORE_URI = "dev://empty" as const;

// Bundle `semio.js` in Storybook, the default `new URL("semio_bg.wasm", import.meta.url)` is often wrong;
// point at the pkg explicitly so `fetch` loads the file.
// Path: `.storybook/semio/algorithms/kit-store/` → three parents = repo root → `semio/client/lib/rs/pkg`.
const semioWasmUrl = new URL("../../../semio/client/lib/rs/pkg/semio_bg.wasm", import.meta.url);

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

export { boot, initSemio, KitStoreHandle };

// #region 🧰StorybookGraphqlWire
/** @emoji 🔌 Storybook-only GraphQL execute boundary (same shape as WASM `KitStoreHandle.execute`). */
export type StorybookKitGraphqlHandle = Pick<KitStoreHandle, "execute" | "subscribe">;

function storybookKitGraphqlData(response: unknown): Record<string, unknown> {
  if (response == null || typeof response !== "object") throw new Error("kitGraphql: response is not an object");
  const r = response as { data?: Record<string, unknown> | null; errors?: readonly { message?: string }[] };
  if (Array.isArray(r.errors) && r.errors.length > 0) throw new Error(r.errors[0]?.message ?? "GraphQL error");
  if (r.data != null && typeof r.data === "object") return r.data as Record<string, unknown>;
  throw new Error("kitGraphql: no data in response");
}

async function sbGqlMut(handle: StorybookKitGraphqlHandle, query: string, variables?: Record<string, unknown>): Promise<Record<string, unknown>> {
  const json = await handle.execute(JSON.stringify({ query, variables }));
  return storybookKitGraphqlData(JSON.parse(json) as unknown);
}

/** @emoji 📥 Opens `dev://empty` then seeds kit JSON through GraphQL `installProjection` (rs/js contract). */
export async function createStorybookKitGraphqlHandle(seedKit: unknown): Promise<KitStoreHandle> {
  await ensureSemioWasm();
  const created = KitStoreHandle.create(RS_WASM_EMPTY_STORE_URI);
  const handle = created instanceof Promise ? await created : created;
  const json = JSON.stringify(JSON.parse(JSON.stringify(seedKit)) as object);
  const stores = await sbGqlMut(handle, `query { session { stores { edges { node { id } } } } }`);
  const edges = ((stores["session"] as Record<string, unknown> | undefined)?.["stores"] as Record<string, unknown> | undefined)?.[
    "edges"
  ] as readonly Record<string, unknown>[] | undefined;
  const storeId = String((edges?.[0]?.["node"] as Record<string, unknown> | undefined)?.["id"] ?? "");
  if (storeId === "") throw new Error("createStorybookKitGraphqlHandle: no session store");
  const installed = await sbGqlMut(
    handle,
    `mutation($storeId: ID!, $json: String!) { session { store(id: $storeId) { installProjection(json: $json) { ok errors { message } } } } } }`,
    { storeId, json },
  );
  const ip = (installed["session"] as Record<string, unknown> | undefined)?.["store"] as Record<string, unknown> | undefined;
  const payload = ip?.["installProjection"] as Record<string, unknown> | undefined;
  if (payload?.["ok"] !== true) {
    const err = (payload?.["errors"] as readonly { message?: string }[] | undefined)?.[0]?.message;
    throw new Error(err ?? "installProjection failed");
  }
  return handle;
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

function str(v: unknown): string | undefined {
  return typeof v === "string" && v.length > 0 ? v : undefined;
}

function altIn(
  payload: { alternativeId?: string | null },
  draftBlock: { alternativeId?: string | null } | null,
  newDraft: { alternativeId?: string | null } | null,
): string {
  const a = str(payload.alternativeId) ?? str(draftBlock?.alternativeId) ?? str(newDraft?.alternativeId);
  if (!a) throw new Error("Storybook VCS: set Alternative id (drafts/transactions are alternative-scoped in GraphQL)");
  return a;
}

/** @emoji 🧾 Maps legacy `executeSessionCommands` / nested draft-transaction shapes to `Mutation.session` (no `kitStore.batch`). */
async function storybookExecuteSessionCommands(
  handle: StorybookKitGraphqlHandle,
  value: Record<string, unknown>,
): Promise<Record<string, unknown>> {
  const commands = value["commands"];
  if (!Array.isArray(commands) || commands.length === 0) throw new Error("executeSessionCommands.commands");
  const results: unknown[] = [];
  for (const raw of commands) {
    if (raw == null || typeof raw !== "object" || Array.isArray(raw)) throw new Error("session command object expected");
    const c = raw as Record<string, unknown>;
    const k = Object.keys(c);
    if (k.length !== 1) throw new Error("single tagged session command");
    const t = k[0]!;
    if (t === "newDraft") {
      const nd = c["newDraft"] as Record<string, unknown> | null;
      if (!nd) throw new Error("newDraft");
      const aid = altIn(value as { alternativeId?: string | null }, null, nd);
      const pcp = nd["checkpointId"] == null ? null : String(nd["checkpointId"]);
      const d = await sbGqlMut(
        handle,
        `mutation($aid: KitAlternativeIdIn!, $pcp: String) { session { alternative(id: $aid) { createDraft(parentCheckpointId: $pcp) { id } } } }`,
        { aid: { id: aid }, pcp: pcp && pcp.length > 0 ? pcp : null },
      );
      const session = d["session"] as Record<string, unknown> | undefined;
      const alt = session?.["alternative"] as Record<string, unknown> | undefined;
      const draft = alt?.["createDraft"] as Record<string, unknown> | undefined;
      const id = str(draft?.["id"]);
      results.push({ newDraft: { draftId: id ?? "" } });
      continue;
    }
    if (t === "executeKitDraftCommands") {
      const ek = c["executeKitDraftCommands"] as Record<string, unknown> | null;
      if (!ek) throw new Error("executeKitDraftCommands");
      const draftId = str(ek["id"]);
      if (!draftId) throw new Error("executeKitDraftCommands.id");
      const aid = altIn(value as { alternativeId?: string | null }, ek, null);
      const dcmds = ek["commands"];
      if (!Array.isArray(dcmds)) throw new Error("executeKitDraftCommands.commands");
      for (const dc of dcmds) {
        if (dc == null || typeof dc !== "object" || Array.isArray(dc)) throw new Error("draft command object");
        const dco = dc as Record<string, unknown>;
        const dk = Object.keys(dco);
        if (dk.length !== 1) throw new Error("single tagged draft command");
        const dt = dk[0]!;
        if (dt === "startTransaction") {
          const d = await sbGqlMut(
            handle,
            `mutation($aid: KitAlternativeIdIn!) { session { alternative(id: $aid) { draft { startTransaction { id } } } } }`,
            { aid: { id: aid } },
          );
          const sessionT = d["session"] as Record<string, unknown> | undefined;
          const altT = sessionT?.["alternative"] as Record<string, unknown> | undefined;
          const draftT = altT?.["draft"] as Record<string, unknown> | undefined;
          const startTx = draftT?.["startTransaction"] as Record<string, unknown> | undefined;
          const tid = str(startTx?.["id"]);
          results.push({ executeKitDraftCommands: { results: [{ startTransaction: { transactionId: tid ?? "" } }] } });
          continue;
        }
        if (dt === "finalizeToKitCheckpoint") {
          const msg = str((dco["finalizeToKitCheckpoint"] as Record<string, unknown> | null)?.["message"]) ?? "checkpoint";
          const d = await sbGqlMut(
            handle,
            `mutation($aid: KitAlternativeIdIn!, $msg: String!) { session { alternative(id: $aid) { draft { finalize(message: $msg) { id } } } } }`,
            { aid: { id: aid }, msg },
          );
          const sessionF = d["session"] as Record<string, unknown> | undefined;
          const altF = sessionF?.["alternative"] as Record<string, unknown> | undefined;
          const draftF = altF?.["draft"] as Record<string, unknown> | undefined;
          const finalize = draftF?.["finalize"] as Record<string, unknown> | undefined;
          const cpid = str(finalize?.["id"]);
          results.push({ executeKitDraftCommands: { results: [{ finalizeToKitCheckpoint: { checkpointId: cpid ?? "" } }] } });
          continue;
        }
        if (dt === "abort") {
          await sbGqlMut(handle, `mutation($aid: KitAlternativeIdIn!) { session { alternative(id: $aid) { draft { abort } } } }`, { aid: { id: aid } });
          results.push({ executeKitDraftCommands: { results: [{ abort: { ok: true } }] } });
          continue;
        }
        if (dt === "undo" || dt === "redo") {
          const cnt = (dco[dt] as Record<string, unknown> | null)?.["count"];
          const count = typeof cnt === "number" ? cnt : 1;
          const d = await sbGqlMut(
            handle,
            `mutation($aid: KitAlternativeIdIn!, $c: Int) { session { alternative(id: $aid) { draft { ${dt}(count: $c) } } } }`,
            { aid: { id: aid }, c: count },
          );
          const sessionU = d["session"] as Record<string, unknown> | undefined;
          const altU = sessionU?.["alternative"] as Record<string, unknown> | undefined;
          const draftU = altU?.["draft"] as Record<string, unknown> | undefined;
          const ok = draftU?.[dt] === true;
          results.push({ executeKitDraftCommands: { results: [{ [dt]: { ok } }] } });
          continue;
        }
        if (dt === "executeTransactionCommands") {
          const etc = dco["executeTransactionCommands"] as Record<string, unknown> | null;
          if (!etc) throw new Error("executeTransactionCommands");
          const txid = str(etc["id"]);
          const txcmds = etc["commands"];
          if (!txid || !Array.isArray(txcmds)) throw new Error("executeTransactionCommands id/commands");
          for (const txc of txcmds) {
            if (txc == null || typeof txc !== "object" || Array.isArray(txc)) throw new Error("tx command");
            const txo = txc as Record<string, unknown>;
            const txk = Object.keys(txo);
            if (txk.length !== 1) throw new Error("single tagged tx command");
            const txt = txk[0]!;
            if (txt === "finalize") {
              const d = await sbGqlMut(
                handle,
                `mutation($aid: KitAlternativeIdIn!) { session { alternative(id: $aid) { draft { transaction { finalize { id } } } } } }`,
                { aid: { id: aid } },
              );
              const sessionX = d["session"] as Record<string, unknown> | undefined;
              const altX = sessionX?.["alternative"] as Record<string, unknown> | undefined;
              const draftX = altX?.["draft"] as Record<string, unknown> | undefined;
              const tx = draftX?.["transaction"] as Record<string, unknown> | undefined;
              const fin = tx?.["finalize"];
              const cpid = str(fin != null && typeof fin === "object" && !Array.isArray(fin) ? (fin as Record<string, unknown>)["id"] : undefined);
              results.push({
                executeKitDraftCommands: { results: [{ executeTransactionCommands: { results: [{ finalize: { checkpointId: cpid ?? "" } }] } }] },
              });
              continue;
            }
            if (txt === "abort") {
              await sbGqlMut(handle, `mutation($aid: KitAlternativeIdIn!) { session { alternative(id: $aid) { draft { transaction { abort } } } } }`, { aid: { id: aid } });
              results.push({
                executeKitDraftCommands: { results: [{ executeTransactionCommands: { results: [{ abort: { ok: true } }] } }] },
              });
              continue;
            }
            if (txt === "undo" || txt === "redo") {
              const cnt = (txo[txt] as Record<string, unknown> | null)?.["count"];
              const count = typeof cnt === "number" ? cnt : 1;
              const d = await sbGqlMut(
                handle,
                `mutation($aid: KitAlternativeIdIn!, $c: Int) { session { alternative(id: $aid) { draft { transaction { ${txt}(count: $c) } } } } }`,
                { aid: { id: aid }, c: count },
              );
              const sessionTx = d["session"] as Record<string, unknown> | undefined;
              const altTx = sessionTx?.["alternative"] as Record<string, unknown> | undefined;
              const draftTx = altTx?.["draft"] as Record<string, unknown> | undefined;
              const transaction = draftTx?.["transaction"] as Record<string, unknown> | undefined;
              const ok = transaction?.[txt] === true;
              results.push({
                executeKitDraftCommands: { results: [{ executeTransactionCommands: { results: [{ [txt]: { ok } }] } }] },
              });
              continue;
            }
            if (txt === "undoAll" || txt === "redoAll") {
              const gqlOp = txt === "undoAll" ? "undo" : "redo";
              const d = await sbGqlMut(
                handle,
                `mutation($aid: KitAlternativeIdIn!) { session { alternative(id: $aid) { draft { transaction { ${gqlOp}(count: 99) } } } } }`,
                { aid: { id: aid } },
              );
              const sessionA = d["session"] as Record<string, unknown> | undefined;
              const altA = sessionA?.["alternative"] as Record<string, unknown> | undefined;
              const draftA = altA?.["draft"] as Record<string, unknown> | undefined;
              const transactionA = draftA?.["transaction"] as Record<string, unknown> | undefined;
              const ok = transactionA?.[gqlOp] === true;
              results.push({
                executeKitDraftCommands: { results: [{ executeTransactionCommands: { results: [{ [txt]: { ok } }] } }] },
              });
              continue;
            }
            throw new Error(`executeTransactionCommands: unhandled ${txt}`);
          }
          continue;
        }
        throw new Error(`executeKitDraftCommands: unhandled ${dt}`);
      }
      continue;
    }
    throw new Error(`executeSessionCommands: unhandled ${t}`);
  }
  return { executeSessionCommands: { results: results } };
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
        data = { newSession: { id: "__wip__" } };
        break;
      case "endSession":
        data = {};
        break;
      case "newAlternative": {
        const v = value as { fromCheckpoint?: string | null; name: string } | null;
        if (v == null || typeof v.name !== "string") throw new Error("newAlternative");
        const d = await sbGqlMut(
          handle,
          `mutation($input: CreateKitAlternativeInput!) { session { createAlternative(input: $input) { id name } } }`,
          { input: { name: v.name, fromCheckpointId: v.fromCheckpoint ?? null } },
        );
        const id = str(((d["session"] as Record<string, unknown> | undefined)?.["createAlternative"] as Record<string, unknown> | undefined)?.["id"]);
        data = { newAlternative: { id: id ?? "" } };
        break;
      }
      case "executeSessionCommands": {
        if (value == null || typeof value !== "object" || Array.isArray(value)) throw new Error("executeSessionCommands value");
        data = await storybookExecuteSessionCommands(handle, value as Record<string, unknown>);
        break;
      }
      case "executeKitCheckpointCommands": {
        const v = value as { id?: string; commands?: unknown[] } | null;
        const cid = str(v?.id);
        const cmds = v?.commands;
        if (!cid || !Array.isArray(cmds) || !cmds.length) throw new Error("executeKitCheckpointCommands");
        const c0 = cmds[0] as Record<string, unknown>;
        const ck = Object.keys(c0);
        if (ck[0] === "markAsRelease") {
          await sbGqlMut(handle, `mutation($cid: KitCheckpointIdIn!) { session { checkpoint(id: $cid) { markRelease } } }`, { cid: { id: cid } });
          data = { executeKitCheckpointCommands: { results: [{ markAsRelease: { ok: true } }] } };
          break;
        }
        throw new Error("executeKitCheckpointCommands: unhandled");
      }
      case "executeKitAlternativeCommands": {
        const v = value as { id?: string; commands?: unknown[] } | null;
        const aid = str(v?.id);
        const cmds = v?.commands;
        if (!aid || !Array.isArray(cmds) || !cmds.length) throw new Error("executeKitAlternativeCommands");
        const c0 = cmds[0] as Record<string, unknown>;
        const ck = Object.keys(c0);
        if (ck[0] === "unifyKitCheckpointsToSingleKitCheckpoint") {
          const msg = str((c0["unifyKitCheckpointsToSingleKitCheckpoint"] as Record<string, unknown> | null)?.["message"]) ?? "unify";
          const d = await sbGqlMut(handle, `mutation($aid: KitAlternativeIdIn!, $msg: String!) { session { alternative(id: $aid) { unify(message: $msg) { id } } } }`, {
            aid: { id: aid },
            msg,
          });
          const sessionU = d["session"] as Record<string, unknown> | undefined;
          const altU = sessionU?.["alternative"] as Record<string, unknown> | undefined;
          const unify = altU?.["unify"] as Record<string, unknown> | undefined;
          const ncp = str(unify?.["id"]);
          data = { executeKitAlternativeCommands: { results: [{ unifyKitCheckpointsToSingleKitCheckpoint: { newCheckpointId: ncp ?? "" } }] } };
          break;
        }
        throw new Error("executeKitAlternativeCommands: unhandled");
      }
      case "batch": {
        const cmds = (value as { commands?: unknown[] } | null)?.commands;
        if (!Array.isArray(cmds)) throw new Error("batch.commands");
        const out: unknown[] = [];
        for (const item of cmds) {
          const r = await storybookKitGraphqlExecuteStoreCommand(handle, item);
          if (!r.ok) throw new Error(r.error.message);
          out.push((r as { ok: true; result: unknown }).result);
        }
        data = { results: out };
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
