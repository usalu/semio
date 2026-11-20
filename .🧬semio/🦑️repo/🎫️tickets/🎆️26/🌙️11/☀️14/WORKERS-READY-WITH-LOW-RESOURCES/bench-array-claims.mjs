import assert from "node:assert/strict";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const modulePath = process.argv[2], fixturePath = process.argv[3];
const previous = Bun.spawnSync(["repo", "tools", "exec", "--light", "git", "--", "show", "3d62b2ef22c:" + modulePath]);
assert.equal(previous.exitCode, 0);
const current = await Bun.file(modulePath).text();
const { rowKeyV1 } = await import(pathToFileURL(resolve(modulePath, "../../🧮️fact-rows/🟨️.mjs")).href);
const { owners, sharedRows, foundRows } = (await Bun.file(fixturePath).json()).memory;
const extract = source => source.slice(source.indexOf("function claimListedV1("), source.indexOf("/** 🌊️ Claims over a row log list"));
const getClaim = source => Function("rowKeyV1", "lazyRowsV1", "return (" + extract(source).trim() + ");")(rowKeyV1, () => false);
const claims = { before: getClaim(previous.stdout.toString()), after: getClaim(current) };
const shared = Array.from({ length: sharedRows }, (_, at) => ({ kind: "exists", path: "/w/lib/f" + at, fact: "v" + at % 7 }));
const all = [...shared], slots = [];
for (let at = 0; at < owners; at++) {
  const own = Array.from({ length: foundRows }, (_, index) => ({ kind: "file", path: "/w/p" + at + "/f" + index, fact: "o" + at }));
  all.push(...own);
  slots.push({ base: [...shared, ...own], prefixes: ["/w/p" + at + "/"] });
}
const NativeSet = globalThis.Set;
let additions = 0;
let visits = 0;
class CountingSet extends NativeSet {
  add(value) {
    additions++;
    return super.add(value);
  }
}
for (const [name, claim] of Object.entries(claims)) {
  additions = 0;
  visits = 0;
  for (const {base} of slots) {
    base[Symbol.iterator] = function* () {
      for (let at = 0; at < base.length; at++) {
        visits++;
        yield base[at];
      }
    };
  }
  globalThis.Set = CountingSet;
  try {
    const result = claim({ all, slots });
    assert.equal(result.rows.length, slots.length);
    result.rows.forEach((rows, at) => assert.equal(rows, slots[at].base));
    assert.deepEqual(result.environment([], []), []);
  } finally {
    globalThis.Set = NativeSet;
    for (const {base} of slots) delete base[Symbol.iterator];
  }
  console.log("[DEBUG] " + JSON.stringify({ phase: name, additions, visits }));
}
const samples = { before: [], after: [] };
for (let trial = 0; trial < 8; trial++) {
  for (const name of trial % 2 ? ["after", "before"] : ["before", "after"]) {
    Bun.gc(true);
    const started = performance.now(), result = claims[name]({ all, slots });
    assert.equal(result.environment([], []).length, 0);
    const elapsed = performance.now() - started;
    if (trial > 0) samples[name].push(elapsed);
  }
}
for (const [name, values] of Object.entries(samples)) {
  values.sort((a, b) => a - b);
  console.log("[DEBUG] " + JSON.stringify({ phase: name, milliseconds: values.map(Math.round), median: Math.round(values[3]) }));
}
