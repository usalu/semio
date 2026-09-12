import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv, { type AnySchema } from "ajv/dist/2020.js";
import { applyPatch, compare } from "fast-json-patch";
import { applyArchitectRegisterWindowConfigMutation, type ArchitectRegisterWindowConfig, type ArchitectRegisterWindowConfigMutation } from "../../🧬️schema/🟦️.ts";
import { applyArchitectAdjacencyWindowConfigMutation, type ArchitectAdjacencyWindowConfig, type ArchitectAdjacencyWindowConfigMutation } from "../../../../↔️adjacency/🎚️config/🧬️schema/🟦️.ts";
import { applyArchitectGraphWindowConfigMutation, type ArchitectGraphWindowConfig, type ArchitectGraphWindowConfigMutation } from "../../../../🕸️graph/🎚️config/🧬️schema/🟦️.ts";
import { applyArchitectReportWindowConfigMutation, type ArchitectReportWindowConfig, type ArchitectReportWindowConfigMutation } from "../../../../📓️report/🎚️config/🧬️schema/🟦️.ts";

type Window<C> = { id: string; kind: string; config: C };
type Pair<C> = { left: Window<C>; right: Window<C> };
type Fixture = {
  document: { bytes: string; reports: { id: string; title: string }[] };
  windows: {
    register: Pair<ArchitectRegisterWindowConfig>;
    adjacency: Pair<ArchitectAdjacencyWindowConfig>;
    graph: Pair<ArchitectGraphWindowConfig>;
    report: Pair<ArchitectReportWindowConfig>;
  };
  leftMutations: {
    register: ArchitectRegisterWindowConfigMutation;
    adjacency: ArchitectAdjacencyWindowConfigMutation;
    graph: ArchitectGraphWindowConfigMutation;
    report: ArchitectReportWindowConfigMutation;
  };
  expectedLeft: {
    register: ArchitectRegisterWindowConfig;
    adjacency: ArchitectAdjacencyWindowConfig;
    graph: ArchitectGraphWindowConfig;
    report: ArchitectReportWindowConfig;
  };
  deletedReportSelection: ArchitectReportWindowConfig;
};

const json = (url: URL): AnySchema => JSON.parse(readFileSync(url, "utf8")) as AnySchema;
const fixture = json(new URL("../../🧫️fixtures/🔬️window-ownership/🔣️.json", import.meta.url)) as Fixture;
const schemas = {
  register: json(new URL("../../🧬️schema/🔣️.json", import.meta.url)),
  adjacency: json(new URL("../../../../↔️adjacency/🎚️config/🧬️schema/🔣️.json", import.meta.url)),
  graph: json(new URL("../../../../🕸️graph/🎚️config/🧬️schema/🔣️.json", import.meta.url)),
  report: json(new URL("../../../../📓️report/🎚️config/🧬️schema/🔣️.json", import.meta.url)),
};
const viewportSchema = json(new URL("../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🔣️.json", import.meta.url));
const appConfigSchema = json(new URL("../../../../../../../🎚️config/🧬️schema/🔣️.json", import.meta.url)) as { properties: Record<string, unknown> };

/** 🧪️ Proves four exact owner kinds isolate same-kind instances and authored report identities. */
export function testArchitectWindowOwnershipOracle(): void {
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(viewportSchema);
  const validators = {
    register: ajv.compile(schemas.register),
    adjacency: ajv.compile(schemas.adjacency),
    graph: ajv.compile(schemas.graph),
    report: ajv.compile(schemas.report),
  };
  for (const key of Object.keys(validators) as (keyof typeof validators)[]) {
    for (const window of Object.values(fixture.windows[key])) assert(validators[key](window.config), JSON.stringify(validators[key].errors));
  }

  const documentBefore = structuredClone(fixture.document);
  const rightBefore = {
    register: structuredClone(fixture.windows.register.right),
    adjacency: structuredClone(fixture.windows.adjacency.right),
    graph: structuredClone(fixture.windows.graph.right),
    report: structuredClone(fixture.windows.report.right),
  };
  const projected = {
    register: applyArchitectRegisterWindowConfigMutation(fixture.windows.register.left.config, fixture.leftMutations.register),
    adjacency: applyArchitectAdjacencyWindowConfigMutation(fixture.windows.adjacency.left.config, fixture.leftMutations.adjacency),
    graph: applyArchitectGraphWindowConfigMutation(fixture.windows.graph.left.config, fixture.leftMutations.graph),
    report: applyArchitectReportWindowConfigMutation(fixture.windows.report.left.config, fixture.leftMutations.report),
  };
  for (const key of Object.keys(projected) as (keyof typeof projected)[]) {
    const base = structuredClone(fixture.windows[key].left.config);
    const independentlyPatched = applyPatch(base, compare(base, projected[key]), true, false).newDocument;
    assert.deepEqual(independentlyPatched, fixture.expectedLeft[key]);
    assert(validators[key](JSON.parse(JSON.stringify(projected[key]))), JSON.stringify(validators[key].errors));
    assert.deepEqual(fixture.windows[key].right, rightBefore[key]);
  }
  assert.deepEqual(fixture.document, documentBefore);

  const selected = fixture.document.reports.find((report) => report.id === projected.report.selectedReportId);
  const missing = fixture.document.reports.find((report) => report.id === fixture.deletedReportSelection.selectedReportId);
  assert.equal(selected?.title, "Left Report");
  assert.equal(missing, undefined);
  assert.deepEqual(Object.keys(fixture.windows.report.left.config), []);
  assert(!("activeReportJson" in appConfigSchema.properties));
  for (const moved of ["activeRegister", "adjacencyKindFilter", "graphCameraX", "graphCameraY", "graphCameraZoom"]) assert(!(moved in appConfigSchema.properties));
}

if (import.meta.main) {
  testArchitectWindowOwnershipOracle();
  console.log("architect-window-ownership ajv=valid json-patch=valid four-owner-isolation=valid reload=valid report-identity=valid");
}
