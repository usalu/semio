import { describe, expect, it } from "vitest";
import { scopeContributionsJson } from "../../🧾️contributions-json/🟦️.ts";
import { examplesForApp, type PluginManifest } from "@semio-tech/framework-manifest";
import fixture from "../🧫️fixtures/🔬️scope-contributions/🔣️.json";

describe("examplesForApp", () => {
  it("offers a surface exactly the rows authored for its dialect and none to a surface without one", () => {
    const dialect = { artifactKind: "s.sourcing.curation", standard: "1", subset: "*" };
    const rows = [{ id: "demo", dialect, artifactJson: "semio …" }, { id: "draw", dialect: { ...dialect, artifactKind: "s.draw.drawing" }, artifactJson: "…" }, { id: "undialected", artifactJson: "…" }];
    expect(examplesForApp(rows, { dialect }).map((row) => row.id)).toEqual(["demo"]);
    expect(examplesForApp(rows, {})).toEqual([]);
  });
});

describe("scopeContributionsJson", () => {
  const loaded = fixture.loaded.map((entry) => ({ pluginId: entry.pluginId, manifest: { topicContributions: [{ topic: entry.topic, payload: { pluginId: entry.pluginId } }] } as Pick<PluginManifest, "topicContributions"> }));
  for (const row of fixture.cases) {
    it(row.id, () => {
      const scoped = JSON.parse(scopeContributionsJson(loaded, row.receiver, row.consumes)) as { pluginId: string; topicContribution: { topic: string } }[];
      expect(scoped.map((entry) => entry.pluginId)).toEqual(row.expect);
      for (const entry of scoped) expect(entry.pluginId === row.receiver || row.consumes.includes(entry.topicContribution.topic)).toBe(true);
    });
  }
  it("forwards a consumed topic whole, independent of any open document", () => {
    const flow = fixture.cases.find((row) => row.id === "flow-receives-every-extension")!;
    const extensions = fixture.loaded.filter((entry) => entry.topic === "flow.extension").map((entry) => entry.pluginId);
    expect(JSON.parse(scopeContributionsJson(loaded, flow.receiver, flow.consumes)).map((entry: { pluginId: string }) => entry.pluginId)).toEqual(extensions);
  });
});
