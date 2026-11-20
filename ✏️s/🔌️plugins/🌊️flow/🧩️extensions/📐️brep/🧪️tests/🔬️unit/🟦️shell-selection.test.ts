import { expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "./🔣️shell-selection.json";

const validate = new Ajv().compile({
  type: "object",
  required: ["$schema"],
  properties: { $schema: { const: "list" } },
  patternProperties: {
    "^(0|[1-9][0-9]*)$": {
      type: "object",
      required: ["$schema", "kind", "handle"],
      additionalProperties: false,
      properties: {
        $schema: { const: "geometry" },
        kind: { const: "shell" },
        handle: { type: "string", pattern: "^[a-f0-9]{64}$" },
      },
    },
  },
  additionalProperties: false,
});

for (const row of fixture.cases) {
  test(row.id, () => {
    const labels = [row.outer, ...row.inners];
    expect(labels).toEqual(row.expected);
    const handles = labels.map(label => label.toString(16).padStart(64, "0"));
    const list = Object.fromEntries([
      ["$schema", fixture.schema],
      ...handles.map((handle, index) => [String(index), {
        $schema: fixture.member.schema,
        kind: fixture.member.kind,
        handle,
      }]),
    ]);
    expect(validate(list)).toBe(true);
    expect(Object.keys(list).filter(key => key !== "$schema")).toEqual(row.expected.map((_, index) => String(index)));
    expect(validate({ ...list, "0": { $schema: fixture.member.schema, kind: "solid", handle: handles[0] } })).toBe(false);
  });
}
