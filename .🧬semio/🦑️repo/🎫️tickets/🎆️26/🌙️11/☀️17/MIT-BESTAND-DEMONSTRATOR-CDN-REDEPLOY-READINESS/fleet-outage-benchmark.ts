import { mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { createFleetOutageCheckV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🧯️outage/🟦️.ts";
import { FLEET_POLICY_V1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🪪️identity/🟦️.ts";

const observations = 240;
const records = 118_523;
const now = 1_700_000_000_000;
const { downFaults, downMilliseconds } = FLEET_POLICY_V1.breaker;
const directory = mkdtempSync(join(import.meta.dirname, "🗑️generated/ob-"));
const history = join(directory, "down");

try {
  writeFileSync(history, "1699990000000\n".repeat(records - 2) + "1699999999999\n1699999999998\n", "utf8");
  const oldCheck = () => readFileSync(history, "utf8").split("\n").filter(Boolean)
    .filter(timestamp => now - Number(timestamp) < downMilliseconds).length >= downFaults;
  const recent = createFleetOutageCheckV1(history, downFaults, downMilliseconds);
  const measure = (check: () => boolean) => {
    const start = performance.now();
    for (let index = 0; index < observations; index++) if (!check()) throw new Error("Outage verdict changed");
    return performance.now() - start;
  };
  console.log(JSON.stringify({ records, bytes: statSync(history).size, observations, formerMs: measure(oldCheck), boundedMs: measure(() => recent(now)) }));
} finally {
  rmSync(directory, { recursive: true, force: true });
}
