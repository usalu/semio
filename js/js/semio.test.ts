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
import { applyDesignDiff, Design, flattenDesign, Kit } from "./semio";

describe("flattenDesign", () => {
    const kit = MetabolismKit as unknown as Kit;

    describe("Nakagin Capsule Tower", () => {
        it("should have same piece planes and centers as the expected design", () => {
            const design = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower")!;
            const expectedDesign = NakaginCapsuleTowerFlatDesign as unknown as Design;
            const flatDesignDiff = flattenDesign(kit, design.guid);
            const flatDesign = applyDesignDiff(design, flatDesignDiff);
            expect(flatDesign.pieces?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.guid)?.plane)).toBe(true);
            expect(flatDesign.pieces?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.guid)?.center)).toBe(true);
        });
    });

    describe("Nakagin Capsule Tower Slanted", () => {
        it("should have same piece planes and centers as the expected design", () => {
            const design = kit.designs?.find((d) => d.name === "Slanted")!;
            const expectedDesign = NakaginCapsuleTowerSlantedFlatDesign as unknown as Design;
            const flatDesignDiff = flattenDesign(kit, design.guid);
            const flatDesign = applyDesignDiff(design, flatDesignDiff);
            expect(flatDesign.pieces?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.guid)?.plane)).toBe(true);
            expect(flatDesign.pieces?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.guid)?.center)).toBe(true);
        });
    });

    describe("Nakagin Capsule Tower Twisted", () => {
        it("should have same piece planes and centers as the expected design", () => {
            const design = kit.designs?.find((d) => d.name === "Twisted")!;
            const expectedDesign = NakaginCapsuleTowerTwistedFlatDesign as unknown as Design;
            const flatDesignDiff = flattenDesign(kit, design.guid);
            const flatDesign = applyDesignDiff(design, flatDesignDiff);
            expect(flatDesign.pieces?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.guid)?.plane)).toBe(true);
            expect(flatDesign.pieces?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.guid)?.center)).toBe(true);
        });
    });

    describe("Nakagin Capsule Tower Dancing", () => {
        it("should have same piece planes and centers as the expected design", () => {
            const design = kit.designs?.find((d) => d.name === "Dancing")!;
            const expectedDesign = NakaginCapsuleTowerDancingFlatDesign as unknown as Design;
            const flatDesignDiff = flattenDesign(kit, design.guid);
            const flatDesign = applyDesignDiff(design, flatDesignDiff);
            expect(flatDesign.pieces?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.guid)?.plane)).toBe(true);
            expect(flatDesign.pieces?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.guid)?.center)).toBe(true);
        });
    });

    describe("Capsule Dream", () => {
        it("should have same piece planes and centers as the expected design", () => {
            const design = kit.designs?.find((d) => d.name === "Capsule Dream")!;
            const expectedDesign = CapsuleDreamFlatDesign as unknown as Design;
            const flatDesignDiff = flattenDesign(kit, design.guid);
            const flatDesign = applyDesignDiff(design, flatDesignDiff);
            expect(flatDesign.pieces?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.guid)?.plane)).toBe(true);
            expect(flatDesign.pieces?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.guid)?.center)).toBe(true);
        });
    });
});
