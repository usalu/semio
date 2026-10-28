/** 🌱️ A patch of the fact index re-partitions only when its partition outgrew the bound AND grew by the declared share since its base: a base that is already the coarsest partition above the bound (the real index: 20,657 classes for 4,020 owners) is patched without the full partition rebuild that cost 1.5 GB and 2.7 s per delta publish. The consumers of every owner equal a naive owner-to-rows table in both cases; the independent oracle is that table. */
import {expect, test} from "bun:test";
import {GENERATION_POLICY_V1, canonicalJsonV1, createFactIndexV1, patchFactIndexV1, refineFactIndexV1, rowKeyV1} from "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram-g-tools/staged/generations-variant.mjs";

type Row = {kind: string; path: string; fact: {sha256: string}};
const row = (n: number): Row => ({kind: "file", path: "/w/" + n, fact: {sha256: String(n).padStart(64, "0")}});
const seeded = (seed: number) => () => (seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0) / 4294967296;
const naive = (table: Map<string, Row[]>) => new Map([...table].map(([owner, rows]) => [owner, new Set(rows.map(item => canonicalJsonV1([item.kind, item.path, item.fact])))]));
const consumers = (index: ReturnType<typeof createFactIndexV1>) => {
  const out = new Map<string, Set<string>>();
  for (const cls of index.classes) for (const id of cls.o) { const set = out.get(index.owners[id]!) ?? new Set<string>(); for (const r of cls.r) set.add(rowKeyV1(index.rows[r]!)); out.set(index.owners[id]!, set); }
  return out;
};
const world = (owners: number, rows: number, per: number, next: () => number) => {
  const table = new Map<string, Row[]>(Array.from({length: owners}, (_, o) => ["o" + String(o).padStart(3, "0"), [] as Row[]]));
  const names = [...table.keys()];
  for (let n = 0; n < rows; n++) for (let k = 0; k < per; k++) { const list = table.get(names[Math.floor(next() * owners)]!)!; if (!list.some(item => item.path === "/w/" + n)) list.push(row(n)); }
  return table;
};

test("a patch above the bound keeps the consumers of the naive table and keeps the partition unless it grew by the declared share", () => {
  const next = seeded(7), table = world(40, 2400, 3, next), index = createFactIndexV1(table);
  const bound = GENERATION_POLICY_V1.compactionFactor * index.owners.length + GENERATION_POLICY_V1.compactionFloor;
  expect(index.classes.length).toBeGreaterThan(bound);
  expect(GENERATION_POLICY_V1.compactionGrowth).toBeGreaterThan(1);
  const small = new Map([["o005", [row(1), row(9000)]]]);
  table.set("o005", small.get("o005")!);
  const patched = patchFactIndexV1(index, {upserts: small});
  expect(patched.classes.length).toBeLessThanOrEqual(index.classes.length + 2);
  expect(consumers(patched)).toEqual(naive(table));
  expect(consumers(refineFactIndexV1(patched))).toEqual(naive(table));
  const many = new Map<string, Row[]>();
  for (let o = 0; o < 40; o++) many.set("o" + String(o).padStart(3, "0"), [row(100000 + o), row(100100 + o), row(1 + (o % 7))]);
  for (const [owner, rows] of many) table.set(owner, rows);
  const grown = patchFactIndexV1(patched, {upserts: many});
  expect(consumers(grown)).toEqual(naive(table));
  expect(consumers(refineFactIndexV1(grown))).toEqual(naive(table));
});

test("a base below the bound is never re-partitioned and stays exact after seeded edits", () => {
  const next = seeded(11), table = world(12, 200, 2, next);
  let index = createFactIndexV1(table);
  for (let step = 0; step < 30; step++) {
    const owner = "o" + String(Math.floor(next() * 12)).padStart(3, "0"), rows = Array.from({length: 1 + Math.floor(next() * 5)}, () => row(Math.floor(next() * 260)));
    const unique = [...new Map(rows.map(item => [item.path, item])).values()];
    table.set(owner, unique);
    index = patchFactIndexV1(index, {upserts: new Map([[owner, unique]])});
    expect(consumers(index), "step " + step).toEqual(naive(table));
  }
});
