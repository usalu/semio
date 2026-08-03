#!/usr/bin/env bun
import fs from "node:fs";
import path from "node:path";

const root = path.resolve(import.meta.dir, "../../../../../..");
const skipDirs = new Set(["node_modules", "target", "dist", "build", ".git", "pkg", "play", "storybook-static"]);
const exts = new Set([".ts", ".tsx", ".rs", ".json", ".graphql", ".py", ".go", ".cs", ".cypher"]);

const schemaReplacements: [string, string][] = [
  ['"dag.fixture/v1"', '"dag.fixture"'],
  ['"flow.fixture/v1"', '"flow.fixture"'],
  ['"flow.document/v1"', '"flow.document"'],
  ['"flow.extension/v1"', '"flow.extension"'],
  ['"flow.dag/v1"', '"flow.dag"'],
  ['"sequence.fixture/v1"', '"sequence.fixture"'],
  ['"puzzle.2d.fixture/v1"', '"puzzle.2d.fixture"'],
  ['"puzzle.2d/v1"', '"puzzle.2d"'],
  ['"puzzle.3d.fixture/v1"', '"puzzle.3d.fixture"'],
  ['"puzzle.3d/v1"', '"puzzle.3d"'],
  ['"puzzle.5d/v1"', '"puzzle.5d"'],
  ['"trinity.graph/v1"', '"trinity.graph"'],
  ['"writer.document/v1"', '"writer.document"'],
  ['"imperative.document/v1"', '"imperative.document"'],
  ['"imperative.catalogue/v1"', '"imperative.catalogue"'],
  ['"raster.document/v1"', '"raster.document"'],
  ['"draw.document/v1"', '"draw.document"'],
  ['"forms.form/v1"', '"forms.form"'],
  ['"forms.dictionary/v1"', '"forms.dictionary"'],
  ['"presentation.deck/v1"', '"presentation.deck"'],
  ['"manifest/v1"', '"manifest"'],
  ['"lowpoly.fixture/v1"', '"lowpoly.fixture"'],
  ['"shooting.fixture/v1"', '"shooting.fixture"'],
  ['"shooting.scene/v1"', '"shooting.scene"'],
  ['"gis.map/v1"', '"gis.map"'],
  ['"gis.map.fixture/v1"', '"gis.map.fixture"'],
  ['"reasoning.wires.fixture/v1"', '"reasoning.wires.fixture"'],
  ['"procedural.2d/v1"', '"procedural.2d"'],
  ['"procedural.3d/v1"', '"procedural.3d"'],
  ['"procedural.fixture/v1"', '"procedural.fixture"'],
  ['"procedural2d.fixture/v1"', '"procedural2d.fixture"'],
  ['"procedural.fixture/v1"', '"procedural.fixture"'],
  ['"compose.kit/v1"', '"compose.kit"'],
  ['"compose.design/v1"', '"compose.design"'],
  ['"compose.type/v1"', '"compose.type"'],
  ['"vcs.demo/v1"', '"vcs.demo"'],
  ['"layout.fixture/v1"', '"layout.fixture"'],
  ['"writer.play/v1"', '"writer.play"'],
  ['"flow.play/v1"', '"flow.play"'],
  ['"flow.play.generate/v1"', '"flow.play.generate"'],
  ['"flow.play.jack/v1"', '"flow.play.jack"'],
  ['"flow.play.compiled-dag/v1"', '"flow.play.compiled-dag"'],
  ['"dag.play/v1"', '"dag.play"'],
  ['"dag.play.jack/v1"', '"dag.play.jack"'],
  ['"sequence.play/v1"', '"sequence.play"'],
  ['"sequence.play.script/v1"', '"sequence.play.script"'],
  ['"sequence.play.jack/v1"', '"sequence.play.jack"'],
  ['"sequence.play.compiled-dag/v1"', '"sequence.play.compiled-dag"'],
  ['"puzzle.2d.play/v1"', '"puzzle.2d.play"'],
  ['"puzzle.2d.play.jack/v1"', '"puzzle.2d.play.jack"'],
  ['"puzzle.2d.play.compiled-dag/v1"', '"puzzle.2d.play.compiled-dag"'],
  ['"puzzle.3d.play.viewport/v1"', '"puzzle.3d.play.viewport"'],
  ['"puzzle.3d.play.jack/v1"', '"puzzle.3d.play.jack"'],
  ['"puzzle.5d.play.jack/v1"', '"puzzle.5d.play.jack"'],
  ['"procedural.play/v1"', '"procedural.play"'],
  ['"procedural.play.preview/v1"', '"procedural.play.preview"'],
  ['"procedural.play.generate/v1"', '"procedural.play.generate"'],
  ['"procedural2d.play/v1"', '"procedural2d.play"'],
  ['"procedural2d.play.preview/v1"', '"procedural2d.play.preview"'],
  ['"procedural2d.play.generate/v1"', '"procedural2d.play.generate"'],
  ['"imperative.play/v1"', '"imperative.play"'],
  ['"raster.play.composite/v1"', '"raster.play.composite"'],
  ['"raster.play.navigator/v1"', '"raster.play.navigator"'],
  ['"draw.play.composite/v1"', '"draw.play.composite"'],
  ['"draw.play.navigator/v1"', '"draw.play.navigator"'],
  ['"shooting.play.model/v1"', '"shooting.play.model"'],
  ['"shooting.play.icon/v1"', '"shooting.play.icon"'],
  ['"gis.map.play/v1"', '"gis.map.play"'],
  ['"lowpoly.play/v1"', '"lowpoly.play"'],
  ['"s.play.media-graph/v1"', '"s.play.media-graph"'],
  ['"s.play.media-vfs/v1"', '"s.play.media-vfs"'],
  ['"s.play.app-host/v1"', '"s.play.app-host"'],
  ['"s.play.launcher/v1"', '"s.play.launcher"'],
  ['"s.play.history/v1"', '"s.play.history"'],
  ['"s.play.jack/v1"', '"s.play.jack"'],
  ['"s.play.compiled-dag/v1"', '"s.play.compiled-dag"'],
  ['"vcs.play.editor/v1"', '"vcs.play.editor"'],
  ['"vcs.play.history/v1"', '"vcs.play.history"'],
  ['"forms.play.edit/v1"', '"forms.play.edit"'],
  ['"forms.play.try/v1"', '"forms.play.try"'],
  ['"presentation.tile.play/v1"', '"presentation.tile.play"'],
  ['"trinity.jack.play/v1"', '"trinity.jack.play"'],
  ['"trinity.jack.editor/v1"', '"trinity.jack.editor"'],
  ['"trinity.jack.play.editor/v1"', '"trinity.jack.play.editor"'],
  ['"trinity.jack.play.results/v1"', '"trinity.jack.play.results"'],
  ['"trinity.rewrite.before/v1"', '"trinity.rewrite.before"'],
  ['"trinity.rewrite.after/v1"', '"trinity.rewrite.after"'],
  ['"trinity.rewrite.lhs/v1"', '"trinity.rewrite.lhs"'],
  ['"trinity.rewrite.rhs/v1"', '"trinity.rewrite.rhs"'],
  ['"trinity.rewrite.jack/v1"', '"trinity.rewrite.jack"'],
  ['"trinity.rewrite.parameters/v1"', '"trinity.rewrite.parameters"'],
  ['"layout.play.blueprint/v1"', '"layout.play.blueprint"'],
  ['"layout.play.preview/v1"', '"layout.play.preview"'],
  ['"note.play.composite/v1"', '"note.play.composite"'],
  ['"note.play.navigator/v1"', '"note.play.navigator"'],
  ['"forms.module/v1"', '"forms.module"'],
  ['"compose.sketchpad.surface.type.representation/v1"', '"compose.sketchpad.surface.type.representation"'],
  ['"compose.sketchpad.surface.docs.page/v1"', '"compose.sketchpad.surface.docs.page"'],
  ['"compose.sketchpad.surface.feedback.form/v1"', '"compose.sketchpad.surface.feedback.form"'],
  ['"compose.sketchpad.surface.kit.wires/v1"', '"compose.sketchpad.surface.kit.wires"'],
  ['"compose.sketchpad.surface.design.scene/v1"', '"compose.sketchpad.surface.design.scene"'],
  ['"compose.sketchpad.surface.design.diagram/v1"', '"compose.sketchpad.surface.design.diagram"'],
  ['"compose.sketchpad.surface.type.scene/v1"', '"compose.sketchpad.surface.type.scene"'],
  ['"test.playground.panel.workbench/v1"', '"test.playground.panel.workbench"'],
  ['"test.playground.panel.details/v1"', '"test.playground.panel.details"'],
  ['"puzzle.2d.fixture/v1"', '"puzzle.2d.fixture"'],
  ['"puzzle.3d.fixture/v1"', '"puzzle.3d.fixture"'],
];

const identifierReplacements: [string, string][] = [
  ["FlowFixtureV1", "FlowFixture"],
  ["FlowWidgetV1", "FlowWidget"],
  ["DagFixtureV1", "DagFixture"],
  ["DagNodeV1", "DagNode"],
  ["SequenceFixtureV1", "SequenceFixture"],
  ["SequenceStepV1", "SequenceStep"],
  ["PresentationDeckV1", "PresentationDeck"],
  ["TrinityFixtureV1", "TrinityFixture"],
  ["RuleParameterV1", "RuleParameter"],
  ["WiresFixtureKindCatalogsV1", "WiresFixtureKindCatalogs"],
  ["WiresFixtureRelationshipV1", "WiresFixtureRelationship"],
  ["WiresFixtureIdentityV1", "WiresFixtureIdentity"],
  ["WiresFixtureV1", "WiresFixture"],
  ["WriterDocumentV1", "WriterDocument"],
  ["ShootingCameraV1", "ShootingCamera"],
  ["ShootingFixtureV1", "ShootingFixture"],
  ["ShootingSceneV1", "ShootingScene"],
  ["ShootingShotV1", "ShootingShot"],
  ["SStudioDocumentV1", "SStudioDocument"],
  ["GisMapFixturePositionV1", "GisMapFixturePosition"],
  ["GisMapFixtureRouteV1", "GisMapFixtureRoute"],
  ["GisMapFixtureV1", "GisMapFixture"],
  ["parseGisMapFixturePositionV1", "parseGisMapFixturePosition"],
  ["parseGisMapFixtureRouteV1", "parseGisMapFixtureRoute"],
  ["parseGisMapFixtureV1", "parseGisMapFixture"],
];

function walk(dir: string, files: string[] = []): string[] {
  for (const name of fs.readdirSync(dir)) {
    if (skipDirs.has(name)) continue;
    const full = path.join(dir, name);
    let st: fs.Stats;
    try {
      st = fs.statSync(full);
    } catch {
      continue;
    }
    if (st.isDirectory()) {
      if (full.includes(`${path.sep}.repo${path.sep}`)) continue;
      walk(full, files);
    } else if (exts.has(path.extname(name))) {
      files.push(full);
    }
  }
  return files;
}

function applyReplacements(content: string, pairs: [string, string][]): string {
  let out = content;
  for (const [from, to] of pairs) {
    out = out.split(from).join(to);
  }
  return out;
}

const files = walk(root);
let changed = 0;
for (const file of files) {
  const before = fs.readFileSync(file, "utf8");
  let next = applyReplacements(before, identifierReplacements);
  next = applyReplacements(next, schemaReplacements);
  if (next !== before) {
    fs.writeFileSync(file, next);
    changed += 1;
  }
}
console.log(`[DEBUG] bulk finish v1 cleanup touched ${changed} files`);
