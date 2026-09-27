/** 🧪️ C11: builds the `semio.text.concurrent-splices/v1` fixture vectors — each author types into its OWN view (seeing only its
 * own keystrokes), one splice per keystroke via `textSpliceFromEditV1`; the hub orders all splices by the listed HLC; the
 * expected text is the fold in that order. Printed for hand review before it becomes the fixture. */
import { applyTextSpliceV1, textSpliceFromEditV1, type TextSpliceV1 } from "./reference.ts";
type Typing = { readonly author: string; readonly at: number; readonly insert?: string; readonly deleteRange?: readonly [number, number]; readonly hlc: readonly number[] };
type Vector = { readonly id: string; readonly base: string; readonly typings: readonly Typing[]; readonly note: string };
const vectors: Vector[] = [
  { id: "one-author-types-a-word", base: "", note: "a typing run is one splice per keystroke; the fold is the typed text", typings: [{ author: "a", at: 0, insert: "hello", hlc: [1, 2, 3, 4, 5] }] },
  { id: "two-authors-type-at-the-end-at-once", base: "Doc: ", note: "both runs anchor after what their author saw (\"Doc: \"); each run stays contiguous; the later-ordered run lands first after the shared context", typings: [{ author: "a", at: 5, insert: "alpha", hlc: [1, 3, 5, 7, 9] }, { author: "b", at: 5, insert: "beta", hlc: [2, 4, 6, 8] }] },
  { id: "two-authors-type-in-different-places", base: "hello world", note: "an insert anchored by its context lands where its author put it whatever the other did", typings: [{ author: "a", at: 6, insert: "big ", hlc: [2, 4, 6, 8] }, { author: "b", at: 11, insert: "!!", hlc: [1, 3] }] },
  { id: "both-delete-the-same-word", base: "keep drop keep", note: "the second deletion of an already deleted run deletes nothing and is clamped", typings: [{ author: "a", at: 5, deleteRange: [5, 10], hlc: [1] }, { author: "b", at: 5, deleteRange: [5, 10], hlc: [2] }] },
  { id: "delete-around-a-concurrent-insert", base: "abcdef", note: "a deletion whose run another author split keeps the inserted text (nothing is deleted, clamped)", typings: [{ author: "a", at: 3, insert: "X", hlc: [1] }, { author: "b", at: 2, deleteRange: [2, 5], hlc: [2] }] },
  { id: "astral-scalars-count-once", base: "a😀b", note: "positions are Unicode scalars: the emoji is one position in every language", typings: [{ author: "a", at: 2, insert: "Z", hlc: [1] }, { author: "b", at: 1, insert: "Y", hlc: [2] }] },
];
const out = [];
for (const vector of vectors) {
  const splices: { author: string; hlc: number; splice: TextSpliceV1 }[] = [];
  for (const typing of vector.typings) {
    let view = vector.base;
    if (typing.deleteRange) {
      const chars = Array.from(view);
      const next = [...chars.slice(0, typing.deleteRange[0]), ...chars.slice(typing.deleteRange[1])].join("");
      splices.push({ author: typing.author, hlc: typing.hlc[0]!, splice: textSpliceFromEditV1(view, next)! });
      view = next;
    }
    let caret = typing.at;
    for (const [index, char] of Array.from(typing.insert ?? "").entries()) {
      const chars = Array.from(view);
      const next = [...chars.slice(0, caret), char, ...chars.slice(caret)].join("");
      splices.push({ author: typing.author, hlc: typing.hlc[index]!, splice: textSpliceFromEditV1(view, next)! });
      view = next;
      caret += 1;
    }
  }
  splices.sort((left, right) => left.hlc - right.hlc);
  let text = vector.base;
  const clamped: number[] = [];
  for (const [index, row] of splices.entries()) {
    const applied = applyTextSpliceV1(text, row.splice);
    text = applied.text;
    if (applied.clamped) clamped.push(index);
  }
  out.push({ id: vector.id, note: vector.note, base: vector.base, splices, expected: { text, clamped } });
  console.log(`${vector.id}: ${JSON.stringify(vector.base)} → ${JSON.stringify(text)} clamped=${JSON.stringify(clamped)}`);
}
await Bun.write(new URL("./fixture-draft.json", import.meta.url), JSON.stringify({ schema: "semio.text.concurrent-splices/v1", contextScalars: 32, vectors: out }, null, 2) + "\n");
