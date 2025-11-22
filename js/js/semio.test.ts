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
import kitDiffData from "../../assets/semio/diff_kit_metabolism.json";
import kitDiffInvertedData from "../../assets/semio/diff_kit_metabolism_inverted.json";
import kitOriginalData from "../../assets/semio/kit_metabolism.json";
import kitDiffedData from "../../assets/semio/kit_metabolism_diffed.json";
import { applyDesignDiff, applyKitDiff, areKitsEqual, deepEqual, Design, exportKit, flattenDesign, getKitDiff, importKit, inverseKitDiff, Kit, Plane } from "./semio";


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

describe("Kit Diff", () => {
    const kitOriginal = kitOriginalData as any;
    const kitDiff = kitDiffData as any;
    const kitDiffInverted = kitDiffInvertedData as any;
    const kitDiffed = kitDiffedData as any;

    it("should compute identical diffs and apply them correctly with full round-trip integrity", () => {
        // Import actual diff functions from semio.ts

        // 1. Compute diff from original to diffed and verify it matches the generated diff exactly
        const computedDiff = getKitDiff(kitOriginal, kitDiffed);
        expect(deepEqual(computedDiff, kitDiff)).toBe(true);

        // 2. Compute inverse diff from diffed to original and verify it matches the generated inverse exactly
        const computedInverseDiff = inverseKitDiff(kitOriginal, kitDiff);
        expect(deepEqual(computedInverseDiff, kitDiffInverted)).toBe(true);

        // 3. Apply forward diff to original and verify result matches diffed kit exactly
        const appliedForward = applyKitDiff(kitOriginal, kitDiff);
        expect(deepEqual(appliedForward, kitDiffed)).toBe(true);

        // 4. Apply inverse diff to diffed kit and verify result matches original exactly
        const appliedInverse = applyKitDiff(kitDiffed, kitDiffInverted);
        expect(deepEqual(appliedInverse, kitOriginal)).toBe(true);
    });
});

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

describe("Kit Import/Export", () => {
    it("should successfully roundtrip export and import a kit", async () => {
        const originalKit = MetabolismKit as unknown as Kit;
        const files = new Map<string, Blob>();

        const zipBlob = await exportKit(originalKit, files);

        expect(zipBlob).toBeInstanceOf(Blob);
        expect(zipBlob.size).toBeGreaterThan(0);

        const url = URL.createObjectURL(zipBlob);

        const { kit: importedKit, files: importedFiles } = await importKit(url);

        URL.revokeObjectURL(url);

        expect(areKitsEqual(originalKit, importedKit)).toBe(true);

        expect(importedFiles.size).toBe(files.size);

        // Export to assets folder if running in export mode
        if (process.env.EXPORT_TO_ASSETS === "true") {
            const fs = await import("fs/promises");
            const path = await import("path");
            const buffer = Buffer.from(await zipBlob.arrayBuffer());
            const outputPath = path.join(process.cwd(), "assets", "metabolism.zip");
            await fs.writeFile(outputPath, buffer);
            console.log(`[EXPORT] Wrote ${outputPath} (${(buffer.length / 1024).toFixed(2)} KB)`);
        }
    });
});
