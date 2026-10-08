import Ajv from "ajv";
import { strict as assert } from "node:assert";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";

interface NeutralPose { position: [number, number, number]; axis: [number, number, number]; angle: number }
interface NeutralStep { id: string; label: string; enabled: boolean; origin?: { machineId: string; capabilityId: string } | null; measure: { measure: "cut" | "attach" | "drill"; pose: NeutralPose; radius?: number; depth?: number } }
interface NeutralSnapshot { stepPayloads: NeutralStep[]; toolSolids: { childId: string }[]; steps: { childId: string; target: { artifactId: string } } }

const cases = ["🌱create-step/🪚️accepts", "🗑️delete-step/🚫️accepts", "🏷️rename-step/🔤️accepts", "🔘change-step-enabled/⏸️accepts", "🧷change-step-origin/🏭️accepts", "📐replace-step-measure/🕳️accepts", "🔀reorder-steps/🔀️accepts"];

function workspace(): string {
  let folder = import.meta.dirname;
  while (!existsSync(join(folder, "nx.json"))) {
    const parent = dirname(folder);
    assert.notEqual(parent, folder, "repository workspace");
    folder = parent;
  }
  return folder;
}

function oracle(): void {
  const neutralFolder = join(workspace(), "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-laws/🧫️fixtures");
  const fixture: { stops: number[]; releaseBytes: number; maximumAdmittedReleaseBytes: number; structuralWork: { nextByteDemand: number; physicalReleaseBytes: number }; decoderBacking: { capacityItems: number; maximumAdvertisedTurns: number; queryEachDemand: boolean; expectedTerminal: boolean }; expected: Record<string, number | boolean> } = JSON.parse(readFileSync(join(neutralFolder, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(neutralFolder, "🧬️schema/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixture, releaseBytes: 0 }), false);
  assert.equal(validate({ ...fixture, expected: { ...fixture.expected, writer: true } }), false);
  assert.equal(validate({ ...fixture, decoderBacking: { ...fixture.decoderBacking, expectedTerminal: false } }), false);
  assert.equal(validate({ ...fixture, decoderBacking: { ...fixture.decoderBacking, maximumAdvertisedTurns: 100000 } }), false);
  assert.equal(validate({ ...fixture, decoderBacking: { ...fixture.decoderBacking, queryEachDemand: false } }), false);
  assert.equal(validate({ ...fixture, structuralWork: { ...fixture.structuralWork, nextByteDemand: 4096 } }), false);
  assert.equal(Buffer.allocUnsafeSlow(fixture.structuralWork.physicalReleaseBytes).buffer.byteLength, 0);
  assert.equal(fixture.structuralWork.nextByteDemand, 1);
  console.log("[DEBUG] Independent Node Buffer structural oracle: physical extent=0, next work demand=" + fixture.structuralWork.nextByteDemand);
  console.log("[DEBUG] Independent Ajv envelope release oracle: " + fixture.decoderBacking.capacityItems + " retained rows, " + fixture.decoderBacking.maximumAdvertisedTurns + " advertised-demand turns, exact terminal=" + fixture.decoderBacking.expectedTerminal);
  console.log("[DEBUG] Independent Ajv cancellation oracle: " + fixture.stops.length + " stops, exact " + fixture.releaseBytes + "-byte release floor, explicitly admitted allocation bound " + fixture.maximumAdmittedReleaseBytes + ", closed terminal-empty record");
  const closeFolder = join(workspace(), "🧰️framework/🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand");
  const closeFixture: { admissionBytes: number; cases: { name: string; physicalBytes: number; callerBytes: number; releasedBytes: number }[] } = JSON.parse(readFileSync(join(closeFolder, "🔣️.json"), "utf8"));
  const closeSchema = JSON.parse(readFileSync(join(closeFolder, "🧬️schema/🔣️.json"), "utf8"));
  const closeValidate = new Ajv({ strict: true, allErrors: true }).compile(closeSchema);
  assert.ok(closeValidate(closeFixture), JSON.stringify(closeValidate.errors));
  for (const row of closeFixture.cases) {
    const allocation = Buffer.allocUnsafeSlow(row.physicalBytes);
    assert.equal(allocation.buffer.byteLength, row.physicalBytes);
    assert.ok(allocation.buffer.byteLength <= closeFixture.admissionBytes);
    const allowedRelease = row.callerBytes >= allocation.buffer.byteLength ? allocation.buffer.byteLength : 0;
    assert.equal(allowedRelease, row.releasedBytes, row.name);
    console.log("[DEBUG] Independent Node Buffer extent/grant oracle " + row.name + ": backing=" + allocation.buffer.byteLength + " caller=" + row.callerBytes + " admitted-release=" + allowedRelease);
  }
  const folder = join(workspace(), "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");
  for (const name of cases) {
    const snapshot: NeutralSnapshot = JSON.parse(readFileSync(join(folder, name, "📸️snapshot/➡️after/🔣️.json"), "utf8"));
    assert.ok(Array.isArray(snapshot.stepPayloads));
    assert.ok(Array.isArray(snapshot.toolSolids));
    let tool = 0;
    const nodes = snapshot.stepPayloads.map((step: NeutralStep, index: number) => {
      const params = [{ key: "enabled", value: String(step.enabled) }];
      if (step.origin) params.push({ key: "originMachineId", value: step.origin.machineId }, { key: "originCapabilityId", value: step.origin.capabilityId });
      for (const [field, prefix] of [["position", "posePosition"], ["axis", "poseAxis"]] as const) {
        for (const [axis, suffix] of ["X", "Y", "Z"].entries()) params.push({ key: prefix + suffix, value: String(step.measure.pose[field][axis]) });
      }
      params.push({ key: "poseAngle", value: String(step.measure.pose.angle) });
      if (step.measure.measure === "drill") {
        assert.equal(typeof step.measure.radius, "number");
        assert.equal(typeof step.measure.depth, "number");
        params.push({ key: "radius", value: String(step.measure.radius) }, { key: "depth", value: String(step.measure.depth) });
      } else {
        params.push({ key: "toolChildId", value: snapshot.toolSolids[tool++].childId });
      }
      return { id: step.id, kind: step.measure.measure, label: step.label, params, position: { x: index * 200, y: 0 } };
    });
    const edges = nodes.slice(1).map((right, index: number) => ({ id: "e-" + nodes[index].id + "-" + right.id, from: { node: nodes[index].id, port: "out" }, to: { node: right.id, port: "in" }, kind: "sequence" }));
    const bytes = JSON.stringify({ schema: "stdio.semio.flow", nodes, edges }).replace(/"position":\{"x":(\d+),"y":0\}/g, '"position":{"x":$1.0,"y":0.0}');
    const expected = "steps-flow-" + createHash("sha256").update(bytes).digest("hex").slice(0, 16);
    assert.equal(snapshot.steps.childId, expected, name);
    assert.equal(snapshot.steps.target.artifactId, expected, name);
    assert.equal(tool, snapshot.toolSolids.length, name);
    console.log("[DEBUG] Independent Node crypto timeline oracle " + name + ": " + expected + " (" + Buffer.byteLength(bytes) + " bytes)");
  }
}

if (import.meta.main) {
  assert.equal(process.argv[2] ?? "oracle", "oracle");
  oracle();
}
