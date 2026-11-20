import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import jsonPatch from "fast-json-patch";

const root = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any";
const createCases = JSON.parse(readFileSync("✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🌱️page-creation/🧪️fixtures/🔣️.json", "utf8")).cases;
const folder = root + "/🧪️tests/🧫️fixtures/🧬️mutations/🌱create-page/➕️appends-page-3";
const read = path => JSON.parse(readFileSync(folder + "/" + path + "/🔣️.json", "utf8"));
const base = read("📸️snapshot/⬅️before");
const source = read("🦠️mutation").CreatePage.page;
for (const row of createCases) {
  const candidate = { ...source, id: row.id, spreadId: row.spread };
  const spread = base.spreads.findIndex(held => held.id === row.spread);
  const position = row.index ?? base.pages.length;
  const error = base.pages.some(page => page.id === row.id) ? "mutation.duplicate-id"
    : position > base.pages.length ? "mutation.target-missing"
    : spread < 0 ? "mutation.invariant"
    : undefined;
  assert.equal(error, row.error, row.name);
  if (row.error) continue;
  assert.deepEqual([position, spread], row.coordinates, row.name);
  const spreadPath = "/spreads/" + spread + "/pageIds";
  const post = jsonPatch.applyPatch(base, [
    { op: "add", path: "/pages/" + position, value: candidate },
    { op: "add", path: spreadPath + "/-", value: candidate.id },
  ], true, false).newDocument;
  assert.deepEqual(post.spreads[spread].pageIds, row.members, row.name);
  assert.deepEqual(post.pages[position], candidate, row.name);
  const undo = jsonPatch.applyPatch(post, [
    { op: "remove", path: "/pages/" + position },
    { op: "remove", path: spreadPath + "/" + (row.members.length - 1) },
  ], true, false).newDocument;
  assert.deepEqual(undo, base, row.name);
  if (spread === 1) assert.deepEqual(post, read("📸️snapshot/➡️after"));
}
console.log(createCases.length + " page creation cases matched the canonical fixtures and JSON Patch oracle");
