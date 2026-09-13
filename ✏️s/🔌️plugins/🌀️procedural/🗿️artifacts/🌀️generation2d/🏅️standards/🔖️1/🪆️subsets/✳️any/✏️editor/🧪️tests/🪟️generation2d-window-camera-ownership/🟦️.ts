import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020";
import { applyPatch } from "fast-json-patch";
import { applyGeneration2dMainWindowConfigMutation, type Generation2dMainWindowConfig } from "../../🎭️modes/✏️edit/🪟️windows/🕸️flow/🎚️config/🧬️schema/🟦️.ts";
import { applyGeneration2dEditPreviewWindowConfigMutation } from "../../🎭️modes/✏️edit/🪟️windows/👁️preview/🎚️config/🧬️schema/🟦️.ts";
import { applyGeneration2dGeneratePreviewWindowConfigMutation } from "../../🎭️modes/🧬️generate/🪟️windows/👁️preview/🎚️config/🧬️schema/🟦️.ts";

type Kind = "generation2d-main" | "generation2d-preview" | "generation2d-generate-preview";
type Config = Generation2dMainWindowConfig;
type Mutation = { kind: "snapshot"; config: Config };
type Fixture = {
  document: Record<string, unknown>;
  appConfig: Record<string, unknown>;
  windowInstances: { id: string; windowKindId: string }[];
  baseConfigs: Record<Kind, Config>;
  renderers: Record<Kind, { bodyKey: string; sceneSchema: "node-graph@1" | "canvas-2d@1" }>;
  mutations: { windowId: string; windowKindId: Kind; mutation: Mutation }[];
  expected: Record<string, Config>;
  rejections: { windowId: string | null; claimedWindowKindId: Kind; code: string }[];
  canvasCommands: string[];
};

const workspace = process.cwd();
const editor = join(workspace, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor");
const readJson = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const fixture = readJson(join(editor, "🧪️tests/🪟️generation2d-window-camera-ownership/🧫️fixtures/🔣️.json")) as Fixture;
const fixtureSchema = readJson(join(editor, "🧪️tests/🪟️generation2d-window-camera-ownership/🧬️schema/🔣️.json"));
const viewportSchema = readJson(join(workspace, "🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🔣️.json"));
const ownerSchemas = new Map<Kind, any>([
  ["generation2d-main", readJson(join(editor, "🎭️modes/✏️edit/🪟️windows/🕸️flow/🎚️config/🧬️schema/🔣️.json"))],
  ["generation2d-preview", readJson(join(editor, "🎭️modes/✏️edit/🪟️windows/👁️preview/🎚️config/🧬️schema/🔣️.json"))],
  ["generation2d-generate-preview", readJson(join(editor, "🎭️modes/🧬️generate/🪟️windows/👁️preview/🎚️config/🧬️schema/🔣️.json"))],
]);

function exactTarget(windowId: string | null, expectedKind: Kind): string {
  if (windowId === null) throw new Error("generation2d-main-window-required");
  const instance = fixture.windowInstances.find((candidate) => candidate.id === windowId);
  if (!instance) throw new Error("generation2d-main-window-stale");
  if (instance.windowKindId !== expectedKind) throw new Error("generation2d-main-window-kind-required");
  return instance.id;
}

function applyOwned(base: Config, kind: Kind, mutation: Mutation): Config {
  if (kind === "generation2d-main") return applyGeneration2dMainWindowConfigMutation(base, mutation);
  if (kind === "generation2d-preview") return applyGeneration2dEditPreviewWindowConfigMutation(base, mutation);
  return applyGeneration2dGeneratePreviewWindowConfigMutation(base, mutation);
}

export function testGeneration2dWindowCameraOwnershipOracle(): void {
  const ajv = new Ajv2020({ strict: true, allErrors: true });
  assert(ajv.compile(fixtureSchema)(fixture));
  ajv.addSchema(viewportSchema);
  const schemaIds = new Set<string>();
  for (const [kind, schema] of ownerSchemas) {
    schemaIds.add(schema.$id);
    const validate = ajv.compile(schema);
    assert(validate(fixture.baseConfigs[kind]), JSON.stringify(validate.errors));
    assert(!validate({ ...fixture.baseConfigs[kind], camera: { x: 0, y: 0, zoom: 1 } }));
  }
  assert.equal(schemaIds.size, 3, "each window kind has a distinct schema identity");
  assert.deepEqual(
    Object.fromEntries(Object.entries(fixture.renderers).map(([kind, renderer]) => [kind, renderer.sceneSchema])),
    {
      "generation2d-main": "node-graph@1",
      "generation2d-preview": "canvas-2d@1",
      "generation2d-generate-preview": "canvas-2d@1",
    },
  );
  assert(!Object.hasOwn(fixture.appConfig, "camera"), "the app config has no camera owner");

  const documentBytes = JSON.stringify(fixture.document);
  const appBytes = JSON.stringify(fixture.appConfig);
  const configs: Record<string, Config> = {};
  const patchOracle: Record<string, Config> = {};
  for (const entry of fixture.mutations) {
    const id = exactTarget(entry.windowId, entry.windowKindId);
    const base = configs[id] ?? fixture.baseConfigs[entry.windowKindId];
    configs[id] = applyOwned(base, entry.windowKindId, entry.mutation);
    patchOracle[id] = applyPatch(
      structuredClone(patchOracle[id] ?? fixture.baseConfigs[entry.windowKindId]),
      [{ op: "replace", path: "", value: entry.mutation.config }],
      false,
      false,
    ).newDocument;
  }
  assert.deepEqual(configs, fixture.expected);
  assert.deepEqual(configs, patchOracle);
  assert.notDeepEqual(configs["main-left"], configs["main-right"]);
  assert.notDeepEqual(configs["edit-left"], configs["edit-right"]);
  assert.notDeepEqual(configs["generate-left"], configs["generate-right"]);
  assert.notDeepEqual(configs["main-left"], configs["edit-left"]);
  assert.notDeepEqual(configs["edit-left"], configs["generate-left"]);

  const reopened = JSON.parse(JSON.stringify(configs));
  assert.deepEqual(reopened, fixture.expected);
  for (const command of fixture.canvasCommands) {
    assert.match(command, /^canvas-(pointer-(down|move|up)|wheel)$/);
    assert.equal(JSON.stringify(configs), JSON.stringify(reopened), `${command} is an explicit no-op`);
  }
  assert.equal(JSON.stringify(fixture.document), documentBytes);
  assert.equal(JSON.stringify(fixture.appConfig), appBytes);
  for (const rejected of fixture.rejections) {
    assert.throws(() => exactTarget(rejected.windowId, rejected.claimedWindowKindId), new RegExp(rejected.code));
  }
  console.log(`[DEBUG] generation2d-window-camera-ownership owners=${schemaIds.size} instances=${Object.keys(configs).length} canvasNoOps=${fixture.canvasCommands.length} appCamera=absent`);
}
