/** 🧪️ W3-T2-TEXT: writes the `compositions` and `typingRuns` vectors of the text-splice fixture. Every expected splice is
 * derived independently of the composition under test: `textSpliceFromEditV1` of the run's first and last text (the
 * canonical single splice), with the run boundaries and the disjoint/cancelled verdicts declared by hand below.
 * Run: `bun T/🧪️w3-t2-text-splice-compositions.ts` from the repo root. */
import { readFileSync, writeFileSync } from "node:fs";
import { textSpliceFromEditV1 } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🟦️.ts";

const path = "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/✂️text-splice/🔣️.json";
const fixture = JSON.parse(readFileSync(path, "utf8"));
const splice = (previous: string, next: string) => {
  const derived = textSpliceFromEditV1(previous, next);
  if (derived === null) throw new Error(`no change ${JSON.stringify(previous)} → ${JSON.stringify(next)}`);
  return derived;
};
const long = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMN";

const pairs: [string, string, [string, string, string], "composed" | "cancelled" | "disjoint"][] = [
  ["forward-typing", "the next scalar typed after the run extends it", ["Hello", "Hello ", "Hello w"], "composed"],
  ["backspace-inside-the-run", "erasing a typed scalar shortens the run", ["ab", "abXY", "abX"], "composed"],
  ["backspace-past-the-run-start", "erasing before the run joins the run and deletes the original scalar", ["abc", "abcX", "abX"], "composed"],
  ["delete-forward-after-the-run", "a forward delete next to the run joins it", ["abc def", "abcX def", "abcXef"], "composed"],
  ["replace-a-selection-inside-the-run", "a selection over typed text is replaced inside the run", ["abc", "abXYc", "abZc"], "composed"],
  ["repeated-letters", "repeated letters stay one canonical run", ["aa", "aaa", "aaaa"], "composed"],
  ["astral-run", "positions count scalars, not UTF-16 units", ["a😀b", "a😀Zb", "a😀ZZb"], "composed"],
  ["newline-in-a-run", "Enter inside prose is just another typed scalar", ["line1\nline2", "line1\n\nline2", "line1\nX\nline2"], "composed"],
  ["long-context", "the composed context is the full 32 scalars of the original text", [long + long, long + "XYZ" + long, long + "XZ" + long], "composed"],
  ["type-then-erase", "a run that erased everything it typed changed nothing", ["ab", "abX", "ab"], "cancelled"],
  ["caret-jump-after", "an edit away from the run starts a new run", ["one two three", "one! two three", "one! two three?"], "disjoint"],
  ["caret-jump-before", "an edit before the run's context starts a new run", [long + "end", long + "end.", "!" + long + "end."], "disjoint"],
  ["edit-inside-context-not-touching", "an edit inside the context that does not touch the inserted run starts a new run", ["abcdef", "abcdXef", "aZbcdXef"], "disjoint"],
];

/** ✍️ Runs whose union is NOT the canonical splice, computed by hand: the run deleted the original "\n", typed "a", the "\n"
 * again and then "a" — the retyped "\n" stays inside the run, so the last "a" still touches it. */
const unions = [
  {
    id: "retyped-scalar-stays-in-the-run",
    note: "a scalar the run deleted and typed again stays inside the run, so typing on at the caret still joins it",
    text: "\n",
    net: { start: 0, deleted: "\n", insert: "a\n", before: "", after: "" },
    next: splice("a\n", "a\na"),
    composition: { kind: "composed", splice: { start: 0, deleted: "\n", insert: "a\na", before: "", after: "" } },
  },
  {
    id: "caret-left-of-a-repeated-scalar",
    note: "the canonical splice sits right of repeated scalars while the caret sits left of them; the run still joins",
    text: "mission",
    net: splice("mission", "misssion"),
    next: splice("misssion", "misxssion"),
    composition: { kind: "composed", splice: splice("mission", "misxssion") },
  },
];

fixture.compositions = [
  ...pairs.map(([id, note, [t0, t1, t2], kind]) => ({
    id,
    note,
    text: t0,
    net: splice(t0, t1),
    next: splice(t1, t2),
    composition: kind === "composed" ? { kind, splice: splice(t0, t2) } : { kind },
  })),
  ...unions,
];

const runs: [string, string, string[], number[]][] = [
  ["one-run-with-corrections", "typos fixed while typing stay one run", ["", "T", "Th", "Teh", "Te", "T", "Th", "The", "The ", "The c", "The ca", "The cat"], []],
  ["caret-jump-splits", "a caret jump to the start of the text ends the first run", ["Dear team,", "Dear team, ", "Dear team, h", "Dear team, hi", "Hi Dear team, hi", "Hi, Dear team, hi"], [3]],
  ["type-and-erase-leaves-no-run", "typing and erasing the same scalars leaves no run", ["abc", "abcd", "abcde", "abcd", "abc"], []],
  ["two-sentences-far-apart", "edits at both ends of a long text are two runs", [long + long, "A " + long + long, "A " + long + long + " Z"], [1]],
  ["mission-typed-at-the-caret", "typing left of repeated scalars stays one run", ["mission", "misssion", "misxssion", "misxyssion"], []],
];

fixture.typingRuns = runs.map(([id, note, texts, splits]) => {
  const bounds = [0, ...splits, texts.length - 1];
  const expected = [];
  for (let index = 0; index + 1 < bounds.length; index += 1) {
    const derived = textSpliceFromEditV1(texts[bounds[index]!]!, texts[bounds[index + 1]!]!);
    if (derived !== null) expected.push(derived);
  }
  return { id, note, texts, runs: expected };
});

writeFileSync(path, JSON.stringify(fixture, null, 2) + "\n");
console.log(`compositions ${fixture.compositions.length}, typingRuns ${fixture.typingRuns.length}`);
