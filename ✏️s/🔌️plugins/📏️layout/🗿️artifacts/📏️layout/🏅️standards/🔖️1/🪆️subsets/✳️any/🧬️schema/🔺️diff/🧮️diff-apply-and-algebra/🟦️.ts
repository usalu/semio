/** 🧮️ Central immutable Layout diff application, replay, inverse and composition. */
import type { LayoutArtifact, Frame, Page } from "../../🟦️.ts";
import type { LayoutDiff, PagePatch, FramePatch, LayoutInsertion, LayoutRemoval, LayoutRelocation } from "../🟦️.ts";

type Row = Record<string, unknown>;
type Delta<T, P = Row> = { removed: LayoutRemoval[]; inserted: LayoutInsertion<T>[]; moved: LayoutRelocation[]; modified?: { id: string; patch: P }[] };

/** ⚠️ A refused diff names the exact collection coordinate and leaves its base unchanged. */
export class LayoutDiffError extends Error {
  constructor(readonly code: string, readonly path: readonly (string | number)[], message: string) { super(message); this.name = "LayoutDiffError"; }
}

const own = (row: object, key: PropertyKey): boolean => Object.hasOwn(row, key);
const copy = <T>(value: T): T => structuredClone(value);
const fail = (code: string, at: readonly (string | number)[], message: string): never => { throw new LayoutDiffError(`mutation.apply.${code}`, at, message); };
const index = (value: number, length: number, at: readonly (string | number)[], code: string): number => Number.isSafeInteger(value) && value >= 0 && value < length ? value : fail(code, at, "row index lies outside the list");
const equal = (a: unknown, b: unknown): boolean => {
  if (Object.is(a, b)) return true;
  if (a === null || b === null || typeof a !== "object" || typeof b !== "object" || Array.isArray(a) !== Array.isArray(b)) return false;
  const keys = Object.keys(a), other = Object.keys(b);
  return keys.length === other.length && keys.every(key => own(b, key) && equal((a as Row)[key], (b as Row)[key]));
};
const empty = <T, P = Row>(): Delta<T, P> => ({ removed: [], inserted: [], moved: [], modified: [] });
const keyOf = (row: unknown): string => {
  const record = row as Row;
  const key = record.id ?? record.objectId ?? record.questionId;
  return typeof key === "string" ? key : fail("invalid-target", [], "row has no stable key");
};

function keyed<T, P>(base: readonly T[], delta: Delta<T, P>, at: readonly (string | number)[], patch: (row: T, value: P, at: readonly (string | number)[]) => T): T[] {
  const taken = new Set<number>();
  const take = (id: string, position: number, section: string, entry: number): void => {
    const where = [...at, section, entry];
    index(position, base.length, where, "missing-target");
    if (keyOf(base[position]) !== id) fail("missing-target", where, "row is not at its base index");
    if (taken.has(position)) fail("duplicate-target", where, "base row is named twice");
    taken.add(position);
  };
  delta.removed.forEach((row, i) => take(row.id, row.index, "removed", i));
  delta.moved.forEach((row, i) => take(row.id, row.from, "moved", i));
  const length = base.length - delta.removed.length + delta.inserted.length;
  const slots: (T | undefined)[] = Array.from({ length });
  const put = (row: T, position: number, section: string, entry: number): void => {
    const where = [...at, section, entry];
    index(position, length, where, section === "inserted" ? "invalid-add-index" : "invalid-move-index");
    if (slots[position] !== undefined) fail("duplicate-target", where, "two rows take the same after index");
    slots[position] = copy(row);
  };
  delta.inserted.forEach((row, i) => put(row.row, row.index, "inserted", i));
  delta.moved.forEach((row, i) => put(base[row.from], row.to, "moved", i));
  const survivors = base.filter((_, i) => !taken.has(i));
  let cursor = 0;
  for (let i = 0; i < length; i++) if (slots[i] === undefined) slots[i] = copy(survivors[cursor++]);
  const rows = slots as T[], positions = new Map<string, number>();
  rows.forEach((row, i) => { const key = keyOf(row); if (positions.has(key)) fail("duplicate-target", [...at, "inserted", i], "after rows carry the same stable key"); positions.set(key, i); });
  delta.modified?.forEach((change, i) => {
    const where = [...at, "modified", i], position = positions.get(change.id);
    if (position === undefined) fail("missing-target", where, "modified row does not exist");
    const target = position as number;
    rows[target] = patch(rows[target], change.patch, where);
    if (keyOf(rows[target]) !== change.id) fail("identity-change", where, "modified row must retain its stable key");
  });
  return rows;
}

function sparse<T>(base: T, patch: Row, clear: readonly string[] = [], names: Readonly<Record<string, string>> = {}): T {
  const next = copy(base) as Row;
  for (const [field, value] of Object.entries(patch)) if (value !== undefined && (value !== null || clear.includes(field))) next[names[field] ?? field] = copy(value);
  return next as T;
}

function positional<T, R extends { index: number }>(base: readonly T[], delta: { len?: number | null; rows: R[] }, read: (row: R) => T, at: readonly (string | number)[]): T[] {
  const length = delta.len ?? base.length;
  if (!Number.isSafeInteger(length) || length < 0) fail("invalid-add-index", at, "invalid positional list length");
  const result: (T | undefined)[] = base.slice(0, length).map(copy), seen = new Set<number>();
  for (const [i, row] of delta.rows.entries()) {
    index(row.index, length, [...at, "rows", i], "invalid-add-index");
    if (seen.has(row.index)) fail("duplicate-target", [...at, "rows", i], "position is named twice");
    seen.add(row.index);
    result[row.index] = copy(read(row));
  }
  for (let i = 0; i < length; i++) if (result[i] === undefined) fail("missing-target", [...at, i], "new positional rows must be supplied");
  return result as T[];
}

function frame(base: Frame, patch: FramePatch): Frame {
  let next = sparse(base, patch as unknown as Row, [], { layer_id: "layerId" });
  next.bounds = sparse(next.bounds, { x: patch.x, y: patch.y, w: patch.width, h: patch.height, rotation: patch.rotation });
  if (patch.locked != null) next.locked = patch.locked ? true : null;
  if (patch.visible != null) next.visible = patch.visible ? null : false;
  if (next.kind === "rect") for (const field of ["fill", "stroke"] as const) if (own(patch, field)) next[field] = copy(patch[field] ?? null);
  if (next.kind === "text") {
    next = sparse(next, patch as unknown as Row, ["thread_next"], { story_id: "storyId", thread_next: "threadNext", wrap_mode: "wrapMode" });
    next.inset = sparse(next.inset, { x: patch.inset_x, y: patch.inset_y, w: patch.inset_width, h: patch.inset_height });
  }
  const allowed = new Set(Object.keys(base));
  for (const field of Object.keys(next)) if (!allowed.has(field)) delete (next as unknown as Row)[field];
  return next;
}

function page(base: Page, patch: PagePatch, at: readonly (string | number)[]): Page {
  const next = sparse(base, { name: patch.name, width: patch.width, height: patch.height, spreadId: patch.spread_id });
  if (own(patch, "parent_page_id")) next.parentPageId = patch.parent_page_id ?? null;
  if (own(patch,"background_drawing")) {if(patch.background_drawing===null)delete next.backgroundDrawing;else next.backgroundDrawing=copy(patch.background_drawing!); }
  next.margins = sparse(next.margins, { top: patch.margin_top, right: patch.margin_right, bottom: patch.margin_bottom, left: patch.margin_left });
  next.columns = sparse(next.columns, { count: patch.columns_count, gutter: patch.columns_gutter });
  if (patch.guides) next.guides = positional(next.guides, patch.guides, row => row.guide, [...at, "guides"]);
  next.overrides = keyed(next.overrides, patch.overrides, [...at, "overrides"], row => row);
  next.frames = keyed(next.frames, patch.frames, [...at, "frames"], frame);
  next.layers = keyed(next.layers, patch.layers, [...at, "layers"], (row, value) => sparse(row, value as Row));
  next.layerIds = next.layers.map(layer => layer.id);
  next.layers.forEach(layer => { layer.objectIds = next.frames.filter(row => row.layerId === layer.id).map(row => row.id); });
  return next;
}

/** 🛡️ Applies one diff atomically through the shared collection and field rules. */
export function applyLayoutDiff(base: LayoutArtifact, diff: LayoutDiff): LayoutArtifact {
  const next = sparse(base, { schema: diff.schema, name: diff.name });
  if (diff.grid) next.grid = sparse(next.grid, diff.grid as Row);
  const plain = <T>(row: T, patch: unknown): T => sparse(row, patch as Row);
  if (diff.paragraphStyles) next.paragraphStyles = keyed(next.paragraphStyles, diff.paragraphStyles, ["paragraphStyles"], plain);
  if (diff.characterStyles) next.characterStyles = keyed(next.characterStyles, diff.characterStyles, ["characterStyles"], (row, patch) => sparse(row, patch as Row, Object.keys(patch)));
  if (diff.parentPages) next.parentPages = keyed(next.parentPages, diff.parentPages, ["parentPages"], plain);
  if (diff.spreads) next.spreads = keyed(next.spreads, diff.spreads, ["spreads"], plain);
  if (diff.pages) next.pages = keyed(next.pages, diff.pages, ["pages"], page);
  if (diff.stories) next.stories = keyed(next.stories, diff.stories, ["stories"], (row, patch, at) => {
    const story = sparse(row, { content: patch.content });
    if (patch.style_runs) story.styleRuns = positional(story.styleRuns, patch.style_runs, row => row.run, [...at, "styleRuns"]);
    return story;
  });
  if (diff.links) next.links = keyed(next.links, diff.links, ["links"], (row, patch) => {
    const link = sparse(row, { path: patch.path, width: patch.width, height: patch.height, dpi: patch.dpi });
    if (own(patch,"color_profile")) link.colorProfile = patch.color_profile ?? null;
    return link;
  });
  if (diff.printTarget) next.printTarget = diff.printTarget.target;
  if (diff.dataFields) {
    const change = diff.dataFields, reset = change.presence === "replaced" || (change.presence === "created" && !next.dataFields);
    if (change.presence === "deleted") delete next.dataFields;
    else if (reset || next.dataFields) next.dataFields = { entries: keyed(reset ? [] : next.dataFields!.entries, change.entries ?? empty(), ["dataFields", "entries"], row => row) };
    else if (change.entries && (change.entries.inserted.length || change.entries.removed.length || change.entries.moved.length)) fail("missing-target", ["dataFields"], "data-field rows need an existing dictionary");
  }
  for (const field of ["backgroundDrawing", "referencedModel"] as const) if (own(diff, field)) {
    if (diff[field] == null) delete next[field];
    else Object.assign(next, { [field]: copy(diff[field]) });
  }
  return next;
}

function difference<T>(before: readonly T[], after: readonly T[]): Omit<Delta<T, never>, "modified"> & { modified: never[] } | null {
  if (equal(before, after)) return null;
  const old = new Map(before.map((row, index) => [keyOf(row), { row, index }])), next = new Map(after.map((row, index) => [keyOf(row), { row, index }]));
  const removed = before.flatMap((row, index) => { const found = next.get(keyOf(row)); return found && equal(row, found.row) ? [] : [{ id: keyOf(row), index }]; });
  const inserted = after.flatMap((row, index) => { const found = old.get(keyOf(row)); return found && equal(row, found.row) ? [] : [{ index, row: copy(row) }]; });
  const moved = after.flatMap((row, to) => { const found = old.get(keyOf(row)); return found && found.index !== to && equal(row, found.row) ? [{ id: keyOf(row), from: found.index, to }] : []; });
  return { removed, inserted, moved, modified: [] };
}

/** 🔺️ Builds an exact reversible diff between two materialized document snapshots. */
export function diffLayoutSnapshots(before: LayoutArtifact, after: LayoutArtifact): LayoutDiff {
  const diff: LayoutDiff = {
    schema: before.schema === after.schema ? null : after.schema, name: before.name === after.name ? null : after.name,
    grid: equal(before.grid, after.grid) ? null : copy(after.grid),
    paragraphStyles: difference(before.paragraphStyles, after.paragraphStyles), characterStyles: difference(before.characterStyles, after.characterStyles),
    stories: difference(before.stories, after.stories), links: difference(before.links, after.links), parentPages: difference(before.parentPages, after.parentPages),
    spreads: difference(before.spreads, after.spreads), pages: difference(before.pages, after.pages),
    printTarget: before.printTarget === after.printTarget ? null : { target: after.printTarget },
    dataFields: equal(before.dataFields, after.dataFields) ? null : after.dataFields ? { presence: before.dataFields ? "replaced" : "created", entries: { removed: [], inserted: after.dataFields.entries.map((row, index) => ({ index, row: copy(row) })), moved: [] } } : { presence: "deleted", entries: null },
  };
  for (const field of ["backgroundDrawing", "referencedModel"] as const) if (!equal(before[field], after[field])) Object.assign(diff, { [field]: copy(after[field] ?? null) });
  return diff;
}

/** ↩️ Inverts every supplied field and ordered row change against its exact base. */
export function inverseLayoutDiff(base: LayoutArtifact, diff: LayoutDiff): LayoutDiff { return diffLayoutSnapshots(applyLayoutDiff(base, diff), base); }

/** ➕️ Composes ordered events against their canonical base through the central applier. */
export function composeLayoutDiffs(base: LayoutArtifact, diffs: readonly LayoutDiff[]): LayoutDiff { return diffLayoutSnapshots(base, replayLayoutDiffs(base, diffs)); }

/** 🎞️ Replays an ordered event stream without mutating any input snapshot or event. */
export function replayLayoutDiffs(base: LayoutArtifact, diffs: readonly LayoutDiff[]): LayoutArtifact { return diffs.reduce((snapshot, diff) => applyLayoutDiff(snapshot, diff), base); }
