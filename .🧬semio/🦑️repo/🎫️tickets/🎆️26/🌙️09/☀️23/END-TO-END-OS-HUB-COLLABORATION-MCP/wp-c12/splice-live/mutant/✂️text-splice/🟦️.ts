//#region ✂️TextSplice
/** @emoji ✂️ One range-text operation as its author saw it: at scalar `start` of the author's text, `deleted` was replaced by
 * `insert`; `before`/`after` are up to {@link TEXT_SPLICE_CONTEXT_SCALARS} scalars the author saw around the replaced run.
 * Positions and lengths count Unicode scalar values (Rust `char`s), the same in every language.
 * @see ../🧬️schema/✂️text-splice/🔣️.json */
export type TextSpliceV1 = { readonly start: number; readonly deleted: string; readonly insert: string; readonly before: string; readonly after: string };

/** @emoji 📍️ Where a splice lands in a concrete text: the run it replaces there, and whether its `deleted` run was already gone
 * (`clamped`: nothing is deleted, the insert still lands) or its context was nowhere to be found. */
export type LocatedTextSpliceV1 = { readonly start: number; readonly deleteLength: number; readonly clamped: boolean };

/** @emoji 🧾️ A splice applied to a text: the new text, where it landed, and the inverse splice that restores the text. */
export type AppliedTextSpliceV1 = { readonly text: string; readonly located: LocatedTextSpliceV1; readonly inverse: TextSpliceV1 };

/** @emoji 📏️ Context scalars a splice carries on each side. */
export const TEXT_SPLICE_CONTEXT_SCALARS = 32;

/** @emoji 🔗️ Two-sided context patterns shorter than this many scalars per side are never searched: two scalars of context
 * match all over a real document, one-sided context of the same length near the author's position is the better witness. */
export const TEXT_SPLICE_MIN_TWO_SIDED_SCALARS = 4;

const scalars = (text: string): string[] => Array.from(text);

/** @emoji 🔎️ Every start index of `needle` in `hay` (scalar arrays, overlapping). */
function occurrencesV1(hay: readonly string[], needle: readonly string[]): number[] {
  const found: number[] = [];
  if (needle.length === 0) {
    for (let index = 0; index <= hay.length; index += 1) found.push(index);
    return found;
  }
  for (let index = 0; index + needle.length <= hay.length; index += 1) {
    let match = true;
    for (let offset = 0; offset < needle.length && match; offset += 1) match = hay[index + offset] === needle[offset];
    if (match) found.push(index);
  }
  return found;
}

/** @emoji 🎯️ The candidate position nearest to `expected`; ties take the lowest position. */
function nearestV1(positions: readonly number[], expected: number): number | null {
  let best: number | null = null;
  for (const position of positions) {
    if (best === null || Math.abs(position - expected) < Math.abs(best - expected) || (Math.abs(position - expected) === Math.abs(best - expected) && position < best)) best = position;
  }
  return best;
}

/** @emoji 📍️ Locates `splice` in `text`, deterministically and identically in every language twin:
 * 1. the run with context on both sides, `k` = the longer side's length down to {@link TEXT_SPLICE_MIN_TWO_SIDED_SCALARS}
 *    (each side capped at its own length);
 * 2. the run with context on one side, `k` = the longer side's length down to 1 (both sides of one `k` compete);
 * 3. the run alone (only when it is not empty);
 * 4. a run that is gone deletes nothing (`clamped`): steps 1–2 again with an empty run find the insertion point;
 * 5. otherwise the author's `start`, clamped to the text (`clamped` when the splice carried any context).
 * Every step takes the match nearest to the author's `start`, ties to the lowest position. */
export function locateTextSpliceV1(text: string, splice: TextSpliceV1): LocatedTextSpliceV1 {
  const hay = scalars(text);
  const before = scalars(splice.before), deleted = scalars(splice.deleted), after = scalars(splice.after);
  const longest = Math.max(before.length, after.length);
  const anchored = (run: readonly string[]): number | null => {
    for (let k = longest; k >= TEXT_SPLICE_MIN_TWO_SIDED_SCALARS; k -= 1) {
      const head = before.slice(before.length - Math.min(k, before.length)), tail = after.slice(0, Math.min(k, after.length));
      if (head.length === 0 || tail.length === 0) break;
      const found = nearestV1(occurrencesV1(hay, [...head, ...run, ...tail]).map((index) => index + head.length), splice.start);
      if (found !== null) return found;
    }
    for (let k = longest; k >= 1; k -= 1) {
      const head = before.slice(before.length - Math.min(k, before.length)), tail = after.slice(0, Math.min(k, after.length));
      const candidates = [...(head.length > 0 ? occurrencesV1(hay, [...head, ...run]).map((index) => index + head.length) : []), ...(tail.length > 0 ? occurrencesV1(hay, [...run, ...tail]) : [])];
      const found = nearestV1(candidates, splice.start);
      if (found !== null) return found;
    }
    return null;
  };
  const exact = anchored(deleted) ?? (deleted.length > 0 ? nearestV1(occurrencesV1(hay, deleted), splice.start) : null);
  if (exact !== null) return { start: exact, deleteLength: deleted.length, clamped: false };
  const insertion = deleted.length > 0 ? anchored([]) : null;
  if (insertion !== null) return { start: insertion, deleteLength: 0, clamped: true };
  return { start: Math.min(Math.max(splice.start, 0), hay.length), deleteLength: 0, clamped: deleted.length > 0 || before.length > 0 || after.length > 0 };
}

/** @emoji ✂️ Applies `splice` to `text` where it locates, answering the new text and the inverse splice (the removed run back
 * in place of the inserted one, with the context of the new text). */
export function applyTextSpliceV1(text: string, splice: TextSpliceV1, context = TEXT_SPLICE_CONTEXT_SCALARS): AppliedTextSpliceV1 {
  const located = locateTextSpliceV1(text, splice);
  const hay = scalars(text), insert = scalars(splice.insert);
  const removed = hay.slice(located.start, located.start + located.deleteLength).join("");
  const next = [...hay.slice(0, located.start), ...insert, ...hay.slice(located.start + located.deleteLength)];
  const end = located.start + insert.length;
  return {
    text: next.join(""),
    located,
    inverse: { start: located.start, deleted: splice.insert, insert: removed, before: next.slice(Math.max(0, located.start - context), located.start).join(""), after: next.slice(end, end + context).join("") },
  };
}

/** @emoji ⌨️ The ONE splice an editor sends for its own change `previous → next`: the common scalar prefix and suffix bound the
 * changed run, the context is what the author saw around it; `null` when nothing changed. */
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

/** @emoji 🧷️ A zero-width splice marking scalar `position` of `text` — how a caret or a selection end travels through other
 * authors' splices: locating the marker in the new text is the position's new place. */
export function textMarkerV1(text: string, position: number, context = TEXT_SPLICE_CONTEXT_SCALARS): TextSpliceV1 {
  const hay = scalars(text);
  const at = Math.min(Math.max(position, 0), hay.length);
  return { start: at, deleted: "", insert: "", before: hay.slice(Math.max(0, at - context), at).join(""), after: hay.slice(at, at + context).join("") };
}

/** @emoji 🔁️ The editor host's view after the guest published `remote`: every splice the guest has not applied yet, in the
 * order the host sent them, folded onto `remote`, and the local selection (scalar anchor/caret in `local`) carried along. */
export function rebaseTextEditsV1(remote: string, unapplied: readonly TextSpliceV1[], local: string, selection: { readonly anchor: number; readonly caret: number }): { readonly text: string; readonly anchor: number; readonly caret: number } {
  let text = remote;
  for (const splice of unapplied) text = applyTextSpliceV1(text, splice).text;
  const place = (position: number) => locateTextSpliceV1(text, textMarkerV1(local, position)).start;
  return { text, anchor: place(selection.anchor), caret: place(selection.caret) };
}

/** @emoji 🔢️ Scalar index of UTF-8 byte offset `bytes` in `text` (the wasm editor session's offsets). */
export function scalarOfUtf8OffsetV1(text: string, bytes: number): number {
  let consumed = 0, index = 0;
  for (const scalar of text) {
    if (consumed >= bytes) return index;
    consumed += new TextEncoder().encode(scalar).length;
    index += 1;
  }
  return index;
}

/** @emoji 🔢️ UTF-8 byte offset of scalar index `index` in `text`. */
export function utf8OffsetOfScalarV1(text: string, index: number): number {
  return new TextEncoder().encode(scalars(text).slice(0, index).join("")).length;
}
//#endregion ✂️TextSplice

//#region ⌨️TextEditorTyping
/** @emoji ⌨️ How a text editor host types into one window (`settingsJson.typing`, schema `TextEditorTypingV1`): `splice` —
 * every typed run is one {@link TextSpliceV1} (`textSplice`, with the host's `seq`), and the scene's `selectionJson.splice`
 * names the last `seq` the window applied. Absent: the window takes whole-text `textEdit`. */
export type TextEditorTypingV1 = { readonly mode: "splice" };

/** @emoji ⌨️ The typing contract a scene's `settingsJson` declares, or `null`. */
export function textEditorTypingV1(settingsJson: string | undefined): TextEditorTypingV1 | null {
  if (!settingsJson) return null;
  try {
    const typing = (JSON.parse(settingsJson) as { readonly typing?: { readonly mode?: unknown } }).typing;
    return typing?.mode === "splice" ? { mode: "splice" } : null;
  } catch {
    return null;
  }
}

/** @emoji 🔢️ The last host splice `seq` a scene's `selectionJson` says its window applied (`0` when none). */
export function textEditorAppliedSpliceV1(selectionJson: string | undefined): number {
  if (!selectionJson) return 0;
  try {
    const splice = (JSON.parse(selectionJson) as { readonly splice?: unknown }).splice;
    return typeof splice === "number" && Number.isSafeInteger(splice) && splice >= 0 ? splice : 0;
  } catch {
    return 0;
  }
}

/** @emoji 🧾️ One splice-typing host's knowledge of one window (the Jupiter client model, no CRDT): `remote` — the text the
 * window last published, `base` — the text the window holds once it applied every splice this host sent (the text the next
 * splice is computed against), `seq` — the last splice sent, `acknowledged` — the last splice a scene said the window applied (a
 * scene naming a lower one comes from a reincarnated window), `unapplied` — the sent splices no scene acknowledged yet, oldest
 * first. */
export type TextEditorSpliceHostV1 = {
  readonly remote: string;
  readonly base: string;
  readonly seq: number;
  readonly acknowledged: number;
  readonly unapplied: readonly { readonly seq: number; readonly splice: TextSpliceV1 }[];
};

/** @emoji 🖼️ What a host's editor shows after a scene: the text and the scalar selection, or `null` when it already shows it. */
export type TextEditorSpliceViewV1 = { readonly text: string; readonly anchor: number; readonly caret: number } | null;

/** @emoji 🌱️ A host that shows `buffer` of a window that applied splice `applied`: its numbers continue after `applied`, so a
 * remounted host never reuses a `seq` the window already took. */
export function textEditorSpliceHostV1(buffer: string, applied: number): TextEditorSpliceHostV1 {
  return { remote: buffer, base: buffer, seq: applied, acknowledged: applied, unapplied: [] };
}

/** @emoji ⌨️ The host's editor now shows `local`: the ONE splice to send (numbered `seq`) and the host after it, or `null` when
 * the window will already hold `local`. */
export function sendTextEditorSpliceV1(host: TextEditorSpliceHostV1, local: string): { readonly host: TextEditorSpliceHostV1; readonly splice: TextSpliceV1; readonly seq: number } | null {
  const splice = textSpliceFromEditV1(host.base, local);
  if (splice === null) return null;
  const seq = host.seq + 1;
  return { host: { ...host, base: local, seq, unapplied: [...host.unapplied, { seq, splice }] }, splice, seq };
}

function textEditorSpliceShowV1(host: TextEditorSpliceHostV1, remote: string, unapplied: TextEditorSpliceHostV1["unapplied"], local: string, selection: { readonly anchor: number; readonly caret: number }, undelivered: TextSpliceV1 | null): { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 } {
  const base = unapplied.reduce((text, entry) => applyTextSpliceV1(text, entry.splice).text, remote);
  const view = rebaseTextEditsV1(remote, undelivered === null ? unapplied.map((entry) => entry.splice) : [...unapplied.map((entry) => entry.splice), undelivered], local, selection);
  return { host: { ...host, remote, base, unapplied }, show: view.text === local ? null : view };
}

/** @emoji 🔁️ The window published `remote` having applied this host's splices up to `applied`, while the editor shows `local` with
 * the scalar `selection`: the editor shows `remote` with every still-unapplied splice and the not-yet-sent local change folded
 * on, the selection carried along by its context — nothing when that is what it shows already (an echo of its own typing),
 * so neither a lagging echo nor a collaborator's run ever reverts or scrambles what this host typed. A scene naming a splice BELOW
 * the one the window already acknowledged comes from a reincarnated window (the document reopened, silently on a hub-ordered
 * reorder too): this host's unapplied splices never reached it, so they are dropped instead of being folded onto its text as if
 * still in flight (which duplicated them once the reopened document carried them); the numbering continues. */
export function receiveTextEditorSceneV1(host: TextEditorSpliceHostV1, remote: string, applied: number, local: string, selection: { readonly anchor: number; readonly caret: number }): { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 } {
  const unapplied = applied < host.acknowledged ? [] : host.unapplied.filter((entry) => entry.seq > applied);
  return textEditorSpliceShowV1({ ...host, seq: Math.max(host.seq, applied), acknowledged: applied }, remote, unapplied, local, selection, textSpliceFromEditV1(host.base, local));
}

/** @emoji ✅️ The round trip of splice `seq` settled APPLIED — the window's typed operation completed, so the window holds it and
 * every splice this host sent before it, whether or not a scene said so yet: they leave the unapplied set, so no later scene (a
 * reincarnated window's included, which may carry them from the hub) folds them onto a text that already holds them. */
export function settleTextEditorSpliceV1(host: TextEditorSpliceHostV1, seq: number): TextEditorSpliceHostV1 {
  return seq > 0 ? host : host;
}

/** @emoji 🚫️ The window refused splice `seq`: it leaves the unapplied set and the editor shows the last published text with the
 * remaining unapplied splices — the refused run, and what was typed after it and not sent yet, disappear instead of looking
 * saved; the caret collapses where the refused run was (located by the run's own context, which is still there). A refusal of a
 * run the host no longer holds (dropped when its window reincarnated) changes nothing. */
export function refuseTextEditorSpliceV1(host: TextEditorSpliceHostV1, seq: number, local: string, selection: { readonly anchor: number; readonly caret: number }): { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 } {
  const refused = host.unapplied.find((entry) => entry.seq === seq);
  if (refused === undefined) return { host, show: null };
  const shown = textEditorSpliceShowV1(host, host.remote, host.unapplied.filter((entry) => entry.seq !== seq), local, selection, null);
  if (shown.show === null) return shown;
  const at = locateTextSpliceV1(shown.show.text, { ...refused.splice, deleted: "", insert: "" }).start;
  return { host: shown.host, show: { text: shown.show.text, anchor: at, caret: at } };
}
//#endregion ⌨️TextEditorTyping
