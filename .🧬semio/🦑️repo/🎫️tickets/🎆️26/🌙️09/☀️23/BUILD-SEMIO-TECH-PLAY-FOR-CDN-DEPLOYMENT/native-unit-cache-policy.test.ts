import { expect, test } from "bun:test";
import vectors from "./native-unit-cache-policy.vectors.json";
import { releaseUnitCacheDemotesV1, releaseUnitCacheModeV1 } from "./native-unit-cache-policy.ts";

test("release unit cache mode follows the language-agnostic vectors", () => {
  for (const row of vectors.mode) expect(releaseUnitCacheModeV1({ optOut: row.optOut ?? undefined, demoted: row.demoted })).toBe(row.expect as "on" | "off");
});
test("only early failures with the cache on demote it", () => {
  for (const row of vectors.demotes) expect(releaseUnitCacheDemotesV1({ mode: row.mode as "on" | "off", status: row.status, elapsedMs: row.elapsedMs, earlyMs: row.earlyMs })).toBe(row.expect);
});
