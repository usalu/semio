import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import cases from "./🧫️cases.json" with { type: "json" };
import { DEMONSTRATOR_LOCKED_LOCALE, demonstratorFormat, demonstratorText, demonstratorTranslationTree, type DemonstratorTextKey } from "../../🔨️modules/🌐️locale/🟦️.ts";

const LOCALES = ["en", "de"] as const;
const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../..");
const read = (path: string): string => readFileSync(join(repoRoot, path), "utf8");
const readJson = (path: string): any => JSON.parse(read(path));
const placeholders = (text: string, pattern: RegExp): string[] => [...text.matchAll(pattern)].map((match) => match[1]!).sort();

function* walk(directory: string, skip: readonly string[]): Generator<string> {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory()) { if (!skip.includes(entry.name)) yield* walk(join(directory, entry.name), skip); }
    else if (entry.name.endsWith(".rs")) yield join(directory, entry.name);
  }
}

/** 🌐️ Every user-visible text of the demonstrator closure exists in English AND German: the page's own catalog, the composed plugin manifests of the eight panes, the framework fault notices, and the plugin sources' localized labels. Language-agnostic twin: `🐍️.py`, driven by the same `🧫️cases.json`. */
export async function registerDemonstratorLocaleTests(vitest: NonNullable<ImportMeta["vitest"]>): Promise<void> {
  const { describe, expect, it } = vitest;
  const catalog = readJson(cases.catalog) as { lockedLocale: string; locales: string[]; texts: Record<string, Record<string, string>> };

  describe("demonstrator page catalog", () => {
    it("satisfies its schema of record (Ajv oracle) and refuses a row without a German cell", () => {
      const validate = new Ajv({ strict: true, allErrors: true }).compile(readJson(cases.schema));
      expect(validate(catalog), JSON.stringify(validate.errors)).toBe(true);
      const [key, cells] = Object.entries(catalog.texts)[0]!;
      expect(validate({ ...catalog, texts: { ...catalog.texts, [key]: { en: cells.en } } })).toBe(false);
      expect(validate({ ...catalog, texts: { ...catalog.texts, [key]: { en: cells.en, de: "  " } } })).toBe(false);
    });

    it("carries every key in every locale with the same placeholders", () => {
      expect(catalog.locales).toEqual([...LOCALES]);
      for (const [key, cells] of Object.entries(catalog.texts)) {
        for (const locale of LOCALES) expect(cells[locale]?.trim(), `${key} ${locale}`).toBeTruthy();
        expect(placeholders(cells.en!, /\{\{(\w+)\}\}/g), key).toEqual(placeholders(cells.de!, /\{\{(\w+)\}\}/g));
      }
    });

    it("pairs every label's normal and beginner wording", () => {
      const keys = Object.keys(catalog.texts);
      for (const key of keys.filter((candidate) => candidate.endsWith(".label.normal"))) expect(keys, key).toContain(key.replace(/normal$/, "beginner"));
      for (const key of keys.filter((candidate) => candidate.endsWith(".label.beginner"))) expect(keys, key).toContain(key.replace(/beginner$/, "normal"));
    });

    it("nests every locale into a translation tree holding exactly its texts", () => {
      for (const locale of LOCALES) {
        const leaves: Record<string, string> = {};
        const visit = (node: Record<string, unknown>, path: string[]): void => { for (const [name, child] of Object.entries(node)) typeof child === "string" ? (leaves[[...path, name].join(".")] = child) : visit(child as Record<string, unknown>, [...path, name]); };
        visit(demonstratorTranslationTree(locale), []);
        expect(leaves).toEqual(Object.fromEntries(Object.entries(catalog.texts).filter(([key]) => !key.startsWith(cases.brandSources.keyPrefix)).map(([key, cells]) => [key, cells[locale]])));
      }
    });

    it("fills placeholders and never falls back to another locale", () => {
      expect(demonstratorFormat("demonstrator.footer.newTab", "en", { name: "X" })).toBe("X (opens in a new tab)");
      expect(demonstratorFormat("demonstrator.footer.newTab", "de", { name: "X" })).toBe("X (öffnet in einem neuen Tab)");
      for (const key of Object.keys(catalog.texts) as DemonstratorTextKey[]) expect(demonstratorText(key, "de")).toBe(catalog.texts[key]!.de);
    });
  });

  describe("demonstrator page chrome", () => {
    it("locks one locale and declares it on the document", () => {
      expect(DEMONSTRATOR_LOCKED_LOCALE).toBe(catalog.lockedLocale);
      expect(read(cases.html)).toContain(`<html lang="${catalog.lockedLocale}">`);
    });

    it("holds no hard-coded user-facing literal or locale ternary in its components", () => {
      for (const path of cases.componentSources) for (const rule of cases.forbiddenSourcePatterns) expect([...read(path).matchAll(new RegExp(rule.pattern, "g"))].map((match) => match[0]), `${path} ${rule.id}`).toEqual([]);
    });
  });

  describe("composed pane manifests", () => {
    for (const manifest of cases.composedManifests) {
      it(`${manifest.path} names every label in both locales and both terminologies`, () => {
        let cells = 0;
        const bad: string[] = [];
        const visit = (node: unknown, path: string): void => {
          if (Array.isArray(node)) return node.forEach((child, index) => visit(child, `${path}/${index}`));
          if (typeof node !== "object" || node === null) return;
          const keys = Object.keys(node).sort().join(",");
          const row = node as Record<string, any>;
          if (keys === "native,reuse") {
            for (const terminology of ["native", "reuse"]) for (const locale of LOCALES) { cells++; if (typeof row[terminology]?.[locale] !== "string" || !row[terminology][locale].trim() || Object.keys(row[terminology]).length !== 2) bad.push(`${path}/${terminology}/${locale}`); }
          } else if (keys === "de,en") {
            for (const locale of LOCALES) { cells++; if (typeof row[locale] !== "string" || !row[locale].trim()) bad.push(`${path}/${locale}`); }
          } else for (const [name, child] of Object.entries(row)) visit(child, `${path}/${name}`);
        };
        visit(readJson(manifest.path), "");
        expect(bad.slice(0, 5)).toEqual([]);
        expect(cells).toBeGreaterThanOrEqual(manifest.minLabelCells);
      });
    }
  });

  describe("framework fault notices", () => {
    for (const fixture of cases.noticeFixtures) {
      it(`${fixture.path} tells every refusal in both locales with the same placeholders`, () => {
        const rows = readJson(fixture.path).notices as { code: string; en: string; de: string }[];
        expect(rows.length).toBeGreaterThanOrEqual(fixture.minRows);
        expect(new Set(rows.map((row) => row.code)).size).toBe(rows.length);
        for (const row of rows) {
          for (const locale of LOCALES) expect(row[locale]?.trim(), `${row.code} ${locale}`).toBeTruthy();
          expect(row.en, `${row.code} is translated`).not.toBe(row.de);
          expect(placeholders(row.en, /\{(\w+)\}/g), row.code).toEqual(placeholders(row.de, /\{(\w+)\}/g));
        }
        for (const code of fixture.requiredCodes) expect(rows.map((row) => row.code), code).toContain(code);
      });
    }
  });

  describe("pane plugin sources", () => {
    it("pass every localized label with an English and a German text", () => {
      const pattern = new RegExp(cases.pluginRoots.pairPattern, "g");
      let pairs = 0;
      const bad: string[] = [];
      for (const root of cases.pluginRoots.paths) for (const file of walk(join(repoRoot, root), cases.pluginRoots.skipDirectories)) for (const match of readFileSync(file, "utf8").matchAll(pattern)) { pairs++; if (!match[1]!.trim() || !match[2]!.trim()) bad.push(`${file}: ${match[0]}`); }
      expect(bad.slice(0, 5)).toEqual([]);
      expect(pairs).toBeGreaterThanOrEqual(cases.pluginRoots.minPairs);
    });

    it("pass every introduction interaction label as an English and German pair, never a plain string", () => {
      const forbidden = new RegExp(cases.pluginRoots.forbiddenPatterns[0]!);
      const localized = new RegExp(cases.pluginRoots.introductionLabelPattern, "g");
      let labels = 0;
      const plain: string[] = [];
      for (const root of cases.pluginRoots.paths) for (const file of walk(join(repoRoot, root), cases.pluginRoots.skipDirectories)) {
        const source = readFileSync(file, "utf8");
        if (forbidden.test(source)) plain.push(file);
        labels += [...source.matchAll(localized)].length;
      }
      expect(plain).toEqual([]);
      expect(labels).toBeGreaterThanOrEqual(cases.pluginRoots.minIntroductionLabels);
    });
  });
  describe("brand manifests", () => {
    const brand = cases.brandSources;
    const used = (): string[] => brand.paths.flatMap((path) => [...read(path).matchAll(new RegExp(brand.keyPattern, "g"))].map((match) => match[1]!));

    it("hold no plain-language literal in a user-facing field", () => {
      for (const path of brand.paths) {
        const source = read(path);
        expect([...source.matchAll(new RegExp(brand.forbiddenPattern, "g"))].map((match) => match[0]), path).toEqual([]);
        expect([...source.matchAll(new RegExp(brand.forbiddenPatternMultiline, "g"))].map((match) => match[0]), path).toEqual([]);
      }
    });

    it("name only catalog keys and leave none of the brand keys unused", () => {
      const keys = Object.keys(catalog.texts).filter((key) => key.startsWith(brand.keyPrefix));
      expect(keys.length).toBeGreaterThanOrEqual(brand.minKeys);
      expect([...new Set(used())].sort()).toEqual([...keys].sort());
    });

    it("translate every brand text rather than copying one language into both cells", () => {
      for (const [key, cells] of Object.entries(catalog.texts)) if (key.startsWith(brand.keyPrefix) && !brand.identicalAllowed.includes(key)) expect(cells.en, key).not.toBe(cells.de);
    });
  });

  describe("framework locale resolution", () => {
    for (const rule of cases.frameworkSources) {
      it(`${rule.path} assumes no language and declares its schema`, () => {
        const source = read(rule.path);
        for (const pattern of rule.forbidden) expect(new RegExp(pattern).test(source), pattern).toBe(false);
        for (const pattern of rule.required) expect(new RegExp(pattern).test(source), pattern).toBe(true);
      });
    }
  });
}
