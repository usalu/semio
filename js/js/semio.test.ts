// #region Header

// semio.test.ts

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


// #endregion

import { CapsuleDreamFlatDesign, MetabolismKit, NakaginCapsuleTowerDancingFlatDesign, NakaginCapsuleTowerFlatDesign, NakaginCapsuleTowerSlantedFlatDesign, NakaginCapsuleTowerTwistedFlatDesign } from "@semio/assets";
import { describe, expect, it } from "vitest";
import { applyDesignDiff, Design, flattenDesign, Kit, Plane } from "./semio";

const TOLERANCE = 0.0001;

const planesEqual = (p1?: Plane, p2?: Plane): boolean => {
    if (!p1 || !p2) return false; // Both must exist to be equal
    if (!p1.origin || !p2.origin || !p1.xAxis || !p2.xAxis || !p1.yAxis || !p2.yAxis) return false;
    return (
        Math.abs(p1.origin.x - p2.origin.x) < TOLERANCE &&
        Math.abs(p1.origin.y - p2.origin.y) < TOLERANCE &&
        Math.abs(p1.origin.z - p2.origin.z) < TOLERANCE &&
        Math.abs(p1.xAxis.x - p2.xAxis.x) < TOLERANCE &&
        Math.abs(p1.xAxis.y - p2.xAxis.y) < TOLERANCE &&
        Math.abs(p1.xAxis.z - p2.xAxis.z) < TOLERANCE &&
        Math.abs(p1.yAxis.x - p2.yAxis.x) < TOLERANCE &&
        Math.abs(p1.yAxis.y - p2.yAxis.y) < TOLERANCE &&
        Math.abs(p1.yAxis.z - p2.yAxis.z) < TOLERANCE
    );
};

const centersEqual = (c1: { u: number, v: number } | undefined, c2: { u: number, v: number } | undefined): boolean => {
    if (!c1 || !c2) return c1 === c2;
    return Math.abs(c1.u - c2.u) < TOLERANCE && Math.abs(c1.v - c2.v) < TOLERANCE;
};

describe("flattenDesign", () => {
    const kit = MetabolismKit as unknown as Kit;

    describe("Nakagin Capsule Tower", () => {
        it("should have same piece planes and centers as the expected design", () => {
            const design = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower")!;
            const expectedDesign = NakaginCapsuleTowerFlatDesign as unknown as Design;
            const flatDesignDiff = flattenDesign(kit, design.guid);
            const flatDesign = applyDesignDiff(design, flatDesignDiff);

            // Verify all pieces have planes after flattening
            const piecesWithoutPlanes = flatDesign.pieces?.filter(p => !p.plane) ?? [];
            expect(piecesWithoutPlanes.length).toBe(0);

            expect(flatDesign.pieces?.every((p) => {
                const expectedPiece = expectedDesign.pieces?.find((ep) => ep.guid === p.guid);
                return planesEqual(p.plane, expectedPiece?.plane);
            })).toBe(true);
            expect(flatDesign.pieces?.every((p) => {
                const expectedPiece = expectedDesign.pieces?.find((ep) => ep.guid === p.guid);
                return centersEqual(p.center, expectedPiece?.center);
            })).toBe(true);
        });
    });

    describe("Nakagin Capsule Tower Slanted", () => {
        it("should have same piece planes and centers as the expected design", () => {
            const design = kit.designs?.find((d) => d.name === "Slanted")!;
            const expectedDesign = NakaginCapsuleTowerSlantedFlatDesign as unknown as Design;
            const flatDesignDiff = flattenDesign(kit, design.guid);
            const flatDesign = applyDesignDiff(design, flatDesignDiff);
            expect(flatDesign.pieces?.every((p) => {
                const expectedPiece = expectedDesign.pieces?.find((ep) => ep.guid === p.guid);
                return planesEqual(p.plane, expectedPiece?.plane);
            })).toBe(true);
            expect(flatDesign.pieces?.every((p) => {
                const expectedPiece = expectedDesign.pieces?.find((ep) => ep.guid === p.guid);
                return centersEqual(p.center, expectedPiece?.center);
            })).toBe(true);
        });
    });

    describe("Nakagin Capsule Tower Twisted", () => {
        it("should have same piece planes and centers as the expected design", () => {
            const design = kit.designs?.find((d) => d.name === "Twisted")!;
            const expectedDesign = NakaginCapsuleTowerTwistedFlatDesign as unknown as Design;
            const flatDesignDiff = flattenDesign(kit, design.guid);
            const flatDesign = applyDesignDiff(design, flatDesignDiff);
            expect(flatDesign.pieces?.every((p) => {
                const expectedPiece = expectedDesign.pieces?.find((ep) => ep.guid === p.guid);
                return planesEqual(p.plane, expectedPiece?.plane);
            })).toBe(true);
            expect(flatDesign.pieces?.every((p) => {
                const expectedPiece = expectedDesign.pieces?.find((ep) => ep.guid === p.guid);
                return centersEqual(p.center, expectedPiece?.center);
            })).toBe(true);
        });
    });

    describe("Nakagin Capsule Tower Dancing", () => {
        it("should have same piece planes and centers as the expected design", () => {
            const design = kit.designs?.find((d) => d.name === "Dancing")!;
            const expectedDesign = NakaginCapsuleTowerDancingFlatDesign as unknown as Design;
            const flatDesignDiff = flattenDesign(kit, design.guid);
            const flatDesign = applyDesignDiff(design, flatDesignDiff);
            expect(flatDesign.pieces?.every((p) => {
                const expectedPiece = expectedDesign.pieces?.find((ep) => ep.guid === p.guid);
                return planesEqual(p.plane, expectedPiece?.plane);
            })).toBe(true);
            expect(flatDesign.pieces?.every((p) => {
                const expectedPiece = expectedDesign.pieces?.find((ep) => ep.guid === p.guid);
                return centersEqual(p.center, expectedPiece?.center);
            })).toBe(true);
        });
    });

    describe("Capsule Dream", () => {
        it("should have same piece planes and centers as the expected design", () => {
            const design = kit.designs?.find((d) => d.name === "Capsule Dream")!;
            const expectedDesign = CapsuleDreamFlatDesign as unknown as Design;
            const flatDesignDiff = flattenDesign(kit, design.guid);
            const flatDesign = applyDesignDiff(design, flatDesignDiff);
            expect(flatDesign.pieces?.every((p) => {
                const expectedPiece = expectedDesign.pieces?.find((ep) => ep.guid === p.guid);
                return planesEqual(p.plane, expectedPiece?.plane);
            })).toBe(true);
            expect(flatDesign.pieces?.every((p) => {
                const expectedPiece = expectedDesign.pieces?.find((ep) => ep.guid === p.guid);
                return centersEqual(p.center, expectedPiece?.center);
            })).toBe(true);
        });
    });
});
