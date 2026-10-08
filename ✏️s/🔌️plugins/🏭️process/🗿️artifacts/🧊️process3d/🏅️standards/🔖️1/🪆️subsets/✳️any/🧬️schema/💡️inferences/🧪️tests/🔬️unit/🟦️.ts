import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import ManifoldModule from "manifold-3d";

type Solid = { kind: "box"; width: number; depth: number; height: number } | { kind: "cylinder"; radius: number; height: number };
type Pose = { position: [number, number, number]; axis: [number, number, number]; angle: number };
type Measure = { measure: "cut"; tool: Solid; pose: Pose } | { measure: "drill"; radius: number; depth: number; pose: Pose } | { measure: "attach"; component: Solid; pose: Pose };
type Fixture = { scene: { stock: { solid: Solid; pose: Pose }; steps: { id: string; enabled: boolean; measure: Measure }[] }; volumes: number[]; volumeTolerance: number; circularSegments: number; poseScalarBits: { step: number; coordinate: number; bits: string }[] };

test("Timber replay has five independent Manifold volume witnesses", async () => {
  const root = resolve(import.meta.dir, "../🧫️fixtures/🪵️timber-replay"), fixture: Fixture = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8"));
  for (const row of fixture.poseScalarBits) {
    const scalar = new DataView(new ArrayBuffer(8));
    scalar.setFloat64(0, fixture.scene.steps[row.step]!.measure.pose.position[row.coordinate]!);
    expect(scalar.getBigUint64(0).toString(16).padStart(16, "0")).toBe(row.bits);
  }
  const module = await ManifoldModule();
  module.setup();
  const owners: InstanceType<typeof module.Manifold>[] = [], retain = (owner: InstanceType<typeof module.Manifold>) => { owners.push(owner); return owner; };
  const solid = (shape: Solid, pose: Pose) => {
    expect(pose.axis).toEqual([0, 0, 1]);
    expect(pose.angle).toBe(0);
    const primitive = retain(shape.kind === "box" ? module.Manifold.cube([shape.width, shape.depth, shape.height], true) : module.Manifold.cylinder(shape.height, shape.radius, shape.radius, fixture.circularSegments, true));
    return retain(primitive.translate(pose.position));
  };
  try {
    let current = solid(fixture.scene.stock.solid, fixture.scene.stock.pose);
    const volumes = [current.volume()];
    for (const step of fixture.scene.steps) {
      expect(step.enabled).toBe(true);
      const measure = step.measure, shape: Solid = measure.measure === "drill" ? { kind: "cylinder", radius: measure.radius, height: measure.depth } : measure.measure === "cut" ? measure.tool : measure.component;
      const tool = solid(shape, measure.pose);
      current = retain(measure.measure === "attach" ? current.add(tool) : current.subtract(tool));
      expect(current.status()).toBe("NoError");
      volumes.push(current.volume());
    }
    expect(volumes.length).toBe(fixture.volumes.length);
    for (let index = 0; index < volumes.length; index++) expect(Math.abs(volumes[index]! - fixture.volumes[index]!)).toBeLessThan(fixture.volumeTolerance);
    console.log(`[DEBUG] Process timber independent Manifold prefixes=${volumes.length} volumes=${JSON.stringify(volumes)}`);
  } finally {
    for (const owner of owners.reverse()) owner.delete();
  }
}, 30000);
