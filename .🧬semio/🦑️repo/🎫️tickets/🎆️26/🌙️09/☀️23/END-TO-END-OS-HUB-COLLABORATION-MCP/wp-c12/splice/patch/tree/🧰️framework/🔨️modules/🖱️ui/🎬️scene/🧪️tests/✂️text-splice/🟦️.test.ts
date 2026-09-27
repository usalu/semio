/** 🧪️ Laws of the range-text operation (`semio.ui.scene.text-splice.v1`): the schema admits the language-neutral fixture, the TS
 * reference replays every vector exactly (edit → splice, application + inverse, concurrent folds in hub order, host rebase,
 * UTF-8 ↔ scalar offsets), jsdiff (third party) derives the same changed run for every edit, and no concurrent insert-only
 * workload loses a typed scalar. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { diffChars } from "diff";
import fixture from "../../🧫️fixtures/✂️text-splice/🔣️.json";
import schema from "../../🧬️schema/✂️text-splice/🔣️.json";
import { applyTextSpliceV1, locateTextSpliceV1, rebaseTextEditsV1, scalarOfUtf8OffsetV1, TEXT_SPLICE_CONTEXT_SCALARS, TEXT_SPLICE_MIN_TWO_SIDED_SCALARS, textEditorAppliedSpliceV1, textEditorTypingV1, textSpliceFromEditV1, utf8OffsetOfScalarV1, type TextSpliceV1 } from "../../✂️text-splice/🟦️.ts";

describe("text splice", () => {
  test("the schema admits the fixture and pins the constants", () => {
    const validate = new Ajv().compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.contextScalars).toBe(TEXT_SPLICE_CONTEXT_SCALARS);
    expect(fixture.minTwoSidedScalars).toBe(TEXT_SPLICE_MIN_TWO_SIDED_SCALARS);
  });

  for (const row of fixture.edits) {
    test(`edit ${row.id}: the splice and jsdiff agree on the one changed run`, () => {
      const splice = textSpliceFromEditV1(row.previous, row.next);
      expect(splice).toEqual(row.splice);
      if (splice === null) return expect(row.previous).toBe(row.next);
      const parts = diffChars(row.previous, row.next);
      expect(parts.filter((part) => part.removed).map((part) => part.value).join("")).toBe(splice.deleted);
      expect(parts.filter((part) => part.added).map((part) => part.value).join("")).toBe(splice.insert);
      expect(applyTextSpliceV1(row.previous, splice).text).toBe(row.next);
    });
  }

  for (const row of fixture.applications) {
    test(`apply ${row.id}: lands, results and inverts exactly`, () => {
      const applied = applyTextSpliceV1(row.text, row.splice);
      expect(locateTextSpliceV1(row.text, row.splice)).toEqual(row.located);
      expect(applied.text).toBe(row.result);
      expect(applied.inverse).toEqual(row.inverse);
      if (!row.located.clamped) expect(applyTextSpliceV1(applied.text, applied.inverse).text).toBe(row.text);
    });
  }

  for (const row of fixture.concurrent) {
    test(`concurrent ${row.id}: the hub-order fold is exact and every author's surviving run stays`, () => {
      let text = row.base;
      const clamped: number[] = [];
      for (const [index, entry] of row.order.entries()) {
        const applied = applyTextSpliceV1(text, entry.splice as TextSpliceV1);
        if (applied.located.clamped) clamped.push(index);
        const undone = applyTextSpliceV1(applied.text, applied.inverse);
        if (!applied.located.clamped) expect(undone.text, `${row.id} #${index} inverse`).toBe(text);
        text = applied.text;
      }
      expect(text).toBe(row.expected.text);
      expect(clamped).toEqual(row.expected.clamped);
      for (const author of new Set(row.order.map((entry) => entry.author))) {
        const inserted = row.order.filter((entry) => entry.author === author).map((entry) => entry.splice.insert).join("");
        for (const scalar of new Set(Array.from(inserted))) expect(Array.from(text).filter((char) => char === scalar).length, `${author}'s ${JSON.stringify(scalar)}`).toBeGreaterThanOrEqual(Array.from(inserted).filter((char) => char === scalar).length - row.order.map((entry) => Array.from(entry.splice.deleted).filter((char) => char === scalar).length).reduce((sum, count) => sum + count, 0));
      }
    });
  }

  for (const row of fixture.rebases) {
    test(`rebase ${row.id}: the host view folds its unapplied splices onto the guest's text`, () => {
      expect(rebaseTextEditsV1(row.remote, row.unapplied, row.local, row.selection)).toEqual(row.expected);
    });
  }

  test("a scene declares splice typing and echoes the applied splice seq", () => {
    const validate = new Ajv().addSchema(schema, "text-splice");
    expect(validate.validate({ $ref: "text-splice#/definitions/typing" }, { mode: "splice" })).toBe(true);
    expect(validate.validate({ $ref: "text-splice#/definitions/selectionEcho" }, { start: 3, end: 3, splice: 7 })).toBe(true);
    expect(textEditorTypingV1(JSON.stringify({ fontPx: 13, typing: { mode: "splice" } }))).toEqual({ mode: "splice" });
    expect(textEditorTypingV1(JSON.stringify({ readOnly: true }))).toBeNull();
    expect(textEditorTypingV1("not json")).toBeNull();
    expect(textEditorAppliedSpliceV1(JSON.stringify({ start: 1, end: 2, splice: 7 }))).toBe(7);
    expect(textEditorAppliedSpliceV1(JSON.stringify({ start: 1, end: 2 }))).toBe(0);
  });

  test("UTF-8 offsets and scalar indices convert both ways", () => {
    for (const row of fixture.offsets) {
      expect(scalarOfUtf8OffsetV1(row.text, row.utf8)).toBe(row.scalar);
      if (new TextEncoder().encode(Array.from(row.text).slice(0, row.scalar).join("")).length === row.utf8) expect(utf8OffsetOfScalarV1(row.text, row.scalar)).toBe(row.utf8);
    }
  });

  test("no concurrent insert-only typing loses a scalar (seeded, 400 workloads)", () => {
    let seed = 0x5eed_c12;
    const random = () => ((seed = (Math.imul(seed ^ (seed >>> 15), 0x2c1b3c6d) + 0x6d2b79f5) >>> 0) / 2 ** 32);
    const alphabet = Array.from("ab ab😀\nxyz");
    for (let workload = 0; workload < 400; workload += 1) {
      const base = Array.from({ length: Math.floor(random() * 24) }, () => alphabet[Math.floor(random() * alphabet.length)]).join("");
      const views = [{ text: base, caret: Math.floor(random() * (Array.from(base).length + 1)) }, { text: base, caret: Math.floor(random() * (Array.from(base).length + 1)) }];
      const committed: TextSpliceV1[] = [];
      const typed = [0, 0];
      for (let key = 0; key < 24; key += 1) {
        const author = random() < 0.5 ? 0 : 1;
        const view = views[author]!;
        const chars = Array.from(view.text);
        const next = [...chars.slice(0, view.caret), author === 0 ? "A" : "B", ...chars.slice(view.caret)].join("");
        committed.push(textSpliceFromEditV1(view.text, next)!);
        views[author] = { text: next, caret: view.caret + 1 };
        typed[author]! += 1;
        if (random() < 0.15) {
          const synced = committed.reduce((text, splice) => applyTextSpliceV1(text, splice).text, base);
          views[author] = { text: synced, caret: locateTextSpliceV1(synced, { start: views[author]!.caret, deleted: "", insert: "", before: Array.from(next).slice(Math.max(0, views[author]!.caret - 32), views[author]!.caret).join(""), after: Array.from(next).slice(views[author]!.caret, views[author]!.caret + 32).join("") }).start };
        }
      }
      const folded = Array.from(committed.reduce((text, splice) => applyTextSpliceV1(text, splice).text, base));
      expect(folded.filter((char) => char === "A").length, `workload ${workload}`).toBe(typed[0]);
      expect(folded.filter((char) => char === "B").length, `workload ${workload}`).toBe(typed[1]);
      expect(folded.length).toBe(Array.from(base).length + typed[0]! + typed[1]!);
    }
  });
});
