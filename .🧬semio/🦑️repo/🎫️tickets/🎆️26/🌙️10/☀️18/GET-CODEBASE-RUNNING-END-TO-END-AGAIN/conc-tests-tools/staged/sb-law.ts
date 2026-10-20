import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import protocol from "../../🔣️.json" with { type: "json" };
import ledger from "../🧫️fixtures/📒️ledger.json" with { type: "json" };
import { compileLawV1, scanIsTestPathV1, scanLanguageV1, scanTestsV1, type TestSandboxProtocolV1 } from "../../🟦️.ts";

const root = resolve(process.env.SEMIO_WORKSPACE_ROOT ?? join(import.meta.dir, "../../../../../../.."));

function tracked(): readonly string[] {
  const run = spawnSync("git", ["ls-files", "-z", "--", "*🧪️tests/*", "*_test.go", "*.test.*", "*.spec.*"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 });
  if (run.status !== 0) throw Error(`git ls-files failed: ${run.stderr}`);
  return run.stdout.split("\0").filter(path => path !== "" && !path.startsWith(".🧬semio/🦑️repo/🎫️tickets/"));
}

test("the concurrency law: no test touches the real checkout, the shared cache partitions, fixed ports or fixed temp names beyond the ratcheted debt ledger", () => {
  const law = compileLawV1(protocol as unknown as TestSandboxProtocolV1), listing = tracked().filter(path => scanLanguageV1(law, path) !== undefined && scanIsTestPathV1(law, path));
  const result = scanTestsV1(law, listing, path => { try { return readFileSync(join(root, path), "utf8"); } catch { return undefined; } });
  const found = new Map<string, number>();
  for (const finding of result.findings) if (finding.severity === "error") found.set(`${finding.file}\0${finding.rule}`, (found.get(`${finding.file}\0${finding.rule}`) ?? 0) + 1);
  const debt = new Map(ledger.entries.map(entry => [`${entry.file}\0${entry.rule}`, entry.count]));
  const fresh = [...found].filter(([key, count]) => debt.get(key) !== count).map(([key, count]) => `${key.replace("\0", " ")} x${count} (ledger ${debt.get(key) ?? "none"})`);
  const stale = [...debt].filter(([key]) => !found.has(key)).map(([key, count]) => `${key.replace("\0", " ")} x${count} no longer occurs`);
  expect({ fresh, stale }, "fix the test (use createTestSandboxV1 / testScratchRootV1 / reserveTestPortV1) or, for debt that cannot be fixed now, record it with a reason in the ledger; remove ledger rows whose finding is gone").toEqual({ fresh: [], stale: [] });
  expect(result.scanned).toBeGreaterThan(1000);
}, 300000);
