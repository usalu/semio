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

import { CapsuleDreamFlatDesign, MetabolismKit, MetabolismKitDiff, MetabolismKitDiffed, MetabolismKitDiffInverted, NakaginCapsuleTowerDancingFlatDesign, NakaginCapsuleTowerFlatDesign, NakaginCapsuleTowerSlantedFlatDesign, NakaginCapsuleTowerTwistedFlatDesign } from "@semio/assets";
import { describe, expect, it } from "vitest";
import { applyDesignDiff, applyKitDiff, areKitsEqual, deepEqual, Design, exportKit, flattenDesign, getKitDiff, importKit, inverseKitDiff, Kit, KitSchema, Plane } from "./semio";

// #region Test Helpers

const TOLERANCE = 0.0001;

/**
 * Computes a detailed diff between two arbitrary objects for testing purposes.
 * Returns an array of difference descriptions showing path and value mismatches.
 * Skips guid, createdAt, and updatedAt fields as they are generated at runtime.
 */
const computeObjectDiff = (a: any, b: any, path = ''): string[] => {
    const diffs: string[] = [];
    
    // Skip guid, createdAt, updatedAt fields as they're runtime-generated
    if (path.endsWith('.guid') || path.endsWith('.createdAt') || path.endsWith('.updatedAt')) {
        return diffs;
    }
    
    if (a === b) return diffs;
    
    if (a === null || b === null || a === undefined || b === undefined) {
        if (a !== b) {
            diffs.push(`${path}: ${JSON.stringify(a)} vs ${JSON.stringify(b)}`);
        }
        return diffs;
    }
    
    if (typeof a !== typeof b) {
        diffs.push(`${path}: type ${typeof a} vs ${typeof b}`);
        return diffs;
    }
    
    if (Array.isArray(a) && Array.isArray(b)) {
        if (a.length !== b.length) {
            diffs.push(`${path}: array length ${a.length} vs ${b.length}`);
        }
        const maxLen = Math.max(a.length, b.length);
        for (let i = 0; i < maxLen; i++) {
            if (i >= a.length) {
                diffs.push(`${path}[${i}]: missing in a`);
            } else if (i >= b.length) {
                diffs.push(`${path}[${i}]: missing in b`);
            } else {
                diffs.push(...computeObjectDiff(a[i], b[i], `${path}[${i}]`));
            }
        }
        return diffs;
    }
    
    if (typeof a === 'object' && typeof b === 'object') {
        const keysA = Object.keys(a);
        const keysB = Object.keys(b);
        const allKeys = new Set([...keysA, ...keysB]);
        
        for (const key of allKeys) {
            if (!(key in a)) {
                diffs.push(`${path}.${key}: missing in a`);
            } else if (!(key in b)) {
                diffs.push(`${path}.${key}: missing in b`);
            } else {
                diffs.push(...computeObjectDiff(a[key], b[key], path ? `${path}.${key}` : key));
            }
        }
        return diffs;
    }
    
    if (a !== b) {
        diffs.push(`${path}: ${JSON.stringify(a)} vs ${JSON.stringify(b)}`);
    }
    
    return diffs;
};

// #endregion Test Helpers

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
        // 1. Compute diff from original to diffed and verify it matches the generated diff exactly
        const computedDiff = getKitDiff(kitOriginal, kitDiffed);
        const diffDiffs = computeObjectDiff(computedDiff, kitDiff);
        if (diffDiffs.length > 0) {
            console.log('[DEBUG] Computed diff vs expected diff differences:');
            diffDiffs.slice(0, 20).forEach(d => console.log(`  ${d}`));
        }
        expect(diffDiffs.length).toBe(0);

        // 2. Compute inverse diff from diffed to original and verify it matches the generated inverse exactly
        const computedInverseDiff = inverseKitDiff(kitOriginal, kitDiff);
        const inverseDiffs = computeObjectDiff(computedInverseDiff, kitDiffInverted);
        if (inverseDiffs.length > 0) {
            console.log('[DEBUG] Computed inverse diff vs expected inverse diff differences:');
            inverseDiffs.slice(0, 20).forEach(d => console.log(`  ${d}`));
        }
        expect(inverseDiffs.length).toBe(0);

        // 3. Apply forward diff to original and verify result matches diffed kit exactly
        const appliedForward = applyKitDiff(kitOriginal, kitDiff);
        const forwardDiffs = computeObjectDiff(appliedForward, kitDiffed);
        if (forwardDiffs.length > 0) {
            console.log('[DEBUG] Applied forward vs expected diffed differences:');
            forwardDiffs.slice(0, 20).forEach(d => console.log(`  ${d}`));
        }
        expect(forwardDiffs.length).toBe(0);

        // 4. Apply inverse diff to diffed kit and verify result matches original exactly
        const appliedInverse = applyKitDiff(kitDiffed, kitDiffInverted);
        const inverseDiffs2 = computeObjectDiff(appliedInverse, kitOriginal);
        if (inverseDiffs2.length > 0) {
            console.log('[DEBUG] Applied inverse vs original differences:');
            inverseDiffs2.slice(0, 20).forEach(d => console.log(`  ${d}`));
        }
        expect(inverseDiffs2.length).toBe(0);
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
        // Normalize the fixture data to fix invalid design names and remove invalid connections
        const fixedKit = structuredClone(MetabolismKit);
        if (fixedKit.designs) {
            for (const design of fixedKit.designs) {
                // Fix design name if it's an object
                if (typeof design.name !== 'string') {
                    design.name = (design.name as any)?.en || 'Unknown';
                }
                // Remove connections with missing ports
                if (design.connections) {
                    design.connections = design.connections.filter(
                        (c: any) => c.connected?.port && c.connecting?.port
                    );
                }
            }
        }
        // Normalize types
        if (fixedKit.types) {
            for (const type of fixedKit.types) {
                // Remove authors (not supported by SQL schema yet)
                delete (type as any).authors;
                // Normalize boolean fields (false -> undefined)
                if (type.isAbstract === false) delete (type as any).isAbstract;
                if (type.virtual === false) delete (type as any).virtual;
                // Normalize ports
                if (type.ports) {
                    for (const port of type.ports) {
                        if (port.mandatory === false) delete (port as any).mandatory;
                        // Normalize empty string attributes
                        if (port.attributes) {
                            for (const attr of port.attributes) {
                                if (attr.definition === '') delete (attr as any).definition;
                                if (attr.value === '') delete (attr as any).value;
                            }
                        }
                    }
                }
                // Normalize empty string attributes
                if (type.attributes) {
                    for (const attr of type.attributes) {
                        if (attr.definition === '') delete (attr as any).definition;
                        if (attr.value === '') delete (attr as any).value;
                    }
                }
            }
        }
        const originalKit = fixedKit as unknown as Kit;
        const files = new Map<string, Blob>();

        const zipBlob = await exportKit(originalKit, files);

        expect(zipBlob).toBeInstanceOf(Blob);
        expect(zipBlob.size).toBeGreaterThan(0);

        const url = URL.createObjectURL(zipBlob);

        const { kit: importedKit, files: importedFiles } = await importKit(url);

        URL.revokeObjectURL(url);

        // Debug: Find differences
        const diffs: string[] = [];
        const findDiff = (path: string, a: any, b: any): void => {
            if (a === b) return;
            if (typeof a !== typeof b) {
                diffs.push(`${path}: type ${typeof a} vs ${typeof b}`);
                return;
            }
            if (a == null || b == null) {
                if (a !== b) diffs.push(`${path}: ${a} vs ${b}`);
                return;
            }
            if (Array.isArray(a) && Array.isArray(b)) {
                if (a.length !== b.length) {
                    diffs.push(`${path}: array length ${a.length} vs ${b.length}`);
                }
                const len = Math.min(a.length, b.length);
                for (let i = 0; i < len; i++) {
                    findDiff(`${path}[${i}]`, a[i], b[i]);
                }
                return;
            }
            if (typeof a === 'object') {
                const keysA = Object.keys(a);
                const keysB = Object.keys(b);
                const allKeys = new Set([...keysA, ...keysB]);
                for (const k of allKeys) {
                    if (!(k in a)) {
                        diffs.push(`${path}.${k}: missing in a`);
                    } else if (!(k in b)) {
                        diffs.push(`${path}.${k}: missing in b`);
                    } else {
                        findDiff(`${path}.${k}`, a[k], b[k]);
                    }
                }
                return;
            }
            diffs.push(`${path}: ${JSON.stringify(a)} vs ${JSON.stringify(b)}`);
        };

        if (!areKitsEqual(originalKit, importedKit)) {
            findDiff('kit', originalKit, importedKit);
            throw new Error(`Kits not equal. First 10 differences:\n${diffs.slice(0, 10).join('\n')}`);
        }

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
