// #region Header

// js/semio/semio.test.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion Header

import { InvalidKit, InvalidKitValidation, MetabolismKit, MetabolismKitDiff, MetabolismKitDiffed, MetabolismKitDiffInverted } from "@semio/assets";
import { describe, expect, it } from "vitest";
import {
  applyDesignDiff,
  applyKitDiff,
  areKitDiffsEqual,
  areKitsEqual,
  areValidationResultsEqual,
  deserializeKit,
  exportKit,
  flattenDesign,
  getKitDiff,
  hasErrors,
  importKit,
  inverseKitDiff,
  Kit,
  Plane,
  serializeKit,
  toValidationResult,
  validateKit,
  ValidationResult,
} from "./semio";
import {
  getKitDiagramShapeStrategy,
  KIT_DIAGRAM_CIRCLE_FRAME,
  KIT_DIAGRAM_COLLIDE_RADIUS,
  KIT_DIAGRAM_LONG_RECTANGLE_FRAME,
  KIT_DIAGRAM_RECTANGLE_FRAME,
  KIT_DIAGRAM_TRIANGLE_FRAME,
  resolveKitDiagramAnchorPair,
  resolveKitDiagramProximityAnchor,
} from "./sketchpad/kitSelectionHelpers";

const TOLERANCE = 0.001;

const planesEqual = (p1?: Plane, p2?: Plane): boolean => {
  if (!p1 || !p2) return false;
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

const findDesign = (kit: Kit, name: string, parentName?: string) => {
  let parentGuid: string | undefined;
  if (parentName) {
    const p = kit.designs?.find(d => d.name === parentName);
    if (!p) throw new Error(`Parent ${parentName} not found`);
    parentGuid = p.guid;
  }
  const d = kit.designs?.find(d => d.name === name && (parentGuid ? d.parent?.guid === parentGuid : !d.parent));
  if (!d) throw new Error(`Design ${name} not found`);
  return d;
}

describe("Diff", () => {
  describe("Metabolism", () => {
    const kitOriginal = { ...(MetabolismKit as any), designs: (MetabolismKit as any).designs?.filter((d: any) => !d.parent) };
    const kitDiff = MetabolismKitDiff as any;
    const kitDiffInverted = MetabolismKitDiffInverted as any;
    const kitDiffed = MetabolismKitDiffed as any;

    it("Kit + Diff = DiffedKit & DiffedKit + InvertedDiff = Kit", () => {
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
});

describe("Flatten", () => {
  const kit = MetabolismKit as Kit;

  const testFlatten = (designName: string, parentName?: string) => {
    const design = findDesign(kit, designName, parentName);
    const expectedDesign = kit.designs?.find((d) => d.name === "Flat" && d.parent?.guid === design.guid);
    expect(expectedDesign).toBeDefined();
    const flatDesignDiff = flattenDesign(kit, design.guid);
    const flatDesign = applyDesignDiff(design, flatDesignDiff);

    flatDesign!.pieces?.forEach((p) => {
      const expectedPiece = expectedDesign!.pieces?.find((ep) => ep.name === p.name);
      expect(expectedPiece).toBeDefined();
      expect(p.plane).toBeDefined();
      expect(p.center).toBeDefined();
      expect(planesEqual(p.plane, expectedPiece!.plane)).toBe(true);
      expect(centersEqual(p.center, expectedPiece!.center)).toBe(true);
    });
  }

  describe("Nakagin Capsule Tower", () => {
    it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
      testFlatten("Nakagin Capsule Tower");
    });
    describe("Slanted", () => {
      it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
        testFlatten("Slanted", "Nakagin Capsule Tower");
      });
    });
    describe("Twisted", () => {
      it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
        testFlatten("Twisted", "Nakagin Capsule Tower");
      });
    });
    describe("Dancing", () => {
      it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
        testFlatten("Dancing", "Nakagin Capsule Tower");
      });
    });
  });

  describe("Capsule Dream", () => {
    it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
      testFlatten("Capsule Dream");
    });
  });
});

describe("Roundtrip", () => {
  describe("Json", () => {
    describe("Metabolism", () => {
      it("Kit -> Json -> Kit", async () => {
        const kit = MetabolismKit as unknown as Kit;
        const serializedKit = serializeKit(kit);
        const deserializedKit = deserializeKit(serializedKit);
        expect(areKitsEqual(kit, deserializedKit)).toBe(true);
      });
    });
  });

  describe("Zip", () => {
    describe("Metabolism", () => {
      it("Zip -> Kit -> Zip -> Kit", async () => {
        const fs = await import("node:fs");
        const path = await import("node:path");
        const zipPath = path.join(__dirname, "../../assets/semio/metabolism.zip");
        const zipBuffer = fs.readFileSync(zipPath);

        const { kit, files } = await importKit(zipBuffer.buffer);
        expect(kit.guid).toBeDefined();
        expect(kit.name).toBe("Metabolism");
        expect(kit.types?.length).toBeGreaterThan(0);
        expect(kit.designs?.length).toBeGreaterThan(0);
        expect(files.size).toBeGreaterThan(0);

        const exportedZip = await exportKit(kit, files);
        const { kit: kit2, files: files2 } = await importKit(exportedZip);

        expect(kit2.guid).toBe(kit.guid);
        expect(kit2.name).toBe(kit.name);
        expect(kit2.types?.length).toBe(kit.types?.length);
        expect(kit2.designs?.length).toBe(kit.designs?.length);
        expect(files2.size).toBe(files.size);
      });
    });
  });
});

describe("Validation", () => {
  describe("Metabolism", () => {
    it("Metabolism Kit -> Validate = Empty report", () => {
      const validKit = MetabolismKit as unknown as Kit;
      expect(hasErrors(validateKit(validKit))).toBe(false);
    });
  });

  describe("Invalid", () => {
    it("Invalid Kit -> Validate = Invalid Report", () => {
      const invalidKit = InvalidKit as unknown as Kit;
      const result = toValidationResult(validateKit(invalidKit));
      const expected = InvalidKitValidation as ValidationResult;
      expect(areValidationResultsEqual(result, expected)).toBe(true);
    });
  });
});

describe("Kit Diagram Geometry", () => {
  it("maps shape strategies to deterministic frames and snap points", () => {
    expect(getKitDiagramShapeStrategy("design").frame).toEqual(KIT_DIAGRAM_CIRCLE_FRAME);
    expect(getKitDiagramShapeStrategy("type").frame).toEqual(KIT_DIAGRAM_RECTANGLE_FRAME);
    expect(getKitDiagramShapeStrategy("file").frame).toEqual(KIT_DIAGRAM_TRIANGLE_FRAME);
    expect(getKitDiagramShapeStrategy("quality").frame).toEqual(KIT_DIAGRAM_LONG_RECTANGLE_FRAME);
    expect(getKitDiagramShapeStrategy("design").getSnapPoints()).toEqual([
      { id: "n", x: 50, y: 0, side: "top" },
      { id: "e", x: 100, y: 50, side: "right" },
      { id: "s", x: 50, y: 100, side: "bottom" },
      { id: "w", x: 0, y: 50, side: "left" },
    ]);
    expect(getKitDiagramShapeStrategy("type").getSnapPoints()).toEqual([
      { id: "n", x: 60, y: 0, side: "top" },
      { id: "e", x: 120, y: 40, side: "right" },
      { id: "s", x: 60, y: 80, side: "bottom" },
      { id: "w", x: 0, y: 40, side: "left" },
    ]);
    expect(getKitDiagramShapeStrategy("file").getSnapPoints()).toEqual([
      { id: "apex", x: 50, y: 0, side: "top" },
      { id: "base-left", x: 0, y: 100, side: "left" },
      { id: "base-right", x: 100, y: 100, side: "right" },
    ]);
    expect(getKitDiagramShapeStrategy("quality").getSnapPoints()).toEqual([
      { id: "n", x: 80, y: 0, side: "top" },
      { id: "e", x: 160, y: 36, side: "right" },
      { id: "s", x: 80, y: 72, side: "bottom" },
      { id: "w", x: 0, y: 36, side: "left" },
    ]);
    expect(KIT_DIAGRAM_COLLIDE_RADIUS).toBe(80);
  });

  it("resolves nearest anchor pairs for shape combinations", () => {
    const horizontal = resolveKitDiagramAnchorPair(
      { kind: "design", position: { x: 0, y: 0 } },
      { kind: "type", position: { x: 300, y: 0 } },
    );
    expect(horizontal.source.localPoint.id).toBe("e");
    expect(horizontal.target.localPoint.id).toBe("w");
    expect(horizontal.source.absolutePoint).toEqual({ x: 100, y: 50 });
    expect(horizontal.target.absolutePoint).toEqual({ x: 300, y: 40 });

    const vertical = resolveKitDiagramAnchorPair(
      { kind: "type", position: { x: 0, y: 0 } },
      { kind: "design", position: { x: 0, y: 280 } },
    );
    expect(vertical.source.localPoint.id).toBe("s");
    expect(vertical.target.localPoint.id).toBe("n");

    const triangleUp = resolveKitDiagramAnchorPair(
      { kind: "file", position: { x: 220, y: 320 } },
      { kind: "design", position: { x: 220, y: 40 } },
    );
    expect(triangleUp.source.localPoint.id).toBe("apex");
    expect(triangleUp.target.localPoint.id).toBe("s");
  });

  it("resolves proximity anchors from snap points", () => {
    const proximity = resolveKitDiagramProximityAnchor("type:1", { kind: "type", position: { x: 200, y: 100 } }, { x: 340, y: 140 });
    expect(proximity.anchor.localPoint.id).toBe("e");
    expect(proximity.anchor.absolutePoint).toEqual({ x: 320, y: 140 });
  });
});
