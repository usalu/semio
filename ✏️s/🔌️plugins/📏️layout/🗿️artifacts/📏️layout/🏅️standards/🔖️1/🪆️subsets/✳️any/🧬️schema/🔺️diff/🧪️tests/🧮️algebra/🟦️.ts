/** 🧪️ Language-neutral Layout events agree with independent RFC 6902 and generative oracles. */
import { deepStrictEqual, strictEqual, throws } from "node:assert";
import { readFileSync, readdirSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import jsonPatch from "fast-json-patch";
import type { Operation } from "fast-json-patch";
import fc from "fast-check";
import corpus from "../../../../🧪️tests/🧫️fixtures/🧮️diff-algebra/🔣️.json" with { type: "json" };
import refusals from "../../../../🧪️tests/🧫️fixtures/🧮️diff-algebra/refusals.json" with { type: "json" };
import structure from "../../../../🧪️tests/🧫️fixtures/🧬️structure/🔣️.json" with { type: "json" };
import { layoutArtifactFromNativeJson, layoutArtifactNativeJson } from "../../../../🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts";
import { layoutDiffFromNativeJson, layoutDiffNativeJson } from "../../../../🚪️io/📝️text/🔺️diff/🪪️native-json/🟦️.ts";
import { applyLayoutDiff, composeLayoutDiffs, diffLayoutSnapshots, inverseLayoutDiff, LayoutDiffError, replayLayoutDiffs } from "../../🧮️diff-apply-and-algebra/🟦️.ts";
import type { LayoutDiff } from "../../🟦️.ts";

/** 🔬️ The test-only RFC 6902 interface keeps the external library out of the runtime API. */
function oracle(before: unknown, patch: unknown): unknown { return jsonPatch.applyPatch(structuredClone(before), patch as Operation[], true, false).newDocument; }

/** ✅️ Runs the shared semantic vectors, malformed-event refusals and randomized inverse/replay laws. */
export function testLayoutDiffAlgebraOracle(): { portableCases: number; goldenCases: number; generatedCases: number; refusalCases: number } {
  for (const fixture of corpus.cases) {
    deepStrictEqual(oracle(fixture.before, fixture.patch), fixture.after, `${fixture.name}: independent oracle`);
    const base = layoutArtifactFromNativeJson(fixture.before), saved = structuredClone(base), diff = layoutDiffFromNativeJson(fixture.diff);
    const after = applyLayoutDiff(base, diff);
    deepStrictEqual(layoutArtifactNativeJson(after), fixture.after, `${fixture.name}: central applier`);
    deepStrictEqual(base, saved, `${fixture.name}: input is immutable`);
    const inverse = inverseLayoutDiff(base, diff);
    deepStrictEqual(applyLayoutDiff(after, inverse), base, `${fixture.name}: inverse`);
    deepStrictEqual(applyLayoutDiff(after, layoutDiffFromNativeJson(layoutDiffNativeJson(inverse))), base, `${fixture.name}: sparse inverse transport`);
    deepStrictEqual(replayLayoutDiffs(base, [diff, inverse]), base, `${fixture.name}: event replay`);
    deepStrictEqual(applyLayoutDiff(base, composeLayoutDiffs(base, [diff, inverse])), base, `${fixture.name}: composition`);
  }
  for (const fixture of structure.cases) {
    deepStrictEqual(oracle(fixture.before, fixture.patch), fixture.after, `${fixture.name}: structure oracle`);
    const before = layoutArtifactFromNativeJson(fixture.before), after = layoutArtifactFromNativeJson(fixture.after), diff = diffLayoutSnapshots(before, after);
    deepStrictEqual(applyLayoutDiff(before, diff), after, `${fixture.name}: structural diff`);
    deepStrictEqual(applyLayoutDiff(after, inverseLayoutDiff(before, diff)), before, `${fixture.name}: structural inverse`);
  }
  const goldenRoot = resolve(process.cwd(), "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations");
  const goldenFiles = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.isDirectory() ? goldenFiles(join(directory, entry.name)) : entry.name === "🔣️.json" && basename(directory) === "🔺️diff" ? [join(directory, entry.name)] : []);
  const goldens = goldenFiles(goldenRoot);
  for (const file of goldens) {
    const root = dirname(dirname(file)), read = (...parts: string[]): unknown => JSON.parse(readFileSync(join(root, ...parts), "utf8"));
    const before = layoutArtifactFromNativeJson(read("📸️snapshot", "⬅️before", "🔣️.json")), after = layoutArtifactFromNativeJson(read("📸️snapshot", "➡️after", "🔣️.json")), diff = layoutDiffFromNativeJson(read("🔺️diff", "🔣️.json"));
    deepStrictEqual(applyLayoutDiff(before, diff), after, `${root}: golden diff`);
    deepStrictEqual(applyLayoutDiff(after, inverseLayoutDiff(before, diff)), before, `${root}: golden inverse`);
  }
  const base = layoutArtifactFromNativeJson(corpus.cases[0].before), noop = diffLayoutSnapshots(base, base);
  const events: LayoutDiff[] = [
    { ...noop, stories: { removed: [{ id: "story-1", index: 1 }], inserted: [], moved: [], modified: [] } },
    { ...noop, stories: { removed: [{ id: "story-1", index: 0 }, { id: "story-1", index: 0 }], inserted: [], moved: [], modified: [] } },
    { ...noop, stories: { removed: [], inserted: [], moved: [{ id: "story-1", from: 0, to: 9 }], modified: [] } },
    { ...noop, stories: { removed: [], inserted: [{ index: 0, row: base.stories[0] }], moved: [], modified: [] } },
    { ...noop, stories: { removed: [], inserted: [], moved: [], modified: [{ id: "absent", patch: { content: "x" } }] } },
    { ...noop, stories: { removed: [], inserted: [], moved: [], modified: [{ id: "story-1", patch: { content: null, style_runs: { len: 2, rows: [] } } }] } },
    { ...noop, dataFields: { entries: { removed: [{ id: "missing", index: 0 }], inserted: [], moved: [] } } },
  ];
  for (const event of events) {
    const saved = structuredClone(base);
    throws(() => applyLayoutDiff(base, event), LayoutDiffError);
    deepStrictEqual(base, saved, "rejected events preserve the canonical base");
  }
  for (const fixture of refusals.cases) {
    const before = layoutArtifactFromNativeJson(fixture.before), saved = structuredClone(before);
    throws(() => applyLayoutDiff(before, layoutDiffFromNativeJson(fixture.diff)), LayoutDiffError, fixture.name);
    deepStrictEqual(before, saved);
  }
  const stories = fc.uniqueArray(fc.record({ id: fc.string({ minLength: 1, maxLength: 12 }), content: fc.string({ maxLength: 80 }) }), { selector: row => row.id, maxLength: 20 });
  fc.assert(fc.property(stories, stories, (first, second) => {
    const before = { ...base, stories: first.map(row => ({ ...row, styleRuns: [] })) }, after = { ...base, stories: second.map(row => ({ ...row, styleRuns: [] })) };
    const diff = diffLayoutSnapshots(before, after);
    deepStrictEqual(applyLayoutDiff(before, diff), after);
    deepStrictEqual(applyLayoutDiff(after, inverseLayoutDiff(before, diff)), before);
  }), { numRuns: 250, seed: 2142026 });
  fc.assert(fc.property(fc.bigInt({ min: 0n, max: 0xffffffffffffffffn }), bits => {
    const before = { ...base, grid: { ...base.grid, baselineGrid: { bits } } }, change = { ...noop, grid: { baselineGrid: { bits: bits ^ 1n } } };
    strictEqual(applyLayoutDiff(applyLayoutDiff(before, change), inverseLayoutDiff(before, change)).grid.baselineGrid.bits, bits);
  }), { numRuns: 250, seed: 2142027 });
  return { portableCases: corpus.cases.length + structure.cases.length, goldenCases: goldens.length, generatedCases: 500, refusalCases: events.length + refusals.cases.length };
}
