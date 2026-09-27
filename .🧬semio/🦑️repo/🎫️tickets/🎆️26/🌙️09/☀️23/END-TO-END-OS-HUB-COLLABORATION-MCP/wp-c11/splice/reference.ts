/** ✂️ C11 window-3 payload draft — TS reference of the context-anchored text splice (twin of the SDK/writer Rust leaf).
 * Positions and lengths are Unicode scalar values. A splice names what its author saw around the edit (`before`, `after`);
 * replaying it against ANY current text relocates it deterministically: exact `before+deleted+after` nearest to `start`, then
 * the same with shrinking context, ties → nearest `start`, then lowest index. A `deleted` run that is gone deletes nothing
 * (the insert still lands, the outcome is clamped). */
export type TextSpliceV1 = { readonly start: number; readonly deleted: string; readonly insert: string; readonly before: string; readonly after: string };
export type LocatedSpliceV1 = { readonly start: number; readonly deleteLength: number; readonly clamped: boolean };

export const TEXT_SPLICE_CONTEXT_SCALARS = 32;

const scalars = (text: string): string[] => Array.from(text);

/** 📍️ Every scalar index where `needle` occurs in `hay` (overlapping). */
function occurrences(hay: readonly string[], needle: readonly string[]): number[] {
  const found: number[] = [];
  for (let index = 0; index + needle.length <= hay.length; index += 1) {
    let match = true;
    for (let offset = 0; offset < needle.length && match; offset += 1) match = hay[index + offset] === needle[offset];
    if (match) found.push(index);
  }
  return found;
}

/** 📍️ The splice's position in `text`: see the module doc. */
export function locateTextSpliceV1(text: string, splice: TextSpliceV1): LocatedSpliceV1 {
  const hay = scalars(text);
  const before = scalars(splice.before), deleted = scalars(splice.deleted), after = scalars(splice.after);
  const search = (run: readonly string[]): { start: number } | null => {
    for (let k = Math.max(before.length, after.length); k >= 0; k -= 1) {
      const head = before.slice(Math.max(0, before.length - k)), tail = after.slice(0, Math.min(k, after.length));
      const starts = occurrences(hay, [...head, ...run, ...tail]).map((index) => index + head.length);
      if (starts.length === 0) continue;
      starts.sort((left, right) => Math.abs(left - splice.start) - Math.abs(right - splice.start) || left - right);
      return { start: starts[0]! };
    }
    return null;
  };
  const exact = search(deleted);
  if (exact) return { start: exact.start, deleteLength: deleted.length, clamped: false };
  const relocated = search([]) ?? { start: Math.min(Math.max(splice.start, 0), hay.length) };
  return { start: relocated.start, deleteLength: 0, clamped: deleted.length > 0 };
}

/** ✂️ Applies one splice to `text` (relocated), answering the new text and whether it was clamped. */
export function applyTextSpliceV1(text: string, splice: TextSpliceV1): { readonly text: string; readonly clamped: boolean; readonly inverse: TextSpliceV1 } {
  const located = locateTextSpliceV1(text, splice);
  const hay = scalars(text);
  const removed = hay.slice(located.start, located.start + located.deleteLength).join("");
  const next = [...hay.slice(0, located.start), ...scalars(splice.insert), ...hay.slice(located.start + located.deleteLength)];
  const insertLength = scalars(splice.insert).length;
  const inverse: TextSpliceV1 = {
    start: located.start,
    deleted: splice.insert,
    insert: removed,
    before: next.slice(Math.max(0, located.start - TEXT_SPLICE_CONTEXT_SCALARS), located.start).join(""),
    after: next.slice(located.start + insertLength, located.start + insertLength + TEXT_SPLICE_CONTEXT_SCALARS).join(""),
  };
  return { text: next.join(""), clamped: located.clamped, inverse };
}

/** ⌨️ The ONE splice an editor host sends for its own change `previous → next`: the common scalar prefix and suffix bound the
 * changed run; the context is what the author saw around it. `null` when nothing changed. */
export function textSpliceFromEditV1(previous: string, next: string, context = TEXT_SPLICE_CONTEXT_SCALARS): TextSpliceV1 | null {
  const a = scalars(previous), b = scalars(next);
  let prefix = 0;
  while (prefix < a.length && prefix < b.length && a[prefix] === b[prefix]) prefix += 1;
  let suffix = 0;
  while (suffix < a.length - prefix && suffix < b.length - prefix && a[a.length - 1 - suffix] === b[b.length - 1 - suffix]) suffix += 1;
  if (prefix === a.length && prefix === b.length) return null;
  return {
    start: prefix,
    deleted: a.slice(prefix, a.length - suffix).join(""),
    insert: b.slice(prefix, b.length - suffix).join(""),
    before: a.slice(Math.max(0, prefix - context), prefix).join(""),
    after: a.slice(a.length - suffix, Math.min(a.length, a.length - suffix + context)).join(""),
  };
}
