import { describe, expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🥒️mutation-fixture-coverage/🔣️.json";
import { mutationFixturesDeclaredByFeature } from "../../🔍️discovery/🥒️mutation-fixture-coverage/🟦️.ts";
import { parseFeature } from "../../../../../../🔨️modules/🧪️test/🥒️gherkin/🟦️.ts";

const declaredByPlan = (source: string): string[] => {
  const plan = parseFeature(source), found: string[] = [];
  for (const scenario of plan.scenarios) for (const step of [...plan.background, ...scenario.steps]) {
    const text = JSON.stringify(step), at = text.indexOf(fixture.policy.stepUriPrefix);
    if (at < 0) continue;
    const segments = text.slice(at + fixture.policy.stepUriPrefix.length).split(/["\s]/u)[0]!.split("/"), facet = segments.findIndex((segment) => fixture.policy.bundleFacetDirectoryNames.includes(segment));
    if (facet > 0) found.push(segments.slice(0, facet).join("/").replaceAll("️", "").normalize("NFC"));
  }
  return found;
};

describe("mutation fixture coverage declared by an exhaustive case (language-agnostic fixture)", () => {
  for (const row of fixture.cases) {
    const source = row.feature.join("\n");
    test(`${row.id}: ${row.note}`, () => {
      expect([...mutationFixturesDeclaredByFeature(source, fixture.policy)]).toEqual(row.expected.map((bundle) => bundle.normalize("NFC")));
    });
    test(`${row.id}: the repository's own scenario planner expands the same bundles`, () => {
      expect([...new Set(declaredByPlan(source))]).toEqual(row.expected.map((bundle) => bundle.normalize("NFC")));
    });
  }
});
