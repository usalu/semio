//#region 🔖composeWasm
import initCompose, { boot, KitStoreHandle } from "@semio-tech/compose-rs-wasm";

/** @emoji 🧪 Empty WASM store URI — must match {@link RS_WASM_EMPTY_STORE_URI} in `@semio-tech/compose-js`. */
export const RS_WASM_EMPTY_STORE_URI = "dev://empty" as const;

// Bundle `compose.js` in Storybook, the default `new URL("compose_bg.wasm", import.meta.url)` is often wrong;
// point at the pkg explicitly so `fetch` loads the file.
// Path: `.storybook/compose/algorithm/kit-store/` → three parents = repo root → `compose/client/lib/rs/pkg`.
const composeWasmUrl = new URL("../../../compose/client/lib/rs/pkg/compose_bg.wasm", import.meta.url);

let initPromise: Promise<void> | null = null;

/** Single-flight wasm init; safe to call from multiple components. */
export function ensureComposeWasm(): Promise<void> {
  if (typeof window === "undefined") {
    return Promise.resolve();
  }
  if (!initPromise) {
    initPromise = (async () => {
      try {
        await initCompose(composeWasmUrl);
        boot();
      } catch (e) {
        initPromise = null;
        throw e;
      }
    })();
  }
  return initPromise;
}

export { boot, initCompose, KitStoreHandle };

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
  await ensureComposeWasm();
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
//#endregion 🔖composeWasm

//#region 🔖commandSchema
/** @emoji 📇 Root `ReadKitCommand` variant keys (camelCase), aligned with `compose/rs` `read::ReadKitCommand`. */
export const ALL_READ_KIT_COMMAND_KEYS: readonly string[] = [
  "readKitFullCommand",
  "readKitShallowCommand",
  "readKitMetadataCommand",
  "readKitIdCommand",
  "readKitNameCommand",
  "readKitDescriptionCommand",
  "readKitIconCommand",
  "readKitImageCommand",
  "readKitPreviewCommand",
  "readKitRemoteCommand",
  "readKitHomepageCommand",
  "readKitLicenseCommand",
  "readKitUriCommand",
  "readKitCreatedCommand",
  "readKitUpdatedCommand",
  "readKitTypesFullCommand",
  "readKitTypesShallowCommand",
  "readKitTypeIdsCommand",
  "readKitTypesMetadataCommand",
  "readKitDesignsFullCommand",
  "readKitDesignsShallowCommand",
  "readKitDesignIdsCommand",
  "readKitDesignsMetadataCommand",
  "readKitFilesFullCommand",
  "readKitFilesShallowCommand",
  "readKitFoldersFullCommand",
  "readKitFoldersShallowCommand",
  "readKitLocationsFullCommand",
  "readKitLocationsShallowCommand",
  "readKitFamiliesFullCommand",
  "readKitFamiliesShallowCommand",
  "readKitPortsFullCommand",
  "readKitAuthorsFullCommand",
  "readKitAuthorsShallowCommand",
  "readKitConceptsFullCommand",
  "readKitConceptsShallowCommand",
  "readKitTagsFullCommand",
  "readKitTagsShallowCommand",
  "readKitQualitiesFullCommand",
  "readKitQualitiesShallowCommand",
  "readKitPropsFullCommand",
  "readKitPropsShallowCommand",
  "readKitAttributesFullCommand",
  "readKitAttributesShallowCommand",
  "readKitTypeCommands",
  "readKitDesignCommands",
  "readKitFileCommands",
  "readKitFolderCommands",
  "readKitLocationCommands",
  "readKitFamilyCommands",
  "readKitPortCommands",
  "readKitAuthorCommands",
  "readKitConceptCommands",
  "readKitTagCommands",
  "readKitQualityCommands",
  "readKitPropCommands",
  "readKitAttributeCommands",
];

/** One JSON object for a single `ChangeKitCommand` (serde **externally tagged**, camelCase variant keys). */
export interface ChangeKitPreset {
  readonly id: string;
  readonly label: string;
  /** Full `ChangeKitCommand` value as JSON object (not wrapped in array). */
  readonly json: string;
}

/** One `kitGraphqlRun` body: `{ query, variables?, operationName? }` (JSON in the textarea). */
export interface ReadKitPreset {
  readonly id: string;
  readonly label: string;
  readonly json: string;
}

function j(obj: unknown): string {
  return JSON.stringify(obj, null, 2);
}

/** Presets for common **root** `ChangeKitCommand` variants. Replace PLACEHOLDER_* before run. */
export const CHANGE_KIT_PRESETS: readonly ChangeKitPreset[] = [
  { id: "ck-name", label: "Kit: name", json: j({ name: { name: "Kit name (story)" } }) },
  { id: "ck-desc", label: "Kit: description", json: j({ description: { description: "story description" } }) },
  { id: "ck-icon", label: "Kit: icon", json: j({ icon: { icon: "icon-url" } }) },
  { id: "ck-version", label: "Kit: version", json: j({ version: { version: "0.0.0-story" } }) },
  {
    id: "ck-replaceKit",
    label: "Kit: replaceKitFromFull (placeholder — replace `dto` with a full `KitFullDto` JSON)",
    json: j({
      replaceKitFromFull: {
        dto: { id: "PLACEHOLDER_KIT_ID", name: "replaced" },
      },
    }),
  },
  {
    id: "ck-changeType-name",
    label: "Nested: changeTypeCommands (name)",
    json: j({
      changeTypeCommands: {
        typeId: { id: "PLACEHOLDER_TYPE_ID" },
        commands: [{ name: { name: "Renamed type" } }],
      },
    }),
  },
  {
    id: "ck-changeDesign-name",
    label: "Nested: changeDesignCommands (name)",
    json: j({
      changeDesignCommands: {
        designId: { id: "PLACEHOLDER_DESIGN_ID" },
        commands: [{ name: { name: "Renamed design" } }],
      },
    }),
  },
  {
    id: "ck-changeFile-url",
    label: "Nested: changeFileCommands (url)",
    json: j({
      changeFileCommands: {
        fileId: { id: "PLACEHOLDER_FILE_ID" },
        commands: [{ url: { url: "https://example.com/file" } }],
      },
    }),
  },
  {
    id: "ck-changeFolder-path",
    label: "Nested: changeFolderCommands (path)",
    json: j({
      changeFolderCommands: {
        folderId: { id: "PLACEHOLDER_FOLDER_ID" },
        commands: [{ path: { path: "/story/folder" } }],
      },
    }),
  },
];

/**
 * Flat index of **all** root `ChangeKitCommand` variant keys from
 * [compose/rs/lib.rs](compose/rs/lib.rs) `ChangeKitCommand` (serde camelCase field names).
 * Use with raw JSON editor; invalid / unwired variants surface as `InvalidOperation` in the UI.
 */
export const ALL_CHANGE_KIT_ROOT_KEYS = [
  "replaceKitFromFull",
  "name",
  "description",
  "icon",
  "image",
  "preview",
  "version",
  "remote",
  "homepage",
  "license",
  "uri",
  "created",
  "updated",
  "addType",
  "removeType",
  "addDesign",
  "removeDesign",
  "addFile",
  "removeFile",
  "addFolder",
  "removeFolder",
  "addAuthor",
  "removeAuthor",
  "addConcept",
  "removeConcept",
  "addTag",
  "removeTag",
  "addQuality",
  "removeQuality",
  "addKitProp",
  "removeKitProp",
  "addKitAttribute",
  "removeKitAttribute",
  "changeFileCommands",
  "changeFolderCommands",
  "changeAuthorCommands",
  "changeConceptCommands",
  "changeTagCommands",
  "changeKitQualityCommands",
  "changeTypeCommands",
  "changeDesignCommands",
] as const;

export const CHANGE_TYPE_COMMAND_KEYS = [
  "name",
  "description",
  "icon",
  "image",
  "variant",
  "stock",
  "typeVirtual",
  "unit",
  "location",
  "created",
  "updated",
  "addPort",
  "removePort",
  "changePortCommands",
  "addConnector",
  "removeConnector",
  "changeConnectorCommands",
  "addRepresentation",
  "removeRepresentation",
  "changeRepresentationCommands",
  "addTypeAuthor",
  "removeTypeAuthor",
  "addTypeConcept",
  "removeTypeConcept",
  "addTypeTag",
  "removeTypeTag",
  "addTypeQuality",
  "removeTypeQuality",
  "addTypeProp",
  "removeTypeProp",
  "addTypeAttribute",
  "removeTypeAttribute",
] as const;

export const READ_KIT_PRESETS: readonly ReadKitPreset[] = [
  {
    id: "rk-full",
    label: "GraphQL: session.wip.theKit name + description + metadata",
    json: j({
      query: `query { session { wip { theKit { name description metadata { id name description icon image preview remote homepage license uri created updated version } } } } }`,
    }),
  },
  { id: "rk-name", label: "GraphQL: session.wip.theKit { name }", json: j({ query: `query { session { wip { theKit { name } } } }` }) },
  {
    id: "rk-types",
    label: "GraphQL: session.wip.theKit shallow.types",
    json: j({ query: `query { session { wip { theKit { shallow { types { id name } } } } } }` }),
  },
  {
    id: "rk-designs",
    label: "GraphQL: session.wip.theKit shallow.designs",
    json: j({ query: `query { session { wip { theKit { shallow { designs { id name } } } } } }` }),
  },
  {
    id: "rk-desc",
    label: "GraphQL: session.wip.theKit { description }",
    json: j({ query: `query { session { wip { theKit { description } } } }` }),
  },
  {
    id: "rk-type-nested",
    label: "GraphQL: session.wip.theKit type(id) { name }",
    json: j({
      query: `query($id: String!) { session { wip { theKit { type(id: $id) { name } } } } }`,
      variables: { id: "PLACEHOLDER_TYPE_ID" },
    }),
  },
  {
    id: "rk-computed",
    label: "GraphQL: session.wip.theKit design(id) { flattenMap }",
    json: j({
      query: `query($id: String!) { session { wip { theKit { design(id: $id) { flattenMap } } } } }`,
      variables: { id: "PLACEHOLDER_DESIGN_ID" },
    }),
  },
];

/** Rows for the on-screen coverage checklist (mirrors `ALL_CHANGE_KIT_ROOT_KEYS` + nested groups). */
export const KIT_STORE_COVERAGE_ROWS: readonly { group: string; key: string }[] = [
  { group: "ChangeKit (root)", key: "replaceKitFromFull + all ALL_CHANGE_KIT_ROOT_KEYS" },
  { group: "ChangeType", key: "see CHANGE_TYPE_COMMAND_KEYS" },
  { group: "ReadKit", key: "see ALL_READ_KIT_COMMAND_KEYS (generated from compose/rs/read_module.rs)" },
];
//#endregion 🔖commandSchema

//#region 🔖useKitStore
import * as React from "react";



// #region 🎨 RJV Theme
// Tracks the `dark` class on <html> and maps it to a base-16 theme name
// consumed by `@microlink/react-json-view` so JSON viewers track Storybook theme toggles.
export type RjvThemeName = "rjv-default" | "monokai";

export function useRjvTheme(): RjvThemeName {
  const [isDark, setIsDark] = React.useState<boolean>(() => {
    if (typeof document === "undefined") return false;
    return document.documentElement.classList.contains("dark");
  });
  React.useEffect(() => {
    if (typeof document === "undefined") return;
    const el = document.documentElement;
    const update = () => setIsDark(el.classList.contains("dark"));
    update();
    const observer = new MutationObserver(update);
    observer.observe(el, { attributes: true, attributeFilter: ["class"] });
    return () => observer.disconnect();
  }, []);
  return isDark ? "monokai" : "rjv-default";
}
// #endregion 🎨 RJV Theme

let evSeq = 0;
function nextEvId() {
  evSeq += 1;
  return `ev-${evSeq}`;
}

export interface LastChangeResult {
  readonly forward: unknown;
  readonly result: unknown;
  readonly error?: string;
  readonly mode: "changeKit" | "readKit" | "execute" | "log";
}

export function useKitStore(seedKit: unknown) {
  const [ready, setReady] = React.useState(false);
  const [initErr, setInitErr] = React.useState<string | null>(null);
  const [handle, setHandle] = React.useState<KitStoreHandle | null>(null);
  const [events, setEvents] = React.useState<readonly LoggedEvent[]>([]);
  const [filter, setFilter] = React.useState("");
  const [last, setLast] = React.useState<LastChangeResult | null>(null);
  const [matAt, setMatAt] = React.useState("");

  const [sessionId, setSessionId] = React.useState("");
  const [draftId, setDraftId] = React.useState("");
  const [txId, setTxId] = React.useState("");
  const [cpId, setCpId] = React.useState("");
  const [altId, setAltId] = React.useState("");
  const [msg, setMsg] = React.useState("checkpoint (story)");

  const [cmdMode, setCmdMode] = React.useState<"changeKit" | "readKit" | "execute">("changeKit");
  const [changeJson, setChangeJson] = React.useState(
    `{\n  "name": { "name": "Kit (story edit)" }\n}`,
  );
  const [readJson, setReadJson] = React.useState(
    `{\n  "query": "query { session { wip { theKit { name } } } }"\n}`,
  );
  const [executeJson, setExecuteJson] = React.useState(`{ "newSession": null }`);

  const pushEvent = React.useCallback((payload: unknown) => {
    setEvents((prev) => [...prev, { id: nextEvId(), t: Date.now(), payload }]);
  }, []);

  const log = React.useCallback(
    (line: string) => {
      pushEvent({ log: line });
      setLast({ forward: null, result: { log: line }, mode: "log" });
    },
    [pushEvent],
  );

  React.useEffect(() => {
    let cancelled = false;
    setInitErr(null);
    setReady(false);
    void (async () => {
      try {
        const h = await createStorybookKitGraphqlHandle(seedKit);
        if (cancelled) {
          h.free();
          return;
        }
        setHandle(h);
        setReady(true);
      } catch (e) {
        if (!cancelled) setInitErr(e instanceof Error ? e.message : String(e));
      }
    })();
    return () => {
      cancelled = true;
      setHandle((prev: KitStoreHandle | null) => {
        try {
          prev?.free();
        } catch {
          /* ignore */
        }
        return null;
      });
    };
  }, [seedKit]);

  React.useEffect(() => {
    if (!handle) return;
    const cb = (payload: unknown) => {
      setEvents((prev) => [...prev, { id: nextEvId(), t: Date.now(), payload }]);
    };
    handle.subscribe(cb);
  }, [handle]);

  const onCommandRun = React.useCallback(
    (o: { mode: "changeKit" | "readKit" | "execute"; forward: unknown; result?: unknown; error?: string; log: string }) => {
      pushEvent({ command: o.mode, log: o.log, forward: o.forward, result: o.result, error: o.error });
      setLast(
        o.error
          ? { forward: o.forward, result: o.result, error: o.error, mode: o.mode }
          : { forward: o.forward, result: o.result, mode: o.mode },
      );
    },
    [pushEvent],
  );

  return {
    ready,
    initErr,
    handle,
    events,
    filter,
    setFilter,
    onClear: () => setEvents([]),
    last,
    matAt,
    setMatAt,
    sessionId,
    setSessionId,
    draftId,
    setDraftId,
    txId,
    setTxId,
    cpId,
    setCpId,
    altId,
    setAltId,
    msg,
    setMsg,
    cmdMode,
    setCmdMode,
    changeJson,
    setChangeJson,
    readJson,
    setReadJson,
    executeJson,
    setExecuteJson,
    log,
    onCommandRun,
  };
}
//#endregion 🔖useKitStore

//#region 🔖HistoryControls
import * as React from "react";

import {
  storybookKitGraphqlExecuteStoreCommand,
  storybookKitGraphqlRun,
  type StorybookKitGraphqlHandle,
} from "./composeWasm";

type IdCallback = (s: string) => void;

export interface VcsIdCallbacks {
  readonly onSessionId?: IdCallback;
  readonly onDraftId?: IdCallback;
  readonly onTxId?: IdCallback;
  readonly onCpId?: IdCallback;
  readonly onAltId?: IdCallback;
}

function strField(obj: unknown, key: string): string | undefined {
  if (obj == null || typeof obj !== "object") return;
  const v = (obj as Record<string, unknown>)[key];
  return typeof v === "string" && v.length > 0 ? v : undefined;
}

/** Extract ids from `KitStoreCommand` result objects (`#[serde(rename_all = "camelCase")]` on Rust enums). */
export function applyKitStoreCommandResultIds(r: unknown, on: VcsIdCallbacks): void {
  if (r == null || typeof r !== "object") return;
  const o = r as Record<string, unknown>;

  const idOf = (x: unknown): string | undefined =>
    x && typeof x === "object" && "id" in x && typeof (x as { id: unknown }).id === "string" ? (x as { id: string }).id : undefined;

  const s = idOf(o.newSession);
  if (s) on.onSessionId?.(s);
  const a = idOf(o.newAlternative);
  if (a) on.onAltId?.(a);

  const sess = o.executeSessionCommands as { results?: unknown[] } | undefined;
  if (sess?.results) for (const item of sess.results) walkSession(item, on);

  const alt = o.executeKitAlternativeCommands as { results?: unknown[] } | undefined;
  if (alt?.results) for (const item of alt.results) walkAlternative(item, on);
}

function walkSession(item: unknown, on: VcsIdCallbacks): void {
  if (item == null || typeof item !== "object") return;
  const it = item as Record<string, unknown>;
  const draftId = strField(it.newDraft, "draftId");
  if (draftId) on.onDraftId?.(draftId);

  const ekd = it.executeKitDraftCommands as { results?: unknown[] } | undefined;
  if (ekd?.results) for (const d of ekd.results) walkDraft(d, on);
}

function walkDraft(item: unknown, on: VcsIdCallbacks): void {
  if (item == null || typeof item !== "object") return;
  const it = item as Record<string, unknown>;
  const st = it.startTransaction;
  const tx = strField(st, "transactionId");
  if (tx) on.onTxId?.(tx);
  const fin = it.finalizeToKitCheckpoint;
  const cp = strField(fin, "checkpointId");
  if (cp) on.onCpId?.(cp);
  void it.executeTransactionCommands;
}

function walkAlternative(item: unknown, on: VcsIdCallbacks): void {
  if (item == null || typeof item !== "object") return;
  const it = item as Record<string, unknown>;
  const u = it.unifyKitCheckpointsToSingleKitCheckpoint;
  const ncp = strField(u, "newCheckpointId");
  if (ncp) on.onCpId?.(ncp);
}

/**
 * `SessionCommand::newDraft` / `is_valid_draft_base`: on the main line, once `theKitHead` exists, use that
 * checkpoint (or a chosen cp in the `cp` field). On an alternative, both `alternativeId` and the tip
 * `checkpointId` are required. `(null,null)` is only valid when the kit has no head yet.
 */
function newDraftPayload(cpId: string, altId: string, theKitHead: string | null): { checkpointId: string | null; alternativeId: string | null } | null {
  const alt = altId.trim() || null;
  const cp = cpId.trim() || null;
  if (alt) {
    if (!cp) return null;
    return { checkpointId: cp, alternativeId: alt };
  }
  if (cp) return { checkpointId: cp, alternativeId: null };
  if (!theKitHead) return { checkpointId: null, alternativeId: null };
  return { checkpointId: theKitHead, alternativeId: null };
}

export const HistoryControls: React.FC<{
  handle: KitStoreHandle | null;
  /** Shown in this pane when `create()` or WASM init failed (in addition to Entity pane). */
  initErr: string | null;
  onLog: (msg: string) => void;
  sessionId: string;
  onSessionId: (s: string) => void;
  draftId: string;
  onDraftId: (s: string) => void;
  txId: string;
  onTxId: (s: string) => void;
  cpId: string;
  onCpId: (s: string) => void;
  altId: string;
  onAltId: (s: string) => void;
  msg: string;
  onMsg: (s: string) => void;
  /** Pushes checkpoint into Snapshot window `readAt` for read-only DTO (empty string = initial). */
  onInspectCheckpoint?: (checkpointId: string) => void;
}> = ({ handle, initErr, onLog, sessionId, onSessionId, onDraftId, onTxId, draftId, txId, cpId, onCpId, altId, onAltId, msg, onMsg, onInspectCheckpoint }) => {
  const gqlHandle = (): StorybookKitGraphqlHandle => {
    if (!handle) throw new Error("KitStore handle not ready");
    return {
      execute: (requestJson: string) => handle.execute(requestJson),
      subscribe: (requestJson: string, onEvent: (msg: string) => void) => handle.subscribe(requestJson, onEvent),
    };
  };

  const ex = (label: string, o: object) => {
    if (!handle) {
      onLog("VCS: KitStore handle not ready yet (WASM still loading or init failed — see Entity ids panel).");
      return;
    }
    void (async () => {
      try {
        const payload: object =
          "executeSessionCommands" in o && altId.trim()
            ? { executeSessionCommands: { ...(o as { executeSessionCommands: Record<string, unknown> }).executeSessionCommands, alternativeId: altId.trim() } }
            : o;
        const r = await storybookKitGraphqlExecuteStoreCommand(gqlHandle(), payload);
        onLog(`execute ${label} → ${JSON.stringify(r).slice(0, 12_000)}`);
        applyKitStoreCommandResultIds(r.ok === true ? r.result : null, {
          onSessionId,
          onDraftId,
          onTxId,
          onCpId,
          onAltId,
        });
      } catch (e) {
        onLog(`execute ${label} ERROR: ${e instanceof Error ? e.message : String(e)}`);
      }
    })();
  };

  const readGql = (label: string, body: { query: string; variables?: Record<string, unknown> }) => {
    if (!handle) {
      onLog("VCS: KitStore handle not ready yet (WASM still loading or init failed — see Entity ids panel).");
      return;
    }
    void (async () => {
      try {
        const r = await storybookKitGraphqlRun(gqlHandle(), body);
        onLog(`read ${label} → ${JSON.stringify(r).slice(0, 12_000)}`);
      } catch (e) {
        onLog(`read ${label} ERROR: ${e instanceof Error ? e.message : String(e)}`);
      }
    })();
  };

  const canVcs = Boolean(handle);

  return (
    <div className="text-foreground min-h-0 space-y-1.5 overflow-auto p-2 text-[10px]">
      {initErr ? (
        <div className="text-destructive wrap-break-word rounded border border-destructive/50 bg-destructive/5 p-1.5 text-[10px]">
          <span className="font-medium">WASM / KitStore failed: </span>
          {initErr}
        </div>
      ) : null}
      {!initErr && !canVcs ? (
        <div className="text-muted-foreground rounded border border-amber-600/50 bg-amber-50 p-1.5 text-[10px] dark:bg-amber-950/40">Loading WASM / KitStore… buttons stay disabled until ready.</div>
      ) : null}
      <div className="text-muted-foreground font-medium">VCS (KitStoreCommand)</div>
      <p className="text-muted-foreground m-0 leading-snug">
        Pick a checkpoint and optional alt in <span className="text-foreground font-medium">Kit tree</span> (or paste ids). <span className="text-foreground">New draft</span> uses
        <code className="bg-muted-foreground/10 rounded px-0.5">checkpoint</code> + <code className="bg-muted-foreground/10 rounded px-0.5">alt</code>; on the main line, cp defaults to
        theKit HEAD. Read-only at any cp: <span className="text-foreground">Preview @ cp</span> → open <span className="text-foreground">Snapshot / theKit</span> →
        <code className="bg-muted-foreground/10 rounded px-0.5">readAt</code>. To commit: use <span className="text-foreground">Close tx</span> first (no open tx), then{" "}
        <span className="text-foreground">Finalize → cp</span>.
      </p>
      <div className="grid grid-cols-2 gap-1">
        <B disabled={!canVcs} onClick={() => ex("newSession", { newSession: null })}>
          New session
        </B>
        <B disabled={!canVcs} onClick={() => ex("end", { endSession: { id: sessionId } })}>
          End session
        </B>
        <B disabled={!canVcs} onClick={() => readGql("kit name", { query: `query { session { wip { theKit { name } } } }` })}>
          Read kit name
        </B>
        <B
          disabled={!canVcs}
          onClick={() =>
            readGql("kit summary", {
              query: `query { session { wip { theKit { name description metadata { id name description icon image preview remote homepage license uri created updated version } } } } }`,
            })
          }
        >
          Read kit full
        </B>
        <B
          onClick={() => ex("newAltFromCp", { newAlternative: { fromCheckpoint: cpId.trim(), name: "alt (from cp)" } })}
          disabled={!canVcs || !cpId.trim()}
        >
          New alt (from cp)
        </B>
        <B onClick={() => ex("newAltRoot", { newAlternative: { name: "alt (initial, no cp)" } })} disabled={!canVcs}>
          New alt (no cp)
        </B>
        <B
          onClick={() => {
            if (!handle || !canVcs || !sessionId.trim()) return;
            const v = handle.vcsState() as { theKitHead?: string | null };
            const head = (v && typeof v.theKitHead === "string" ? v.theKitHead : null) as string | null;
            const base = newDraftPayload(cpId, altId, head);
            if (base == null) {
              onLog("newDraft: set checkpoint to the line tip (required when an alternative is set).");
              return;
            }
            ex("newDraft", { executeSessionCommands: { id: sessionId, commands: [{ newDraft: base }] } });
          }}
          disabled={!canVcs || !sessionId.trim()}
        >
          New draft (cp + alt)
        </B>
        <B
          onClick={() => {
            if (!handle || !canVcs) return;
            const v = handle.vcsState() as { theKitHead?: string | null };
            const h = v && typeof v.theKitHead === "string" ? v.theKitHead : null;
            if (h) onCpId(h);
            else onLog("No theKit head yet — leave checkpoint empty for first draft.");
          }}
          disabled={!canVcs}
        >
          Set cp = HEAD
        </B>
        <B
          onClick={() => {
            if (!canVcs || !onInspectCheckpoint) {
              onLog("Preview: connect onInspectCheckpoint (Storybook) or set checkpoint id.");
              return;
            }
            onInspectCheckpoint(cpId.trim());
            onLog(
              `Snapshot window: "readAt" → ${cpId.trim() ? `checkpoint ${cpId.trim()}` : "empty = initial"}. Open that tab and click refresh.`,
            );
          }}
          disabled={!canVcs}
        >
          Preview @ cp (read-only)
        </B>
        <B
          onClick={() =>
            ex("startTx", {
              executeSessionCommands: { id: sessionId, commands: [{ executeKitDraftCommands: { id: draftId, commands: [{ startTransaction: null }] } }] },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim()}
        >
          Start tx
        </B>
        <B
          onClick={() =>
            ex("finalizeTx", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [
                        { executeTransactionCommands: { id: txId, commands: [{ finalize: null }] } },
                      ],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Close tx (finalize)
        </B>
        <B
          onClick={() =>
            ex("abortTx", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [
                        { executeTransactionCommands: { id: txId, commands: [{ abort: null }] } },
                      ],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Abort tx (revert)
        </B>
        <B
          onClick={() =>
            ex("txUndo", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [{ executeTransactionCommands: { id: txId, commands: [{ undo: null }] } }],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Tx undo
        </B>
        <B
          onClick={() =>
            ex("txRedo", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [{ executeTransactionCommands: { id: txId, commands: [{ redo: null }] } }],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Tx redo
        </B>
        <B
          onClick={() =>
            ex("txUndoAll", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [{ executeTransactionCommands: { id: txId, commands: [{ undoAll: null }] } }],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Tx undo all
        </B>
        <B
          onClick={() =>
            ex("txRedoAll", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [{ executeTransactionCommands: { id: txId, commands: [{ redoAll: null }] } }],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Tx redo all
        </B>
        <B
          onClick={() =>
            ex("finalize", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [
                        {
                          finalizeToKitCheckpoint: { message: msg.trim() || "checkpoint" },
                        },
                      ],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim()}
        >
          Finalize → cp
        </B>
        <B
          onClick={() => ex("abortDraft", { executeSessionCommands: { id: sessionId, commands: [{ executeKitDraftCommands: { id: draftId, commands: [{ abort: null }] } }] } })}
          disabled={!canVcs || !sessionId.trim() || !draftId.trim()}
        >
          Discard draft
        </B>
        <B onClick={() => ex("markRel", { executeKitCheckpointCommands: { id: cpId, commands: [{ markAsRelease: null }] } })} disabled={!canVcs || !cpId.trim()}>
          Mark cp release
        </B>
        <B
          onClick={() =>
            ex("unifyAlt", {
              executeKitAlternativeCommands: { id: altId, commands: [{ unifyKitCheckpointsToSingleKitCheckpoint: { message: "unify story" } }] },
            })
          }
          disabled={!canVcs || !altId.trim()}
        >
          Unify alt checkpoints
        </B>
        <B
          onClick={() =>
            ex("draftUndo", { executeSessionCommands: { id: sessionId, commands: [{ executeKitDraftCommands: { id: draftId, commands: [{ undo: { count: 1 } }] } }] } })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim()}
        >
          Draft undo
        </B>
        <B
          onClick={() =>
            ex("draftRedo", { executeSessionCommands: { id: sessionId, commands: [{ executeKitDraftCommands: { id: draftId, commands: [{ redo: { count: 1 } }] } }] } })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim()}
        >
          Draft redo
        </B>
      </div>
      <label className="text-muted-foreground flex items-center gap-1">
        session
        <input className="bg-background flex-1 font-mono" value={sessionId} onChange={(e) => onSessionId(e.target.value)} />
      </label>
      <label className="text-muted-foreground flex items-center gap-1">
        draft
        <input className="bg-background flex-1 font-mono" value={draftId} onChange={(e) => onDraftId(e.target.value)} />
      </label>
      <label className="text-muted-foreground flex items-center gap-1">
        tx
        <input className="bg-background flex-1 font-mono" value={txId} onChange={(e) => onTxId(e.target.value)} />
      </label>
      <label className="text-muted-foreground flex items-center gap-1">
        checkpoint (from finalize / unify)
        <input className="bg-background flex-1 font-mono" value={cpId} onChange={(e) => onCpId(e.target.value)} />
      </label>
      <label className="text-muted-foreground flex items-center gap-1">
        alt
        <input className="bg-background flex-1 font-mono" value={altId} onChange={(e) => onAltId(e.target.value)} />
      </label>
      <label className="text-muted-foreground flex flex-col gap-0.5">
        <span>Message stored on the new checkpoint (Finalize → cp)</span>
        <input
          className="bg-background w-full"
          value={msg}
          placeholder="e.g. release-42 — required string on the command"
          onChange={(e) => onMsg(e.target.value)}
        />
      </label>
    </div>
  );
};

const B: React.FC<{ onClick: () => void; disabled?: boolean; children: React.ReactNode }> = ({ onClick, disabled, children }) => (
  <button type="button" disabled={disabled} className="rounded border border-zinc-300 px-1 py-0.5 text-left text-[10px] disabled:opacity-50 dark:border-zinc-600" onClick={onClick}>
    {children}
  </button>
);

// #region 🌳KitTreeGraph
// 🌳 GitKraken-inspired visualisation of the complete kit history: root (initial kit),
// checkpoints (chronological column, latest top — uuidv7 is time-sortable), alternatives
// (left lane, hover highlights the full line), drafts (bubbles pinned to their parent
// checkpoint showing session + transaction state), release badges.

// #region 📦VcsState shape
interface VcsCheckpointDto {
  readonly id: string;
  readonly parent: string | null;
  readonly message: string | null;
  readonly time: string | null;
  readonly authors: readonly string[];
  readonly hash: string;
  readonly isRelease: boolean;
  readonly changeCount: number;
}

interface VcsAlternativeDto {
  readonly id: string;
  readonly name: string;
  readonly root: string;
  readonly checkpoints: readonly string[];
}

interface VcsDraftDto {
  readonly id: string;
  readonly parentCheckpoint: string | null;
  readonly targetAlternative: string | null;
  readonly finalizedTransactionCount: number;
  readonly redoTransactionCount: number;
  readonly openTransactionId: string | null;
  readonly canUndo: boolean;
  readonly canRedo: boolean;
}

interface VcsSessionDto {
  readonly id: string;
  readonly drafts: readonly VcsDraftDto[];
}

interface VcsRootDto {
  readonly id: string;
  readonly name: string;
}

interface VcsStateDto {
  readonly theKitHead: string | null;
  readonly root: VcsRootDto;
  readonly checkpoints: readonly VcsCheckpointDto[];
  readonly alternatives: readonly VcsAlternativeDto[];
  readonly sessions: readonly VcsSessionDto[];
  readonly theKitLine: readonly string[];
}
// #endregion 📦VcsState shape

// #region 🎨Lane palette
const KIT_TREE_LANE_COLORS = [
  "#0ea5e9", // sky-500 → the kit
  "#f97316", // orange-500
  "#a855f7", // purple-500
  "#22c55e", // green-500
  "#ef4444", // red-500
  "#eab308", // yellow-500
  "#14b8a6", // teal-500
  "#ec4899", // pink-500
] as const;

function kitTreeLaneColor(index: number): string {
  if (index < 0) return "#71717a";
  return KIT_TREE_LANE_COLORS[index % KIT_TREE_LANE_COLORS.length];
}

function kitTreeShortId(id: string, len = 8): string {
  return id.length <= len ? id : id.slice(0, len);
}
// #endregion 🎨Lane palette

// #region 🔎Selection
export interface KitTreeSelection {
  readonly onCheckpointSelect: (id: string) => void;
  readonly onAlternativeSelect: (id: string) => void;
  readonly onSessionSelect: (id: string) => void;
  readonly onDraftSelect: (id: string) => void;
}
// #endregion 🔎Selection

// #region 🧮Layout derivation
interface KitTreeCheckpointRowModel {
  readonly checkpoint: VcsCheckpointDto;
  readonly laneIndex: number;
  readonly onTheKit: boolean;
  readonly altIds: readonly string[];
  readonly drafts: readonly { readonly session: VcsSessionDto; readonly draft: VcsDraftDto }[];
}

function buildKitTreeRows(state: VcsStateDto): readonly KitTreeCheckpointRowModel[] {
  const mainLine = new Set(state.theKitLine);
  const altMembership = new Map<string, string[]>();
  state.alternatives.forEach((alt) => {
    alt.checkpoints.forEach((cp) => {
      const bucket = altMembership.get(cp) ?? [];
      bucket.push(alt.id);
      altMembership.set(cp, bucket);
    });
  });
  const altLane = new Map<string, number>();
  state.alternatives.forEach((alt, i) => altLane.set(alt.id, i + 1));

  const draftsByCp = new Map<string, { session: VcsSessionDto; draft: VcsDraftDto }[]>();
  state.sessions.forEach((session) => {
    session.drafts.forEach((draft) => {
      const key = draft.parentCheckpoint ?? "__root__";
      const bucket = draftsByCp.get(key) ?? [];
      bucket.push({ session, draft });
      draftsByCp.set(key, bucket);
    });
  });

  // uuidv7 ids are chronologically sortable; latest first.
  const sorted = [...state.checkpoints].sort((a, b) => (a.id < b.id ? 1 : a.id > b.id ? -1 : 0));
  return sorted.map((cp) => {
    const altIds = altMembership.get(cp.id) ?? [];
    const onTheKit = mainLine.has(cp.id);
    const laneIndex = onTheKit ? 0 : altIds.length > 0 ? altLane.get(altIds[0]) ?? -1 : -1;
    return {
      checkpoint: cp,
      laneIndex,
      onTheKit,
      altIds,
      drafts: draftsByCp.get(cp.id) ?? [],
    };
  });
}
// #endregion 🧮Layout derivation

// #region 🖼️KitTreeGraph component
export interface KitTreeGraphProps {
  readonly handle: KitStoreHandle | null;
  readonly selection: KitTreeSelection;
  readonly selectedCheckpointId?: string;
  readonly selectedAlternativeId?: string;
  readonly selectedSessionId?: string;
  readonly selectedDraftId?: string;
  /** Increment to force a VCS re-read (e.g. after commands finish). */
  readonly refreshToken?: number;
}

export const KitTreeGraph: React.FC<KitTreeGraphProps> = ({
  handle,
  selection,
  selectedCheckpointId,
  selectedAlternativeId,
  selectedSessionId,
  selectedDraftId,
  refreshToken,
}) => {
  const [state, setState] = React.useState<VcsStateDto | null>(null);
  const [errorText, setErrorText] = React.useState<string | null>(null);
  const [hoveredAltId, setHoveredAltId] = React.useState<string | null>(null);

  const refresh = React.useCallback(() => {
    if (!handle) {
      setState(null);
      setErrorText(null);
      return;
    }
    try {
      const raw = handle.vcsState() as VcsStateDto;
      setState(raw);
      setErrorText(null);
    } catch (e) {
      setErrorText(e instanceof Error ? e.message : String(e));
      setState(null);
    }
  }, [handle]);

  React.useEffect(() => {
    refresh();
  }, [refresh, refreshToken]);

  React.useEffect(() => {
    if (!handle) return;
    const cb = () => refresh();
    handle.subscribe(cb);
  }, [handle, refresh]);

  const rows = React.useMemo(() => (state ? buildKitTreeRows(state) : []), [state]);
  const alternatives = state?.alternatives ?? [];
  const highlightedCheckpoints = React.useMemo(() => {
    if (!hoveredAltId || !state) return new Set<string>();
    const alt = state.alternatives.find((a) => a.id === hoveredAltId);
    return new Set<string>(alt?.checkpoints ?? []);
  }, [hoveredAltId, state]);

  if (!handle) {
    return <div className="text-muted-foreground p-2 text-xs">KitStore not ready — waiting for WASM.</div>;
  }

  return (
    <div className="text-foreground flex h-full min-h-0 flex-col text-[10px]">
      <div className="flex items-center justify-between border-b border-zinc-200 p-1.5 dark:border-zinc-800">
        <div className="flex items-center gap-2">
          <span className="font-semibold">Kit tree</span>
          {state ? (
            <span className="text-muted-foreground">
              root: <span className="font-mono">{kitTreeShortId(state.root.id)}</span> — {state.root.name || "(unnamed)"}
            </span>
          ) : null}
        </div>
        <button type="button" className="rounded border border-zinc-300 px-1.5 py-0.5 text-[10px] dark:border-zinc-600" onClick={refresh}>
          refresh
        </button>
      </div>
      {errorText ? <div className="text-destructive border-b border-destructive/50 bg-destructive/5 p-1.5 text-[10px] wrap-break-word">vcsState failed: {errorText}</div> : null}
      <div className="flex min-h-0 flex-1">
        <KitTreeAlternatives
          alternatives={alternatives}
          onHover={setHoveredAltId}
          onSelect={selection.onAlternativeSelect}
          selectedId={selectedAlternativeId}
          theKitHead={state?.theKitHead ?? null}
          theKitLineLength={state?.theKitLine.length ?? 0}
        />
        <KitTreeCheckpoints
          rows={rows}
          theKitHead={state?.theKitHead ?? null}
          rootId={state?.root.id ?? ""}
          rootName={state?.root.name ?? ""}
          highlightedCheckpoints={highlightedCheckpoints}
          selectedCheckpointId={selectedCheckpointId}
          selectedDraftId={selectedDraftId}
          selectedSessionId={selectedSessionId}
          onCheckpointSelect={selection.onCheckpointSelect}
          onDraftSelect={selection.onDraftSelect}
          onSessionSelect={selection.onSessionSelect}
        />
      </div>
      <KitTreeOrphanDrafts
        sessions={state?.sessions ?? []}
        onSessionSelect={selection.onSessionSelect}
        onDraftSelect={selection.onDraftSelect}
        selectedSessionId={selectedSessionId}
        selectedDraftId={selectedDraftId}
      />
    </div>
  );
};
// #endregion 🖼️KitTreeGraph component

// #region 🧭KitTreeAlternatives panel
const KitTreeAlternatives: React.FC<{
  readonly alternatives: readonly VcsAlternativeDto[];
  readonly onHover: (id: string | null) => void;
  readonly onSelect: (id: string) => void;
  readonly selectedId?: string;
  readonly theKitHead: string | null;
  readonly theKitLineLength: number;
}> = ({ alternatives, onHover, onSelect, selectedId, theKitHead, theKitLineLength }) => (
  <aside className="flex w-36 min-w-0 flex-col gap-0.5 overflow-auto border-r border-zinc-200 bg-zinc-50 p-1 dark:border-zinc-800 dark:bg-zinc-950">
    <div className="text-muted-foreground px-1 pt-0.5 pb-1 font-medium uppercase tracking-wide">Alternatives</div>
    <div className="flex items-center gap-1 rounded border px-1 py-1" style={{ borderColor: kitTreeLaneColor(0), background: `${kitTreeLaneColor(0)}14` }}>
      <span className="inline-block h-2 w-2 rounded-full" style={{ background: kitTreeLaneColor(0) }} />
      <div className="min-w-0 flex-1">
        <div className="truncate font-medium">the kit</div>
        <div className="text-muted-foreground truncate">
          {theKitLineLength} cp · head {theKitHead ? kitTreeShortId(theKitHead, 6) : "—"}
        </div>
      </div>
    </div>
    {alternatives.length === 0 ? (
      <div className="text-muted-foreground px-1 py-1 italic">no alternatives yet</div>
    ) : (
      alternatives.map((alt, i) => {
        const color = kitTreeLaneColor(i + 1);
        const isSelected = selectedId === alt.id;
        return (
          <button
            key={alt.id}
            type="button"
            className={"flex items-center gap-1 rounded border px-1 py-1 text-left text-[10px] " + (isSelected ? "ring-1 ring-offset-1 dark:ring-offset-zinc-950" : "")}
            style={{ borderColor: color, background: isSelected ? `${color}33` : `${color}14` }}
            onMouseEnter={() => onHover(alt.id)}
            onMouseLeave={() => onHover(null)}
            onClick={() => onSelect(alt.id)}
            title={alt.id}
          >
            <span className="inline-block h-2 w-2 rounded-full" style={{ background: color }} />
            <div className="min-w-0 flex-1">
              <div className="truncate font-medium">{alt.name || "(unnamed)"}</div>
              <div className="text-muted-foreground truncate">
                {alt.checkpoints.length} cp · root{" "}
                {alt.root && alt.root.length > 0 ? kitTreeShortId(alt.root, 6) : "initial"}
              </div>
            </div>
          </button>
        );
      })
    )}
  </aside>
);
// #endregion 🧭KitTreeAlternatives panel

// #region 📜KitTreeCheckpoints column
const KitTreeCheckpoints: React.FC<{
  readonly rows: readonly KitTreeCheckpointRowModel[];
  readonly theKitHead: string | null;
  readonly rootId: string;
  readonly rootName: string;
  readonly highlightedCheckpoints: ReadonlySet<string>;
  readonly selectedCheckpointId?: string;
  readonly selectedDraftId?: string;
  readonly selectedSessionId?: string;
  readonly onCheckpointSelect: (id: string) => void;
  readonly onDraftSelect: (id: string) => void;
  readonly onSessionSelect: (id: string) => void;
}> = ({ rows, theKitHead, rootId, rootName, highlightedCheckpoints, selectedCheckpointId, selectedDraftId, selectedSessionId, onCheckpointSelect, onDraftSelect, onSessionSelect }) => (
  <div className="flex min-w-0 flex-1 flex-col overflow-auto">
    {rows.length === 0 ? (
      <div className="text-muted-foreground p-2 italic">no checkpoints yet — finalize a draft to create one</div>
    ) : (
      rows.map((row) => (
        <KitTreeCheckpointRow
          key={row.checkpoint.id}
          row={row}
          isHead={theKitHead === row.checkpoint.id}
          isHighlighted={highlightedCheckpoints.has(row.checkpoint.id)}
          isSelected={selectedCheckpointId === row.checkpoint.id}
          selectedDraftId={selectedDraftId}
          selectedSessionId={selectedSessionId}
          onCheckpointSelect={onCheckpointSelect}
          onDraftSelect={onDraftSelect}
          onSessionSelect={onSessionSelect}
        />
      ))
    )}
    <div className="border-t border-dashed border-zinc-300 p-1.5 dark:border-zinc-700">
      <div className="text-muted-foreground">
        <span className="font-semibold text-foreground">root (initial kit)</span> · <span className="font-mono">{kitTreeShortId(rootId)}</span> — {rootName || "(unnamed)"}
      </div>
    </div>
  </div>
);

const KitTreeCheckpointRow: React.FC<{
  readonly row: KitTreeCheckpointRowModel;
  readonly isHead: boolean;
  readonly isHighlighted: boolean;
  readonly isSelected: boolean;
  readonly selectedDraftId?: string;
  readonly selectedSessionId?: string;
  readonly onCheckpointSelect: (id: string) => void;
  readonly onDraftSelect: (id: string) => void;
  readonly onSessionSelect: (id: string) => void;
}> = ({ row, isHead, isHighlighted, isSelected, selectedDraftId, selectedSessionId, onCheckpointSelect, onDraftSelect, onSessionSelect }) => {
  const { checkpoint: cp, laneIndex, onTheKit, altIds, drafts } = row;
  const color = kitTreeLaneColor(laneIndex);
  const border = isSelected ? "border-cyan-500" : isHighlighted ? "border-amber-400" : "border-zinc-200 dark:border-zinc-800";
  const bg = isSelected ? "bg-cyan-50 dark:bg-cyan-950/40" : isHighlighted ? "bg-amber-50 dark:bg-amber-950/30" : "";
  return (
    <div className={`flex gap-1.5 border-b px-1.5 py-1 ${border} ${bg}`}>
      <div className="flex flex-col items-center pt-0.5">
        <span
          className="inline-block h-2.5 w-2.5 rounded-full ring-2 ring-white dark:ring-zinc-950"
          style={{ background: color, outline: isHead ? "1px solid #0ea5e9" : undefined }}
          title={onTheKit ? "on the kit" : altIds.length ? `on alternatives: ${altIds.length}` : "detached"}
        />
        <span className="mt-0.5 block h-6 w-px" style={{ background: color }} />
      </div>
      <button type="button" className="flex min-w-0 flex-1 flex-col items-start text-left" onClick={() => onCheckpointSelect(cp.id)} title={cp.id}>
        <div className="flex w-full items-center gap-1.5">
          <span className="font-mono text-[10px]">{kitTreeShortId(cp.id)}</span>
          {isHead ? <span className="rounded bg-sky-600 px-1 text-[9px] text-white">HEAD</span> : null}
          {cp.isRelease ? <span className="rounded bg-emerald-600 px-1 text-[9px] text-white">release</span> : null}
          {onTheKit ? <span className="rounded border border-sky-600 px-1 text-[9px] text-sky-700 dark:text-sky-300">the kit</span> : null}
          {altIds.length > 0 ? <span className="text-muted-foreground text-[9px]">alts: {altIds.length}</span> : null}
          <span className="text-muted-foreground ml-auto text-[9px]">
            Δ{cp.changeCount}
            {cp.authors.length ? ` · 👤${cp.authors.length}` : ""}
          </span>
        </div>
        <div className="w-full truncate text-foreground">{cp.message || <span className="text-muted-foreground italic">(no message)</span>}</div>
        <div className="text-muted-foreground flex w-full items-center gap-2 text-[9px]">
          {cp.parent ? (
            <span>
              parent <span className="font-mono">{kitTreeShortId(cp.parent, 6)}</span>
            </span>
          ) : (
            <span className="italic">root parent</span>
          )}
          {cp.time ? <span>{cp.time}</span> : null}
          <span className="font-mono">hash {kitTreeShortId(cp.hash, 6)}</span>
        </div>
      </button>
      {drafts.length > 0 ? (
        <div className="flex shrink-0 flex-col items-end gap-0.5">
          {drafts.map(({ session, draft }) => (
            <KitTreeDraftBubble
              key={`${session.id}:${draft.id}`}
              session={session}
              draft={draft}
              selected={selectedDraftId === draft.id}
              sessionSelected={selectedSessionId === session.id}
              onDraftSelect={onDraftSelect}
              onSessionSelect={onSessionSelect}
            />
          ))}
        </div>
      ) : null}
    </div>
  );
};
// #endregion 📜KitTreeCheckpoints column

// #region 💾KitTreeDraftBubble
const KitTreeDraftBubble: React.FC<{
  readonly session: VcsSessionDto;
  readonly draft: VcsDraftDto;
  readonly selected: boolean;
  readonly sessionSelected: boolean;
  readonly onDraftSelect: (id: string) => void;
  readonly onSessionSelect: (id: string) => void;
}> = ({ session, draft, selected, sessionSelected, onDraftSelect, onSessionSelect }) => {
  const border = selected ? "border-cyan-500" : sessionSelected ? "border-amber-400" : "border-zinc-300 dark:border-zinc-600";
  return (
    <div className={`flex items-center gap-1 rounded border px-1 py-0.5 ${border} bg-white dark:bg-zinc-900`} title={`draft ${draft.id}\nsession ${session.id}`}>
      <button type="button" className="text-[9px]" onClick={() => onSessionSelect(session.id)}>
        📦<span className="font-mono">{kitTreeShortId(session.id, 6)}</span>
      </button>
      <button type="button" className="text-[9px]" onClick={() => onDraftSelect(draft.id)}>
        ✏️<span className="font-mono">{kitTreeShortId(draft.id, 6)}</span>
      </button>
      <span className="text-muted-foreground text-[9px]">
        tx {draft.finalizedTransactionCount}
        {draft.redoTransactionCount ? `↩${draft.redoTransactionCount}` : ""}
        {draft.openTransactionId ? " · ⏺" : ""}
        {draft.targetAlternative ? " · alt" : ""}
      </span>
    </div>
  );
};
// #endregion 💾KitTreeDraftBubble

// #region 🏷️KitTreeOrphanDrafts
const KitTreeOrphanDrafts: React.FC<{
  readonly sessions: readonly VcsSessionDto[];
  readonly onSessionSelect: (id: string) => void;
  readonly onDraftSelect: (id: string) => void;
  readonly selectedSessionId?: string;
  readonly selectedDraftId?: string;
}> = ({ sessions, onSessionSelect, onDraftSelect, selectedSessionId, selectedDraftId }) => {
  const orphans = React.useMemo(() => {
    const out: { session: VcsSessionDto; draft: VcsDraftDto }[] = [];
    sessions.forEach((session) => {
      session.drafts.forEach((draft) => {
        if (!draft.parentCheckpoint) out.push({ session, draft });
      });
    });
    return out;
  }, [sessions]);
  if (orphans.length === 0) return null;
  return (
    <div className="border-t border-zinc-200 bg-zinc-50 p-1 dark:border-zinc-800 dark:bg-zinc-950">
      <div className="text-muted-foreground pb-0.5 font-medium uppercase tracking-wide">Drafts on root</div>
      <div className="flex flex-wrap gap-1">
        {orphans.map(({ session, draft }) => (
          <KitTreeDraftBubble
            key={`${session.id}:${draft.id}`}
            session={session}
            draft={draft}
            selected={selectedDraftId === draft.id}
            sessionSelected={selectedSessionId === session.id}
            onDraftSelect={onDraftSelect}
            onSessionSelect={onSessionSelect}
          />
        ))}
      </div>
    </div>
  );
};
// #endregion 🏷️KitTreeOrphanDrafts

// #endregion 🌳KitTreeGraph
//#endregion 🔖HistoryControls

//#region 🔖CommandForm
import * as React from "react";

import {
  storybookKitGraphqlExecuteStoreCommand,
  storybookKitGraphqlRun,
  type StorybookKitGraphqlHandle,
} from "./composeWasm";

type Mode = "changeKit" | "readKit" | "execute";

export const CommandForm: React.FC<{
  handle: KitStoreHandle | null;
  mode: Mode;
  onMode: (m: Mode) => void;
  changeJson: string;
  onChangeJson: (s: string) => void;
  readJson: string;
  onReadJson: (s: string) => void;
  executeJson: string;
  onExecuteJson: (s: string) => void;
  onCommandRun: (o: { mode: Mode; forward: unknown; result?: unknown; error?: string; log: string }) => void;
}> = ({ handle, mode, onMode, changeJson, onChangeJson, readJson, onReadJson, executeJson, onExecuteJson, onCommandRun }) => {
  const area = mode === "changeKit" ? changeJson : mode === "readKit" ? readJson : executeJson;
  const setArea = mode === "changeKit" ? onChangeJson : mode === "readKit" ? onReadJson : onExecuteJson;

  return (
    <div className="text-foreground flex h-full min-h-0 flex-col gap-1 p-2 text-xs">
      <div className="flex flex-wrap items-center gap-1">
        {(["changeKit", "readKit", "execute"] as const).map((m) => (
          <button
            key={m}
            type="button"
            className={
              "rounded border px-1.5 py-0.5 text-[10px] " + (mode === m ? "border-violet-600 bg-violet-100 dark:bg-violet-950" : "border-zinc-300 dark:border-zinc-600")
            }
            onClick={() => onMode(m)}
          >
            {m}
          </button>
        ))}
        <span className="text-muted-foreground ml-auto text-[10px]">
          {ALL_CHANGE_KIT_ROOT_KEYS.length} ch · {ALL_READ_KIT_COMMAND_KEYS.length} read
        </span>
      </div>

      {mode === "changeKit" ? (
        <div className="flex flex-wrap gap-1">
          <select
            className="bg-background max-w-[14rem] rounded border border-zinc-300 px-1 py-0.5 text-[10px] dark:border-zinc-600"
            onChange={(e) => {
              const p = CHANGE_KIT_PRESETS.find((x) => x.id === e.target.value);
              if (p) onChangeJson(p.json);
              e.target.value = "";
            }}
            defaultValue=""
          >
            <option value="">preset…</option>
            {CHANGE_KIT_PRESETS.map((p) => (
              <option key={p.id} value={p.id}>
                {p.label}
              </option>
            ))}
          </select>
        </div>
      ) : null}
      {mode === "readKit" ? (
        <div className="flex flex-wrap gap-1">
          <select
            className="bg-background max-w-[14rem] rounded border border-zinc-300 px-1 py-0.5 text-[10px] dark:border-zinc-600"
            onChange={(e) => {
              const p = READ_KIT_PRESETS.find((x) => x.id === e.target.value);
              if (p) onReadJson(p.json);
              e.target.value = "";
            }}
            defaultValue=""
          >
            <option value="">read preset…</option>
            {READ_KIT_PRESETS.map((p) => (
              <option key={p.id} value={p.id}>
                {p.label}
              </option>
            ))}
          </select>
        </div>
      ) : null}

      <textarea
        className="bg-background min-h-[8rem] flex-1 resize-y rounded border border-zinc-300 p-1 font-mono text-[10px] leading-tight dark:border-zinc-600"
        spellCheck={false}
        value={area}
        onChange={(e) => setArea(e.target.value)}
      />
      <button
        type="button"
        className="rounded border border-violet-600 bg-violet-100 px-2 py-1 text-[11px] font-medium dark:bg-violet-950"
        disabled={!handle}
        onClick={() => {
          void (async () => {
            if (!handle) return;
            const gql: StorybookKitGraphqlHandle = {
              execute: (requestJson: string) => handle.execute(requestJson),
            };
            try {
              if (mode === "changeKit") {
                const parsed = JSON.parse(changeJson);
                const arr = Array.isArray(parsed) ? parsed : [parsed];
                const r = await handle.executeChangeKitCommands(arr);
                onCommandRun({ mode, forward: arr, result: r, log: `executeChangeKitCommands → ${JSON.stringify(r)}` });
              } else if (mode === "readKit") {
                const parsed = JSON.parse(readJson) as unknown;
                if (parsed == null || typeof parsed !== "object" || Array.isArray(parsed) || typeof (parsed as { query?: unknown }).query !== "string") {
                  throw new Error('readKit mode expects a JSON object with a string "query" field (and optional variables)');
                }
                const body = parsed as { query: string; variables?: Record<string, unknown>; operationName?: string };
                const r = await storybookKitGraphqlRun(gql, body);
                onCommandRun({ mode, forward: body, result: r, log: `kitGraphqlRun → ${JSON.stringify(r)}` });
              } else {
                const raw = JSON.parse(executeJson);
                const r = await storybookKitGraphqlExecuteStoreCommand(gql, raw);
                onCommandRun({ mode, forward: raw, result: r, log: `kitStoreExecute → ${JSON.stringify(r)}` });
              }
            } catch (e) {
              const err = e instanceof Error ? e.message : String(e);
              onCommandRun({ mode, forward: null, error: err, log: `ERROR: ${err}` });
            }
          })();
        }}
      >
        Run
      </button>
    </div>
  );
};
//#endregion 🔖CommandForm

//#region 🔖EntityPicker
import * as React from "react";


export const EntityPicker: React.FC<{
  handle: KitStoreHandle | null;
  onApplyPlaceholders: (s: string) => string;
  jsonForPlaceholders: string;
  onJsonChange: (s: string) => void;
}> = ({ handle, onApplyPlaceholders, jsonForPlaceholders, onJsonChange }) => {
  const snap = handle?.snapshot() as
    | {
        types?: { id: string; name?: string }[];
        designs?: { id: string; name?: string }[];
        files?: { id: string }[];
        folders?: { id: string }[];
        authors?: { id: string }[];
      }
    | undefined;

  const [ti, setTi] = React.useState(0);
  const [di, setDi] = React.useState(0);
  const [fi, setFi] = React.useState(0);
  const [foi, setFoi] = React.useState(0);
  const [ai, setAi] = React.useState(0);

  const t = snap?.types ?? [];
  const d = snap?.designs ?? [];
  const f = snap?.files ?? [];
  const fo = snap?.folders ?? [];
  const a = snap?.authors ?? [];

  return (
    <div className="text-foreground flex h-full min-h-0 flex-col gap-1 overflow-auto border-b border-zinc-200 p-2 text-xs dark:border-zinc-800">
      <div className="text-muted-foreground font-medium">Entity ids (for JSON placeholders)</div>
      {!snap ? <div className="text-muted-foreground">(no kit)</div> : null}
      <div className="grid max-w-full grid-cols-1 gap-1 text-[10px]">
        <L sel={ti} set={setTi} label="Type" options={t.map((x) => ({ id: x.id, n: x.name ?? x.id }))} onPick={onJsonChange} json={jsonForPlaceholders} ph="PLACEHOLDER_TYPE_ID" />
        <L sel={di} set={setDi} label="Design" options={d.map((x) => ({ id: x.id, n: x.name ?? x.id }))} onPick={onJsonChange} json={jsonForPlaceholders} ph="PLACEHOLDER_DESIGN_ID" />
        <L sel={fi} set={setFi} label="File" options={f.map((x) => ({ id: x.id, n: x.id }))} onPick={onJsonChange} json={jsonForPlaceholders} ph="PLACEHOLDER_FILE_ID" />
        <L sel={foi} set={setFoi} label="Folder" options={fo.map((x) => ({ id: x.id, n: x.id }))} onPick={onJsonChange} json={jsonForPlaceholders} ph="PLACEHOLDER_FOLDER_ID" />
        <L sel={ai} set={setAi} label="Author" options={a.map((x) => ({ id: x.id, n: x.id }))} onPick={onJsonChange} json={jsonForPlaceholders} ph="PLACEHOLDER_AUTHOR_ID" />
      </div>
      <button
        type="button"
        className="mt-1 w-full rounded border border-cyan-600 px-1 py-0.5 text-[10px] text-cyan-800 dark:text-cyan-200"
        onClick={() => onJsonChange(onApplyPlaceholders(jsonForPlaceholders))}
      >
        Replace all PLACEHOLDER_* in command JSON
      </button>
    </div>
  );
};

const L: React.FC<{
  label: string;
  options: { id: string; n: string }[];
  sel: number;
  set: (n: number) => void;
  json: string;
  onPick: (s: string) => void;
  ph: string;
}> = ({ label, options, sel, set, onPick, json, ph }) => (
  <label className="text-muted-foreground flex flex-wrap items-center gap-1">
    {label}
    <select
      className="bg-background max-w-full flex-1 rounded border border-zinc-300 px-1 py-0.5 font-mono dark:border-zinc-600"
      value={String(sel)}
      onChange={(e) => {
        const n = Number(e.target.value);
        set(n);
        const id = options[n]?.id;
        if (id) onPick(json.split(ph).join(id));
      }}
    >
      {options.length === 0 ? (
        <option value={0}>(empty)</option>
      ) : (
        options.map((o, i) => (
          <option key={o.id} value={i}>
            {o.n}
          </option>
        ))
      )}
    </select>
  </label>
);

/** Replace all known PLACEHOLDER_* in one pass. */
export function applyEntityPlaceholders(
  s: string,
  ctx: { typeId: string; designId: string; fileId: string; folderId: string; authorId: string; pieceId: string; connectionId: string },
): string {
  return s
    .split("PLACEHOLDER_TYPE_ID")
    .join(ctx.typeId)
    .split("PLACEHOLDER_DESIGN_ID")
    .join(ctx.designId)
    .split("PLACEHOLDER_FILE_ID")
    .join(ctx.fileId)
    .split("PLACEHOLDER_FOLDER_ID")
    .join(ctx.folderId)
    .split("PLACEHOLDER_AUTHOR_ID")
    .join(ctx.authorId)
    .split("PLACEHOLDER_PIECE_ID")
    .join(ctx.pieceId)
    .split("PLACEHOLDER_CONNECTION_ID")
    .join(ctx.connectionId);
}
//#endregion 🔖EntityPicker

//#region 🔖EventsFeed
import ReactJson from "@microlink/react-json-view";
import * as React from "react";


export interface LoggedEvent {
  readonly id: string;
  readonly t: number;
  readonly payload: unknown;
}

const fmtTime = (t: number) => new Date(t).toISOString().split("T")[1]!.slice(0, 12);

export const EventsFeed: React.FC<{
  events: readonly LoggedEvent[];
  onClear: () => void;
  filter: string;
  onFilterChange: (v: string) => void;
}> = ({ events, onClear, filter, onFilterChange }) => {
  const f = filter.trim().toLowerCase();
  const rows = f ? events.filter((e) => JSON.stringify(e.payload).toLowerCase().includes(f)) : events;
  const theme = useRjvTheme();

  return (
    <div className="text-foreground flex h-full min-h-0 flex-col gap-2 border-t border-zinc-200 p-2 text-xs dark:border-zinc-800">
      <div className="text-muted-foreground flex shrink-0 items-center justify-between gap-2">
        <span className="font-medium">Events ({rows.length})</span>
        <div className="flex items-center gap-1">
          <input
            className="bg-background w-32 rounded border border-zinc-300 px-1 py-0.5 text-[10px] dark:border-zinc-600"
            placeholder="filter…"
            value={filter}
            onChange={(e) => onFilterChange(e.target.value)}
          />
          <button type="button" className="rounded border border-zinc-300 px-1.5 py-0.5 dark:border-zinc-600" onClick={onClear}>
            clear
          </button>
        </div>
      </div>
      <ul className="min-h-0 flex-1 list-none space-y-1 overflow-auto p-0 font-mono text-[10px]">
        {rows.map((e) => (
          <li
            key={e.id}
            className="border-b border-zinc-100 py-0.5 dark:border-zinc-900"
            style={{ color: isErr(e.payload) ? "var(--destructive, #b91c1c)" : undefined }}
          >
            <div className="text-muted-foreground">
              {fmtTime(e.t)} {eventKind(e.payload)}
            </div>
            <div className="m-0 max-h-24 overflow-auto">
              <RjvPayload payload={e.payload} theme={theme} />
            </div>
          </li>
        ))}
        {rows.length === 0 ? <li className="text-muted-foreground">(no events)</li> : null}
      </ul>
    </div>
  );
};

// #region 🔖 RjvPayload
// Renders a single event payload with @microlink/react-json-view.
// Non-object payloads are wrapped into `{ value: ... }` because rjv requires an object root.
const RjvPayload: React.FC<{ payload: unknown; theme: "rjv-default" | "monokai" }> = ({ payload, theme }) => {
  const src = payload && typeof payload === "object" ? (payload as object) : { value: payload };
  const name = payload && typeof payload === "object" ? false : "value";
  return (
    <ReactJson
      src={src}
      name={name as false | string}
      theme={theme}
      iconStyle="triangle"
      indentWidth={2}
      collapsed={1}
      displayDataTypes={false}
      displayObjectSize={false}
      enableClipboard={false}
      style={{ background: "transparent", fontSize: "10px" }}
    />
  );
};
// #endregion 🔖 RjvPayload

function isErr(p: unknown): boolean {
  if (p == null) return false;
  const s = JSON.stringify(p);
  return s.includes("InvalidOperation") || s.includes("not yet") || s.includes("error");
}

function eventKind(p: unknown): string {
  if (p && typeof p === "object" && "log" in p) return "log";
  if (p && typeof p === "object" && "field" in p) return "field";
  if (p && typeof p === "object" && "SetRejected" in (p as object)) return "reject";
  return "event";
}
//#endregion 🔖EventsFeed

//#region 🔖SnapshotViewer
import ReactJson from "@microlink/react-json-view";
import * as React from "react";


type Tab = "live" | "theKit" | "mat" | "vcs";

//#region 🧮Snapshot value helpers
function cloneSnapshotValue<T>(value: T): T {
  try {
    return typeof structuredClone === "function" ? structuredClone(value) : (JSON.parse(JSON.stringify(value)) as T);
  } catch {
    return value;
  }
}

function readHandleValue(handle: KitStoreHandle, tab: Tab, matAt: string): unknown {
  if (tab === "live") return cloneSnapshotValue(handle.snapshot());
  if (tab === "theKit") return cloneSnapshotValue(handle.theKitDto());
  if (tab === "mat") {
    const at = matAt.trim();
    return cloneSnapshotValue(handle.readAt(at.length ? at : null));
  }
  return cloneSnapshotValue(handle.vcsState());
}
//#endregion 🧮Snapshot value helpers

export const SnapshotViewer: React.FC<{
  handle: KitStoreHandle | null;
  matAt: string;
  onMatAt: (s: string) => void;
}> = ({ handle, matAt, onMatAt }) => {
  const [tab, setTab] = React.useState<Tab>("live");
  const [value, setValue] = React.useState<unknown>({});
  const [errorText, setErrorText] = React.useState<string | null>(null);
  const theme = useRjvTheme();

  const load = React.useCallback(() => {
    if (!handle) {
      setValue({});
      setErrorText(null);
      return;
    }
    try {
      setValue(readHandleValue(handle, tab, matAt));
      setErrorText(null);
    } catch (e) {
      setErrorText(e instanceof Error ? e.message : String(e));
      setValue({});
    }
  }, [handle, tab, matAt]);

  React.useEffect(() => {
    load();
  }, [load]);

  const srcObject = value && typeof value === "object" ? (value as object) : { value };
  const rootName = value && typeof value === "object" ? false : "value";

  return (
    <div className="text-foreground flex h-full min-h-0 flex-col gap-1 p-2 text-xs">
      <div className="flex flex-wrap items-center gap-1">
        {(
          [
            ["live", "live snapshot()"],
            ["theKit", "theKitDto()"],
            ["mat", "readAt"],
            ["vcs", "vcsState()"],
          ] as const
        ).map(([k, lab]) => (
          <button
            key={k}
            type="button"
            className={
              "rounded border px-1.5 py-0.5 text-[10px] " + (tab === k ? "border-cyan-600 bg-cyan-100 dark:bg-cyan-950" : "border-zinc-300 dark:border-zinc-600")
            }
            onClick={() => setTab(k)}
          >
            {lab}
          </button>
        ))}
        <button
          type="button"
          className="ml-auto border border-zinc-300 px-1.5 py-0.5 text-[10px] dark:border-zinc-600"
          onClick={load}
        >
          refresh
        </button>
      </div>
      {tab === "mat" ? (
        <div className="space-y-1">
          <p className="text-muted-foreground m-0 text-[10px] leading-snug">
            Read-only: <code className="bg-muted-foreground/10 rounded px-0.5">KitFullDto</code> at the checkpoint (or initial when empty). Does not change the live store.
            Use <span className="text-foreground font-medium">VCS → Preview @ cp</span> to jump here from a selected checkpoint.
          </p>
          <label className="text-muted-foreground flex items-center gap-1 text-[10px]">
            at (checkpoint id, empty = initial only)
            <input
              className="bg-background flex-1 rounded border border-zinc-300 px-1 py-0.5 font-mono dark:border-zinc-600"
              value={matAt}
              onChange={(e) => onMatAt(e.target.value)}
            />
          </label>
        </div>
      ) : null}
      {errorText ? (
        <pre className="text-destructive m-0 max-h-24 overflow-auto font-mono text-[10px] wrap-break-word whitespace-pre-wrap">{errorText}</pre>
      ) : null}
      <div className="m-0 min-h-0 flex-1 overflow-auto rounded border border-zinc-200 bg-zinc-50 p-1 dark:border-zinc-800 dark:bg-zinc-950">
        <ReactJson
          src={srcObject}
          name={rootName as false | string}
          theme={theme}
          iconStyle="triangle"
          indentWidth={2}
          collapsed={2}
          displayDataTypes={false}
          displayObjectSize={false}
          enableClipboard
          style={{ background: "transparent", fontSize: "9px" }}
        />
      </div>
    </div>
  );
};

const snapshotViewerVitest = (
  import.meta as ImportMeta & {
    vitest?: {
      describe: typeof import("vitest").describe;
      expect: typeof import("vitest").expect;
      it: typeof import("vitest").it;
    };
  }
).vitest;

if (snapshotViewerVitest) {
  const { describe, expect, it } = snapshotViewerVitest;

  describe("SnapshotViewer helpers", () => {
    it("clones loaded values so refreshes always hand React a fresh state reference", () => {
      const live = { kit: { name: "Step2" } };

      const first = cloneSnapshotValue(live);
      live.kit.name = "Step1";
      const second = cloneSnapshotValue(live);

      expect(first).toEqual({ kit: { name: "Step2" } });
      expect(second).toEqual({ kit: { name: "Step1" } });
      expect(first).not.toBe(live);
      expect(second).not.toBe(live);
      expect((first as { kit: { name: string } }).kit).not.toBe(live.kit);
      expect((second as { kit: { name: string } }).kit).not.toBe(live.kit);
    });

    it("reads live snapshots through a detached clone", () => {
      const live = { kit: { name: "Step2" } };
      const handle = {
        snapshot: () => live,
        theKitDto: () => ({ source: "theKit" }),
        readAt: (at: string | null) => ({ at }),
        vcsState: () => ({ head: "cp-1" }),
      } as unknown as KitStoreHandle;

      const first = readHandleValue(handle, "live", "");
      live.kit.name = "Step1";
      const second = readHandleValue(handle, "live", "");

      expect(first).toEqual({ kit: { name: "Step2" } });
      expect(second).toEqual({ kit: { name: "Step1" } });
      expect(second).not.toBe(live);
    });
  });
}
//#endregion 🔖SnapshotViewer

//#region 🔖DiffViewer
import ReactJson from "@microlink/react-json-view";
import * as React from "react";


export const DiffViewer: React.FC<{
  last: { forward: unknown; result: unknown; error?: string } | null;
}> = ({ last }) => {
  const theme = useRjvTheme();
  return (
    <div className="text-foreground flex h-full min-h-0 flex-col gap-1 border-t border-zinc-200 p-2 text-xs dark:border-zinc-800">
      <div className="text-muted-foreground font-medium">Last change / inverse</div>
      {!last ? (
        <div className="text-muted-foreground">(no commands run yet)</div>
      ) : (
        <div className="min-h-0 flex-1 space-y-2 overflow-auto font-mono text-[10px]">
          {last.error ? (
            <pre className="text-destructive m-0 wrap-break-word whitespace-pre-wrap">{last.error}</pre>
          ) : null}
          <div>
            <div className="text-muted-foreground">forward</div>
            <div className="bg-muted/30 m-0 max-h-40 overflow-auto rounded p-1">
              <RjvValue value={last.forward} theme={theme} />
            </div>
          </div>
          <div>
            <div className="text-muted-foreground">result (kind + inverse)</div>
            <div className="bg-muted/30 m-0 max-h-40 overflow-auto rounded p-1">
              <RjvValue value={last.result} theme={theme} />
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

// #region 🔖 RjvValue
// Renders any JSON-serialisable value with @microlink/react-json-view.
// Wraps primitives into `{ value: ... }` because rjv requires an `object` root.
const RjvValue: React.FC<{ value: unknown; theme: "rjv-default" | "monokai" }> = ({ value, theme }) => {
  const src = value && typeof value === "object" ? (value as object) : { value };
  const name = value && typeof value === "object" ? false : "value";
  return (
    <ReactJson
      src={src}
      name={name as false | string}
      theme={theme}
      iconStyle="triangle"
      indentWidth={2}
      collapsed={2}
      displayDataTypes={false}
      displayObjectSize={false}
      enableClipboard={false}
      style={{ background: "transparent", fontSize: "10px" }}
    />
  );
};
// #endregion 🔖 RjvValue
//#endregion 🔖DiffViewer
