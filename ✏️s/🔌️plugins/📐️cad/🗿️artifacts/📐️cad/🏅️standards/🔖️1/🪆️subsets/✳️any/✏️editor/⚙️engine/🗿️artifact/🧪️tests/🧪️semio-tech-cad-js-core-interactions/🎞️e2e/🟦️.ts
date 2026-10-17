/** 🎞️ End-to-end interaction fixtures over every shipped interaction. */
import { ObjectRef, TypologyRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { ModelSpace, ModelSpaceJson } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import { isCallableOnlyInteraction } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import { parseModelJson } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { listModelObjectsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { listSelectionOperationsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/selection-operations";
import { collectGeometrySelectionTargets, selectionTargetsFromContext } from "../../../../🎬️actions/🧲️selection-targets/🟦️.ts";
import { preciseSpatialKernelMath } from "@semio-tech/s-spatial-kernel-ts/preview";
import type { ArtifactTestDependencies } from "../../../🟦️.ts";
import type { InteractionSnapshot } from "../../../📨️interaction-model/🟦️.ts";
import type { ModelEntityKind } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { InteractionSpec } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import type { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import type { InteractionEvent, SelectionTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import type { SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { SelectionOperationInteractionDef } from "../../../../🎬️actions/🧲️selection-targets/🟦️.ts";
import type { Vec3 } from "@semio-tech/framework/3d";

export async function registerInteractionE2eTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ArtifactTestDependencies): Promise<void> {
  const { CAD_E2E_BUILDING_BOOLEAN_MODEL_SPACE_JSON, CAD_E2E_LOOM_MODEL_SPACE_JSON, CAD_E2E_ROUTES_MODEL_SPACE_JSON, EMPTY_MODEL_DIFF, Model, __artifactTestKernel, applyModelDiff, buildAreaInteractionSpec, buildBoxInteractionSpec, buildDistanceInteractionSpec, createInteractionRuntime, defaultModelDefinitionId, isEmptyModelDiff, isFinalInteractionState, listSpatialInteractionsForModelDefinition, loadSpatialInteraction, runSelectionOperationInteraction, selectionSeedTargetsForOperation, solidRef } = dependencies;
  const { BrepjsKernel } = __artifactTestKernel!;
  const geometryLoomFixtureJson = JSON.parse(CAD_E2E_LOOM_MODEL_SPACE_JSON) as ModelSpaceJson;
  const geometryRoutesFixtureJson = JSON.parse(CAD_E2E_ROUTES_MODEL_SPACE_JSON) as ModelSpaceJson;
  const buildingBooleanFixtureJson = JSON.parse(CAD_E2E_BUILDING_BOOLEAN_MODEL_SPACE_JSON) as ModelSpaceJson;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = vitest;

  describe("@semio-tech/cad-js/core interaction e2e fixtures", () => {
    type InteractionE2EFixtureKind = "loom" | "routes" | "building" | "empty";

    const MOD: InteractionEvent["modifiers"] = {};

    const p = (x: number, y: number, z = 0): Vec3 => [x, y, z];

    const sel = (kind: ModelEntityKind, id: string, editable = true): SelectionTarget => ({
      kind,
      id,
      editable,
    });

    const modelFromFixture = (kind: InteractionE2EFixtureKind): Model => {
      if (kind === "empty") return new Model();
      const raw = kind === "loom" ? geometryLoomFixtureJson : kind === "routes" ? geometryRoutesFixtureJson : buildingBooleanFixtureJson;
      if (raw && typeof raw === "object" && (raw as ModelSpaceJson).schema === "spatial.modelspace") {
        const space = ModelSpace.fromJSON(raw as ModelSpaceJson);
        return space.models["spatial.shape"] ?? space.models[Object.keys(space.models)[0]!] ?? new Model();
      }
      return parseModelJson(raw) ?? new Model();
    };

    const seedBoxCell = (model: Model, tag = "e2e-box"): SelectionTarget => {
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 1 }, solidRef(tag)));
      return sel("solid", tag);
    };

    const entityCounts = (model: Model) => ({
      vertices: Object.keys(model.vertices).length,
      edges: Object.keys(model.edges).length,
      wires: Object.keys(model.wires).length,
      faces: Object.keys(model.faces).length,
      solids: Object.keys(model.solids).length,
      anchors: Object.keys(model.anchors).length,
    });

    const TRANSFORM_IDS = new Set(["transform.move", "transform.copy", "transform.rotate", "transform.mirror", "transform.scale1d", "transform.scale3d"]);

    const BOX_FACE_TOP = "box-e2e-box-face-top";

    const archivedSelectionTargets = (snap: InteractionSnapshot): readonly SelectionTarget[] => selectionTargetsFromContext(snap.lastResponse?.archiveContext ?? {});

    const assertSelectionCommandArchive = (defn: SelectionOperationInteractionDef, targets: readonly SelectionTarget[], model: Model, activeModelDefinitionId?: string | null): void => {
      switch (defn.operation) {
        case "deselectAll":
          expect(targets).toEqual([]);
          return;
        case "selectAll":
          expect(targets.length).toBeGreaterThanOrEqual(8);
          expect(targets.some((t) => t.kind === "solid" && t.id === "e2e-box")).toBe(true);
          expect(targets.some((t) => t.kind === "vertex")).toBe(true);
          expect(targets.some((t) => t.kind === "face")).toBe(true);
          return;
        case "invert":
          expect(targets.some((t) => t.kind === "solid" && t.id === "e2e-box")).toBe(false);
          expect(targets.length).toBeGreaterThan(0);
          return;
        case "selectKinds": {
          const kinds = defn.kinds ?? [];
          expect(targets.every((t) => kinds.includes(t.kind))).toBe(true);
          const expected = collectGeometrySelectionTargets(model, kinds, activeModelDefinitionId ?? null);
          expect(targets).toEqual(expected);
          if (defn.id === "selection.selectAnchors") {
            expect(targets).toEqual([]);
          } else {
            expect(targets.length).toBeGreaterThan(0);
          }
          return;
        }
      }
    };

    const e2eCases: readonly {
      readonly id: string;
      readonly fixture: InteractionE2EFixtureKind;
      readonly steps: readonly InteractionEvent[];
      readonly seedBox?: boolean;
      readonly useModelObjects?: boolean;
      readonly spec?: InteractionSpec;
      readonly assert?: (ctx: { readonly snap: InteractionSnapshot; readonly model: Model; readonly before: ReturnType<typeof entityCounts>; readonly after: ReturnType<typeof entityCounts>; readonly activeModelDefinitionId?: string | null }) => void;
    }[] = [
      {
        id: "entity.createAnchor",
        fixture: "empty",
        steps: [
          {
            kind: "selection.changed",
            targets: [sel("edge", "box-e2e-box-eb0")],
            point: p(2.5, 0),
            modifiers: MOD,
          },
        ],
        seedBox: true,
        assert: ({ after }) => expect(after.anchors).toBe(1),
      },
      {
        id: "primitive.box",
        fixture: "empty",
        spec: buildBoxInteractionSpec(),
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(3, 2), modifiers: MOD },
          { kind: "set.height", value: 1.25, modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.solids).toBeGreaterThanOrEqual(1),
      },
      {
        id: "feature.extrudeWire",
        fixture: "loom",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "w-deck")], modifiers: MOD },
          { kind: "set.distance", value: 1.2, modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
      },
      {
        id: "feature.offsetSurface",
        fixture: "empty",
        steps: [
          { kind: "selection.changed", targets: [sel("face", BOX_FACE_TOP)], modifiers: MOD },
          { kind: "set.distance", value: 0.15, modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        seedBox: true,
      },
      {
        id: "measure.distance",
        fixture: "loom",
        spec: buildDistanceInteractionSpec(),
        steps: [
          { kind: "selection.changed", targets: [sel("vertex", "v0")], modifiers: MOD },
          { kind: "selection.changed", targets: [sel("vertex", "v1")], modifiers: MOD },
        ],
        assert: ({ snap, after, before }) => {
          expect(typeof snap.lastResponse?.data).toBe("number");
          expect(isEmptyModelDiff(snap.lastResponse?.diff)).toBe(false);
          expect(after.edges).toBeGreaterThan(before.edges);
        },
      },
      {
        id: "measure.area",
        fixture: "empty",
        spec: buildAreaInteractionSpec(),
        steps: [
          { kind: "selection.changed", targets: [sel("face", BOX_FACE_TOP)], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        seedBox: true,
        assert: ({ snap, after, before }) => {
          expect(typeof snap.lastResponse?.data).toBe("number");
          expect(isEmptyModelDiff(snap.lastResponse?.diff)).toBe(false);
          expect(after.anchors).toBeGreaterThan(before.anchors);
        },
      },
      {
        id: "curve.line",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(5, 1), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.vertices).toBeGreaterThanOrEqual(2),
      },
      {
        id: "curve.polyline",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(4, 2), modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.vertices).toBeGreaterThanOrEqual(3),
      },
      {
        id: "curve.arc",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(0, 2), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.edges).toBeGreaterThanOrEqual(1),
      },
      {
        id: "curve.circle",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 0), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.edges).toBeGreaterThanOrEqual(1),
      },
      {
        id: "curve.controlPointCurve",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 2), modifiers: MOD },
          { kind: "pointer.down", point: p(4, 0), modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.edges).toBeGreaterThanOrEqual(1),
      },
      {
        id: "curve.interpolateCurve",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 1), modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.edges).toBeGreaterThanOrEqual(1),
      },
      {
        id: "transform.move",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 0.5), modifiers: MOD },
        ],
        assert: ({ model }) => {
          const moved = Object.values(model.vertices).some((v) => v.position[0] > 0.5);
          expect(moved).toBe(true);
        },
      },
      {
        id: "transform.copy",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(4, 0), modifiers: MOD },
        ],
        assert: ({ after, before }) => expect(after.vertices).toBeGreaterThan(before.vertices),
      },
      {
        id: "transform.rotate",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "pointer.down", point: p(1, 1), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 1), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 2), modifiers: MOD },
        ],
      },
      {
        id: "transform.mirror",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(3, 0), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.vertices).toBeGreaterThanOrEqual(1),
      },
      {
        id: "transform.scale1d",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 0), modifiers: MOD },
        ],
      },
      {
        id: "transform.scale3d",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "pointer.down", point: p(1, 1), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 1), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 2), modifiers: MOD },
        ],
      },
      {
        id: "solid.sphere",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(1.5, 0), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.solids).toBeGreaterThanOrEqual(1),
      },
      {
        id: "solid.cylinder",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(0, 0, 2), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.solids).toBeGreaterThanOrEqual(1),
      },
      {
        id: "surface.plane",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(4, 0), modifiers: MOD },
        ],
      },
      {
        id: "edit.join",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire"), sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
      },
      {
        id: "edit.explode",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
      },
      {
        id: "edit.chamfer",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("edge", "re0")], modifiers: MOD },
          { kind: "selection.changed", targets: [sel("edge", "re1")], modifiers: MOD },
        ],
      },
      {
        id: "edit.fillet",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("edge", "re0")], modifiers: MOD },
          { kind: "selection.changed", targets: [sel("edge", "re2")], modifiers: MOD },
        ],
      },
      {
        id: "edit.split",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "selection.changed", targets: [sel("edge", "re10")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
      },
      {
        id: "edit.trim",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "selection.changed", targets: [sel("edge", "re0")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
      },
      {
        id: "surface.loft",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire")], modifiers: MOD },
          { kind: "selection.changed", targets: [sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "dialog.ok", modifiers: MOD },
        ],
      },
      {
        id: "surface.sweep1",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "selection.changed", targets: [sel("wire", "spine-b")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "dialog.ok", modifiers: MOD },
        ],
      },
      {
        id: "surface.sweep2",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "selection.changed", targets: [sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "selection.changed", targets: [sel("wire", "spine-b")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "dialog.ok", modifiers: MOD },
        ],
      },
      {
        id: "surface.networkSrf",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire"), sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "dialog.ok", modifiers: MOD },
        ],
      },
      {
        id: "surface.extrudeCrv",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "set.distance", value: 0.8, modifiers: MOD },
        ],
        assert: ({ after, model }) => {
          expect(after.solids).toBeGreaterThanOrEqual(1);
          expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId()).length).toBeGreaterThanOrEqual(1);
        },
      },
      {
        id: "solid.booleanUnion",
        fixture: "building",
        steps: (() => {
          const c0 = "small-building-cell-123052045";
          const c1 = "small-building-cell-1278694563";
          return [
            { kind: "selection.changed", targets: [sel("solid", c0), sel("solid", c1)], modifiers: MOD },
            { kind: "confirm", modifiers: MOD },
          ];
        })(),
      },
      {
        id: "solid.booleanDifference",
        fixture: "building",
        steps: (() => {
          const c0 = "small-building-cell-123052045";
          const c1 = "small-building-cell-1278694563";
          return [
            { kind: "selection.changed", targets: [sel("solid", c0)], modifiers: MOD },
            { kind: "confirm", modifiers: MOD },
            { kind: "selection.changed", targets: [sel("solid", c1)], modifiers: MOD },
            { kind: "confirm", modifiers: MOD },
          ];
        })(),
      },
      {
        id: "solid.booleanIntersection",
        fixture: "building",
        steps: (() => {
          const c0 = "small-building-cell-123052045";
          const c1 = "small-building-cell-1278694563";
          return [
            { kind: "selection.changed", targets: [sel("solid", c0)], modifiers: MOD },
            { kind: "confirm", modifiers: MOD },
            { kind: "selection.changed", targets: [sel("solid", c1)], modifiers: MOD },
            { kind: "confirm", modifiers: MOD },
          ];
        })(),
      },
    ];

    it("covers every shipped interaction", () => {
      const ids = listSpatialInteractionsForModelDefinition(defaultModelDefinitionId())
        .map((row) => row.id)
        .filter((id) => {
          const spec = loadSpatialInteraction(id);
          return spec && !isCallableOnlyInteraction(spec);
        })
        .sort();
      expect(e2eCases.map((c) => c.id).sort()).toEqual(ids);
    });

    it.each(listSelectionOperationsForModelDefinition(defaultModelDefinitionId()))("$id selection action completes on seeded box", async (defn) => {
      const model = modelFromFixture("empty");
      seedBoxCell(model);
      if (defn.kinds?.includes("object")) {
        const solidId = Object.keys(model.solids)[0]!;
        model.objects["e2e-object"] = {
          id: "e2e-object" as ObjectRef,
          typology: "spatial.shape.primitive.box" as TypologyRef,
          primitives: { solid: solidId },
        };
      }
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const seed = selectionSeedTargetsForOperation(defn.operation);
      const result = await runSelectionOperationInteraction(defn.id, {
        kernel,
        document: { model, nodes: [] },
        seedTargets: seed,
      });
      expect(isEmptyModelDiff(result.response.diff ?? EMPTY_MODEL_DIFF)).toBe(true);
      assertSelectionCommandArchive(defn, result.targets, model);
    });

    it.each(e2eCases)("$id completes end-to-end on $fixture fixture", async (row) => {
      const spec = row.spec ?? loadSpatialInteraction(row.id);
      expect(spec).not.toBeNull();
      const model = modelFromFixture(row.fixture);
      if (row.seedBox || TRANSFORM_IDS.has(row.id)) seedBoxCell(model);
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const before = entityCounts(model);
      const rt = createInteractionRuntime(spec!, {
        kernel,
        document: { model: model, nodes: [] },
      });
      for (const step of row.steps) {
        await rt.send(step);
        if (isFinalInteractionState(spec!, rt.getSnapshot().state)) break;
      }
      const st = rt.getSnapshot().state;
      if (st === "ready") await rt.send({ kind: "confirm", modifiers: MOD });
      const snap = rt.getSnapshot();
      expect(snap.state, row.id).toBe("committed");
      expect(snap.lastResponse?.ok, row.id).toBe(true);
      expect(snap.lastResponse?.errors ?? [], row.id).toEqual([]);
      row.assert?.({ snap, model, before, after: entityCounts(model) });
    });
  });
}
