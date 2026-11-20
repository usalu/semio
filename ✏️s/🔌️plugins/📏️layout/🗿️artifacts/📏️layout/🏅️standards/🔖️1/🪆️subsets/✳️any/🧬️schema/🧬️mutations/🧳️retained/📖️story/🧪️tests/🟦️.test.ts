import { expect, test } from "bun:test";
import { applyPatch, type Operation } from "fast-json-patch";
import { spliceStory } from "../../../../✂️story-splice/🟦️.ts";
import fixture from "./🔣️.json";

for (const row of fixture.cases) test(row.name, () => {
  const { offset, delete: count, insert } = row.operation;
  const story = { ...row.before, styleRuns: row.before.styleRuns.map(run => ({ ...run, start: BigInt(run.start), end: BigInt(run.end) })) };
  const after = spliceStory(story, offset, count, insert)!;
  const expectedStyleRows = row.after.styleRuns.map(run => ({ ...run, start: BigInt(run.start), end: BigInt(run.end) }));
  expect(after).toEqual({ ...row.after, styleRuns: expectedStyleRows });
  const characters = Array.from(row.before.content);
  const edits: Operation[] = Array.from({ length: count }, () => ({ op: "remove", path: "/" + offset }));
  edits.push(...Array.from(insert, (value, index): Operation => ({ op: "add", path: "/" + (offset + index), value })));
  const patched = applyPatch(characters, edits, true, false).newDocument;
  expect(patched.join("")).toEqual(row.after.content);
  const undo: Operation[] = Array.from({ length: row.inverse.delete }, () => ({ op: "remove", path: "/" + row.inverse.offset }));
  undo.push(...Array.from(row.inverse.insert, (value, index): Operation => ({ op: "add", path: "/" + (row.inverse.offset + index), value })));
  expect(applyPatch(patched, undo, true, false).newDocument.join("")).toEqual(row.before.content);
});
