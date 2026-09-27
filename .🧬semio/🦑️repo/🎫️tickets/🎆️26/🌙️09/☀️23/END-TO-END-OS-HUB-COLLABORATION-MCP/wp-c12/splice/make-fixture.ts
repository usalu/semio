/** 🧪️ C12: builds the `semio.ui.scene.text-splice.v1` vectors with the TS reference and prints every outcome for hand review
 * before the fixture is copied into the tree (window 3). Concurrent vectors model the hub: every keystroke commits in the
 * listed order; an author sees only its own keystrokes until a `sync` rebuilds its view from the committed fold.
 * usage: bun make-fixture.ts > review.txt */
import { applyTextSpliceV1, locateTextSpliceV1, rebaseTextEditsV1, scalarOfUtf8OffsetV1, textSpliceFromEditV1, type TextSpliceV1 } from "./text-splice.ts";

type Step = { readonly author: string; readonly type?: string; readonly at?: number; readonly back?: number; readonly select?: readonly [number, number] } | { readonly sync: string };
type Scenario = { readonly id: string; readonly note: string; readonly base: string; readonly steps: readonly Step[] };

const scenarios: Scenario[] = [
  { id: "one-author-types-a-word", note: "a typing run is one splice per keystroke; the fold is the typed text", base: "", steps: [{ author: "a", type: "hello", at: 0 }] },
  { id: "two-authors-type-at-the-end-at-once", note: "both runs anchor after what their author saw; each run stays contiguous", base: "Doc: ", steps: [{ author: "a", type: "al", at: 5 }, { author: "b", type: "be", at: 5 }, { author: "a", type: "pha", at: 7 }, { author: "b", type: "ta", at: 7 }] },
  { id: "two-authors-type-in-different-places", note: "an insert anchored by its context lands where its author put it whatever the other did", base: "hello world, see you", steps: [{ author: "a", type: "big ", at: 6 }, { author: "b", type: "!!", at: 11 }] },
  { id: "both-delete-the-same-word", note: "the second deletion of an already deleted run deletes nothing and is clamped", base: "keep drop keep", steps: [{ author: "a", select: [5, 10], type: "" }, { author: "b", select: [5, 10], type: "" }] },
  { id: "a-deletion-around-a-concurrent-insert", note: "a deletion whose run another author changed deletes nothing (clamped); the insert survives", base: "one two three four", steps: [{ author: "a", type: "X", at: 6 }, { author: "b", select: [4, 8], type: "" }] },
  { id: "astral-scalars-count-once", note: "positions are Unicode scalars: the emoji is one position in every language", base: "a😀b c", steps: [{ author: "a", type: "Z", at: 2 }, { author: "b", type: "Y", at: 1 }] },
  { id: "typing-after-seeing-the-other", note: "after a sync the author's context includes the other's run and its run follows it", base: "Title\n", steps: [{ author: "a", type: "first", at: 6 }, { sync: "b" }, { author: "b", type: " second", at: 11 }, { author: "a", type: "!", at: 11 }] },
  { id: "a-typo-corrected-while-the-other-types-at-the-same-point", note: "an author's backspace removes only its own keystroke even when the other typed at the same point", base: "x = ", steps: [{ author: "a", type: "12", at: 4 }, { author: "b", type: "ab", at: 4 }, { author: "a", back: 1 }, { author: "a", type: "3" }] },
  { id: "a-replaced-word-while-the-other-types-inside-it", note: "replacing a word the other just changed deletes nothing (clamped) and inserts; both authors' text survives", base: "the quick fox", steps: [{ author: "b", type: "k", at: 9 }, { author: "a", select: [4, 9], type: "slow" }] },
  { id: "repeated-context-picks-the-nearest", note: "in repetitive text the match nearest to the author's position wins", base: "ab ab ab ab ab ab", steps: [{ author: "a", type: "X", at: 9 }, { author: "b", type: "Y", at: 3 }] },
];

const concurrent = scenarios.map((scenario) => {
  const committed: { author: string; splice: TextSpliceV1 }[] = [];
  const views = new Map<string, { text: string; caret: number }>();
  const fold = () => committed.reduce((text, row) => applyTextSpliceV1(text, row.splice).text, scenario.base);
  for (const step of scenario.steps) {
    if ("sync" in step) {
      const text = fold();
      const view = views.get(step.sync) ?? { text: scenario.base, caret: 0 };
      views.set(step.sync, { text, caret: Array.from(text).length });
      void view;
      continue;
    }
    const view = views.get(step.author) ?? { text: scenario.base, caret: 0 };
    let caret = step.at ?? view.caret;
    let text = view.text;
    const commit = (next: string) => {
      const splice = textSpliceFromEditV1(text, next);
      if (splice) committed.push({ author: step.author, splice });
      text = next;
    };
    if (step.select) {
      const chars = Array.from(text);
      commit([...chars.slice(0, step.select[0]), ...chars.slice(step.select[1])].join(""));
      caret = step.select[0];
    }
    for (let index = 0; index < (step.back ?? 0); index += 1) {
      const chars = Array.from(text);
      commit([...chars.slice(0, caret - 1), ...chars.slice(caret)].join(""));
      caret -= 1;
    }
    for (const char of Array.from(step.type ?? "")) {
      const chars = Array.from(text);
      commit([...chars.slice(0, caret), char, ...chars.slice(caret)].join(""));
      caret += 1;
    }
    views.set(step.author, { text, caret });
  }
  let text = scenario.base;
  const clamped: number[] = [];
  for (const [index, row] of committed.entries()) {
    const applied = applyTextSpliceV1(text, row.splice);
    if (applied.located.clamped) clamped.push(index);
    text = applied.text;
  }
  const runs = Object.fromEntries([...views.entries()].map(([author, view]) => [author, view.text]));
  return { id: scenario.id, note: scenario.note, base: scenario.base, order: committed, expected: { text, clamped, runs } };
});

const edits = ([
  ["insert-into-empty", "", "a"],
  ["replace-a-middle-run", "abcdef", "abXYef"],
  ["delete-a-suffix", "hello", "help"],
  ["astral-insert", "a😀b", "a😀Zb"],
  ["unchanged", "same", "same"],
  ["delete-everything", "xyz", ""],
  ["repeated-letter-inserts-after-the-run", "aa", "aaa"],
  ["context-is-capped-at-32", `${"p".repeat(40)}|${"s".repeat(40)}`, `${"p".repeat(40)}+|${"s".repeat(40)}`],
] as const).map(([id, previous, next]) => ({ id, previous, next, splice: textSpliceFromEditV1(previous, next) }));

const application = (id: string, note: string, text: string, splice: TextSpliceV1) => {
  const applied = applyTextSpliceV1(text, splice);
  return { id, note, text, splice, located: applied.located, result: applied.text, inverse: applied.inverse };
};
const applications = [
  application("in-place", "the text around the splice is unchanged: it lands at its start", "hello world", { start: 6, deleted: "", insert: "big ", before: "hello ", after: "world" }),
  application("shifted-by-an-earlier-insert", "text inserted before the splice moves it; the context finds it", "oh, hello world", { start: 6, deleted: "", insert: "big ", before: "hello ", after: "world" }),
  application("deleted-run-moved", "a deletion follows its run", "XXhello world", { start: 0, deleted: "hello", insert: "", before: "", after: " world" }),
  application("deleted-run-gone", "a deletion whose run is gone deletes nothing", "hi world", { start: 0, deleted: "hello", insert: "", before: "", after: " world" }),
  application("one-sided-context", "an insert whose two-sided context was split by another insert lands after its left context", "hello big world", { start: 6, deleted: "", insert: "new ", before: "hello ", after: "world" }),
  application("no-context-anywhere", "nothing of the context survives: the author's start, clamped to the text", "zzz", { start: 9, deleted: "", insert: "!", before: "abc", after: "def" }),
  application("empty-text", "a splice into an empty text lands at 0", "", { start: 0, deleted: "", insert: "a", before: "", after: "" }),
  application("astral-context", "context and positions count scalars", "😀😀x😀", { start: 3, deleted: "", insert: "y", before: "😀😀x", after: "😀" }),
];

const rebaseVector = (id: string, note: string, remote: string, unapplied: TextSpliceV1[], local: string, selection: { anchor: number; caret: number }) => ({ id, note, remote, unapplied, local, selection, expected: rebaseTextEditsV1(remote, unapplied, local, selection) });
const localTyped = "Hello there";
const typedSplice = textSpliceFromEditV1("Hello", localTyped)!;
const rebases = [
  rebaseVector("nothing-pending", "no local splice pending: the remote text, the caret keeps its context", "Oh. Hello", [], "Hello", { anchor: 5, caret: 5 }),
  rebaseVector("pending-typing-over-a-remote-prefix", "the unapplied local run folds onto the remote text; the caret follows it", "Oh. Hello", [typedSplice], localTyped, { anchor: 11, caret: 11 }),
  rebaseVector("pending-typing-at-the-same-point", "another author typed at the same point: the local run still ends at the caret", "Hello friend", [typedSplice], localTyped, { anchor: 11, caret: 11 }),
  rebaseVector("a-selection-travels", "both selection ends keep their context", "## Hello", [], "Hello", { anchor: 0, caret: 5 }),
];

const offsets = ([["a😀b", 0], ["a😀b", 1], ["a😀b", 5], ["a😀b", 6], ["äöü", 4]] as const).map(([text, utf8]) => ({ text, utf8, scalar: scalarOfUtf8OffsetV1(text, utf8) }));

const fixture = { schema: "semio.ui.scene.text-splice.v1", contextScalars: 32, minTwoSidedScalars: 4, edits, applications, concurrent, rebases, offsets };
await Bun.write(new URL("./fixture.json", import.meta.url), `${JSON.stringify(fixture, null, 2)}\n`);
for (const row of concurrent) console.log(`concurrent ${row.id}: ${JSON.stringify(row.base)} → ${JSON.stringify(row.expected.text)} clamped=${JSON.stringify(row.expected.clamped)} views=${JSON.stringify(row.expected.runs)}`);
for (const row of applications) console.log(`apply ${row.id}: ${JSON.stringify(row.text)} → ${JSON.stringify(row.result)} ${JSON.stringify(row.located)}`);
for (const row of rebases) console.log(`rebase ${row.id}: ${JSON.stringify(row.remote)} → ${JSON.stringify(row.expected)}`);
for (const row of edits) console.log(`edit ${row.id}: ${JSON.stringify(row.splice)}`);
console.log(`offsets ${JSON.stringify(offsets)}`);
void locateTextSpliceV1;
