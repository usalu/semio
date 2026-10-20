/** 📐️ CAD renderer geometry targets: the box and sphere preview layouts, the vector and bounding box readers and the display preview transforms. */
import { type Object3D as ThreeObject3D } from "@semio-tech/ui-react";
import { type Vec3 } from "@semio-tech/cad-engine-ts/registry";
import { type ModelEntityKind } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { type SpatialPreviewKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { DisplayModel } from "../../../🎬️actions/🖼️display-model/🟦️.ts";
import { scenePreview } from "../../🔌️ports/🟦️.ts";

// #region 🪩️ArchivedFootprints
export function isVec3Record(v: unknown): v is Vec3 {
  return Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number");
}

// #endregion 🪩️ArchivedFootprints
// #region 📐️Layout
/** 📐️ Center and axis-aligned scale for a unit `BoxGeometry` from two XY footprint corners and height. */
export function computeBoxPreviewLayout(cornerA: Vec3, cornerB: Vec3, height: number, preview: SpatialPreviewKernel = scenePreview()): { readonly position: Vec3; readonly scale: Vec3 } {
  return preview.computeBoxPreviewLayout(cornerA, cornerB, height);
}

/** 🟦️ Center and radius for the live sphere preview while the radius point is moving. */
export function computeSpherePreviewLayout(center: Vec3 | null, cursor: Vec3 | null): { readonly position: Vec3; readonly radius: number } | null {
  if (!center || !cursor) return null;
  const radius = Math.hypot(cursor[0] - center[0], cursor[1] - center[1], cursor[2] - center[2]);
  return radius > 1e-9 ? { position: center, radius } : null;
}

export function readVec3(v: unknown): Vec3 | null {
  if (Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number")) return v as unknown as Vec3;
  return null;
}

export function readNumber(v: unknown): number | null {
  return typeof v === "number" && Number.isFinite(v) ? v : null;
}

export function readVec3Array(v: unknown): readonly Vec3[] {
  if (!Array.isArray(v)) return [];
  return v.filter(isVec3Record) as readonly Vec3[];
}

/** 📦️ Axis-aligned bounds for geometry highlight wireframes. */
export function bboxFromPoints(points: readonly Vec3[], preview: SpatialPreviewKernel = scenePreview()): { readonly min: Vec3; readonly max: Vec3 } | null {
  return preview.aabbFromPoints(points);
}

/** 📦️ Twelve edges of an axis-aligned box for preview line rendering. */
export function bboxWireSegments(min: Vec3, max: Vec3): readonly (readonly [Vec3, Vec3])[] {
  const [x0, y0, z0] = min;
  const [x1, y1, z1] = max;
  const c: readonly Vec3[] = [
    [x0, y0, z0],
    [x1, y0, z0],
    [x1, y1, z0],
    [x0, y1, z0],
    [x0, y0, z1],
    [x1, y0, z1],
    [x1, y1, z1],
    [x0, y1, z1],
  ];
  const idx: readonly (readonly [number, number])[] = [
    [0, 1],
    [1, 2],
    [2, 3],
    [3, 0],
    [4, 5],
    [5, 6],
    [6, 7],
    [7, 4],
    [0, 4],
    [1, 5],
    [2, 6],
    [3, 7],
  ];
  return idx.map(([a, b]) => [c[a]!, c[b]!] as const);
}

export function parseDisplaySelectionTargets(v: unknown): readonly { readonly kind: ModelEntityKind; readonly id: string }[] {
  if (!Array.isArray(v)) return [];
  const out: { kind: ModelEntityKind; id: string }[] = [];
  for (const raw of v) {
    if (!raw || typeof raw !== "object") continue;
    const o = raw as Record<string, unknown>;
    const kind = o.kind;
    const id = o.id;
    if (typeof kind === "string" && typeof id === "string") out.push({ kind: kind as ModelEntityKind, id });
  }
  return out;
}

/** 🖼️ Maps declarative `previewKind` + params to a point transform for geometry wireframes. */
export function transformPointsForPreviewKind(previewKind: string, params: Record<string, unknown>, preview: SpatialPreviewKernel = scenePreview()): (point: Vec3) => Vec3 {
  return preview.transformPointsForPreviewKind(previewKind, params);
}

/** 🖼️ Active geometry point transform from move/copy/mirror/rotate/scale preview display items. */
export function geometryPreviewTransformFromDisplay(model: DisplayModel): ((point: Vec3) => Vec3) | null {
  for (const item of model.items) {
    if (item.kind !== "preview" || !item.params) continue;
    const previewKind = typeof item.params.previewKind === "string" ? item.params.previewKind : "";
    if (!previewKindUsesGeometryWireframe(previewKind)) continue;
    if (previewKind === "move-preview" || previewKind === "copy-preview" || previewKind === "mirror-preview" || previewKind === "rotate-preview" || previewKind === "scale-preview" || previewKind === "scale1d-preview") {
      return transformPointsForPreviewKind(previewKind, item.params);
    }
  }
  return null;
}

export function previewKindUsesGeometryWireframe(previewKind: string): boolean {
  return (
    previewKind === "selected-objects" ||
    previewKind === "move-preview" ||
    previewKind === "copy-preview" ||
    previewKind === "mirror-preview" ||
    previewKind === "rotate-preview" ||
    previewKind === "scale-preview" ||
    previewKind === "scale1d-preview" ||
    previewKind.endsWith("-selection") ||
    previewKind.startsWith("boolean-") ||
    previewKind === "highlight-curves" ||
    previewKind === "cutters" ||
    previewKind === "split-objects" ||
    previewKind === "trim-preview" ||
    previewKind === "extrusion" ||
    previewKind === "network-curves"
  );
}

export const raycastNone: ThreeObject3D["raycast"] = () => undefined;
// #endregion 📐️Layout
