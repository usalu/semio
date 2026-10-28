import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import lodash from "lodash";
test("devcontainer storage funds the current fixed cache and fresh checkout", () => {
  const policy = JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json", "utf8"));
  const fixture = JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json", "utf8"));
  const config = Bun.JSONC.parse(readFileSync(".devcontainer/devcontainer.json", "utf8"));
  const sum = lodash.sumBy(Object.values(policy.partitions), (partition: any) => partition.capacityBytes);
  expect(sum + policy.maximumControlBytes).toBeLessThanOrEqual(policy.maximumBytes);
  const needed = Math.ceil((policy.maximumBytes / 1e9 + fixture.storage.workingSetGigabytes) * (100 + fixture.storage.headroomPercent) / 100);
  expect(needed).toBe(193);
  expect(Number(/^([0-9]+)gb$/.exec(config.hostRequirements.storage)?.[1])).toBeGreaterThanOrEqual(needed);
});
