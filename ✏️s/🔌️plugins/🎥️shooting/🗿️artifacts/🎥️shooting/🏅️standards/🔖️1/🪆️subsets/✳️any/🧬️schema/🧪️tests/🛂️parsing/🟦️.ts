import { expect, test } from "bun:test";
import Ajv from "ajv";
import { artifactReferenceSchema, artifactChildSchema } from "@semio-tech/framework/schema/artifact-reference";
import { ioSchema } from "@semio-tech/framework/io/schema";
import schema from "../../🔣️.json";
import { parseShootingArtifact } from "../../🟦️.ts";
import snapshot from "../../../🧪️tests/🧫️fixtures/🧬️mutations/↕️scale-assets/📏️doubles/📸️snapshot/⬅️before/🔣️.json";
import fixture from "../🧫️fixtures/🛂️parsing/🔣️.json";

const validate = new Ajv({ strict: false, validateFormats: false })
  .addSchema(artifactReferenceSchema).addSchema(artifactChildSchema).addSchema(ioSchema).compile(schema);

for (const scenario of fixture.cases) {
  test(`shooting artifact parsing agrees with JSON Schema: ${scenario.name}`, () => {
    const input = structuredClone(snapshot);
    let parent: any = input;
    for (const key of scenario.path.slice(0, -1)) parent = parent[key];
    const key = scenario.path.at(-1);
    if (key !== undefined) {
      if (scenario.remove) delete parent[key];
      else parent[key] = scenario.value;
    }
    expect(validate(input)).toBe(scenario.valid);
    if (scenario.valid) {
      const expected = structuredClone(input);
      expected.scene.material.stroke ??= "#000000";
      expect(JSON.parse(JSON.stringify(parseShootingArtifact(input)))).toEqual(expected);
    } else expect(() => parseShootingArtifact(input)).toThrow();
  });
}
