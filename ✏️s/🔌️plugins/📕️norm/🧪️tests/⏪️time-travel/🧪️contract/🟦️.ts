/** 🧾️ Validates the renderer-neutral norm engineering matrix against independent oracles before a browser is launched. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import matrix from "../🧫️witness/🔣️.json" with { type: "json" };
import { timeTravelScenarioV1 } from "../🟦️.ts";

const playgrounds = JSON.parse(readFileSync(new URL("../../../../../🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json", import.meta.url), "utf8")) as { variant: string; pluginId: string; ports: { react: number; wgpu: number } }[];
const scope = (path: string): string => {
  const [, collection, id] = /^([\w.]+)\[id=([^\]]+)\]/u.exec(path)!;
  return `scope:1:${collection.length}:${collection}${id.length}:${id}`;
};
const pinned = matrix.artifacts.filter(artifact => artifact.pin !== null);

test("engineering matrix is closed, bilingual and covers all fifteen norm artifacts under the independent schema oracle", () => {
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  expect(validate(matrix), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(matrix.artifacts.map(artifact => artifact.variant)).size).toBe(15);
  expect(validate({ ...matrix, locales: ["en", "en"] })).toBe(false);
  expect(validate({ ...matrix, journey: ["boot", "edit", "reload"] })).toBe(false);
  expect(validate({ ...matrix, artifacts: matrix.artifacts.slice(1) })).toBe(false);
  expect(validate({ ...matrix, artifacts: [{ ...matrix.artifacts[0], pin: { field: {} } }, ...matrix.artifacts.slice(1)] })).toBe(false);
});

test("every matrix artifact is served on the port the generated playground catalog declares", () => {
  const norm = new Map(playgrounds.filter(playground => playground.pluginId === "norm").map(playground => [playground.variant, playground.ports]));
  expect(norm.size).toBe(15);
  for (const artifact of matrix.artifacts) expect(artifact.ports).toEqual(norm.get(artifact.variant)!);
});

test("each pinned witness changes the dependent utilization and preserves its owner", () => {
  expect(pinned.length).toBeGreaterThan(0);
  for (const { variant, pin } of pinned) {
    const witness = pin!;
    if (witness.linear) for (const state of Object.values(witness.states)) expect(state.value / witness.field.limit).toBeCloseTo(state.utilization, 3);
    expect(witness.states.after.value).not.toBe(witness.states.before.value);
    expect(witness.states.after.utilization).not.toBe(witness.states.before.utilization);
    expect(witness.field.key).toBe(`norm-inputs.${witness.field.path}`);
    expect(witness.field.ownerWindow).toBe(`norm-${variant}-outliner`);
    if (witness.field.owner === "") expect(witness.field.path).not.toContain("[id=");
    else expect(witness.field.owner).toBe(scope(witness.field.path));
  }
});

test("browser scenario exposes every schema-owned journey step through the public harness", () => {
  const selected = matrix.artifacts.find(artifact => artifact.variant === timeTravelScenarioV1.variant)!;
  expect(selected).toBeDefined();
  expect(timeTravelScenarioV1.defaultServes.react).toContain(`:${selected.ports.react}/?plugin=${selected.variant}&role=editor&example=${selected.example}`);
  expect(timeTravelScenarioV1.defaultServes.wgpu).toContain(`:${selected.ports.wgpu}/`);
  expect(timeTravelScenarioV1.order).toEqual(matrix.journey.map((_, index) => index + 1));
  expect(timeTravelScenarioV1.stepNumbers).toEqual(timeTravelScenarioV1.order);
  expect(Object.keys(timeTravelScenarioV1.steps).map(Number)).toEqual(timeTravelScenarioV1.order);
  expect(timeTravelScenarioV1.context()).toEqual({});
});
