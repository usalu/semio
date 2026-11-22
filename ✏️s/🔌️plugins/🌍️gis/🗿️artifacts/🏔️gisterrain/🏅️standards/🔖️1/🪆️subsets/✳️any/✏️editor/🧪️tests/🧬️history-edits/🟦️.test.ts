import { expect, test } from "bun:test";
import Ajv from "ajv";
import { mutationInputDefs } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/📥️mutation-input-schema/🟦️.ts";
import elevationSchema from "../../../🧬️schema/🗺️imported-map/🏔️elevation/🔣️.json";
import exaggerationSchema from "../../../🧬️schema/🧬️mutations/🎚️change-exaggeration/🧬️schema/🔣️.json";
import replaceElevationSchema from "../../../🧬️schema/🧬️mutations/🏔️replace-elevation-source/🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🧬️history-edits/🎚️exaggeration/🦠️mutation/🔣️.json";

test("Terrain History has a self-contained exaggeration edit", () => {
  const validate = new Ajv({ strict: false }).compile(exaggerationSchema);
  const mutation = fixture.mutation.ChangeExaggeration;
  expect(validate(mutation)).toBe(true);
  expect(fixture.before.exaggeration).not.toBe(mutation.newExaggeration);
  expect({ ...fixture.before, exaggeration: mutation.newExaggeration }).toEqual(fixture.after);
});

test("Terrain History resolves and validates the authored elevation payload", () => {
  const inputs = mutationInputDefs(replaceElevationSchema, (id) => id === elevationSchema.$id ? elevationSchema : undefined);
  expect(inputs).toHaveLength(1);
  expect(inputs[0]).toMatchObject({ id: "/newElevationSource", schema: { kind: "object" } });
  const schema = inputs[0]?.schema;
  if (schema?.kind !== "object") throw new Error("Terrain elevation is an editable object");
  expect(schema.fields.map((field) => field.id)).toEqual(Object.keys(elevationSchema.properties).map((key) => `/${key}`));
  const validate = new Ajv({ strict: false }).addSchema(elevationSchema).compile(replaceElevationSchema);
  const rowsMillimetres = [[12_000, 12_125, 13_000], [12_001, 0, -500]];
  const elevation = { bounds: [0, 0, 1, 1], rowsMillimetres };
  expect(validate({ newElevationSource: elevation })).toBe(true);
  expect(validate({ newElevationSource: { ...elevation, missingSamples: "10" } })).toBe(true);
  expect(validate({ newElevationSource: { ...elevation, missingSamples: "00" } })).toBe(false);
  expect(validate({ newElevationSource: { ...elevation, rowsMillimetres: rowsMillimetres.slice(1) } })).toBe(false);
  expect(validate({})).toBe(true);
});
