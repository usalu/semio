import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import Ajv from "ajv";
import { buildSchema, graphql } from "graphql";
import protobuf from "protobufjs";

const facets = resolve(process.argv[2], "🧬️schema/🗺️imported-map/🏔️elevation");
const [fixtureText, jsonText, graphText, protoText] = await Promise.all([
  readFile(new URL("./terrain-fractional-grid.json", import.meta.url), "utf8"),
  readFile(resolve(facets, "🔣️.json"), "utf8"),
  readFile(resolve(facets, "🔗️.graphql"), "utf8"),
  readFile(resolve(facets, "🛰️.proto"), "utf8"),
]);
const fixture = JSON.parse(fixtureText).elevation;
const minimum = -2_147_483_648;
const maximum = 2_147_483_647;
const samples = [
  { bounds: fixture.bounds, rowsMillimetres: fixture.millimetres },
  { bounds: fixture.bounds, rowsMillimetres: [[minimum, maximum], [-1, 0]] },
];
const validate = new Ajv({ strict: false }).compile(JSON.parse(jsonText));
assert.equal(validate({ bounds: fixture.bounds, rows: fixture.millimetres }), false);
assert.equal(validate({ bounds: fixture.bounds, rowsMillimetres: [[maximum + 1, 0], [0, 0]] }), false);

const schema = buildSchema(graphText + "\ntype Query { echo(input: ElevationSourceInput!): ElevationSource! }");
const query = "query($input: ElevationSourceInput!) { echo(input: $input) { bounds rowsMillimetres } }";
const root = protobuf.parse(protoText).root;
const Binary = root.lookupType("semio.s.gis.gisterrain.ElevationSource");
const Row = root.lookupType("semio.s.gis.gisterrain.ElevationRow");
assert.equal(Binary.fields.rowsMillimetres.id, 2);
assert.equal(Row.fields.heightMillimetres.type, "sint32");

for (const sample of samples) {
  assert(validate(sample), JSON.stringify(validate.errors));
  const graph = await graphql({
    schema,
    source: query,
    variableValues: { input: sample },
    rootValue: { echo: ({ input }) => input },
  });
  assert.equal(graph.errors, undefined);
  assert.deepEqual(JSON.parse(JSON.stringify(graph.data.echo)), sample);
  const protoSample = {
    bounds: sample.bounds,
    rowsMillimetres: sample.rowsMillimetres.map((heightMillimetres) => ({ heightMillimetres })),
  };
  assert.equal(Binary.verify(protoSample), null);
  const bytes = Binary.encode(Binary.create(protoSample)).finish();
  const received = Binary.toObject(Binary.decode(bytes));
  assert.deepEqual({
    bounds: received.bounds,
    rowsMillimetres: received.rowsMillimetres.map((row) => row.heightMillimetres),
  }, sample);
}
console.log("[DEBUG] Fractional and signed-limit grids agree across JSON Schema, GraphQL, and Protobuf.");
