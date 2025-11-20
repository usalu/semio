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

import { MetabolismKit, NakaginCapsuleTowerDancingFlatDesign, NakaginCapsuleTowerFlatDesign, NakaginCapsuleTowerSlantedFlatDesign, NakaginCapsuleTowerTwistedFlatDesign } from "@semio/assets";
import { describe, expect, it } from "vitest";
import { Design, flattenDesign, Kit } from "./semio";

const designShouldHaveSamePiecePlanesAndCenters = (kit: Kit, design: Design, expectedDesign: Design) => {
    it("should have same piece planes and centers as the expected design", () => {
        const flatDesign = flattenDesign(kit, design.guid);
        // every piece with the same guid needs to have same plane
        expect(flatDesign.updated?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.id)?.plane)).toBe(true);
        // every piece with the same guid needs to have same center
        expect(flatDesign.updated?.every((p) => expectedDesign.pieces?.find((ep) => ep.guid === p.id)?.center)).toBe(true);
    });
}

describe("flattenDesign", () => {
    const kit = MetabolismKit as unknown as Kit;
    describe("Nakagin Capsule Tower", () => {
        designShouldHaveSamePiecePlanesAndCenters(kit, kit.designs?.find((d) => d.name === "Nakagin Capsule Tower"), NakaginCapsuleTowerFlatDesign);
    });
    describe("Nakagin Capsule Tower Slanted", () => {
        designShouldHaveSamePiecePlanesAndCenters(kit, kit.designs?.find((d) => d.name === "Slanted"), NakaginCapsuleTowerSlantedFlatDesign);
    });
    describe("Nakagin Capsule Tower Twisted", () => {
        designShouldHaveSamePiecePlanesAndCenters(kit, kit.designs?.find((d) => d.name === "Twisted"), NakaginCapsuleTowerTwistedFlatDesign);
    });
    describe("Nakagin Capsule Tower Dancing", () => {
        designShouldHaveSamePiecePlanesAndCenters(kit, kit.designs?.find((d) => d.name === "Dancing"), NakaginCapsuleTowerDancingFlatDesign);
    });
    describe("Capsule Dream", () => {
        designShouldHaveSamePiecePlanesAndCenters(kit, kit.designs?.find((d) => d.name === "Capsule Dream"), CapsuleDreamFlatDesign);
    });
});
