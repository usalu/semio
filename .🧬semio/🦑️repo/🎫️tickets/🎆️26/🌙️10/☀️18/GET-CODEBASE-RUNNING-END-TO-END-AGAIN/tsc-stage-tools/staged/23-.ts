import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import fixture from "../🧫️fixtures/🧱️package-implementation-admission/🔣️.json";
import { loadCatalogTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { packageImplementationAdmitted, type PackageImplementationAdmission } from "../../🔍️discovery/🧱️package-implementation-admission/🟦️.ts";
import { classifyPackageSource } from "../../🔍️discovery/🏷️package-source-classification/🟦️.ts";

/** 🧷️ A source fixture row whose authored `expectedRole` is read as the closed role the classifier returns. */
type SourceCaseRow = Omit<(typeof fixture.sourceCases)[number], "expectedRole"> & { readonly expectedRole: ReturnType<typeof classifyPackageSource>["role"] };

const formatterParses = (language: string, source: string): boolean | null => {
  const run = language === "go" ? spawnSync("gofmt", ["-e"], { input: source, encoding: "utf8" }) : spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout"], { input: source, encoding: "utf8" });
  return run.error ? null : run.status === 0;
};

describe("package implementation admission (language-agnostic fixture)", () => {
  for (const row of fixture.admissionCases) {
    test(`${row.id}`, () => {
      const names = new Set(row.directoryFiles);
      expect(packageImplementationAdmitted(row.admission as PackageImplementationAdmission | null | undefined, { relativePath: row.relativePath, kindId: row.kindId, directoryHasFile: (name) => names.has(name), manifestFilename: (id) => (fixture.manifestFilenames as Record<string, string>)[id] ?? null })).toBe(row.expected);
    });
  }

  test("the live schema admits Go sub-packages and Cargo crates and nothing else", () => {
    const rules = loadCatalogTaxonomy().packageBoundaryRules;
    expect(rules["🐹️go"]!.implementationAdmission).toEqual({ form: "nested-source-unit", unitSourceFileKindId: "go-source" });
    expect(rules["🦀️rust"]!.implementationAdmission).toEqual({ form: "nested-manifest-unit", unitManifestContractId: "cargo-manifest" });
    for (const [id, rule] of Object.entries(rules)) if (id !== "🐹️go" && id !== "🦀️rust") expect(rule.implementationAdmission ?? null).toBeNull();
  });

  test("every rule of the unparsed taxonomy document answers the admission, whether or not it declares one", () => {
    const rules = JSON.parse(readFileSync(new URL("../../🔣️taxonomy.json", import.meta.url), "utf8")).packageBoundaryRules as Record<string, { implementationAdmission?: PackageImplementationAdmission | null }>;
    const input = { relativePath: "🧩️unit/🦀️.rs", kindId: "rust-source", directoryHasFile: () => false, manifestFilename: () => null };
    for (const [id, rule] of Object.entries(loadCatalogTaxonomy().packageBoundaryRules)) {
      const declared = rules[id]!.implementationAdmission;
      expect(rule.implementationAdmission ?? null).toEqual(declared ?? null);
      expect(() => packageImplementationAdmitted(rule.implementationAdmission, input)).not.toThrow();
      if (declared === undefined || declared === null) expect(packageImplementationAdmitted(rule.implementationAdmission, input)).toBe(false);
    }
  });

  for (const row of fixture.sourceCases as readonly SourceCaseRow[]) {
    const source = row.source.join("\n") + "\n";
    test(`${row.id}: role`, () => {
      const grammar = loadCatalogTaxonomy().packageGlueGrammar[row.language]!;
      expect(classifyPackageSource(source, grammar).role).toBe(row.expectedRole);
    });
    test(`${row.id}: the language's own formatter parses the source`, () => {
      const parses = formatterParses(row.language, source);
      if (parses !== null) expect(parses).toBe(true);
    });
  }
});
