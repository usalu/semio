/** 🧊️ Prism depth: an extruded solid shape is ranked by the nearest corner of its own prism for every sweep and in every view.
 *
 * THE REFERENCE is numpy (🐍️.py): it places the profile at both ends of the sweep in the frame of the bone by the definition of the sweeps.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️prism-depth/🔣️.json — the vectors
 * @see ../../🔨️modules/🧮️evaluation/🟦️.ts — `shapeDepth`
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, PlanarViewpoint, Shape } from "../../🧬️schema/🟦️.ts";
import { evaluate, shapeDepth } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Prism = { id: string; solid: { kind: "extruded"; sweep: "u" | "v" | "w"; z0: number; z1: number; zTip?: [number, number] }; points: [number, number][] };
type Vectors = { scale: number; bone: string; toleranceM: number; poses: { id: string; offset?: Record<string, number> }[]; prisms: Prism[]; rolled: { pose: string; prism: string; view: PlanarViewpoint; planeOfBoneOffM: number } };
const vectors = fixture<Vectors>("🧊️prism-depth");
const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const key = String(vectors.scale) as keyof Asset["representations"];
const authored = sample.representations[key] as AuthoredRepresentation;
const VIEWS: readonly PlanarViewpoint[] = ["elevation", "front", "plan"];
const shapes = vectors.prisms.map((prism, i) => ({ id: prism.id, bone: vectors.bone, role: "body", z: i + 1, filled: true, tone: 0.5, kind: "poly", closed: true, points: prism.points, solid: prism.solid }) as unknown as Shape);
const figure: Asset = {
  ...sample,
  poses: [...sample.poses, ...vectors.poses.filter((pose) => pose.offset !== undefined).map((pose) => ({ ...sample.poses[0]!, id: pose.id, bones: { [vectors.bone]: pose.offset } }) as Asset["poses"][number])],
  representations: { ...sample.representations, [key]: { ...authored, geometry: { elevation: shapes, front: shapes, plan: shapes } } } as Asset["representations"],
};
const stateOf = (pose: string) => evaluate(figure, { kind: "pose", pose }, { kind: "scale", scale: { denominator: vectors.scale } });
const triple = (v: { readonly x: number; readonly y: number; readonly z: number }): [number, number, number] => [v.x, v.y, v.z];

describe("🧊️ the depth of an extruded solid shape", () => {
  requireTool("uv");
  const states = Object.fromEntries(vectors.poses.map((pose) => [pose.id, stateOf(pose.id)]));
  const frames = Object.fromEntries(Object.entries(states).map(([pose, state]) => [pose, Object.fromEntries((["o", "u", "v", "w"] as const).map((axis) => [axis, triple(state.bones[vectors.bone]!.frame[axis])]))]));
  const directory = mkdtempSync(join(tmpdir(), "staffage-prism-depth-"));
  let reference: Record<string, Record<string, Record<PlanarViewpoint, number>>>;
  try {
    const input = join(directory, "input.json");
    writeFileSync(input, JSON.stringify({ length: figure.rig.bones.find((bone) => bone.id === vectors.bone)!.lengthM, restOriginZ: states.stand!.bones[vectors.bone]!.frame.o.z, prisms: vectors.prisms, frames }));
    reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🧊️prism-depth/🐍️.py"), "geometry", [input]));
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }

  for (const prism of vectors.prisms)
    it(`equals the nearest corner numpy finds in every pose and view: ${prism.id} (sweep ${prism.solid.sweep}${prism.solid.zTip === undefined ? "" : ", tapering"})`, () => {
      for (const pose of vectors.poses)
        for (const view of VIEWS) {
          const shape = states[pose.id]!.shapes[view].find((evaluated) => evaluated.shape.id === prism.id)!.shape;
          expect(Math.abs(shapeDepth(states[pose.id]!, shape, view) - reference[pose.id]![prism.id]![view]), `${pose.id} ${view}`).toBeLessThanOrEqual(vectors.toleranceM);
        }
    });

  it("ranks a rolled case bottom by its own lateral face, not by the plane of its bone", () => {
    const { pose, prism, view, planeOfBoneOffM } = vectors.rolled;
    const state = states[pose]!;
    const shape = state.shapes[view].find((evaluated) => evaluated.shape.id === prism)!.shape;
    expect(Math.abs(-state.bones[vectors.bone]!.frame.o.z - reference[pose]![prism]![view])).toBeGreaterThan(planeOfBoneOffM);
    expect(Math.abs(shapeDepth(state, shape, view) - reference[pose]![prism]![view])).toBeLessThanOrEqual(vectors.toleranceM);
  });
});
