import { expect, test } from "bun:test";
import { Plane, Vector3 } from "three";
import fixture from "./🧭️plane-marker.json";

test("plane channel glyph draws an oriented square and its normal", () => {
    for (const row of fixture.cases) {
        const origin = new Vector3().fromArray(row.origin);
        const direction = new Vector3().fromArray(row.normal);
        expect(direction.lengthSq() > 0).toBe(row.valid);
        if (!row.valid) continue;
        direction.normalize();
        const plane = new Plane().setFromNormalAndCoplanarPoint(direction, origin);
        const u = new Vector3().crossVectors(direction, Math.abs(direction.x) < 0.8 ? new Vector3(1,0,0) : new Vector3(0,1,0)).normalize();
        const v = new Vector3().crossVectors(direction, u);
        const corners = [[-1,-1],[1,-1],[1,1],[-1,1]].map(([a,b]) =>
            origin.clone().addScaledVector(u, a! * fixture.halfExtent).addScaledVector(v, b! * fixture.halfExtent));
        for (let i = 0; i < corners.length; ++i) {
            expect(plane.distanceToPoint(corners[i]!)).toBeCloseTo(0, 12);
            expect(corners[i]!.distanceTo(corners[(i+1)%4]!)).toBeCloseTo(2 * fixture.halfExtent, 12);
        }
        expect(plane.distanceToPoint(origin.clone().addScaledVector(direction, fixture.normalLength))).toBeCloseTo(fixture.normalLength, 12);
    }
});
