import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
import descriptorSchema from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🧬️schema/🔣️.json" with { type: "json" };
import runSchema from "../../../../../../../🧰️framework/🔨️modules/⏯️tool-run/🧬️schema/🔣️.json" with { type: "json" };
import fixture from "../../../../../../../🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/🎟️limits/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };

const ajv = new Ajv({ strict: false, allErrors: true }).addSchema(runSchema);
const validateDescriptor = ajv.compile(descriptorSchema);
const validateReceiving = ajv.compile({ $ref: `${runSchema.$id}#/$defs/ToolRunReceivingLimits` });
const outputDirectory = process.env.SEMIO_MCP_DESCRIPTOR_OUTPUT;
const cases = [
  ["dag", "🕸️dag", ["dag.dag.layoutRun"]],
  ["reasoning", "💡️reasoning", ["reasoning.wires.layoutRun"]],
  ["remodel", "📸️remodel", ["remodeling.reconstruction.run"]],
  ["trinity", "🔱️trinity", ["trinity.jack.layoutRun"]],
  ["wfc", "🀄️wfc", [
    "s.wfc.bitmap.fill.run",
    "s.wfc.grid2d.fill.run",
    "s.wfc.grid3d.fill.run",
    "s.wfc.wfc2d.fill.run",
    "s.wfc.wfc3d.fill.run",
  ]],
] as const;

type Run = { runJob: string; receiving?: unknown };
type Tool = { run?: Run };
type App = { tools?: Tool[]; utilities?: Tool[] };

for (const [owner, directory, expectedJobs] of cases) {
  test(`${owner} publishes its authored run receiving limits`, () => {
    const root = outputDirectory
      ? resolve(outputDirectory, owner)
      : resolve("🌎️hub/🧩️compositions", directory, "🧩️component");
    const json = readFileSync(join(root, "🔣️.json"));
    const descriptor: { manifest: { apps: App[] } } = JSON.parse(json.toString("utf8"));
    const runs = descriptor.manifest.apps.flatMap(app =>
      [...(app.tools ?? []), ...(app.utilities ?? [])].flatMap(tool => tool.run ? [tool.run] : []));
    expect(runs.map(run => run.runJob).sort()).toEqual([...expectedJobs].sort());
    for (const run of runs) {
      expect(validateReceiving(run.receiving), `${run.runJob}: ${JSON.stringify(validateReceiving.errors)}`).toBe(true);
      expect(run.receiving).toEqual(fixture.policy);
    }
    expect(validateDescriptor(descriptor), JSON.stringify(validateDescriptor.errors)).toBe(true);
    const pair = verifySemanticDescriptorPairBytesV1(json, readFileSync(join(root, "🛂️.descriptor.semio")));
    expect(pair.packageId).toBe(`semio:${owner}`);
  });
}

test("norm publishes current inspector field kinds in both formats", () => {
  const root = outputDirectory
    ? resolve(outputDirectory, "norm")
    : resolve("🌎️hub/🧩️compositions/📕️norm/🧩️component");
  const json = readFileSync(join(root, "🔣️.json"));
  const descriptor = JSON.parse(json.toString("utf8"), (key, value) => {
    if (key === "schema" && value?.kind === "vec3") throw Error("Obsolete Norm inspector field kind");
    return value;
  });
  expect(validateDescriptor(descriptor), JSON.stringify(validateDescriptor.errors)).toBe(true);
  const pair = verifySemanticDescriptorPairBytesV1(json, readFileSync(join(root, "🛂️.descriptor.semio")));
  expect(pair.packageId).toBe("semio:norm");
});

test("note publishes the authored hover parameters in both formats", () => {
  const root = outputDirectory
    ? resolve(outputDirectory, "note")
    : resolve("🌎️hub/🧩️compositions/🗒️note/🧩️component");
  const fixture = JSON.parse(readFileSync(join(import.meta.dirname, "note-hover-descriptor.json"), "utf8")) as {
    artifactPackage: string; action: string; windows: string[]; arguments: { id: string; required: boolean }[];
  };
  const validArguments = ajv.compile({
    type: "array",
    minItems: fixture.arguments.length,
    additionalItems: false,
    items: fixture.arguments.map(({ id, required }) => ({
      type: "object",
      required: ["id", "required", "schema"],
      properties: {
        id: { const: id },
        required: { const: required },
        schema: { type: "object", required: ["kind"], properties: { kind: { const: "string" } } },
      },
    })),
  });
  const json = readFileSync(join(root, "🔣️.json"));
  const descriptor: { manifest: { apps: { windowKinds: { id: string; actions: { id: string; args: unknown }[] }[] }[] } } = JSON.parse(json.toString("utf8"));
  expect(validateDescriptor(descriptor), JSON.stringify(validateDescriptor.errors)).toBe(true);
  const windows = descriptor.manifest.apps.flatMap(app => app.windowKinds);
  for (const id of fixture.windows) {
    const rows = windows.filter(window => window.id === id).flatMap(window => window.actions).filter(action => action.id === fixture.action);
    expect(rows).toHaveLength(1);
    expect(validArguments(rows[0].args), JSON.stringify(validArguments.errors)).toBe(true);
  }
  const pair = verifySemanticDescriptorPairBytesV1(json, readFileSync(join(root, "🛂️.descriptor.semio")));
  expect(pair.packageId).toBe(fixture.artifactPackage);
});
