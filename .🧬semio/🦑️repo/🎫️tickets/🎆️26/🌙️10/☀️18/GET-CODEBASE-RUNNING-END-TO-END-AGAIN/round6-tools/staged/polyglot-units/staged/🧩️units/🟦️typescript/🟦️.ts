import type {TypeScriptUnitPlanV1, TypeScriptUnitReceiptV1, TypeScriptUnitV1} from "../../🟦️typescript/🧩️units/🟦️.ts";
import {sha256V1} from "../🔏️key/🟦️.ts";
import {sortedCodePointsV1, type UnitGraphV1, type UnitV1} from "../🕸️graph/🟦️.ts";
import type {ExecuteOutcomeV1} from "../🏃️engine/🟦️.ts";

/** 🟦️ The id a TypeScript check unit has in the neutral graph. */
export const typeScriptNeutralIdV1 = (unit: Pick<TypeScriptUnitV1, "id">): string => "typescript:" + unit.id;

/** 🟦️ Wraps the TypeScript unit plan (the live engine in `🏗️build/🟦️typescript/🧩️units`) into the neutral graph without re-deriving anything: files become sources, JSON, ambient and manifest files inputs, dependencies `interface` edges, a leaf unit (nobody consumes it, no declaration emit) has no interface. The TypeScript engine stays the executor; this graph is what the Nx and Bazel emitters consume. */
export function typeScriptPlanToUnitsV1(plan: TypeScriptUnitPlanV1, host: {readonly compiler: string; readonly options: string; readonly environment: string}): UnitGraphV1 {
  const units: UnitV1[] = plan.units.map(unit => ({
    id: typeScriptNeutralIdV1(unit), language: "typescript", kind: "library", label: unit.label,
    sources: sortedCodePointsV1(unit.files), inputs: sortedCodePointsV1(new Set([...unit.json, ...plan.ambient, ...plan.manifests])).filter(path => !unit.files.includes(path)),
    dependencies: unit.dependencies.map(dependency => ({unit: "typescript:" + dependency, use: "interface" as const})).sort((a, b) => a.unit < b.unit ? -1 : 1), externals: [],
    interface: unit.leaf ? null : {kind: "typescript-declarations"}, outputs: unit.leaf ? [] : ["declarations"], toolchain: {id: host.compiler, parts: [host.options, host.environment]},
    action: {argv: ["repo", "typecheck", "units", "--only", unit.id], env: {}},
  }));
  return {version: 1, units};
}
/** 🟦️ Maps a finished TypeScript unit check onto the neutral executor outcome: errors fail the unit, the declaration tree digest is both output and interface (a leaf has none). */
export function typeScriptOutcomeV1(receipt: Pick<TypeScriptUnitReceiptV1, "errors" | "boundary" | "diagnostics" | "declarations" | "milliseconds">, unit: Pick<TypeScriptUnitV1, "leaf">): ExecuteOutcomeV1 {
  return {ok: receipt.errors === 0, diagnostics: receipt.diagnostics, outputs: sha256V1(receipt.declarations), interface: unit.leaf ? null : receipt.declarations, milliseconds: receipt.milliseconds};
}
