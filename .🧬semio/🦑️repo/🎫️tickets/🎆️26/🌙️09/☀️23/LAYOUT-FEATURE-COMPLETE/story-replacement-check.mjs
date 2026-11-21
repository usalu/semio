import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { takeWhile, zip } from "lodash-es";

const root = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✂️story-splice";
const { storyReplacement } = await import(resolve(root, "🟦️.ts"));
const fixture = JSON.parse(readFileSync(resolve(root, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
for (const { before, after, expected } of fixture.cases) {
  const actual = storyReplacement(before, after);
  assert.deepEqual(actual, expected);
  const [left, right] = [Array.from(before), Array.from(after)];
  const same = ([a, b]) => a === b;
  const prefix = takeWhile(zip(left, right), same).length;
  const suffix = takeWhile(zip(left.slice(prefix).reverse(), right.slice(prefix).reverse()), same).length;
  const independent = before === after ? null : [prefix, left.length - prefix - suffix, right.slice(prefix, right.length - suffix).join("")];
  assert.deepEqual(independent, expected);
}
console.log(`${fixture.cases.length} scalar splice replacements match the independent Lodash oracle`);
