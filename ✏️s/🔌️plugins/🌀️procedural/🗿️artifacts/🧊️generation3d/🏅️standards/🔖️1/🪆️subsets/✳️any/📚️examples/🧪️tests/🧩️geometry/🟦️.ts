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
export const EXAMPLE_GEOMETRY_KERNEL_STATUSES = ["green", "blocked-on-fillet-kernel"];

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
  kernelStatus: string;
};

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
