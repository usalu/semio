import { test } from "bun:test";
import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { privateRepositoryWorkspaceV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/🟦️.ts";
const checkout = findWorkspaceRoot(import.meta.dirname), stale = join(checkout, ".cache/test-sandbox/rsLAcRv");
assert.equal(process.env.SEMIO_PRIVATE_REPOSITORY, undefined, "Cleanup must begin without adopted private authority");
assert.equal(process.env.SEMIO_PRIVATE_REPOSITORY_PARENT, undefined, "Cleanup uses the framework default sandbox parent");
assert.equal(resolve(process.cwd()), checkout);
test("official fresh private factory retires stale thesis workspace", async () => {
  const existed = existsSync(stale), repository = await privateRepositoryWorkspaceV1();
  assert.equal(repository.owned, true);
  assert.equal(await repository.requireIsolated(), repository.root);
  assert.notEqual(repository.root, stale);
  assert.equal(existsSync(stale), false, "Official dead-owner retirement completed");
  console.log("[DEBUG] " + JSON.stringify({ kind: "framework-private-cleanup", staleRoot: stale, existedBeforeFactory: existed, retired: !existsSync(stale), freshOwnedRoot: repository.root, retirement: "framework-afterAll" }));
}, 600000);
