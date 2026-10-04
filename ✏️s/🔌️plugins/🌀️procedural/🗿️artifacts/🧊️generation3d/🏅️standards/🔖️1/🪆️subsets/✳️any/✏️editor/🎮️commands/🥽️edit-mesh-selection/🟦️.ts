import { componentGroup, parseComponentTarget, selectedAnalyticLabels } from "../../🎯️selection/🟦️.ts";

/** 🎛️ User parameters retained as absolute inputs on one mesh operation. */
export interface MeshSelectionEdit { operation: string; amount: number; cuts: number; dx: number; dy: number; dz: number; width: number; segments: number; mergeMode: string; tolerance: number; radius: number; grid: number; center: number[] }

/** 🥽️ Validates a selected component set and resolves the typed graph inputs. */
export function meshSelectionInputs(ids: readonly string[], payload: MeshSelectionEdit): Record<string, number | string | number[]> {
  const operation = payload.operation;
  const vertices = ["moveVertices", "dissolveVertices", "mergeVertices", "moveProportional", "snapVertices"];
  const edges = ["loopCut", "bevel", "dissolveEdges", "filletEdges", "chamferEdges"];
  const analytic = ["filletEdges", "chamferEdges", "shell"].includes(operation);
  if (!["extrude", "inset", "subdivide", "flip", "deleteFaces", "shell", ...vertices, ...edges].includes(operation)) throw new Error("Unknown mesh operation");
  const granularity = vertices.includes(operation) ? "vertex" : edges.includes(operation) ? "edge" : "face";
  const group = componentGroup(ids), target = parseComponentTarget(ids[0])!;
  if (group.granularity !== granularity || !analytic && target.index !== 0) throw new Error("Select matching components of a single mesh");
  if (!Array.isArray(payload.center) || payload.center.length !== 3 || [payload.amount, payload.dx, payload.dy, payload.dz, payload.width, payload.tolerance, payload.radius, payload.grid, ...payload.center].some(value => !Number.isFinite(value))) throw new Error("Mesh edit values must be finite");
  if (["inset", "chamferEdges", "shell"].includes(operation) && payload.amount <= 0) throw new Error("Inset amount must be positive");
  if (!Number.isInteger(payload.cuts) || payload.cuts < 1 || payload.cuts > 256) throw new Error("Cuts must be an integer from 1 to 256");
  if (payload.width <= 0 || payload.radius <= 0 || payload.grid <= 0) throw new Error("Width, radius, and grid must be positive");
  if (!Number.isInteger(payload.segments) || payload.segments < 1 || payload.segments > 64) throw new Error("Bevel segments must be an integer from 1 to 64");
  if (payload.tolerance < 0 || !["first", "center", "distance"].includes(payload.mergeMode)) throw new Error("Merge mode must be first, center, or distance with nonnegative tolerance");
  if (analytic) {
    const labels = selectedAnalyticLabels(ids);
    return operation === "filletEdges" ? { edgeLabels: JSON.stringify(labels), radius: payload.radius } : operation === "chamferEdges" ? { edgeLabels: JSON.stringify(labels), distance: payload.amount } : { faceLabels: JSON.stringify(labels), thickness: payload.amount };
  }
  if (target.analytic) throw new Error("Analytic components require a B-Rep operation");
  const field = operation === "moveVertices" ? "vertices" : granularity === "edge" ? "edges" : granularity === "vertex" ? "selection" : "faces";
  const inputs: Record<string, number | string | number[]> = { [field]: JSON.stringify(group.components) };
  if (operation === "moveVertices" || operation === "moveProportional") inputs.offset = [payload.dx, payload.dy, payload.dz];
  if (operation === "loopCut") inputs.cuts = payload.cuts;
  if (operation === "extrude") inputs.distance = payload.amount;
  if (operation === "inset") inputs.amount = payload.amount;
  if (operation === "bevel") Object.assign(inputs, { amount: payload.width, segments: payload.segments });
  if (operation === "mergeVertices") Object.assign(inputs, { mode: payload.mergeMode, tolerance: payload.tolerance });
  if (operation === "moveProportional") Object.assign(inputs, { center: [...payload.center], radius: payload.radius });
  if (operation === "snapVertices") inputs.grid = payload.grid;
  return inputs;
}
