import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { chooseLanguageV1, languageFlagV1, languageRefusalV1, languageSubtagV1, LANGUAGE_PROTOCOL_V1, noticeLanguagesV1, osLanguageListV1, resolveLanguageV1 } from "../🟦️.ts";
import protocol from "../🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import vectors from "./🔣️.json" with { type: "json" };

type Row = Readonly<{ name: string; args: string[]; environment: Record<string, string>; os: string[]; expected?: { language: string; source: string }; refusal?: "none" | "flag" | "variable" }>;
type OracleRow = Readonly<{ name: string; choice: { language?: string; source?: string; refusal?: string }; notices: string[]; ok: boolean }>;

const rows = vectors.resolve as unknown as readonly Row[];
const here = dirname(fileURLToPath(import.meta.url));
const root = ((start: string): string => { let at = start; while (!existsSync(join(at, "🧰️framework")) && dirname(at) !== at) at = dirname(at); return at; })(here);

test("the language protocol satisfies its schema for the independent validator", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(protocol), JSON.stringify(validate.errors)).toBe(true);
  expect(LANGUAGE_PROTOCOL_V1).toBe(protocol);
});

for (const row of rows) {
  test(`language resolution: ${row.name}`, () => {
    const os = (): readonly string[] => row.os, choice = chooseLanguageV1(vectors.languages, row.args, row.environment, os);
    const spoken = chooseLanguageV1(vectors.languages, row.args, row.environment, () => []);
    expect(noticeLanguagesV1(vectors.languages, row.environment, row.args)).toEqual("refusal" in spoken ? vectors.languages : [spoken.language]);
    if (row.expected) {
      expect(choice).toEqual(row.expected);
      return void expect(resolveLanguageV1(vectors.languages, row.args, row.environment, os)).toEqual(row.expected);
    }
    expect(choice).toMatchObject({ refusal: { kind: row.refusal } });
    let error: Error | undefined;
    try { resolveLanguageV1(vectors.languages, row.args, row.environment, os); } catch (caught) { error = caught as Error; }
    const lines = error!.message.split("\n");
    expect(lines.length, "one line per offered language").toBe(vectors.languages.length);
    for (const [index, language] of vectors.languages.entries()) expect(lines[index]!.startsWith(protocol.refusal[row.refusal!][language as "en" | "de"].split("{")[0]!), language).toBe(true);
    if (row.refusal === "none") for (const choice of ["--lang=en|de", "SEMIO_LANGUAGE=en|de"]) expect(lines.every((line) => line.includes(choice)), choice).toBe(true);
    if (row.refusal === "variable") expect(lines[0]).toContain(`SEMIO_LANGUAGE names a language that is not offered: ${row.environment.SEMIO_LANGUAGE}`);
  });
}

test("the independent Python oracle reads the same protocol and resolves every vector the same way", () => {
  const python = Bun.which("python3") ?? Bun.which("python") ?? "python3";
  const run = spawnSync(python, ["-I", join(here, "🐍️.py"), join(here, "..", "🔣️.json"), join(here, "🔣️.json")], { encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  const oracle = run.stdout.trim().split("\n").map((line) => JSON.parse(line) as OracleRow);
  expect(oracle.map((row) => row.name)).toEqual(rows.map((row) => row.name));
  for (const [index, row] of rows.entries()) {
    const os = (): readonly string[] => row.os, choice = chooseLanguageV1(vectors.languages, row.args, row.environment, os);
    expect(oracle[index]!.choice, row.name).toEqual("refusal" in choice ? { refusal: choice.refusal.kind } : choice);
    expect(oracle[index]!.notices, row.name).toEqual([...noticeLanguagesV1(vectors.languages, row.environment, row.args)]);
  }
});

test("the language subtag agrees with the platform locale parser", () => {
  for (const [tag, language] of vectors.tags as unknown as readonly (readonly [string, string])[]) {
    expect(languageSubtagV1(tag), tag).toBe(language);
    expect(new Intl.Locale(tag.replace(/[.@].*$/u, "").replace("_", "-")).language, tag).toBe(language);
  }
  for (const tag of vectors.neutral) expect(languageSubtagV1(tag), JSON.stringify(tag)).toBeUndefined();
  expect(languageSubtagV1(undefined)).toBeUndefined();
});

test("the flag reader stops at the separator and keeps the last value", () => {
  expect(languageFlagV1(["a", "--lang", "de", "--", "--lang=en"])).toBe("de");
  expect(languageFlagV1(["--lang=en", "--lang=de"])).toBe("de");
  expect(languageFlagV1([])).toBeUndefined();
});

test("the refusal is built for any offered language set and names its choices", () => {
  expect(languageRefusalV1(["en"]).message.split("\n")).toHaveLength(1);
  expect(languageRefusalV1(["en", "de", "fr"]).message).toContain("--lang=en|de|fr");
  expect(languageRefusalV1(["en", "de"], { kind: "flag", value: "x$&" }).message).toBe("Unknown language: x$& (en|de)\nUnbekannte Sprache: x$& (en|de)");
});

test("a notice never reads the operating system list, a command does", () => {
  expect(noticeLanguagesV1(["en", "de"], { LANG: "C" })).toEqual(["en", "de"]);
  expect(noticeLanguagesV1(["en", "de"], { LANG: "C" }, ["--lang=de"])).toEqual(["de"]);
  expect(chooseLanguageV1(["en", "de"], [], { LANG: "C" }, () => ["de-AT"])).toEqual({ language: "de", source: "os" });
  expect(protocol.withoutChoice).toEqual({ commands: "operatingSystemThenRefuse", notices: "everyOffered" });
});

test("the operating system list is read by the protocol command and is a list of language tags on every platform", () => {
  const list = osLanguageListV1();
  expect(osLanguageListV1()).toBe(list);
  for (const tag of list) expect(languageSubtagV1(tag), tag).toMatch(/^[a-z]{2,3}$/u);
  const sample = '(\n    "en-US",\n    "de-CH",\n    "zh-Hans-CN"\n)';
  expect([...sample.matchAll(new RegExp(protocol.osLocale.darwin.pattern, "gu"))].map((match) => match[1])).toEqual(["en-US", "de-CH", "zh-Hans-CN"]);
});

test("the Go launcher reads this protocol and resolves the same vectors", () => {
  const registry = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json"), "utf8")) as { language: { protocol: string } };
  expect(join(root, registry.language.protocol)).toBe(join(here, "..", "🔣️.json"));
  const contract = readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🗣️language-contract/🐹️.go"), "utf8");
  expect(contract).toContain(`"${join(here, "🔣️.json").slice(root.length + 1).split("\\").join("/")}"`);
});

test("no other module picks a language: the protocol is the one source every runtime reads", () => {
  const roots = ["🧰️framework/🔨️modules/⚙️runtime", "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/📁️filesystem", "🧰️framework/🛍️products/🦑️repo/🔨️modules"];
  const readers = ["🧰️framework/🔨️modules/⚙️runtime/🗣️language/", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go/🗣️language/🐹️.go", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🌐️language/🦀️.rs", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/"];
  const idioms = [/split\(\s*\/\[[-_.:@\\]{3,7}\]\/u?\s*\)/u, /\.slice\(0,\s*2\)\.toLowerCase\(\)|toLowerCase\(\)\.slice\(0,\s*2\)/u, /re\.split\(r"\[[-_.:@]{3,5}\]"/u, /split\(\['\.',\s*'@'\]\)/u, /MustCompile\(`\[\._@-\]`\)/u, /\bSEMIO_DASHBOARD_LOCALE\b|\bSEMIO_LOCALE\b/u];
  const variables = new Set([protocol.explicitVariable, protocol.preferenceListVariable, ...protocol.environmentOrder]);
  const findings: string[] = [];
  const orderCopies = (value: unknown, path: string): void => {
    if (Array.isArray(value)) { if (value.length >= 3 && value.every((item) => typeof item === "string" && (variables.has(item) || /^SEMIO_[A-Z_]*LOCALE$/u.test(item))) && value.filter((item) => protocol.environmentOrder.includes(item)).length >= 2) findings.push(`${path}: a copy of the variable order`); value.forEach((item, index) => orderCopies(item, `${path}[${index}]`)); }
    else if (value !== null && typeof value === "object") for (const [key, item] of Object.entries(value)) orderCopies(item, `${path}/${key}`);
  };
  const walk = (directory: string): void => {
    for (const entry of readdirSync(join(root, directory), { withFileTypes: true })) {
      const rel = `${directory}/${entry.name}`;
      if (entry.name.startsWith(".") || ["node_modules", "dist", "target", "🤖️generated"].includes(entry.name) || readers.some((reader) => rel.startsWith(reader))) continue;
      if (entry.isDirectory()) { walk(rel); continue; }
      if (!/\.(ts|mts|mjs|cjs|js|rs|go|py|json)$/u.test(entry.name) || statSync(join(root, rel)).size > 4 << 20) continue;
      const text = readFileSync(join(root, rel), "utf8");
      for (const idiom of idioms) if (idiom.test(text)) findings.push(`${rel}: ${idiom.source}`);
      if (entry.name.endsWith(".json")) { try { orderCopies(JSON.parse(text), rel); } catch {} }
    }
  };
  for (const directory of roots) walk(directory);
  expect(findings).toEqual([]);
});
