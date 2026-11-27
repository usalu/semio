import {expect, test} from "bun:test";
import {readFileSync} from "node:fs";
import {parseFeature} from "../../../../../../../🧰️framework/🔨️modules/🧪️test/🥒️gherkin/🟦️.ts";
import {loadOracleRegistry} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🤝️contributions/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const tests = `${root}/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests`;

for (const slug of ["💰️costs", "🧙️modify-bim-1", "🛠️gestures-bim-1"]) {
  test(`${slug} parses and its oracle and capability are registered`, () => {
    const feature = parseFeature(readFileSync(`${tests}/${slug}/🥒️.feature`, "utf8"));
    expect(feature.errors).toEqual([]);
    const registry = loadOracleRegistry(root);
    const oracle = registry.oracles.find((entry) => entry.id === feature.oracle);
    expect(oracle?.capabilities).toContain(feature.capability!);
    expect(oracle?.comparisonProfiles).toContain(feature.comparison!);
    console.log(slug, feature.scenarios.map((scenario) => scenario.id).join(" "));
  });
}
