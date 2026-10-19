import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import fixture from "../🧫️fixtures/🧱️package-implementation-admission/🔣️.json";
import { loadCatalogTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { packageImplementationAdmitted, type PackageImplementationAdmission } from "../../🔍️discovery/🧱️package-implementation-admission/🟦️.ts";
import { classifyPackageSource } from "../../🔍️discovery/🏷️package-source-classification/🟦️.ts";

const formatterParses = (language: string, source: string): boolean | null => {
  const run = language === "go" ? spawnSync("gofmt", ["-e"], { input: source, encoding: "utf8" }) : spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout"], { input: source, encoding: "utf8" });
  return run.error ? null : run.status === 0;
};

describe("package implementation admission (language-agnostic fixture)", () => {
  for (const row of fixture.admissionCases) {
    test(`${row.id}`, () => {
      const names = new Set(row.directoryFiles);
      expect(packageImplementationAdmitted(row.admission as PackageImplementationAdmission | null, { relativePath: row.relativePath, kindId: row.kindId, directoryHasFile: (name) => names.has(name), manifestFilename: (id) => (fixture.manifestFilenames as Record<string, string>)[id] ?? null })).toBe(row.expected);
    });
  }

  test("the live schema admits Go sub-packages and Cargo crates and nothing else", () => {
    const rules = loadCatalogTaxonomy().packageBoundaryRules;
    expect(rules["🐹️go"]!.implementationAdmission).toEqual({ form: "nested-source-unit", unitSourceFileKindId: "go-source" });
    expect(rules["🦀️rust"]!.implementationAdmission).toEqual({ form: "nested-manifest-unit", unitManifestContractId: "cargo-manifest" });
    for (const [id, rule] of Object.entries(rules)) if (id !== "🐹️go" && id !== "🦀️rust") expect(rule.implementationAdmission ?? null).toBeNull();
  });

  for (const row of fixture.sourceCases) {
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
