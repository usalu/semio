import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { allocateV1, BUCKET_AGE_MS, type Input } from "./allocator.ts";

const corpus: { input: Input; expected: unknown }[] = JSON.parse(readFileSync(new URL("./corpus.json", import.meta.url), "utf8"));

test("equals the Python oracle on every seeded case", () => {
  expect(corpus.length).toBeGreaterThanOrEqual(400);
  corpus.forEach((c, i) => {
    const got = JSON.parse(JSON.stringify(allocateV1(c.input)));
    expect(got, `case ${i}`).toEqual(c.expected);
  });
});

test("invariants: budget, protected bucket, oldest-first weighted by cost, shard cap", () => {
  let evicting = 0;
  for (const c of corpus) {
    const o = allocateV1(c.input);
    const byName = new Map(c.input.purposes.map((p) => [p.name, p]));
    const sumTarget = o.purposes.reduce((s, p) => s + p.targetBytes, 0);
    if (!o.unmet) expect(sumTarget).toBeLessThanOrEqual(o.budgetBytes);
    for (const e of o.evictions) expect(e.bucket).toBeGreaterThan(0);
    for (const p of o.purposes) {
      const q = byName.get(p.name)!;
      expect(p.usedAfterBytes).toBeGreaterThanOrEqual(q.buckets[0]);
      expect(p.shards.reduce((s, v) => s + v, 0)).toBe(q.imageless ? 0 : p.targetBytes);
      for (const s of p.shards) expect(s).toBeLessThanOrEqual(c.input.policy.imageCapBytes);
    }
    if (o.evictions.length) evicting++;
    const evicted = new Map<string, number>();
    o.evictions.forEach((e) => evicted.set(`${e.purpose}/${e.bucket}`, e.bytes));
    const last = o.evictions[o.evictions.length - 1];
    if (last && !o.unmet) {
      const lp = byName.get(last.purpose)!;
      const lScore = (BUCKET_AGE_MS[last.bucket] / lp.costMsPerGiB);
      for (const p of c.input.purposes) {
        p.buckets.forEach((bytes, b) => {
          if (b === 0 || bytes === 0) return;
          const score = BUCKET_AGE_MS[b] / p.costMsPerGiB;
          const taken = evicted.get(`${p.name}/${b}`) ?? 0;
          if (score > lScore) expect(taken).toBe(bytes);
        });
      }
    }
  }
  expect(evicting).toBeGreaterThan(30);
});

test("ample host: nothing evicted, every purpose keeps its measured peak", () => {
  const c = corpus.find((x) => allocateV1(x.input).evictions.length === 0 && x.input.purposes.length > 2)!;
  const o = allocateV1(c.input);
  expect(o.unmet).toBe(false);
  for (const p of o.purposes) expect(p.usedAfterBytes).toBe(p.usedBeforeBytes);
});

test("host pressure shrinks the budget below today's physical use", () => {
  const c = corpus.find((x) => x.input.host.freeBytes === 0 && allocateV1(x.input).physicalNowBytes > 0)!;
  const o = allocateV1(c.input);
  expect(o.budgetBytes).toBeLessThanOrEqual(Math.max(0, o.physicalNowBytes - o.reserveBytes));
});
