/**
 * 🔬️ TypeScript twin of the example-geometry lane. The brep kernel is Rust-only — no reachable
 * TypeScript evaluation path exists for `neuron-kind` chains from this plugin (`brepjs` stays scoped
 * to the `cad` plugin), so this side does not re-run the geometry. It holds the *same committed
 * `🔣️.json` fixture* the Rust lane asserts against to a second, independent derivation: the fixture's
 * schema, its op chain against the example's own `.dsl.semio`, and every closed-form expected number
 * recomputed here from the DSL's own slider values.
 *
 * @see 🦀️.rs — the Rust half, which additionally evaluates and tessellates.
 */

import { readFileSync } from "node:fs";
import { join } from "node:path";
import { expect } from "bun:test";

export const EXAMPLE_GEOMETRY_FIXTURE_SCHEMA = "s.procedural.generation3d.example-geometry/v1";

/**
 * 🚧️ The machine-readable kernel standings a fixture may declare, mirroring `🦀️.rs`'s
 * `KERNEL_STATUSES` exactly. Every `blocked-*` value names one SPECIFIC located kernel defect the
 * Rust lane's run reproduced — never a relaxed expectation: the committed numbers stay what the
 * geometry must be, and the Rust run keeps failing until the named defect is fixed. A fixed defect
 * takes its value out of this list with it.
 */
export const EXAMPLE_GEOMETRY_KERNEL_STATUSES = ["green"];

/** 📐️ One example's committed expected-geometry statement. */
export type ExampleGeometryFixture = {
  schema: string;
  example: string;
  opChain: string[];
  preview: { node: string; channel: string; kind: string };
  tessellationTolerance: number;
  expect: {
    minTriangles: number;
    closed: boolean;
    volume: number | null;
    volumeTolerance: number;
    volumeSource: string;
    edgePerimeter: number | null;
    edgePerimeterTolerance: number;
    boundingBoxMin: [number, number, number];
    boundingBoxMax: [number, number, number];
    boundingBoxTolerance: number;
    kernelVolumeNode: string | null;
    kernelVolumeChannel: string | null;
    kernelVolumeTolerance: number | null;
  };
  delivery: ExampleDeliveryExpectation;
  budget: ExampleBudgetExpectation;
  kernelStatus: string;
};

/**
 * ⏱️ What one example's chain is allowed to COST in wall microseconds on the Rust lane's own native
 * `test` (unoptimized) profile. Geometry that is right and delivered but takes 78 s to appear is an
 * example the user never sees converge, so this row sits beside the geometry and the delivery rather
 * than in a separate performance suite (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️kernel-performance-2026-09-13.md`).
 *
 * These are CEILINGS with deliberate headroom over the measured value, meant to convict an
 * algorithmic regression — the boolean kernel's face-stitch once spent 99% of
 * `🍩️sphere-cut-with-torus`'s 61 s in arbitrary-precision rational predicates — never to police
 * machine-to-machine variance. A number here is lowered after a measured improvement, never raised
 * to admit a regression.
 */
export type ExampleBudgetExpectation = {
  maxEvaluateMicros: number;
  maxTessellateMicros: number;
  maxPreviewTessellateMicros: number;
};

/**
 * 🚚️ What one example's preview must DELIVER across the extension boundary at the LOD the live
 * surface asks for. A correct mesh nobody receives is a blank viewport, so this row is asserted
 * beside the geometry itself.
 */
export type ExampleDeliveryExpectation = {
  lodMode: string;
  minMeshes: number;
  minTriangles: number;
  minEdgeSegments: number;
  maxRoundTrips: number;
  maxChunks: number;
};

/**
 * ⏱️ Tessellate round trips one preview may cost before it has painted. One round trip is one whole
 * `flowEvalTick` — evaluate, invoke the kernel, fold the answer, refresh every window — which is
 * SECONDS in a served wasm build, so this is a user-facing promise, not an efficiency preference.
 * Five of the eight bundled examples used to need five to ten of them and never painted
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️preview-mesh-delivery-2026-09-12.md`).
 */
export const EXAMPLE_DELIVERY_ROUND_TRIP_CEILING = 2;

/** 📥️ Reads the example's DSL asset and its committed expected-stats fixture. */
export function loadExample(here: string, assetDirName: string, assetName: string): { dsl: string; fixture: ExampleGeometryFixture } {
  const dsl = readFileSync(join(here, "../../🖼️assets", assetDirName, "🗣️.dsl.semio"), "utf8");
  const fixture = JSON.parse(readFileSync(join(here, "🔣️.json"), "utf8")) as ExampleGeometryFixture;
  expect(assetName.length).toBeGreaterThan(0);
  return { dsl, fixture };
}

/** ✅️ Every structural invariant the fixture format itself declares. */
export function assertFixtureContract(fixture: ExampleGeometryFixture, exampleId: string): void {
  expect(fixture.schema).toBe(EXAMPLE_GEOMETRY_FIXTURE_SCHEMA);
  expect(fixture.example).toBe(exampleId);
  expect(fixture.opChain.length).toBeGreaterThan(0);
  expect(fixture.preview.kind === "solid" || fixture.preview.kind === "wire").toBe(true);
  expect(fixture.tessellationTolerance).toBeGreaterThan(0);
  expect(fixture.expect.boundingBoxMin.length).toBe(3);
  expect(fixture.expect.boundingBoxMax.length).toBe(3);
  for (let axis = 0; axis < 3; axis += 1) expect(fixture.expect.boundingBoxMax[axis]).toBeGreaterThanOrEqual(fixture.expect.boundingBoxMin[axis]);
  expect(fixture.expect.volume === null || fixture.expect.volumeTolerance > 0).toBe(true);
  expect(fixture.expect.kernelVolumeTolerance === null).toBe(fixture.expect.kernelVolumeNode === null);
  expect(fixture.expect.kernelVolumeTolerance === null || (fixture.expect.kernelVolumeTolerance as number) > 0).toBe(true);
  expect(EXAMPLE_GEOMETRY_KERNEL_STATUSES).toContain(fixture.kernelStatus);
  assertDeliveryContract(fixture);
  assertBudgetContract(fixture);
}

/**
 * ✅️ Every invariant the budget row states, derived here independently of the Rust run. A ceiling
 * above {@link EXAMPLE_BUDGET_INTERACTIVE_CEILING_MICROS} (for a user-facing phase) or
 * {@link EXAMPLE_BUDGET_FIDELITY_CEILING_MICROS} (for the fidelity phase) is not a budget at all —
 * it is the absence of one — and the preview LOD is by construction coarser than the fixture's own
 * tessellation tolerance, so its ceiling may never exceed the full-tolerance one.
 */
export function assertBudgetContract(fixture: ExampleGeometryFixture): void {
  const budget = fixture.budget;
  for (const micros of [budget.maxEvaluateMicros, budget.maxTessellateMicros, budget.maxPreviewTessellateMicros]) {
    expect(Number.isInteger(micros)).toBe(true);
    expect(micros).toBeGreaterThan(0);
  }
  for (const micros of [budget.maxEvaluateMicros, budget.maxPreviewTessellateMicros]) expect(micros).toBeLessThanOrEqual(EXAMPLE_BUDGET_INTERACTIVE_CEILING_MICROS);
  expect(budget.maxTessellateMicros).toBeLessThanOrEqual(EXAMPLE_BUDGET_FIDELITY_CEILING_MICROS);
  expect(budget.maxPreviewTessellateMicros).toBeLessThanOrEqual(budget.maxTessellateMicros);
}

/**
 * ⏱️ The hard bound the two USER-FACING phases sit under: evaluating the op chain and tessellating at
 * the LOD the live preview asks for are what the playground waits on, and neither may be allowed
 * more than two seconds of the Rust lane's native `test`-profile wall time. The wasm guest runs the
 * identical code at `opt-level = 2` (root `Cargo.toml`'s `[profile.wasm-dev.package]` overrides)
 * inside a budgeted resumable job, so a phase over this bound cannot converge in the playground
 * within the user-facing target the ticket set.
 */
export const EXAMPLE_BUDGET_INTERACTIVE_CEILING_MICROS = 2_000_000;

/**
 * ⏱️ The bound on the FIDELITY phase — tessellation at the fixture's own `tessellationTolerance`,
 * far finer than any LOD the live preview requests, which exists so the committed geometry statement
 * is measured on a converged mesh. Nobody waits on this number in the app, so its ceiling is a
 * regression guard rather than a user-facing promise and is set looser.
 */
export const EXAMPLE_BUDGET_FIDELITY_CEILING_MICROS = 8_000_000;

/**
 * ✅️ Every invariant the delivery row states, derived here independently of the Rust run:
 * a preview that publishes no mesh is a blank viewport, a preview whose mesh carries neither
 * triangles NOR edge segments cannot paint at all, and a preview that needs more than
 * {@link EXAMPLE_DELIVERY_ROUND_TRIP_CEILING} round trips is not delivered in any useful sense.
 * The `wire` previews are the ones that must paint on edges alone.
 */
export function assertDeliveryContract(fixture: ExampleGeometryFixture): void {
  const delivery = fixture.delivery;
  expect(typeof delivery.lodMode).toBe("string");
  expect(delivery.minMeshes).toBeGreaterThanOrEqual(1);
  expect(delivery.minTriangles + delivery.minEdgeSegments).toBeGreaterThan(0);
  expect(delivery.maxRoundTrips).toBeGreaterThanOrEqual(1);
  expect(delivery.maxRoundTrips).toBeLessThanOrEqual(EXAMPLE_DELIVERY_ROUND_TRIP_CEILING);
  expect(delivery.maxChunks).toBe(1);
  if (fixture.preview.kind === "wire") {
    expect(delivery.minTriangles).toBe(0);
    expect(delivery.minEdgeSegments).toBeGreaterThanOrEqual(1);
  } else {
    expect(delivery.minTriangles).toBeGreaterThanOrEqual(fixture.expect.minTriangles);
  }
}

/** 🔗️ The op chain the fixture claims is exactly what the example's DSL wires, node by node. */
export function assertOpChainDeclared(dsl: string, fixture: ExampleGeometryFixture): void {
  for (const kind of fixture.opChain) expect(dsl).toContain(kind);
  expect(dsl).toContain(`id="${fixture.preview.node}"`);
  const declared = [...dsl.matchAll(/neuron-kind=([\w.]+)/g)].map(match => match[1]);
  for (const kind of declared) expect(fixture.opChain).toContain(kind);
}

/** 🎚️ Every `input-slider`'s committed value, keyed by widget id. */
export function sliders(dsl: string): Record<string, number> {
  const values: Record<string, number> = {};
  for (const match of dsl.matchAll(/input-slider id="([^"]+)"[^\n]*?\svalue=(-?[\d.]+)/g)) values[match[1]] = Number(match[2]);
  return values;
}

/** ⚖️ Holds the committed expected volume to a number recomputed here, inside the fixture's own tolerance. */
export function assertExpectedVolume(fixture: ExampleGeometryFixture, recomputed: number): void {
  expect(fixture.expect.volume).not.toBeNull();
  expect(Math.abs((fixture.expect.volume as number) - recomputed)).toBeLessThanOrEqual(fixture.expect.volumeTolerance);
}

/** ⚖️ Same, for the wire-only examples' perimeter. */
export function assertExpectedPerimeter(fixture: ExampleGeometryFixture, recomputed: number): void {
  expect(fixture.expect.edgePerimeter).not.toBeNull();
  expect(Math.abs((fixture.expect.edgePerimeter as number) - recomputed)).toBeLessThanOrEqual(fixture.expect.edgePerimeterTolerance);
}

/** 📦️ Holds the committed bounding box to one recomputed here. */
export function assertExpectedBoundingBox(fixture: ExampleGeometryFixture, min: [number, number, number], max: [number, number, number]): void {
  for (let axis = 0; axis < 3; axis += 1) {
    expect(Math.abs(fixture.expect.boundingBoxMin[axis] - min[axis])).toBeLessThanOrEqual(fixture.expect.boundingBoxTolerance);
    expect(Math.abs(fixture.expect.boundingBoxMax[axis] - max[axis])).toBeLessThanOrEqual(fixture.expect.boundingBoxTolerance);
  }
}

/** ∫️ Composite Simpson quadrature — this side's own numeric integrator, no dependency shared with the Rust or Python lanes. */
export function simpson(f: (x: number) => number, lower: number, upper: number, panels: number): number {
  const steps = panels % 2 === 0 ? panels : panels + 1;
  const width = (upper - lower) / steps;
  let total = f(lower) + f(upper);
  for (let index = 1; index < steps; index += 1) total += f(lower + index * width) * (index % 2 === 0 ? 2 : 4);
  return (total * width) / 3;
}
