/** 🏗️ Geometric construction capabilities selected by CAD extension owners. */
import type { Vec3 } from "@semio-tech/framework-3d-js";
import { Model } from "../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
import { applyModelDiff, type SpatialKernel } from "../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts";

function constructionPoint(value: unknown): Vec3 {
  if (!Array.isArray(value) || value.length !== 3 || !value.every((coordinate) => typeof coordinate === "number" && Number.isFinite(coordinate))) throw new Error("Construction point must contain three finite coordinates");
  return value as unknown as Vec3;
}

function constructionDimension(value: unknown, fallback: number): number {
  const dimension = value === undefined ? fallback : value;
  if (typeof dimension !== "number" || !Number.isFinite(dimension) || dimension <= 0) throw new Error("Construction dimension must be positive and finite");
  return dimension;
}

/** 🧱️ Extrudes a horizontal linear footprint with explicit thickness, independent of typology names. */
export async function constructLinearPrism(kernel: SpatialKernel, params: Readonly<Record<string, unknown>>) {
  const a = constructionPoint(params.pointA);
  const b = constructionPoint(params.pointB);
  const length = Math.hypot(b[0] - a[0], b[1] - a[1]);
  if (length === 0 || a[2] !== b[2]) throw new Error("Linear prism requires distinct coplanar horizontal points");
  const thickness = constructionDimension(params.thickness, 0.2);
  const height = constructionDimension(params.height, 2.7);
  const offset: Vec3 = [-(b[1] - a[1]) * thickness / (2 * length), (b[0] - a[0]) * thickness / (2 * length), 0];
  const point = (source: Vec3, sign: number): Vec3 => [source[0] + sign * offset[0], source[1] + sign * offset[1], source[2]];
  const points = [point(a, 1), point(a, -1), point(b, -1), point(b, 1), point(a, 1)];
  const model = new Model();
  const profile = await kernel.executeCommandDiff("curve.polyline", { points });
  applyModelDiff(model, profile.diff);
  const wireId = profile.diff.wires?.added?.[0]?.id;
  if (!wireId) throw new Error("Kernel refused the linear prism footprint");
  return kernel.extrudeWireDiff({ wireId, distance: height, direction: [0, 0, 1], model });
}

/** 📦️ Constructs a box from diagonal plan points; zero plan dimensions use the supplied thickness. */
export async function constructBoxFromPoints(kernel: SpatialKernel, params: Readonly<Record<string, unknown>>) {
  const a = constructionPoint(params.pointA);
  const b = constructionPoint(params.pointB);
  const thickness = constructionDimension(params.thickness, 0.2);
  const cornerA: Vec3 = [Math.min(a[0], b[0]), Math.min(a[1], b[1]), Math.min(a[2], b[2])];
  const cornerB: Vec3 = [Math.max(a[0], b[0]) + (a[0] === b[0] ? thickness : 0), Math.max(a[1], b[1]) + (a[1] === b[1] ? thickness : 0), cornerA[2]];
  return kernel.createBoxFromCornersDiff({ cornerA, cornerB, height: constructionDimension(params.height, 2.7) });
}

/** 🧵️ Extrudes a caller-owned profile with a positive height. */
export async function constructPrismFromCurve(kernel: SpatialKernel, params: Readonly<Record<string, unknown>>) {
  if (!(params.model instanceof Model) || typeof params.wireId !== "string" || !params.wireId) throw new Error("Prism construction requires a model and wire id");
  return kernel.extrudeWireDiff({ wireId: params.wireId, distance: constructionDimension(params.height, 2.7), direction: [0, 0, 1], model: params.model });
}
