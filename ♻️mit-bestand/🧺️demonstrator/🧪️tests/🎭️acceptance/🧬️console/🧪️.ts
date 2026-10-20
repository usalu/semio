import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import { significantConsoleErrors } from "./🟦️.ts";

const cases = JSON.parse(readFileSync(new URL("./🧫️cases.json", import.meta.url), "utf8")) as {
  readonly cases: readonly { readonly name: string; readonly message: string; readonly significant: boolean }[];
};
const schema = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
const reference = new Ajv({ strict: true }).compile({
  type: "string",
  not: { enum: [
    "failed to load resource: the server responded with a status of 404",
    "failed to load resource: the server responded with a status of 404 (not found)",
  ] },
});

describe("acceptance console errors", () => {
  it("validates language-neutral examples with Ajv", () => {
    expect(new Ajv({ strict: true }).validate(schema, cases)).toBe(true);
  });
  it.each(cases.cases)("$name", ({ message, significant }) => {
    expect(reference(message.toLowerCase())).toBe(significant);
    expect(significantConsoleErrors([message])).toEqual(significant ? [message] : []);
  });
});

it("validates the authored curation interaction contract with Ajv", () => {
  const workflow = JSON.parse(readFileSync(new URL("../🧬️curation/🧫️cases.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️curation/🔣️.json", import.meta.url), "utf8"));
  expect(new Ajv({ strict: true }).validate(schema, workflow)).toBe(true);
});

it("validates the authored Energy and FEM interaction contract with Ajv", () => {
  const cases = JSON.parse(readFileSync(new URL("../🧬️simulation/🧫️cases.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️simulation/🔣️.json", import.meta.url), "utf8"));
  expect(new Ajv({ strict: true }).validate(schema, cases)).toBe(true);
});

it("validates all five authored document-edit contracts with Ajv", () => {
  const cases = JSON.parse(readFileSync(new URL("../🧬️edits/🧫️cases.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️edits/🔣️.json", import.meta.url), "utf8"));
  expect(new Ajv({ strict: true }).validate(schema, cases)).toBe(true);
});
