import { expect, test } from "bun:test";
import Ajv from "ajv";
import schema from "../🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const validate = new Ajv().compile(schema);

type Data = { source_url?: unknown; sourceUrl?: unknown };

const link = (data: Data): string | null => [data.source_url, data.sourceUrl].find((value): value is string => typeof value === "string") ?? null;

test("shell commands satisfy the schema and malformed ones do not", () => {
  for (const { command } of fixture.cases) expect(validate(command)).toBe(true);
  for (const command of fixture.invalidCommands) expect(validate(command)).toBe(false);
});

test("the opened URL is the first textual link member of the addressed position", () => {
  for (const { command, url } of fixture.cases) {
    const position = fixture.positions.find(({ id }) => id === command.featureId);
    expect(position === undefined ? null : link(position.data)).toBe(url);
  }
});

test("every opened URL is a valid absolute HTTPS URL", () => {
  for (const { url } of fixture.cases) {
    if (url === null) continue;
    const parsed = new URL(url);
    expect(parsed.protocol).toBe("https:");
    expect(parsed.href).toBe(url);
  }
});
