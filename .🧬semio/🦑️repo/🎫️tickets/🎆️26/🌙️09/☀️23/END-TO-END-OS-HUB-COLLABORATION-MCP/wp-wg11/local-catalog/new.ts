// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHost/local-catalog/component.ts
/** @emoji 🗂️ The shell's host-owned local document catalog — `os.config.local-catalog`, persisted local-only in the device's
 * data folder (`${S_DATA_DIR}/os`) — and the guest route to it. A guest runs without a filesystem (every wasm32 shell), so a
 * document it wants kept on this device (a studio persisted into a folder, bound to a file, or imported) reaches the host as
 * `replayShellCommand os.local-catalog.admit` with the document's own pack/spr pair: the host writes the document into its
 * folder lane through the store worker (the sync card's `open` + `localDocumentArchive` path), then records the admission as
 * an `admitLocalDocument` mutation of the catalog facet — write first, record second, so a failed or abandoned write never
 * lists a document that is not on disk (a write counts only once the lane reads the same archive back).
 * `os.local-catalog.retire` unlists one (its events stay on disk). The landing app is handed every kept document it has not
 * seen yet as `applyLocalCatalogDocument {documentId, pack, spr}` (a retained, idempotent catalog job), so a studio kept
 * before a reload is listed and opens again. Pure helpers only; the lane's commands, bounds, en + de notices and the shared
 * vectors live in the schema-first vocabulary `🔣️.json`, which the wgpu shell reads too. The shell wires them.
 * Ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP (SH2, route B).
 * @see ../../../../../../🎚️config/🧬️schema/🧬️mutations/📥️admit-local-document/🟦️.ts */
// #endregion 🧲️Header

import { decodeDocumentArchiveBytes, decodePackValue, encodeDocumentArchiveBytes, encodePackValue, packUInt, packUIntSafeOrNull, type PackValue, type PersistenceBinding } from "@semio-tech/framework-os";
import type { MutationEnvelope } from "@semio-tech/framework-replication";
import {
  admitLocalDocument,
  applyLocalCatalogConfigMutation,
  LOCAL_CATALOG_CONFIG_SCHEMA,
  type LocalCatalog,
  type LocalCatalogConfigMutation,
  type LocalDocument,
  type LocalDocumentStorage,
} from "../../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
import vocabulary from "./🔣️.json" with { type: "json" };

//#region 🔖️Contract
/** 🧭️ The lane's two guest commands and the landing app's re-hydration action (`🔣️.json`). */
export const LOCAL_CATALOG_ACTIONS_V1: Readonly<{ admit: string; retire: string; rehydrate: string }> = vocabulary.actions;

/** 🗂️ The catalog before its persisted archive arrives (or on a device without a data folder). */
export const EMPTY_LOCAL_CATALOG_V1: LocalCatalog = { documents: [] };

/** 📏️ Upper bound of one identifier, schema id or name the lane admits. */
export const LOCAL_CATALOG_TEXT_MAXIMUM_BYTES_V1: number = vocabulary.bounds.textMaximumBytes;

/** 📏️ Upper bound of one target path. */
export const LOCAL_CATALOG_TARGET_MAXIMUM_BYTES_V1: number = vocabulary.bounds.targetMaximumBytes;

/** 📏️ Upper bound of the base64 pack + spr text one admission carries (the archive's own byte authority is checked again
 * when it is encoded). */
export const LOCAL_CATALOG_DOCUMENT_MAXIMUM_BASE64_BYTES_V1: number = vocabulary.bounds.documentMaximumBase64Bytes;

/** ⏱️ How long the host waits for a folder write to read back identically before it reports the write as failed. */
export const LOCAL_CATALOG_WRITE_DEADLINE_MS_V1: number = vocabulary.bounds.writeDeadlineMs;

/** ⏱️ How long the host waits for a kept document's lane to answer a read before it skips the document. */
export const LOCAL_CATALOG_READ_DEADLINE_MS_V1: number = vocabulary.bounds.readDeadlineMs;

/** 🗣️ Everything the lane tells the user: its progress, its outcomes and every refusal (`🔣️.json`). */
export type LocalCatalogNoticeV1 = keyof typeof vocabulary.notices;

/** 🚫️ Every way the lane refuses a guest command, one reason per thing the user can act on. */
export type LocalCatalogRefusalV1 = Exclude<LocalCatalogNoticeV1, "keeping" | "kept" | "retired">;

/** 📥️ A validated admission: the catalog entry, the document archive to write and the folder binding it goes to. */
export type LocalCatalogAdmissionV1 = Readonly<{ document: LocalDocument; archive: Uint8Array; binding: Extract<PersistenceBinding, { kind: "folder" }> }>;
//#endregion 🔖️Contract

//#region 🔖️Admission
const utf8 = new TextEncoder();

function boundedText(value: unknown, maximumBytes: number, allowEmpty = false): string | null {
  if (typeof value !== "string" || (!allowEmpty && value.length === 0) || utf8.encode(value).byteLength > maximumBytes || /[\u0000-\u001f\u007f]/u.test(value) || value.trim() !== value) return null;
  return value;
}

function decodeBase64(value: unknown): Uint8Array | null {
  if (typeof value !== "string" || value.length > LOCAL_CATALOG_DOCUMENT_MAXIMUM_BASE64_BYTES_V1 || !/^[A-Za-z0-9+/]*={0,2}$/u.test(value) || value.length % 4 !== 0) return null;
  try {
    return Uint8Array.from(atob(value), (character) => character.charCodeAt(0));
  } catch {
    return null;
  }
}

function parentFolder(path: string): string {
  const cut = path.replace(/\/+$/u, "").lastIndexOf("/");
  return cut <= 0 ? "/" : path.slice(0, cut);
}

/** 📁️ Where a catalog entry's events live: its own folder, or the folder of its file (the document id names it inside that
 * folder's event log, exactly as the sync card binds a file). */
export function localCatalogBindingV1(document: LocalDocument): Extract<PersistenceBinding, { kind: "folder" }> {
  return { kind: "folder", dataClass: "persistedLocalOnly", path: document.storage === "file" ? parentFolder(document.target) : document.target };
}

/** 🔎️ Validates one `os.local-catalog.admit` request against the lane's bounds and resolves its target: an empty folder
 * target means the device's own `${dataDir}/os/local-documents/<id>`, which needs a data folder; an id that is a path segment
 * of its own (`/`, `.`, `..`) never names that folder. */
export function localCatalogAdmissionV1(args: Readonly<Record<string, unknown>> | undefined, dataDir: string | undefined, nowMs: number): LocalCatalogAdmissionV1 | { readonly refusal: LocalCatalogRefusalV1 } {
  const documentId = boundedText(args?.documentId, LOCAL_CATALOG_TEXT_MAXIMUM_BYTES_V1);
  const schema = boundedText(args?.schema, LOCAL_CATALOG_TEXT_MAXIMUM_BYTES_V1);
  const name = boundedText(args?.name, LOCAL_CATALOG_TEXT_MAXIMUM_BYTES_V1, true);
  const storage: LocalDocumentStorage | null = args?.storage === "folder" || args?.storage === "file" ? args.storage : null;
  const requested = boundedText(args?.target ?? "", LOCAL_CATALOG_TARGET_MAXIMUM_BYTES_V1, true);
  const pack = decodeBase64(args?.pack);
  const spr = decodeBase64(args?.spr);
  if (documentId === null || schema === null || name === null || storage === null || requested === null || pack === null || spr === null || pack.byteLength === 0 || documentId.includes("/") || documentId === "." || documentId === "..") return { refusal: "invalid-request" };
  if (storage === "file" && !requested.startsWith("/")) return { refusal: "invalid-request" };
  if (storage === "folder" && requested !== "" && !requested.startsWith("/")) return { refusal: "invalid-request" };
  const folderRoot = dataDir?.replace(/\/+$/u, "");
  if (requested === "" && !folderRoot) return { refusal: "no-data-folder" };
  const target = requested === "" ? `${folderRoot}/os/local-documents/${documentId}` : requested;
  let archive: Uint8Array;
  try {
    archive = encodeDocumentArchiveBytes({ parent_pack: Array.from(pack), parent_spr: Array.from(spr), members: [] });
  } catch {
    return { refusal: "invalid-request" };
  }
  const document: LocalDocument = { documentId, schema, name, storage, target, admittedAtMs: Math.max(0, Math.floor(nowMs)) };
  return { document, archive, binding: localCatalogBindingV1(document) };
}

/** 🔎️ The document id of an `os.local-catalog.retire` request, or `null`. */
export function localCatalogDocumentIdV1(args: Readonly<Record<string, unknown>> | undefined): string | null {
  return boundedText(args?.documentId, LOCAL_CATALOG_TEXT_MAXIMUM_BYTES_V1);
}
//#endregion 🔖️Admission

//#region 🔖️Facet
/** 🧾️ One whole-record `MutationEnvelope` of the catalog facet (the identity facet's shape): the diff carries the next
 * catalog, the inverse the one before, so a cross-tab listener folds by replacement. */
export function localCatalogMutationEnvelopeV1(actor: string, mutation: LocalCatalogConfigMutation, base: LocalCatalog): MutationEnvelope {
  return {
    id: `local-catalog-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}`,
    actor,
    document: LOCAL_CATALOG_CONFIG_SCHEMA,
    schemaVersion: LOCAL_CATALOG_CONFIG_SCHEMA,
    payloadHash: "",
    diff: { schemaId: LOCAL_CATALOG_CONFIG_SCHEMA, payload: applyLocalCatalogConfigMutation(base, mutation) },
    inverse: { targetOperation: mutation.mutation, inverseDiff: { schemaId: LOCAL_CATALOG_CONFIG_SCHEMA, payload: base }, baseVersion: 0, undoPolicy: "exactBaseOnly" },
  };
}

/** 🔎️ A catalog carried by an envelope payload or a persisted archive, or `undefined` for anything else. `admittedAtMs` is an
 * exact unsigned integer: a JSON number from an envelope, a `uint` carrier from a pack. */
export function decodeLocalCatalogPayloadV1(payload: unknown): LocalCatalog | undefined {
  if (payload === null || typeof payload !== "object" || !Array.isArray((payload as { documents?: unknown }).documents)) return undefined;
  const documents: LocalDocument[] = [];
  for (const entry of (payload as { documents: unknown[] }).documents) {
    const candidate = entry as Record<string, unknown> | null;
    const admittedAtMs = candidate === null || typeof candidate !== "object" ? null : packUIntSafeOrNull(candidate.admittedAtMs as PackValue);
    if (candidate === null || typeof candidate !== "object" || typeof candidate.documentId !== "string" || typeof candidate.schema !== "string" || typeof candidate.name !== "string" || (candidate.storage !== "folder" && candidate.storage !== "file") || typeof candidate.target !== "string" || admittedAtMs === null) return undefined;
    documents.push({ documentId: candidate.documentId, schema: candidate.schema, name: candidate.name, storage: candidate.storage, target: candidate.target, admittedAtMs });
  }
  return { documents };
}

/** 🗃️ The facet's persisted form: the whole catalog as the archive's pack value (the identity facet's form), each
 * `admittedAtMs` an exact `uint` — the schema's `u64`, byte-identical to the wgpu shell's archive (`🔣️.json` vectors). */
export function localCatalogArchiveV1(catalog: LocalCatalog): Uint8Array {
  const documents = catalog.documents.map((document) => ({ ...document, admittedAtMs: packUInt(BigInt(document.admittedAtMs)) }));
  return encodeDocumentArchiveBytes({ parent_pack: Array.from(encodePackValue({ documents })), parent_spr: [], members: [] });
}

/** 🗃️ The catalog a persisted archive holds, or `undefined` when the archive is not one. */
export function decodeLocalCatalogArchiveV1(archive: Uint8Array): LocalCatalog | undefined {
  try {
    return decodeLocalCatalogPayloadV1(decodePackValue(new Uint8Array(decodeDocumentArchiveBytes(archive).parent_pack)));
  } catch {
    return undefined;
  }
}

/** 📥️ The admission mutation for a validated request. */
export function localCatalogAdmitMutationV1(admission: LocalCatalogAdmissionV1): LocalCatalogConfigMutation {
  return admitLocalDocument(admission.document);
}

/** 🟰️ Whether two archives are byte for byte the same — the write verification's read-back test. */
export function sameLocalDocumentArchiveV1(left: Uint8Array, right: Uint8Array): boolean {
  return left.byteLength === right.byteLength && left.every((byte, index) => byte === right[index]);
}

function encodeBase64(bytes: readonly number[]): string {
  let text = "";
  for (let index = 0; index < bytes.length; index += 0x8000) text += String.fromCharCode(...bytes.slice(index, index + 0x8000));
  return btoa(text);
}

/** 🗃️ The landing app's `applyLocalCatalogDocument` arguments for one kept document: its id and its archive's own pack/spr
 * pair (base64), or `null` when the bytes are not a document archive. */
export function localCatalogRehydrationArgumentsV1(kept: Readonly<{ document: LocalDocument; archive: Uint8Array }>): { readonly documentId: string; readonly pack: string; readonly spr: string } | null {
  try {
    const archive = decodeDocumentArchiveBytes(kept.archive);
    return { documentId: kept.document.documentId, pack: encodeBase64(archive.parent_pack), spr: encodeBase64(archive.parent_spr) };
  } catch {
    return null;
  }
}
//#endregion 🔖️Facet

//#region 🔖️Copy
type Copy = { readonly en: string; readonly de: string };

/** 🗣️ What the lane tells the user, en + de (`🔣️.json`); `{name}` is the document's own name. */
export const LOCAL_CATALOG_NOTICES_V1: Readonly<Record<LocalCatalogNoticeV1, Copy>> = vocabulary.notices;

/** 🗣️ One notice in the shell's locale (`de`, else English). */
export function localCatalogNoticeTextV1(key: LocalCatalogNoticeV1, locale: string, name = ""): string {
  const copy = LOCAL_CATALOG_NOTICES_V1[key];
  return (locale === "de" ? copy.de : copy.en).replace("{name}", name);
}

/** 🩺️ The fault code a notice carries, so a probe, a law and the console line name the same outcome. */
export function localCatalogNoticeCodeV1(key: LocalCatalogNoticeV1): string {
  return `${vocabulary.codePrefix}${key}`;
}
//#endregion 🔖️Copy
