/** 🗑️ Batch delete, TypeScript twin of `./🦀️.rs`: the rows of a multi-selection leave their collection as ONE atomic edit batch. Every row is removed by the per-kind remove mutation the single `removeItem` raises (a kind whose invariant refuses a removal refuses the whole batch, nothing is half deleted); positions are removed from the end of the list so the earlier positions stay valid while the mutations apply in order. Pure: the document tree and the ids in, edits, mutations and a verdict out.
 *
 * 🧫️ Replayed on `🧪️tests/🧫️fixtures/🗑️delete-entities/🔣️.json` by both implementations; the retained-command-dispositions row of `deleteEntities` points at the same fixture, so the route is executed and not only counted.
 *
 * 🔗️ Rust twin: `./🦀️.rs` (`delete_edits`, `DeleteRefusal`) and `handle_edits` of `../🦀️.rs` (one emit = one undo step). */

/** 🆔️ The live tool id of the command. */
export const DELETE_ENTITIES_COMMAND_V1 = "deleteEntities";

/** 🗑️ One edit of the batch: the removal of the row at `index` of the list `path`. */
export type RemoveItemEditV1 = Readonly<{ kind: "removeItem"; path: string; index: number }>;

/** 🌳️ A plain document tree; the collections are arrays of objects with a string `id`. */
export type DocumentTreeV1 = Readonly<Record<string, unknown>>;

/** 🚫️ Why a batch delete is refused before anything is emitted. */
export type DeleteRefusalV1 = Readonly<{ kind: "nothing" } | { kind: "unknownCollection"; collection: string } | { kind: "unknownId"; collection: string; id: string }>;

/** 🌐️ A sentence in both interface languages (no default language). */
export type BilingualTextV1 = Readonly<{ en: string; de: string }>;

/** 🆔️ The fault code of a refusal, a row of {@link NORM_DELETE_NOTICES_V1}. */
export function deleteRefusalCodeV1(refusal: DeleteRefusalV1): "norm.entity-delete-empty" | "norm.entity-delete-unknown" {
  return refusal.kind === "nothing" ? "norm.entity-delete-empty" : "norm.entity-delete-unknown";
}

/** 💬️ The reason of a refusal in both languages. */
export function deleteRefusalMessageV1(refusal: DeleteRefusalV1): BilingualTextV1 {
  switch (refusal.kind) {
    case "nothing":
      return { en: "Select at least one row to delete", de: "Mindestens eine Zeile zum Löschen auswählen" };
    case "unknownCollection":
      return { en: `'${refusal.collection}' is not a list of this document`, de: `'${refusal.collection}' ist keine Liste dieses Dokuments` };
    case "unknownId":
      return { en: `'${refusal.id}' is not a row of '${refusal.collection}'`, de: `'${refusal.id}' ist keine Zeile von '${refusal.collection}'` };
  }
}

/** 📣️ The notices of the faults a batch delete can end in (twin of `norm_fault_notices`). */
export const NORM_DELETE_NOTICES_V1: Readonly<Record<string, BilingualTextV1>> = {
  "norm.entity-delete-empty": { en: "Select at least one row to delete.", de: "Mindestens eine Zeile zum Löschen auswählen." },
  "norm.entity-delete-unknown": { en: "A selected row no longer exists, so nothing was deleted.", de: "Eine ausgewählte Zeile existiert nicht mehr, daher wurde nichts gelöscht." },
  "norm.edit-unresolved": { en: "The edit could not be resolved against the document.", de: "Die Änderung konnte im Dokument nicht aufgelöst werden." },
};

const idOfRow = (row: unknown): string | undefined => {
  const id = typeof row === "object" && row !== null ? (row as Readonly<Record<string, unknown>>).id : undefined;
  return typeof id === "string" ? id : undefined;
};

/** 🧾️ The collection and ids a `deleteEntities` action names; the error is the sentence of the Rust `entities_arg`. */
export function entitiesArgsV1(args: unknown): Readonly<{ ok: true; collection: string; ids: readonly string[] } | { ok: false; error: string }> {
  const record = typeof args === "object" && args !== null ? (args as Readonly<Record<string, unknown>>) : {};
  const collection = record.collection;
  if (typeof collection !== "string" || collection.length === 0) return { ok: false, error: "an entity command needs a 'collection'" };
  if (!Array.isArray(record.ids)) return { ok: false, error: "an entity command needs an 'ids' array" };
  const ids = record.ids as readonly unknown[];
  if (!ids.every((id) => typeof id === "string")) return { ok: false, error: "every entity id must be a string" };
  return ids.length === 0 ? { ok: false, error: "an entity command needs at least one id" } : { ok: true, collection, ids: ids as readonly string[] };
}

/** 🗑️ The edits that delete the rows `ids` of `collection`: one removal per distinct row, the last position first. An empty selection, an unknown collection and an unknown id are refused so nothing is committed. */
export function deleteEditsV1(tree: DocumentTreeV1, collection: string, ids: readonly string[]): Readonly<{ ok: true; edits: readonly RemoveItemEditV1[] } | { ok: false; refusal: DeleteRefusalV1 }> {
  if (ids.length === 0) return { ok: false, refusal: { kind: "nothing" } };
  const rows = tree[collection];
  if (!Array.isArray(rows)) return { ok: false, refusal: { kind: "unknownCollection", collection } };
  const indices: number[] = [];
  for (const id of ids) {
    const index = rows.findIndex((row) => idOfRow(row) === id);
    if (index < 0) return { ok: false, refusal: { kind: "unknownId", collection, id } };
    indices.push(index);
  }
  const distinct = [...new Set(indices)].sort((left, right) => right - left);
  return { ok: true, edits: distinct.map((index) => ({ kind: "removeItem", path: collection, index })) };
}

/** 🌳️ The tree after the edits, in order (the reducer behind live previews and the test oracle); the input is never mutated. */
export function applyEditsToTreeV1(tree: DocumentTreeV1, edits: readonly RemoveItemEditV1[]): DocumentTreeV1 {
  const next: Record<string, unknown> = { ...tree };
  for (const edit of edits) {
    const rows = next[edit.path];
    if (!Array.isArray(rows) || edit.index < 0 || edit.index >= rows.length) throw new Error(`removeItem index ${edit.index} out of range for '${edit.path}'`);
    next[edit.path] = rows.filter((_, index) => index !== edit.index);
  }
  return next;
}

/** 🧩️ The per-kind remove mutations of one edit, or the reason the kind refuses. */
export type ResolveRemovalV1<M> = (tree: DocumentTreeV1, edit: RemoveItemEditV1) => Readonly<{ ok: true; mutations: readonly M[] } | { ok: false; error: string }>;

/** 📤️ The typed selection produces one atomic emission and one undo step, or a bilingual domain refusal before any mutation. */
export type DeleteSelectionVerdictV1<M> =
  | Readonly<{ outcome: "emitted"; edits: readonly RemoveItemEditV1[]; mutations: readonly M[]; undoSteps: 1; tree: DocumentTreeV1 }>
  | Readonly<{ outcome: "refused"; code: string; message: BilingualTextV1 | string; notice: BilingualTextV1 }>;

/** 🚪️ Raw action admission followed by the typed selection verdict. */
export type DeleteEntitiesVerdictV1<M> = DeleteSelectionVerdictV1<M> | Readonly<{ outcome: "invalid"; error: string }>;

/** 🗑️ Executes `deleteEntities` on `tree`: the edits of {@link deleteEditsV1}, each resolved by `resolve` into the mutations of its kind, all-or-nothing and in order; the document is untouched on a refusal. */
export function executeDeleteEntitiesV1<M>(tree: DocumentTreeV1, args: unknown, resolve: ResolveRemovalV1<M>): DeleteEntitiesVerdictV1<M> {
  const parsed = entitiesArgsV1(args);
  if (!parsed.ok) return { outcome: "invalid", error: parsed.error };
  return executeDeleteSelectionV1(tree, parsed.collection, parsed.ids, resolve);
}

/** 🗑️ Resolves a typed deletion selection atomically, including reducer refusals before mutation emission. */
export function executeDeleteSelectionV1<M>(tree: DocumentTreeV1, collection: string, ids: readonly string[], resolve: ResolveRemovalV1<M>): DeleteSelectionVerdictV1<M> {
  const planned = deleteEditsV1(tree, collection, ids);
  if (!planned.ok) {
    const code = deleteRefusalCodeV1(planned.refusal);
    return { outcome: "refused", code, message: deleteRefusalMessageV1(planned.refusal), notice: NORM_DELETE_NOTICES_V1[code]! };
  }
  const mutations: M[] = [];
  for (const edit of planned.edits) {
    const resolved = resolve(tree, edit);
    if (!resolved.ok) return { outcome: "refused", code: "norm.edit-unresolved", message: resolved.error, notice: NORM_DELETE_NOTICES_V1["norm.edit-unresolved"]! };
    mutations.push(...resolved.mutations);
  }
  return { outcome: "emitted", edits: planned.edits, mutations, undoSteps: 1, tree: applyEditsToTreeV1(tree, planned.edits) };
}
