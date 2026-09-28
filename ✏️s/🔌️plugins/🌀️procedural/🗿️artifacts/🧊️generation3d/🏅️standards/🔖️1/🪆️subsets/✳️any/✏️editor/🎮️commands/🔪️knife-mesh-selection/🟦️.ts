import { componentGroup, parseComponentTarget } from "../../🎯️selection/🟦️.ts";

/** 📍️ Parameters of a knife cut resolved from one selected mesh face. */
export interface KnifeSelectionParameters { face: number; start: [number, number, number]; end: [number, number, number] }

/** 🔪️ Validates one face and finite distinct points before a graph edit is admitted. */
export function knifeSelectionParameters(ids: readonly string[], payload: { start: number[]; end: number[] }): KnifeSelectionParameters {
  const group = componentGroup(ids), target = parseComponentTarget(ids[0])!;
  if (target.index !== 0 || group.granularity !== "face" || group.components.length !== 1) throw new Error("Select exactly one face of a single mesh");
  const valid = (point: number[]) => Array.isArray(point) && point.length === 3 && point.every(value => typeof value === "number" && Number.isFinite(value) && Math.abs(value) <= 3.4028234663852886e38);
  if (!valid(payload.start) || !valid(payload.end)) throw new Error("Knife points must be finite mesh coordinates");
  if (payload.start.every((value, axis) => Math.fround(value) === Math.fround(payload.end[axis]))) throw new Error("Knife start and end must be different points");
  return { face: group.components[0], start: [...payload.start] as [number, number, number], end: [...payload.end] as [number, number, number] };
}
