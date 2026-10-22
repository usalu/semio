import { expect, test } from "bun:test";
import Ajv from "ajv";
import { LanguageTextErrorV1, bindCatalogV1 } from "../../📖️catalog/🟦️.ts";
import { PROGRESS_LEVELS_V1, createCliSinkV1, emitProgressV1, progressV1, renderProgressV1, sayV1, withProgressSinkV1, type ProgressEventV1, type ProgressLevelV1 } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };

const steps = bindCatalogV1("steps", corpus.catalog);

test("the corpus wire forms agree with the strict schema validator", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  for (const row of corpus.wire) expect(ajv.validate(schema, row.data), `${row.name} ${JSON.stringify(ajv.errors)}`).toBe(row.valid);
  expect([...PROGRESS_LEVELS_V1]).toEqual(schema.properties.level.enum);
});

for (const row of corpus.events) {
  test(`event: ${row.name}`, () => {
    const event = progressV1(steps, row.key, row.values, row.level as ProgressLevelV1);
    expect(event).toEqual({ version: 1, key: `steps.${row.key}`, values: row.values, level: row.level } as never);
    expect(renderProgressV1(event, "en")).toBe(row.en);
    expect(renderProgressV1(event, "de")).toBe(row.de);
    const lines: [string, string][] = [];
    createCliSinkV1({ environment: { SEMIO_LANGUAGE: "de" }, osLanguages: () => [], write: (stream, line) => lines.push([stream, line]) })(event);
    expect(lines).toEqual([[row.stream, row.de]]);
  });
}

test("an event carries data and is checked when it is built", () => {
  expect(() => progressV1(steps, "start", {})).toThrow(LanguageTextErrorV1);
  expect(() => progressV1(steps, "absent")).toThrow(LanguageTextErrorV1);
  expect(Object.isFrozen(progressV1(steps, "done"))).toBe(true);
});

test("a scope receives every event emitted inside it, across awaits, and nothing leaks outside", async () => {
  const seen: ProgressEventV1[] = [];
  await withProgressSinkV1((event) => seen.push(event), async () => {
    sayV1(steps, "done");
    await Promise.resolve();
    emitProgressV1(progressV1(steps, "start", { count: 2 }));
  });
  expect(seen.map((event) => event.key)).toEqual(["steps.done", "steps.start"]);
  const inner: string[] = [];
  withProgressSinkV1(() => undefined, () => withProgressSinkV1((event) => inner.push(event.key), () => sayV1(steps, "done")));
  expect(inner).toEqual(["steps.done"]);
});

test("the CLI sink renders in the language the shared resolver chooses and refuses bilingually when none is chosen", () => {
  const lines: string[] = [];
  const event = progressV1(steps, "done");
  createCliSinkV1({ environment: { LANG: "en_GB.UTF-8" }, osLanguages: () => [], write: (_stream, line) => lines.push(line) })(event);
  createCliSinkV1({ args: ["--lang=de"], environment: { LANG: "en_GB.UTF-8" }, osLanguages: () => [], write: (_stream, line) => lines.push(line) })(event);
  expect(lines).toEqual(["[steps] done.", "[steps] fertig."]);
  expect(() => createCliSinkV1({ environment: {}, osLanguages: () => [], write: () => undefined })(event)).toThrow(/--lang=en\|de[\s\S]*--lang=en\|de/u);
  expect(() => createCliSinkV1({ environment: { SEMIO_LANGUAGE: "fr" }, osLanguages: () => [], write: () => undefined })(event)).toThrow(/Unbekannte|nicht angebotene/u);
});
