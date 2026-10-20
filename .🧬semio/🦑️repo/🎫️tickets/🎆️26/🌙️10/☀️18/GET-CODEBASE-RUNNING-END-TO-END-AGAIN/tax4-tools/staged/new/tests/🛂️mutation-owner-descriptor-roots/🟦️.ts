import { describe, expect, test } from "bun:test";
import picomatch from "picomatch";
import fixture from "../🧫️fixtures/🛂️mutation-owner-descriptor-roots/🔣️.json";
import { loadCatalogTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { mutationOwnerRootHoldsDescriptors } from "../../🧹️normalization/🗺️artifact-projection-catalogs/🟦️.ts";

const taxonomy = loadCatalogTaxonomy();
const consumerMatchers = fixture.consumerRootPatterns.map((pattern) => picomatch(pattern, { dot: true }));

describe("mutation owner roots that hold payload descriptors (language-agnostic fixture)", () => {
  test("the fixture vocabulary is the taxonomy vocabulary", () => {
    expect({ fixtureMirror: taxonomy.testFixturesDirName, representations: [...taxonomy.representationDirs] }).toEqual(fixture.vocabulary);
  });

  for (const row of fixture.cases) {
    test(`${row.id}: ${row.note}`, () => {
      expect(mutationOwnerRootHoldsDescriptors(row.root, taxonomy)).toBe(row.holdsDescriptors);
    });

    test(`${row.id}: picomatch agrees on the consumer-root classification`, () => {
      expect(!consumerMatchers.some((matches) => matches(row.root))).toBe(row.holdsDescriptors);
    });
  }
});
