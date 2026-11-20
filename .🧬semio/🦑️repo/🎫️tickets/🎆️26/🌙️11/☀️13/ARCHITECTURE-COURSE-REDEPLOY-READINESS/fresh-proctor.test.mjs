/** 🧪️ Laws of the fresh-proctor helper: an artifact older than its newest source is stale, one at least as new is fresh, and the newest of several built products wins. Run: node --test fresh-proctor.test.mjs */
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { newestProduct, newestSource, staleReason } from "./fresh-proctor.mjs";

const touch = (file, seconds) => (writeFileSync(file, "x"), utimesSync(file, seconds, seconds));

test("an artifact older than its newest source is stale and says which source", () => {
  assert.match(staleReason(100_000, { file: "a.rs", mtimeMs: 200_000 }) ?? "", /a\.rs/);
  assert.equal(staleReason(200_000, { file: "a.rs", mtimeMs: 200_000 }), undefined);
  assert.equal(staleReason(300_000, { file: "a.rs", mtimeMs: 200_000 }), undefined);
  assert.equal(staleReason(300_000, undefined), undefined);
});

test("the newest source is found among files, only of the wanted kinds", () => {
  const root = mkdtempSync(join(tmpdir(), "fresh-"));
  mkdirSync(join(root, "sub"));
  touch(join(root, "old.rs"), 100);
  touch(join(root, "sub", "new.rs"), 300);
  touch(join(root, "other.md"), 900);
  assert.equal(newestSource([root], /\.rs$/)?.file, join(root, "sub", "new.rs"));
  assert.equal(newestSource([join(root, "none")], /\.rs$/), undefined);
});

test("the newest product wins", () => {
  const root = mkdtempSync(join(tmpdir(), "fresh-"));
  for (const [hash, seconds] of [["a", 100], ["b", 500], ["c", 300]]) {
    mkdirSync(join(root, hash, "out"), { recursive: true });
    touch(join(root, hash, "out", "proctor"), seconds);
  }
  assert.equal(newestProduct(root), join(root, "b", "out", "proctor"));
  assert.equal(newestProduct(join(root, "missing")), undefined);
});
