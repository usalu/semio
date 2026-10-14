import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv/dist/2020.js";
const channelContract = JSON.parse(readFileSync(new URL("../../../🧬️schema/🪪️channels/🔣️.json", import.meta.url), "utf8"));
const channelFixtures = JSON.parse(readFileSync(new URL("../../../🧪️tests/🧫️fixtures/🪪️channels/🔣️.json", import.meta.url), "utf8"));
test("packaged BRep channels satisfy typed schema and portable identities", () => {
  const descriptor = JSON.parse(readFileSync(new URL("../../../🧩️component/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv().compile(channelContract);
  for (const topic of descriptor.manifest.topicContributions) {
    const manifest = JSON.parse(topic.payload.manifestJson);
    for (const operator of manifest.contributes.operators) for (const direction of ["inputs", "outputs"]) {
      const channels = operator[direction];
      for (const channel of channels) expect(validate(channel), `${operator.id} ${direction} ${channel.name}: ${JSON.stringify(validate.errors)}`).toBe(true);
      for (const key of ["code", "abbreviation", "name", "fullName"]) expect(new Set(channels.map((channel: Record<string, unknown>) => channel[key])).size, `${operator.id} ${direction} ${key}`).toBe(channels.length);
    }
    for (const fixture of channelFixtures.cases) {
      const operator = manifest.contributes.operators.find((operator: { id: string }) => operator.id === fixture.operator);
      for (const [key, fields] of Object.entries(fixture.expected)) {
        const channel = operator[fixture.direction].find((channel: { name: string }) => channel.name === key);
        for (const [field, expected] of Object.entries(fields as object)) expect(channel[field]).toEqual(expected);
      }
    }
    for (const kind of ["Vertex", "Edge", "Face"]) expect(manifest.contributes.operators.some((operator: { id: string }) => operator.id === `brep.mesh.inspect${kind}`)).toBe(true);
  }
});
