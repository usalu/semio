import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { planGcV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts";
import { decideHeadroomV1, headroomEstimateV1, headroomPriorV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🏦️headroom/🟦️.ts";
import fixture from "./🔣️procedural-headroom-replay.json";

test("the observed Procedural request still required its sole idle lane after unit reclamation", () => {
  const volume = fixture.imageBytes;
  const now = fixture.timeAfterUnitGcMs;
  const classes = ["component#warm", fixture.requestClass];
  const history = { version: 1 as const, classes: { "component#warm": fixture.warmSamples } };
  const estimate = headroomEstimateV1(history, classes, headroomPriorV1(["component"], "warm"), volume);
  const sql = new Database(":memory:");
  try {
    const fixtureOracle = sql.query<{ reservation: number; floor: number }, [number, string]>(`
      with samples as (select json_extract(value, '$.bytes') as amount from json_each(?2))
      select min(cast(?1 * 1500 / 10000 as integer), cast(max(amount) * 11500 / 10000 as integer)) as reservation,
             cast(?1 * 300 / 10000 as integer) as floor from samples`);
    const independent = fixtureOracle.get(volume, JSON.stringify(fixture.warmSamples))!;
    expect(estimate.class).toBe("component#warm");
    expect(estimate.reservedBytes).toBe(independent.reservation);
    const request = { id: "procedural", arrivedMs: now - 4000, reservedBytes: estimate.reservedBytes };
    const energy = { id: "energy", state: "running" as const, reservedBytes: estimate.reservedBytes, rampMilliseconds: estimate.rampMilliseconds, arrivedMs: now - estimate.rampMilliseconds, admittedMs: now - estimate.rampMilliseconds };
    const decide = (freeBytes: number) => decideHeadroomV1({ freeBytes, volumeBytes: volume, nowMs: now, request, rows: [energy], reclaimed: true });
    const before = decide(fixture.initialFreeBytes);
    const afterUnits = decide(fixture.freeBeforeLaneGcBytes);
    expect(before.heldBytes).toBe(0);
    expect(before.verdict).toBe("wait");
    expect(before.shortfallBytes).toBe(independent.reservation + independent.floor - fixture.initialFreeBytes);
    expect(afterUnits.shortfallBytes).toBe(independent.reservation + independent.floor - fixture.freeBeforeLaneGcBytes);
    const eligible = sql.query<{ id: string }, [string, number]>("select json_extract(value, '$.id') as id from json_each(?1) where json_extract(value, '$.live') = 0 and json_extract(value, '$.bytes') >= ?2");
    const plan = planGcV1({ capacityBytes: volume, volumeBytes: volume, freeBytes: fixture.freeBeforeLaneGcBytes, nowMs: now, demandBytes: afterUnits.shortfallBytes, lanes: fixture.lanes.map((lane) => ({ ...lane, lastUsedMs: now, incrementalBytes: 0 })) });
    expect(plan.removeLanes).toEqual(eligible.all(JSON.stringify(fixture.lanes), afterUnits.shortfallBytes).map((lane) => lane.id));
    expect(plan.removeLanes).toEqual(["procedural"]);
    expect(decide(fixture.freeAfterLaneGcBytes).reason).toBe("fits");
  } finally {
    sql.close();
  }
});
