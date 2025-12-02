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

import { InvalidKit, MetabolismKit, MetabolismKitDiff, MetabolismKitDiffed, MetabolismKitDiffInverted } from "@semio/assets";
import { describe, expect, it } from "vitest";
import { applyDesignDiff, applyKitDiff, areKitDiffsEqual, areKitsEqual, deserializeKit, exportKit, flattenDesign, getKitDiff, hasSemioErrors, importKit, inverseKitDiff, Kit, Plane, serializeKit, validateSemioKit } from "./semio";

const TOLERANCE = 0.001;

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

const centersEqual = (c1: { u: number; v: number } | undefined, c2: { u: number; v: number } | undefined): boolean => {
  if (!c1 || !c2) return c1 === c2;
  return Math.abs(c1.u - c2.u) < TOLERANCE && Math.abs(c1.v - c2.v) < TOLERANCE;
};

describe("Diffs", () => {
  const kitOriginal = { ...(MetabolismKit as any), designs: (MetabolismKit as any).designs?.filter((d: any) => !d.parent) };
  const kitDiff = MetabolismKitDiff as any;
  const kitDiffInverted = MetabolismKitDiffInverted as any;
  const kitDiffed = MetabolismKitDiffed as any;

  it("Kit + Diff = DiffedKit & DiffedKit + InverseDiff = Kit", () => {
    const computedDiff = getKitDiff(kitOriginal, kitDiffed);
    expect(areKitDiffsEqual(computedDiff, kitDiff)).toBe(true);
    const computedInverseDiff = inverseKitDiff(kitOriginal, kitDiff);
    expect(areKitDiffsEqual(computedInverseDiff, kitDiffInverted)).toBe(true);
    const appliedForward = applyKitDiff(kitOriginal, kitDiff);
    expect(areKitsEqual(appliedForward, kitDiffed)).toBe(true);
    const appliedInverse = applyKitDiff(kitDiffed, kitDiffInverted);
    expect(areKitsEqual(appliedInverse, kitOriginal)).toBe(true);
  });
});

describe("Flattening Designs", () => {
  const kit = MetabolismKit as Kit;
  describe("Nakagin Capsule Tower", () => {
    it("Normal", () => {
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
    it("Slanted", () => {
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

    it("Twisted", () => {
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

    it("Dancing", () => {
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
  });

  it("Capsule Dream", () => {
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

describe("Import/Export", () => {
  it("Kit -> JSON -> Kit", async () => {
    const kit = MetabolismKit as unknown as Kit;
    const serializedKit = serializeKit(kit);
    const deserializedKit = deserializeKit(serializedKit);
    expect(areKitsEqual(kit, deserializedKit)).toBe(true);
  });
  it("Kit -> Zip -> Kit", async () => {
    const originalKit = MetabolismKit as unknown as Kit;
    const files = new Map<string, Blob>();
    const dummyFile = new Blob(["dummy content"], { type: "text/plain" });
    files.set("dummy.txt", dummyFile);
    const zipBlob = await exportKit(originalKit, files);
    expect(zipBlob).toBeInstanceOf(Blob);
    expect(zipBlob.size).toBeGreaterThan(0);
    const url = URL.createObjectURL(zipBlob);
    const { kit: importedKit, files: importedFiles } = await importKit(url);
    URL.revokeObjectURL(url);
    expect(areKitsEqual(originalKit, importedKit)).toBe(true);
    expect(importedFiles.size).toBe(files.size);
  });
});

describe("Validation", () => {
  it("Valid kit has no errors", () => {
    const kit = MetabolismKit as unknown as Kit;
    const result = validateSemioKit(kit);
    expect(hasSemioErrors(result)).toBe(false);
  });

  it("Invalid kit has all expected errors", () => {
    const kit = InvalidKit as unknown as Kit;
    const result = validateSemioKit(kit);
    expect(hasSemioErrors(result)).toBe(true);
    const ruleIds = new Set(result.issues.map((i) => i.ruleId));
    const expectedRules = [
      "guid-unique",
      "type-name-unique",
      "design-name-unique",
      "piece-name-unique",
      "quality-name-unique",
      "interface-name-unique",
      "file-name-unique",
      "folder-name-unique",
      "port-name-unique",
      "model-name-unique",
      "layer-path-unique",
    ];
    expectedRules.forEach((ruleId) => {
      expect(ruleIds.has(ruleId), `Missing validation rule: ${ruleId}`).toBe(true);
    });
    expect(ruleIds.size).toBe(expectedRules.length);
  });

  it("Fixes can be applied to resolve issues", () => {
    const kit = InvalidKit as unknown as Kit;
    const result = validateSemioKit(kit);
    expect(result.issues.length).toBeGreaterThan(0);
    const issue = result.issues[0];
    expect(issue.fixes.length).toBeGreaterThan(0);
    const fix = issue.fixes[0];
    const fixedKit = applyKitDiff(kit, fix.diff);
    const revalidated = validateSemioKit(fixedKit);
    expect(revalidated.issues.some((i) => i.ruleId === issue.ruleId && i.location.entityGuid === issue.location.entityGuid)).toBe(false);
  });
});
