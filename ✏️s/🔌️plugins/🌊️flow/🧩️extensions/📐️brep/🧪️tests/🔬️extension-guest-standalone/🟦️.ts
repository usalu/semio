/** 🧳️ AJV validates the actual BREP guest retirement vector and its geometry request. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020.js";
import { BoxGeometry, Vector3 } from "three";

/** 🪪️ Validates portable channel declarations against the packaged owning manifest. */
export function channelIdentityOracle(directory: string): number {
  const root = resolve(directory, "../..");
  const fixture = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🪪️channels/🔣️.json"), "utf8"));
  const bundle = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8"));
  const contributions = [...bundle.contributions.topicContributions, ...bundle.manifest.topicContributions];
  const manifests = contributions.filter((entry: { payload: { manifestJson?: string } }) => entry.payload.manifestJson).map((entry: { payload: { manifestJson: string } }) => JSON.parse(entry.payload.manifestJson));
  const validate = new Ajv2020({ strict: true }).compile(JSON.parse(readFileSync(resolve(root, "🧬️schema/🪪️channels/🔣️.json"), "utf8")));
  let checks = 0;
  for (const manifest of manifests) for (const row of fixture.cases) {
    const operators = manifest.contributes.operators;
    const operator = operators.find((entry: { id: string }) => entry.id === row.operator);
    assert(operator, row.operator);
    for (const [name, fields] of Object.entries(row.expected)) {
      const channel = operator[row.direction].find((entry: { name: string }) => entry.name === name);
      assert(validate(channel), `${row.operator}.${name}: ${JSON.stringify(validate.errors)}`);
      for (const [field, expected] of Object.entries(fields as Record<string, unknown>)) { assert.deepEqual(channel[field], expected, `${row.operator}.${name}.${field}`); checks++; }
    }
  }
  return checks;
}

export function brepExtensionRetirementOracle(directory: string): number {
  const fixtures = resolve(directory, "../../🧫️fixtures/🚪️retirement");
  const fixture = JSON.parse(readFileSync(resolve(fixtures, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(resolve(fixtures, "📐️schema.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const input = JSON.parse(fixture.evaluate.inputJson);
  for (const axis of ["width", "depth", "height"]) assert.deepEqual(input[axis], { $schema: "number", value: 1 });
  assert.equal(validate({ ...fixture, tessellate: { ...fixture.tessellate, budget: 0 } }), false);
  assert.equal(validate({ ...fixture, grant: [0,65536] }), false);
  assert.equal(BigInt(fixture.exactCancellation.nodeHashLiteral), (1n << 64n) - 3n);
  assert.equal(validate({ ...fixture, exactCancellation: { ...fixture.exactCancellation, nodeHashLiteral: "18446744073709551616" } }), false);
  assert.equal(validate({ ...fixture, exactCancellation: { ...fixture.exactCancellation, guessedIdentity: 1 } }), false);
  assert.equal(validate({ ...fixture, ownerIsolation: { ...fixture.ownerIsolation, originalReaderHops: 0 } }), false);
  assert.equal(validate({ ...fixture, bypassIdentity: { ...fixture.bypassIdentity, nodeHashes: [0,700022] } }), false);
  assert.equal(validate({ ...fixture, bypassIdentity: { ...fixture.bypassIdentity, nodeHashes: [700021,700021] } }), false);
  const bypass = fixture.bypassIdentity;
  assert.equal(new Set(bypass.nodeHashes).size, 2);
  assert.equal(bypass.compactHops * bypass.roundUnits, 8);
  const identity = new Ajv({ strict: true }).compile({ type: "object", additionalProperties: false, required: ["operatorId", "nodeHash"], properties: { operatorId: { const: fixture.evaluate.operatorId }, nodeHash: { type: "integer", minimum: 0 } } });
  for (const text of fixture.exactCancellation.malformed) assert.equal(identity(JSON.parse(text)), false);
  return 18;
}


/** 📐️ The third-party JSON schema oracle validates the same inference payload and dependency laws. */
export function geometryInferenceOracle(directory: string): number {
  const root = resolve(directory, "../../💡️inferences/📐️geometry");
  const fixture = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🔣️.json"), "utf8"));
  const ajv = new Ajv2020({ strict: true }).addKeyword("x-semio-reads");
  const request = ajv.compile(JSON.parse(readFileSync(resolve(root, "📥️request.json"), "utf8")));
  const result = ajv.compile(JSON.parse(readFileSync(resolve(root, "📤️result.json"), "utf8")));
  assert(request(fixture.request), JSON.stringify(request.errors));
  assert(result({ ...fixture.expected, outputJson: "{}" }), JSON.stringify(result.errors));
  assert.equal(request({ ...fixture.request, operatorId: "math.add" }), false);
  assert.equal(result({ ...fixture.expected, outputJson: "{}", unitsDone: -1 }), false);
  const input = JSON.parse(fixture.request.inputJson);
  const volume = (value: typeof input): number => {
    const geometry = new BoxGeometry(value.width.value, value.depth.value, value.height.value).toNonIndexed();
    const points = geometry.getAttribute("position");
    let signed = 0;
    for (let i = 0; i < points.count; i += 3) {
      const a = new Vector3().fromBufferAttribute(points, i), b = new Vector3().fromBufferAttribute(points, i + 1), c = new Vector3().fromBufferAttribute(points, i + 2);
      signed += a.dot(b.cross(c)) / 6;
    }
    geometry.dispose();
    return Math.abs(signed);
  };
  assert.equal(volume(input), fixture.expectedVolume);
  const changed = JSON.parse(fixture.changedInput.inputJson);
  assert.equal(volume(changed), fixture.changedVolume);
  assert.notDeepEqual(JSON.parse(fixture.request.dependencyJson), JSON.parse(fixture.changedDependencies.dependencyJson));
  for (const phase of fixture.replacement.pendingPhases) assert(result({ done:false,cancellable:true,phase,unitsDone:0,unitsTotal:0,outputJson:fixture.replacement.outputJson }));
  assert.equal(fixture.replacement.initialUnits,fixture.expected.unitsDone);
  return 10;
}
