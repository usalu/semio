import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
const here = dirname(fileURLToPath(import.meta.url));
describe("concrete-forest", () => {
  it("ships primary asset", () => {
    expect(readFileSync(join(here, "../../../../🖼️assets/🌲️concrete-forest/🗣️.dsl.semio"), "utf8").length).toBeGreaterThan(8);
  });
});

import budget from "./🪙️budget.json";
describe("forest brep seed budget", () => {
  it("fits the one pane load of a host lifetime into its fuel", () => {
    const law = budget.forestBrep;
    expect(law.targetSeedBytes * law.loadsPerLifetime).toBeLessThanOrEqual(law.hostLifetimeFuel);
    expect(law.ceilingSeedBytes).toBeLessThanOrEqual(law.targetSeedBytes);
    expect(law.rows.coedges).toBeGreaterThan(0);
  });
  it("seeds the whole brep from its rows at the pinned slot size", () => {
    const slot = budget.receiveSlot;
    expect(slot.ceilingBytes).toBeLessThan(slot.beforeBytes);
    expect(Math.ceil((1 - slot.ceilingBytes / slot.beforeBytes) * 100)).toBeGreaterThanOrEqual(32);
  });
  it("keeps every row receiver below its size before the shared frame", () => {
    const rows = budget.recordReceiver;
    for (const name of Object.keys(rows.beforeBytes)) expect(rows.ceilingBytes[name]).toBeLessThan(rows.beforeBytes[name] * 0.8);
    expect(rows.ceilingBytes.BrepCoedge).toBeLessThan(rows.beforeBytes.BrepCoedge / 5);
  });
});
