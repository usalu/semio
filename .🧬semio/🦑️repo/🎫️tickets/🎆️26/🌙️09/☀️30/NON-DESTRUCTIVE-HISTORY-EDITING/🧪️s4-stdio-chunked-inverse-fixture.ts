/**
 * 🧩️ Writes (or with `--check` verifies) the `chunkedInverses` block of the snapshot-patch fixture: small documents whose
 * exact inverse exceeds a tiny part budget, with the parts the TypeScript twin plans. The Rust planner must answer the same
 * parts (cross-language law) and both suites replay them against third-party RFC 6902 appliers.
 *
 * @see ../../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts
 */
import { applySnapshotPatch, inverseSnapshotPatches, type SnapshotPatch } from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts";
import type { SnapshotValue } from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🟦️.ts";

const FIXTURE = new URL("../../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json", import.meta.url);
const BUDGET = 160;
const words = (count: number, seed: string): string[] => Array.from({ length: count }, (_, position) => `${seed}-${String(position).padStart(3, "0")}-${"abcdefghij".slice(0, 3 + (position % 7))}`);
const prose = (length: number): string => Array.from({ length }, (_, position) => "Grüße 🌍 \"quoted\"\nline\tend ".at(position % 27)!).join("");

const cases: { id: string; base: SnapshotValue; patch: SnapshotPatch }[] = [
  { id: "array-of-text", base: { title: "Doc", rows: [...words(12, "row"), prose(400), ...words(3, "tail")] }, patch: { operation: "remove", path: "/rows" } },
  { id: "long-text", base: { title: "Doc", body: prose(600) }, patch: { operation: "set", path: "/body", value: "short" } },
  { id: "struct-like-object", base: { meta: { name: "n", notes: prose(300), tags: words(20, "tag"), flag: true }, x: 1 }, patch: { operation: "remove", path: "/meta" } },
  { id: "map-like-object", base: { map: Object.fromEntries(words(30, "k").map((key, position) => [key, `v${position}`])) }, patch: { operation: "set", path: "/map", value: {} } },
  { id: "array-item-object", base: { items: [{ id: "a", points: Array.from({ length: 60 }, (_, position) => position * 7) }, { id: "b", points: [1, 2] }] }, patch: { operation: "remove", path: "/items/0" } },
  { id: "splice-inverse", base: { list: words(30, "entry") }, patch: { operation: "splice", path: "/list", offset: 2, remove: 25, value: ["x"] } },
  { id: "root-replace", base: { a: prose(300), b: words(10, "b") }, patch: { operation: "set", path: "", value: { z: 1 } } },
  { id: "root-kind-change", base: words(25, "root"), patch: { operation: "set", path: "", value: { z: 1 } } },
];

const block = { budget: BUDGET, cases: cases.map((row) => {
  const parts = inverseSnapshotPatches(row.base, row.patch, BUDGET);
  const restored = parts.reduce(applySnapshotPatch, applySnapshotPatch(row.base, row.patch));
  if (JSON.stringify(restored) !== JSON.stringify(row.base)) throw new Error(`${row.id}: parts do not restore the base`);
  if (parts.length < 2) throw new Error(`${row.id}: the inverse fits one part; enlarge the case`);
  return { ...row, parts };
}) };

const text = await Bun.file(FIXTURE).text();
const current = JSON.stringify((JSON.parse(text) as Record<string, unknown>).chunkedInverses);
if (process.argv.includes("--check")) {
  console.log(current === JSON.stringify(block) ? "chunkedInverses: fresh" : "chunkedInverses: STALE");
  process.exitCode = current === JSON.stringify(block) ? 0 : 1;
} else {
  const marker = ',\n  "chunkedInverses": ';
  const head = text.includes(marker) ? text.slice(0, text.indexOf(marker)) : text.replace(/\n\}\n?$/u, "");
  await Bun.write(FIXTURE, `${head}${marker}${JSON.stringify(block, null, 2).replaceAll("\n", "\n  ")}\n}\n`);
  console.log(`chunkedInverses: ${block.cases.length} cases, ${block.cases.reduce((sum, row) => sum + row.parts.length, 0)} parts`);
}
