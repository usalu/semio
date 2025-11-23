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

import { MetabolismKit, MetabolismKitDiff, MetabolismKitDiffed, MetabolismKitDiffInverted } from "@semio/assets";
import { describe, expect, it } from "vitest";
import { applyDesignDiff, applyKitDiff, areKitsEqual, deepEqual, exportKit, flattenDesign, getKitDiff, importKit, inverseKitDiff, Kit, Plane } from "./semio";


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
    const kitOriginal = MetabolismKit as any;
    const kitDiff = MetabolismKitDiff as any;
    const kitDiffInverted = MetabolismKitDiffInverted as any;
    const kitDiffed = MetabolismKitDiffed as any;

    it("should compute identical diffs and apply them correctly with full round-trip integrity", () => {
        // Import actual diff functions from semio.ts

        // 1. Compute diff from original to diffed and verify it matches the generated diff exactly
        const computedDiff = getKitDiff(kitOriginal, kitDiffed);
        if (!deepEqual(computedDiff, kitDiff)) {
            console.log("[DEBUG] [KIT-DIFF-TEST] Computed diff does not match expected diff");
            console.log("[DEBUG] [KIT-DIFF-TEST] Computed keys:", Object.keys(computedDiff));
            console.log("[DEBUG] [KIT-DIFF-TEST] Expected keys:", Object.keys(kitDiff));
            const allKeys = new Set([...Object.keys(computedDiff), ...Object.keys(kitDiff)]);
            for (const key of allKeys) {
                if (!deepEqual((computedDiff as any)[key], (kitDiff as any)[key])) {
                    console.log(`[DEBUG] [KIT-DIFF-TEST] Difference in key: ${key}`);
                    console.log("[DEBUG] [KIT-DIFF-TEST] Computed:", JSON.stringify((computedDiff as any)[key], null, 2).substring(0, 500));
                    console.log("[DEBUG] [KIT-DIFF-TEST] Expected:", JSON.stringify((kitDiff as any)[key], null, 2).substring(0, 500));
                }
            }
        }
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
    const kit = MetabolismKit as Kit;
    it("Nakagin Capsule Tower should flatten with correct planes and centers", () => {
        const design = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower");
        expect(design).toBeDefined();
        const expectedDesign = kit.designs?.find((d) => d.name === "Flat" && d.parent?.guid === design?.guid);
        expect(expectedDesign).toBeDefined();
        const flatDesignDiff = flattenDesign(kit, design!.guid);
        const flatDesign = applyDesignDiff(design!, flatDesignDiff);
        flatDesign!.pieces?.forEach((p) => {
            const expectedPiece = expectedDesign!.pieces?.find((ep) => ep.name === p.name);
            expect(expectedPiece).toBeDefined();
            expect(p.plane).toBeDefined();
            expect(p.center).toBeDefined();
            expect(planesEqual(p.plane, expectedPiece!.plane)).toBe(true);
            expect(centersEqual(p.center, expectedPiece!.center)).toBe(true);
        });
    });
    it("Slanted should flatten with correct planes and centers", () => {
        const design = kit.designs?.find((d) => d.name === "Slanted");
        expect(design).toBeDefined();
        const expectedDesign = kit.designs?.find((d) => d.name === "Flat" && d.parent?.guid === design?.guid);
        expect(expectedDesign).toBeDefined();
        const flatDesignDiff = flattenDesign(kit, design!.guid);
        const flatDesign = applyDesignDiff(design!, flatDesignDiff);
        flatDesign!.pieces?.forEach((p) => {
            const expectedPiece = expectedDesign!.pieces?.find((ep) => ep.name === p.name);
            expect(expectedPiece).toBeDefined();
            expect(p.plane).toBeDefined();
            expect(p.center).toBeDefined();
            expect(planesEqual(p.plane, expectedPiece!.plane)).toBe(true);
            expect(centersEqual(p.center, expectedPiece!.center)).toBe(true);
        });
    });

    it("Twisted should flatten with correct planes and centers", () => {
        const design = kit.designs?.find((d) => d.name === "Twisted");
        expect(design).toBeDefined();
        const expectedDesign = kit.designs?.find((d) => d.name === "Flat" && d.parent?.guid === design?.guid);
        expect(expectedDesign).toBeDefined();
        const flatDesignDiff = flattenDesign(kit, design!.guid);
        const flatDesign = applyDesignDiff(design!, flatDesignDiff);
        flatDesign!.pieces?.forEach((p) => {
            const expectedPiece = expectedDesign!.pieces?.find((ep) => ep.name === p.name);
            expect(expectedPiece).toBeDefined();
            expect(p.plane).toBeDefined();
            expect(p.center).toBeDefined();
            expect(planesEqual(p.plane, expectedPiece!.plane)).toBe(true);
            expect(centersEqual(p.center, expectedPiece!.center)).toBe(true);
        });
    });

    it("Dancing should flatten with correct planes and centers", () => {
        const design = kit.designs?.find((d) => d.name === "Dancing");
        expect(design).toBeDefined();
        const expectedDesign = kit.designs?.find((d) => d.name === "Flat" && d.parent?.guid === design?.guid);
        expect(expectedDesign).toBeDefined();
        const flatDesignDiff = flattenDesign(kit, design!.guid);
        const flatDesign = applyDesignDiff(design!, flatDesignDiff);
        flatDesign!.pieces?.forEach((p) => {
            const expectedPiece = expectedDesign!.pieces?.find((ep) => ep.name === p.name);
            expect(expectedPiece).toBeDefined();
            expect(p.plane).toBeDefined();
            expect(p.center).toBeDefined();
            expect(planesEqual(p.plane, expectedPiece!.plane)).toBe(true);
            expect(centersEqual(p.center, expectedPiece!.center)).toBe(true);
        });
    });

    it("Capsule Dream should flatten with correct planes and centers", () => {
        const design = kit.designs?.find((d) => d.name === "Capsule Dream");
        expect(design).toBeDefined();
        const expectedDesign = kit.designs?.find((d) => d.name === "Flat" && d.parent?.guid === design?.guid);
        expect(expectedDesign).toBeDefined();
        const flatDesignDiff = flattenDesign(kit, design!.guid);
        const flatDesign = applyDesignDiff(design!, flatDesignDiff);
        flatDesign!.pieces?.forEach((p) => {
            const expectedPiece = expectedDesign!.pieces?.find((ep) => ep.name === p.name);
            expect(expectedPiece).toBeDefined();
            expect(p.plane).toBeDefined();
            expect(p.center).toBeDefined();
            expect(planesEqual(p.plane, expectedPiece!.plane)).toBe(true);
            expect(centersEqual(p.center, expectedPiece!.center)).toBe(true);
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
