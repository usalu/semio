import { expect, test } from "bun:test";
import { Triangle, Vector3 } from "three";
import fixture from "./🧭️mesh-provenance.json";

const previous = new Map(Object.entries(fixture.previous));
const packs = new Map(Object.entries(fixture.packs).map(([handle, x]) =>
    [handle, new Triangle(new Vector3(x, 0, 0), new Vector3(x, 1, 0), new Vector3(x, 0, 1))]));
test("pending channels retain only their own previous geometry", () => {
    for (const row of fixture.cases) {
        const resident = new Set(row.available);
        const history = "advanced" in row && row.advanced ? [fixture.live, previous.get(row.channel)] : [previous.get(row.channel)];
        const handle = resident.has(fixture.live) ? fixture.live
            : row.diagnosed ? null : history.find(handle => handle !== undefined && resident.has(handle)) ?? null;
        const triangle = handle ? packs.get(handle) : undefined;
        expect(handle).toBe(row.expected);
        expect(triangle?.getMidpoint(new Vector3()).x ?? null).toBe(row.position);
        expect(handle === fixture.live).toBe(row.component);
    }
    const stale = packs.get("face-a")!;
    const current = packs.get("face-live")!;
    expect(current.getArea()).toBe(stale.getArea());
    expect(current.getMidpoint(new Vector3()).x).not.toBe(stale.getMidpoint(new Vector3()).x);
});
