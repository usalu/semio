import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv/dist/2020";

const module = new URL("../../", import.meta.url);
const read = (relative: string): any => JSON.parse(readFileSync(new URL(relative, module), "utf8"));
const registry = read("🗂️registry/🔣️.json"), schema = read("🗂️registry/🧬️schema/🔣️.json");
const bunRecord = read("../📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🧬️schema/🔣️.json").definitions.Release.const;
const validate = new Ajv({ strict: true }).compile(schema);

test("the runtime registry satisfies its closed schema for the independent validator", () => {
  expect(validate(registry), JSON.stringify(validate.errors)).toBe(true);
});

test("the schema refuses what the launcher refuses", () => {
  const mutate = (change: (copy: any) => void): boolean => { const copy = structuredClone(registry); change(copy); return validate(copy) as boolean; };
  expect(mutate((r) => { r.extra = 1; })).toBe(false);
  expect(mutate((r) => { r.contract = 2; })).toBe(false);
  expect(mutate((r) => { r.runtimes.bun.distributions[0].sha256 = "XYZ"; })).toBe(false);
  expect(mutate((r) => { r.runtimes.bun.download.base = "http://example.com/x"; })).toBe(false);
  expect(mutate((r) => { r.languages = ["en"]; })).toBe(false);
  expect(mutate((r) => { delete r.messages.ready.de; })).toBe(false);
  expect(mutate((r) => { r.implementation.entries.build.script = "/absolute/script.ts"; })).toBe(false);
});

test("every selected and entry runtime is declared and every text keeps its placeholders in both languages", () => {
  expect(Object.keys(registry.runtimes)).toContain(registry.selected);
  for (const entry of Object.values<any>(registry.implementation.entries)) expect(Object.keys(registry.runtimes)).toContain(entry.runtime ?? registry.selected);
  const names = (text: string): string[] => [...text.matchAll(/\{[A-Za-z]+\}/gu)].map((match) => match[0]).sort();
  for (const [id, text] of Object.entries<any>(registry.messages)) expect(names(text.de), id).toEqual(names(text.en));
});

test("the registry pins the same Bun release as the tools record", () => {
  const bun = registry.runtimes.bun;
  expect(bun.version).toBe(bunRecord.version);
  expect(bun.download.base).toBe(bunRecord.release);
  expect(bun.distributions.length).toBe(bunRecord.distributions.length);
  for (const row of bunRecord.distributions) {
    const found = bun.distributions.find((candidate: any) => candidate.platform === row.platform && candidate.arch === row.arch && (candidate.libc === "any" || candidate.libc === (row.musl ? "musl" : "gnu")));
    expect(found, `${row.platform}/${row.arch}/${row.musl}`).toMatchObject({ archive: row.archive, bytes: row.bytes, sha256: row.sha256 });
  }
});

test("the pinned runtime version equals the packageManager pin of the workspace package", () => {
  const manifest = read("../../../../../package.json");
  expect(manifest.packageManager).toBe(`bun@${registry.runtimes.bun.version}`);
});
