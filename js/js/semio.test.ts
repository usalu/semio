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

import { describe, expect, it } from "vitest";
import { applyDesignDiff, cn, deepEqual, flattenDesign, Generator, guid, jaccard, Kit, normalize, round } from "./semio";

describe("semio utilities", () => {
    describe("normalize", () => {
        it("should return empty string for undefined", () => {
            expect(normalize(undefined)).toBe("");
        });
        it("should return empty string for null", () => {
            expect(normalize(null)).toBe("");
        });
        it("should return the value for defined strings", () => {
            expect(normalize("test")).toBe("test");
        });
    });
    describe("round", () => {
        it("should round to nearest tolerance", () => {
            expect(round(0.123)).toBeCloseTo(0.123, 5);
            expect(round(0.5)).toBeCloseTo(0.5, 5);
            expect(round(1.7)).toBeCloseTo(1.7, 5);
            expect(round(0.123456789)).toBeCloseTo(0.12346, 5);
        });
    });
    describe("jaccard", () => {
        it("should return 1 for identical sets", () => {
            expect(jaccard(["a", "b"], ["a", "b"])).toBe(1);
        });
        it("should return 0 for disjoint sets", () => {
            expect(jaccard(["a"], ["b"])).toBe(0);
        });
        it("should return correct value for partial overlap", () => {
            expect(jaccard(["a", "b"], ["a", "c"])).toBeCloseTo(1 / 3, 5);
        });
        it("should return 1 for both undefined", () => {
            expect(jaccard(undefined, undefined)).toBe(1);
        });
        it("should return 0 for one undefined", () => {
            expect(jaccard(["a"], undefined)).toBe(0);
            expect(jaccard(undefined, ["a"])).toBe(0);
        });
    });
    describe("deepEqual", () => {
        it("should return true for identical primitives", () => {
            expect(deepEqual(1, 1)).toBe(true);
            expect(deepEqual("test", "test")).toBe(true);
            expect(deepEqual(true, true)).toBe(true);
        });
        it("should return false for different primitives", () => {
            expect(deepEqual(1, 2)).toBe(false);
            expect(deepEqual("test", "other")).toBe(false);
        });
        it("should return true for identical arrays", () => {
            expect(deepEqual([1, 2, 3], [1, 2, 3])).toBe(true);
        });
        it("should return false for different arrays", () => {
            expect(deepEqual([1, 2], [1, 3])).toBe(false);
            expect(deepEqual([1, 2], [1, 2, 3])).toBe(false);
        });
        it("should return true for identical objects", () => {
            expect(deepEqual({ a: 1, b: 2 }, { a: 1, b: 2 })).toBe(true);
        });
        it("should return false for different objects", () => {
            expect(deepEqual({ a: 1 }, { a: 2 })).toBe(false);
        });
    });
    describe("cn", () => {
        it("should merge class names", () => {
            expect(cn("a", "b")).toBe("a b");
        });
        it("should handle conditional classes", () => {
            expect(cn("a", false && "b", "c")).toBe("a c");
        });
    });
    describe("guid", () => {
        it("should generate a unique identifier", () => {
            const id1 = guid();
            const id2 = guid();
            expect(id1).toBeTruthy();
            expect(id2).toBeTruthy();
            expect(id1).not.toBe(id2);
        });
    });
    describe("Generator", () => {
        describe("randomId", () => {
            it("should generate a random ID with seed", () => {
                const id1 = Generator.randomId(123);
                const id2 = Generator.randomId(123);
                expect(id1).toBeTruthy();
                expect(id2).toBeTruthy();
                expect(id1).toBe(id2);
            });
            it("should generate different IDs with different seeds", () => {
                const id1 = Generator.randomId(123);
                const id2 = Generator.randomId(456);
                expect(id1).not.toBe(id2);
            });
        });
        describe("randomName", () => {
            it("should generate a random name with seed", () => {
                const name1 = Generator.randomName(123);
                const name2 = Generator.randomName(123);
                expect(name1).toBeTruthy();
                expect(name2).toBeTruthy();
                expect(name1).toBe(name2);
            });
        });
    });
    describe("flattenDesign", () => {
        const designs = [
            "Capsule Dream",
            "Nakagin Capsule Tower",
            "Dancing",
            "Slanted",
            "Twisted",
        ];
        designs.forEach((name) => {
            it(`should successfully flatten "${name}"`, async () => {
                const kitModule = await import("../../assets/semio/kit_metabolism.json");
                const kit = kitModule.default as unknown as Kit;
                const connectedDesign = kit.designs?.find((d) => d.name === name);
                if (!connectedDesign) throw new Error(`Design "${name}" not found in kit`);
                const originalConnectionCount = connectedDesign.connections?.length || 0;
                expect(originalConnectionCount).toBeGreaterThan(0);
                const flattenDiff = flattenDesign(kit, connectedDesign.guid);
                expect(flattenDiff).toBeDefined();
                expect(flattenDiff.connections).toBeDefined();
                expect(flattenDiff.connections?.removed).toBeDefined();
                expect(flattenDiff.connections!.removed!.length).toBe(originalConnectionCount);
                const flattenedDesign = applyDesignDiff(connectedDesign, flattenDiff);
                expect(flattenedDesign.pieces).toBeDefined();
                expect(flattenedDesign.connections).toBeDefined();
                expect(flattenedDesign.connections!.length).toBe(0);
                const originalPieceCount = connectedDesign.pieces?.length || 0;
                const flattenedPieceCount = flattenedDesign.pieces?.length || 0;
                expect(flattenedPieceCount).toBe(originalPieceCount);
                const missingPlanes: string[] = [];
                const missingCenters: string[] = [];
                flattenedDesign.pieces!.forEach((piece) => {
                    if (!piece.plane) missingPlanes.push(piece.guid);
                    if (!piece.center) missingCenters.push(piece.guid);
                });
                if (missingPlanes.length > 0 || missingCenters.length > 0) {
                    const msg = [
                        `Flatten failed for "${name}":`,
                        missingPlanes.length > 0 ? `  ${missingPlanes.length}/${originalPieceCount} pieces missing plane` : null,
                        missingCenters.length > 0 ? `  ${missingCenters.length}/${originalPieceCount} pieces missing center` : null,
                    ].filter(Boolean).join("\n");
                    throw new Error(msg);
                }
                expect(flattenedDesign.pieces!.length).toBe(originalPieceCount);
                expect(flattenedDesign.pieces!.every((p) => p.plane && p.center)).toBe(true);
            });
        });
    });
});

