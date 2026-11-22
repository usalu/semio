import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const load = (path: string) => import(pathToFileURL(resolve(path)).href);
const { decodePackValue, encodePackValue, isPackInteger, packUInt } = await load("🧰️framework/🔨️modules/🎁️packs/🟦️.ts");
const { verifySemanticDescriptorPairBytesV1 } = await load("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts");
const owner = "🌎️hub/🧩️compositions/🌍️gis/🧩️component";
const terrain = "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any";
const read = (path: string) => readFileSync(terrain + "/" + path, "utf8");
const jsonPath = owner + "/🔣️.json";
const packPath = owner + "/🛂️.descriptor.semio";
const jsonText = readFileSync(jsonPath, "utf8");
const originalPack = readFileSync(packPath);
const baseline = verifySemanticDescriptorPairBytesV1(Buffer.from(jsonText), originalPack);
if (process.argv.includes("--verify")) {
  console.log(JSON.stringify({ plugin: baseline.pluginId, sha256: baseline.semanticSha256 }));
  process.exit(0);
}
const json = JSON.parse(jsonText);
const pack = decodePackValue(originalPack);
const labels = (en: string, de: string) => ({ native: { en, de }, reuse: { en, de } });
const unique = (source: string, expression: RegExp) => {
  const rows = [...source.matchAll(new RegExp(expression.source, "gu"))];
  assert.equal(rows.length, 1, String(expression));
  return rows[0]!;
};
const named = (items: any[], id: string) => {
  const matches = items.filter(item => item.id === id);
  assert.equal(matches.length, 1, id);
  return matches[0];
};
const number = (min: number, max: number, step: number | undefined, integer: boolean) =>
  ({ kind: "number", min, max, ...(step === undefined ? {} : { step }), integer });
const signedMin = -(2 ** 31) / 1000;
const signedMax = (2 ** 31 - 1) / 1000;
const grid = JSON.parse(read("🖼️assets/🎬️demo/🏔️elevation.json"));
const windowSource = read("✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️.rs");
const utilityId = unique(windowSource, /const MOVE_POSITION_UTILITY: &str = "([^"]+)"/u)[1];
const [, moveEn, moveDe, moveIcon] = unique(windowSource, /\(MOVE_POSITION_UTILITY, "([^"]+)", "([^"]+)", "([^"]+)"\)/u);
const appSource = read("✏️editor/🦀️.rs");
const [, brushIcon] = unique(appSource, /ActionDefinition::new\("configureTerrainBrush", LocalizedLabel::native\("[^"]+", "[^"]+"\), semio_plugin::ActionKind::View, "([^"]+)"\)/u);
const [, importEn, importDe] = unique(appSource, /\.action_describe\(import_elevation::REQUEST_ACTION, LocalizedLabel::native\("([^"]+)", "([^"]+)"\)\)/u);
const brushSource = read("✏️editor/🎮️commands/👁️view/🦀️.rs");
const [, brushEn, brushDe] = unique(brushSource, /\(INVALID, LocalizedLabel::native\("([^"]+)", "([^"]+)"\)\)/u);
const brushSettings = read("✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🖌️brush/🦀️.rs");
const maximumStep = Number(unique(brushSettings, /MAX_ELEVATION_STEP_MILLIMETRES: u32 = ([0-9_]+);/u)[1].replaceAll("_", "")) / 1000;
const defaultStep = Number(unique(brushSettings, /Self \{ radius: \d+, elevation_step_millimetres: ([0-9_]+), smooth_strength: \d+ \}/u)[1].replaceAll("_", "")) / 1000;
const gridSource = read("✏️editor/🎮️commands/🏔️elevation-source/➕️create-grid/🦀️.rs");
const defaultGrid = Number(unique(gridSource, /DEFAULT_GRID_SIZE: u32 = (\d+);/u)[1]);
const snapshotSource = read("🧬️schema/📸️snapshot/🦀️.rs");
const minSide = Number(unique(snapshotSource, /ELEVATION_MIN_SIDE: usize = (\d+);/u)[1]);
const maxSide = Number(unique(snapshotSource, /ELEVATION_MAX_SIDE: usize = (\d+);/u)[1]);
const editSource = read("✏️editor/🎮️commands/📐️edit-elevation/🦀️.rs");
const [, fillId, fillEn, fillDe] = unique(editSource, /\(Operation::FillMissing, "([^"]+)", "([^"]+)", "([^"]+)"\)/u);
const [, strokeEn, strokeDe] = unique(editSource, /"stroke", LocalizedLabel::native\("([^"]+)", "([^"]+)"\)/u);
const editLimits = read("🧬️schema/💡️inferences/📐️elevation-edit/🦀️.rs");
const strokeMaximum = Number(unique(editLimits, /MAX_STROKE_SAMPLES: usize = (\d+);/u)[1]);
const asciiSource = read("✏️editor/🎮️commands/📥️import-elevation/🦀️.rs");
const asciiNotices = [...asciiSource.matchAll(/Self::\w+ => \("([^"]+)", "([^"]+)", "([^"]+)"\)/gu)]
  .map(([, code, en, de]) => ({ code, label: labels(en, de) }));
assert.equal(asciiNotices.length, 7);

function update(descriptor: any, uint: (value: number) => unknown) {
  const editor = named(descriptor.manifest.apps, "s.gis.gisterrain@1/*#editor");
  const brush = named(editor.actions, "configureTerrainBrush");
  brush.iconId = brushIcon;
  const elevationStep = named(brush.args, "elevationStep");
  elevationStep.schema = number(0.001, maximumStep, 0.001, false);
  elevationStep.default = defaultStep;

  const create = named(editor.actions, "createElevationGrid");
  const axes = [...gridSource.matchAll(/dimension\("([^"]+)", "([^"]+)", "([^"]+)"\),/gu)];
  assert.equal(axes.length, 2);
  create.args.splice(4, 0, ...axes.map(([, id, en, de]) => ({
    id, label: labels(en, de), schema: number(minSide, maxSide, 1, true), required: false, default: uint(defaultGrid),
  })));
  const height = named(create.args, "height");
  height.schema = number(signedMin, signedMax, 0.001, false);
  height.default = 100;
  const grade = named(named(editor.actions, "gradeTerrainRegion").args, "height");
  grade.schema = number(signedMin, signedMax, 0.001, false);
  grade.default = 100;

  const edit = named(editor.actions, "editElevation");
  named(edit.args, "operation").schema.options.push({ value: fillId, label: labels(fillEn, fillDe) });
  for (const id of ["column", "row", "radius"]) named(edit.args, id).schema.max = maxSide - 1;
  const value = named(edit.args, "value");
  value.schema = number(signedMin, signedMax, 0.001, false);
  value.default = 25;
  edit.args.push({
    id: "stroke",
    label: labels(strokeEn, strokeDe),
    schema: {
      kind: "array",
      items: { kind: "array", items: number(0, maxSide - 1, undefined, true), minItems: uint(2), maxItems: uint(2) },
      minItems: uint(1),
      maxItems: uint(strokeMaximum),
    },
    presentation: { kind: "hidden" },
    required: false,
  });

  editor.utilities.push({
    id: utilityId, label: labels(moveEn, moveDe), iconId: moveIcon, cursor: "crosshair", allowsActionsWhileActive: true,
  });
  named(editor.windowKinds, "gis3d-main").utilities.push(utilityId);
  named(editor.actions, "importElevationGrid").semantics.description = labels(importEn, importDe);
  const prefix = "gis.terrain.import.";
  const end = editor.faultNotices.findIndex((notice: any) => notice.code === "gis.terrain.export.cell-shape");
  assert(end > 0 && editor.faultNotices[0].code.startsWith(prefix));
  editor.faultNotices.splice(0, end + 1, ...asciiNotices);
  const brushFault = editor.faultNotices.find((notice: any) => notice.code === "gis.terrain.brush.settings-invalid");
  brushFault.label = labels(brushEn, brushDe);

  const example = descriptor.manifest.examples.find((candidate: any) => candidate.id === "demo" && candidate.dialect.artifactKind === "s.gis.gisterrain");
  const oldGrid = /(\belevation-source=bounds=)(\S+) rows=\[ (.*) \]\n$/u.exec(example.artifactJson);
  assert(oldGrid);
  assert.deepEqual(oldGrid[2].split(",").map(Number), grid.bounds);
  const oldSamples = [...oldGrid[3].matchAll(/\[ ([^[]+) \]/gu)].map(([, row]) => row.split(" ").map(Number));
  assert.deepEqual(oldSamples.map(row => row.map(value => value * 1000)), grid.rowsMillimetres);
  const sourceRows = grid.rowsMillimetres.map((row: number[]) => "[ " + row.join(" ") + " ]").join(" ");
  example.artifactJson = example.artifactJson.slice(0, oldGrid.index) + oldGrid[1] + oldGrid[2] + " rows-millimetres=[ " + sourceRows + " ]\n";
}

function pretty(value: any, exact: any, depth = 0): string {
  const inset = "  ".repeat(depth + 1);
  const close = "  ".repeat(depth);
  if (typeof value === "number") {
    if (isPackInteger(exact)) return String(value);
    assert.equal(typeof exact, "number");
    if (Object.is(exact, -0)) return "-0.0";
    const numeric = String(value);
    return Number.isInteger(value) && !numeric.includes("e") ? numeric + ".0" : numeric;
  }
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  const wrap = (parts: string[], start: string, stop: string) => parts.length
    ? start + "\n" + inset + parts.join(",\n" + inset) + "\n" + close + stop : start + stop;
  if (Array.isArray(value)) return wrap(value.map((item, index) => pretty(item, exact[index], depth + 1)), "[", "]");
  return wrap(Object.entries(value).map(([key, item]) => JSON.stringify(key) + ": " + pretty(item, exact[key], depth + 1)), "{", "}");
}

assert.equal(pretty(json, pack) + "\n", jsonText, "existing JSON must match the descriptor writer's formatting");
update(json, value => value);
update(pack, value => packUInt(BigInt(value)));
const replacementJson = pretty(json, pack) + "\n";
const replacementPack = encodePackValue(pack);
const pair = verifySemanticDescriptorPairBytesV1(Buffer.from(replacementJson), replacementPack);
assert.equal(pair.pluginId, "gis");
if (process.argv.includes("--write")) {
  writeFileSync(jsonPath, replacementJson);
  writeFileSync(packPath, replacementPack);
}
console.log(JSON.stringify({ plugin: pair.pluginId, sha256: pair.semanticSha256, written: process.argv.includes("--write") }));
