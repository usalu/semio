/** 🗂️ Action and interaction registries: declarative specs, typology construction, nested interaction calls, selection actions. */
import { ObjectRef, VertexRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { listModelDefinitionTypologies, loadTypology } from "@semio-tech/s-spatial-kernel-ts/geometry/typology";
import { isCallableOnlyInteraction } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import { isShapeModelDefinition, listModelDefinitionManifests } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { actionOwnedByModelDefinition, listModelObjectsForModelDefinition, listTypologiesForModelDefinition, modelDefinitionIdForInteraction } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { listSelectionOperationsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/selection-operations";
import { listConstructableTypologiesForModelDefinition, typologyConstructAssetIds, typologyConstructModeActionIds, typologyHasNativeConstructKit } from "../../../../🧬️typology/🟦️.ts";
import { listModelDefinitionActionSpecs, parseActionSpec } from "../../../../🎬️actions/🧬️action-contract/🟦️.ts";
import { listActionDefs, registerActionDef, runSelectionApply } from "../../../../🎬️actions/🏃️action-execution/🟦️.ts";
import { executeSelectionApply, selectionTargetsPointTransformDiff } from "../../../../🎬️actions/🧲️selection-targets/🟦️.ts";
import { interactionNumericEntryCommitEvent, interactionNumericEntryExplicitLockValue, interactionNumericEntryLockedValue, interactionStepFinalizeEvent } from "../../../../🎬️actions/🎰️interaction-state/🟦️.ts";
import { preciseSpatialKernelMath } from "@semio-tech/s-spatial-kernel-ts/preview";
import type { ArtifactTestDependencies } from "../../../🎒️test-dependencies/🟦️.ts";
import type { InteractionEvent, SelectionTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import type { SpatialKernel, SpatialPreviewKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { Vec3 } from "@semio-tech/framework/3d";

export async function registerRegistryTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ArtifactTestDependencies): Promise<void> {
  const { DocumentHistory, EMPTY_MODEL_DIFF, Model, __artifactTestKernel, applyModelDiff, buildBoxInteractionSpec, compileInteraction, createInteractionRuntime, defaultModelDefinitionId, emptyMeshTransfer, interactionRecordsDocumentHistory, isEmptyModelDiff, listInteractionSpecs, loadSpatialInteraction, mergeInteractionCallOutputs, modelDefinitionActionRegistry, modelDefinitionInteractionRegistry, parseInteractionSpec, registerInteractionSpec, runRegisteredAction, runSelectionOperationInteraction, selectionApplyParamsForInteraction, selectionSeedTargetsForOperation, selectionTargetsFromActionResult, solidRef, typologyConstructCommitActionForMode, typologyConstructKitByInteraction, typologyIdForInteractionCommit } = dependencies;
  const { BrepjsKernel } = __artifactTestKernel!;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = vitest;

  describe("@semio-tech/cad-js/core action and interaction registries", () => {
    it("rejects executable action document fields", () => {
      const base = {
        schema: "spatial.action",
        id: "x",
        version: "1.0.0",
        steps: [{ operation: "return", data: { kind: "const", value: 1 } }],
      };
      expect(parseActionSpec({ ...base, run: "x" })).toBeNull();
      expect(parseActionSpec({ ...base, code: "x" })).toBeNull();
      expect(parseActionSpec({ ...base, function: "x" })).toBeNull();
      expect(parseActionSpec({ ...base, steps: [{ operation: "eval", code: "x" }] })).toBeNull();
    });
    it("loads model-definition actions from data-only JSON specs", () => {
      const specs = listModelDefinitionActionSpecs();
      const registry = modelDefinitionActionRegistry();
      expect(specs.length).toBeGreaterThan(0);
      expect(specs.every((s) => registry.get(s.id)?.spec?.schema === "spatial.action")).toBe(true);
      expect(specs.every((s) => registry.get(s.id) !== null)).toBe(true);
      expect(registry.get("command.finish")?.spec?.schema).toBe("spatial.action");
      expect(registry.get("selection.selectAll")?.spec?.steps.some((s) => s.operation === "kernel.call" && s.function === "spatial.selection.apply")).toBe(true);
      expect(registry.get("command.addPoint")?.spec?.steps.some((s) => s.operation === "kernel.call" && s.function === "spatial.action.capability")).toBe(true);
      const allowedKernelFunctions = new Set(["spatial.selection.apply", "spatial.action.capability"]);
      expect(specs.every((spec) => spec.steps.every((step) => step.operation !== "kernel.call" || allowedKernelFunctions.has(step.function)))).toBe(true);
    });
    it("typology actions reference shipped declarative action specs", () => {
      const actionIds = new Set(listModelDefinitionActionSpecs().map((row) => row.id));
      for (const typology of listModelDefinitionTypologies()) {
        for (const actionId of typology.actions) {
          expect(actionIds.has(actionId), `${typology.id} → ${actionId}`).toBe(true);
        }
      }
    });
    it("every typology ships construct kit or legacy create interactions", () => {
      const actionIds = new Set(listModelDefinitionActionSpecs().map((row) => row.id));
      const interactionIds = new Set(
        listInteractionSpecs(modelDefinitionInteractionRegistry())
          .map((row) => row.id),
      );
      for (const typology of listModelDefinitionTypologies()) {
        if (typology.id.includes(".kernel.")) continue;
        const ids = typologyConstructAssetIds(typology.id, typology.label);
        if (!typology.interactions.includes(ids.construct)) {
          expect(typology.actions.length).toBeGreaterThan(0);
          expect(typology.interactions.length).toBeGreaterThan(0);
          continue;
        }
        expect(typologyHasNativeConstructKit(typology)).toBe(true);
        expect(typology.interactions).toEqual([ids.interaction]);
        expect(typology.actions).not.toContain(ids.interaction);
        for (const actionId of typologyConstructModeActionIds(typology.id, typology.label)) {
          expect(typology.actions).toContain(actionId);
          expect(actionIds.has(actionId)).toBe(true);
        }
        expect(interactionIds.has(ids.interaction)).toBe(true);
        const commitAction = loadSpatialInteraction(ids.interaction)?.commit.operation.action;
        expect(typologyConstructModeActionIds(typology.id, typology.label)).toContain(commitAction);
        expect(commitAction).not.toBe(ids.interaction);
      }
    });
    it("every model-definition typology is constructable with native assets in its folder", () => {
      for (const manifest of listModelDefinitionManifests()) {
        const mdId = manifest.id;
        if (isShapeModelDefinition(mdId)) continue;
        for (const typology of listTypologiesForModelDefinition(mdId)) {
          if (!typologyHasNativeConstructKit(typology)) continue;
          const ids = typologyConstructAssetIds(typology.id, typology.label);
          expect(modelDefinitionIdForInteraction(ids.interaction), `${mdId} → ${ids.interaction}`).toBe(mdId);
          for (const actionId of typologyConstructModeActionIds(typology.id, typology.label)) {
            expect(actionOwnedByModelDefinition(actionId, mdId), `${mdId} → ${typology.id} → ${actionId}`).toBe(true);
          }
          for (const actionId of typology.actions) {
            expect(actionOwnedByModelDefinition(actionId, mdId), `${mdId} → ${typology.id} → ${actionId}`).toBe(true);
          }
        }
        expect(listConstructableTypologiesForModelDefinition(mdId).length).toBe(listTypologiesForModelDefinition(mdId).filter(typologyHasNativeConstructKit).length);
      }
    });
    it("typology constructFrom2PointsAndHeight adds an object row for the typology", async () => {
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const typology = "energy.energy.hull";
      const ids = typologyConstructAssetIds(typology, "Hull");
      await runRegisteredAction(modelDefinitionActionRegistry(),
        ids.constructFrom2PointsAndHeight,
        { typology, constructMode: "2PointsAndHeight", pointA: [0, 0, 0], pointB: [3, 2, 0], height: 2.5 },
        { kernel, preview: kernel as unknown as SpatialPreviewKernel, model, activeModelDefinitionId: "aec.building.energy" },
      );
      expect(model.objects[typology]?.typology).toBe(typology);
      expect(model.objects[typology]?.primitives.solid).toBeTruthy();
    });
    it("curve.interpolateCurve rolls back from committed when kernel returns empty commit diff", async () => {
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const model = new Model();
      const kernel = {
        executeCommandDiff: async () => ({ diff: {} }),
      } as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        previewKernel: kernel as unknown as SpatialPreviewKernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 1, 0], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      expect(rt.getSnapshot().state).toBe("next_point");
      expect(rt.getSnapshot().lastResponse?.ok).toBe(false);
      expect(rt.getSnapshot().lastResponse?.errors?.[0]?.code).toBe("interaction.emptyCommit");
    });

    it("interactionStepFinalizeEvent confirms interpolate curve with two points instead of pointer.down", () => {
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const ctx = { points: [[0, 0, 0] as Vec3, [2, 1, 0] as Vec3], cursor: [5, 5, 0] as Vec3 };
      expect(interactionStepFinalizeEvent(spec, "next_point", ctx, M)?.kind).toBe("confirm");
      expect(interactionNumericEntryCommitEvent(spec, "next_point", ctx, M)?.kind).toBe("pointer.down");
    });

    it("interactionNumericEntryExplicitLockValue ignores live rubber-band distance", () => {
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const ctx = { points: [[0, 0, 0] as Vec3], cursor: [4, 0, 0] as Vec3, __lengthLock: 2 };
      expect(interactionNumericEntryExplicitLockValue(spec, "next_point", ctx)).toBe(2);
      const liveOnly = { points: [[0, 0, 0] as Vec3], cursor: [4, 0, 0] as Vec3 };
      expect(interactionNumericEntryExplicitLockValue(spec, "next_point", liveOnly)).toBeNull();
      expect(interactionNumericEntryLockedValue(spec, "next_point", liveOnly)).toBeCloseTo(4, 5);
    });

    it("curve.interpolateCurve confirm with one point stays in next_point", async () => {
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      expect(rt.getSnapshot().state).toBe("next_point");
      expect(Object.keys(model.edges).length).toBe(0);
    });

    it("curve.interpolateCurve commit binds typology object rows for document", async () => {
      const typology = "spatial.shape.curve.interpolate-curve";
      expect(typologyIdForInteractionCommit("curve.interpolateCurve")).toBe(typology);
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 1, 0], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      const res = rt.getSnapshot().lastResponse!;
      expect(res.ok).toBe(true);
      expect(Object.keys(model.edges).length).toBeGreaterThan(0);
      expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId())).toHaveLength(1);
      expect(model.objects[typology as ObjectRef]?.typology).toBe(typology);
      expect(model.objects[typology as ObjectRef]?.primitives.curve).toBeTruthy();
    });

    it("curve.interpolateCurve commit uses live snapped vertex positions after vertex move", async () => {
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const model = new Model();
      const vertexId = "v-live" as VertexRef;
      model.vertices[vertexId] = { id: vertexId, position: [0, 0, 0] };
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({
        kind: "pointer.down",
        point: [0, 0, 0],
        modifiers: {},
        snap: { kind: "vertex", id: vertexId, point: [0, 0, 0] },
      } as InteractionEvent);
      await rt.send({ kind: "pointer.down", point: [2, 0, 0], modifiers: {} });
      model.vertices[vertexId] = { id: vertexId, position: [0, 4, 0] };
      await rt.send({ kind: "confirm", modifiers: {} });
      const edge = Object.values(model.edges)[0];
      expect(edge?.curve?.kind).toBe("nurbs");
      if (edge?.curve?.kind === "nurbs") {
        expect(edge.curve.poles[0]).toEqual([0, 4, 0]);
      }
    });

    it("applyModelDiff syncs nurbs through-curve poles when endpoint vertices move", async () => {
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const created = await kernel.executeCommandDiff("curve.interpolateCurve", {
        model,
        points: [
          [0, 0, 0],
          [2, 1, 0],
          [4, 0, 0],
        ],
      });
      applyModelDiff(model, created.diff);
      const edge = Object.values(model.edges)[0]!;
      const startId = edge.vertexIds[0]!;
      applyModelDiff(model, { vertices: { modified: [{ id: startId, position: [0, 3, 0] }] } });
      const updated = model.edges[edge.id]!;
      expect(updated.curve?.kind).toBe("nurbs");
      if (updated.curve?.kind === "nurbs") {
        expect(updated.curve.poles[0]).toEqual([0, 3, 0]);
      }
    });

    it("selectionTargetsPointTransformDiff moves all nurbs poles when an edge is selected", async () => {
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const created = await kernel.executeCommandDiff("curve.controlPointCurve", {
        model,
        points: [
          [0, 0, 0],
          [1, 2, 0],
          [3, 0, 0],
        ],
      });
      applyModelDiff(model, created.diff);
      const edge = Object.values(model.edges)[0]!;
      const before: readonly Vec3[] = edge.curve?.kind === "nurbs" ? edge.curve.poles.map((pole: Vec3) => [...pole] as Vec3) : [];
      applyModelDiff(
        model,
        selectionTargetsPointTransformDiff(model, [{ kind: "edge", id: edge.id, editable: true }], (point) => [point[0] + 1, point[1], point[2]]),
      );
      const updated = model.edges[edge.id]!;
      expect(updated.curve?.kind).toBe("nurbs");
      if (updated.curve?.kind === "nurbs") {
        expect(updated.curve.poles).toEqual(before.map((pole: Vec3) => [pole[0] + 1, pole[1], pole[2]]));
      }
    });

    it("selectionTargetsPointTransformDiff leaves interior poles when only a vertex is selected", async () => {
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const created = await kernel.executeCommandDiff("curve.controlPointCurve", {
        model,
        points: [
          [0, 0, 0],
          [1, 2, 0],
          [3, 0, 0],
        ],
      });
      applyModelDiff(model, created.diff);
      const edge = Object.values(model.edges)[0]!;
      const midPole = edge.curve?.kind === "nurbs" ? [...edge.curve.poles[1]!] : null;
      const startId = edge.vertexIds[0]!;
      applyModelDiff(
        model,
        selectionTargetsPointTransformDiff(model, [{ kind: "vertex", id: startId, editable: true }], (point) => [point[0], point[1] + 5, point[2]]),
      );
      const updated = model.edges[edge.id]!;
      expect(updated.curve?.kind).toBe("nurbs");
      if (updated.curve?.kind === "nurbs" && midPole) {
        expect(updated.curve.poles[1]).toEqual(midPole);
      }
    });

    it("primitive.box commit binds typology object rows for document", async () => {
      const typology = "spatial.shape.primitive.box";
      const spec = loadSpatialInteraction("primitive.box")!;
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 3, 0], modifiers: {} });
      await rt.send({ kind: "set.height", value: 4, modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      const res = rt.getSnapshot().lastResponse!;
      expect(res.ok).toBe(true);
      expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId())).toHaveLength(1);
      expect(model.objects[typology as ObjectRef]?.typology).toBe(typology);
      expect(model.objects[typology as ObjectRef]?.primitives.solid).toBeTruthy();
      const rt2 = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt2.send({ kind: "pointer.down", point: [5, 0, 0], modifiers: {} });
      await rt2.send({ kind: "pointer.down", point: [7, 2, 0], modifiers: {} });
      await rt2.send({ kind: "set.height", value: 2, modifiers: {} });
      await rt2.send({ kind: "confirm", modifiers: {} });
      expect(rt2.getSnapshot().lastResponse?.ok).toBe(true);
      expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId())).toHaveLength(2);
    });
    it("typologyConstructCommitActionForMode resolves exactly one mode construct action", () => {
      const ids = typologyConstructAssetIds("energy.energy.hull", "Hull");
      const kit = typologyConstructKitByInteraction().get(ids.interaction)!;
      expect(typologyConstructCommitActionForMode(kit, "2PointsAndHeight")).toBe(ids.constructFrom2PointsAndHeight);
      expect(typologyConstructCommitActionForMode(kit, "curveAndHeight")).toBe(ids.constructFromCurveAndHeight);
      expect(typologyConstructCommitActionForMode(kit, "surface")).toBe(ids.constructFromSurface);
    });
    it("base plate typology lists only constructFromSurface among mode actions", () => {
      const typology = loadTypology("energy.energy.baseplate")!;
      const ids = typologyConstructAssetIds(typology.id, typology.label);
      expect(typologyConstructModeActionIds(typology.id, typology.label)).toEqual([ids.constructFromSurface]);
      expect(typologyHasNativeConstructKit(typology)).toBe(true);
    });
    it("aborting nested interaction.call rolls back the calling transition", async () => {
      const pickChild = parseInteractionSpec({
        schema: "spatial.interaction",
        id: "test.nested.pick",
        version: "1",
        invocation: "callable",
        machine: {
          initial: "pick",
          states: [
            {
              name: "pick",
              selection: { accept: ["face"], multiple: false, prompt: "Pick surface" },
              on: [
                {
                  event: "selection.changed",
                  transitions: [
                    {
                      target: "committed",
                      effects: [
                        {
                          mutation: "assign",
                          target: { root: "context", segments: [{ kind: "field", name: "faceId" }] },
                          value: {
                            kind: "path",
                            root: "event",
                            segments: [
                              { kind: "field", name: "targets" },
                              { kind: "index", index: 0 },
                              { kind: "field", name: "id" },
                            ],
                          },
                        },
                      ],
                    },
                  ],
                },
              ],
            },
            { name: "committed", final: true },
          ],
        },
        commit: { fromStates: ["committed"], operation: { kind: "action", action: "command.finish", params: {} } },
      })!;
      const host = parseInteractionSpec({
        schema: "spatial.interaction",
        id: "test.nested.host",
        version: "1",
        machine: {
          initial: "choose_mode",
          states: [
            {
              name: "choose_mode",
              on: [
                {
                  event: "mode.surface",
                  transitions: [
                    {
                      target: "committed",
                      effects: [
                        { mutation: "assign", target: { root: "context", segments: [{ kind: "field", name: "constructMode" }] }, value: { kind: "const", value: "surface" } },
                        {
                          mutation: "interaction.call",
                          interaction: "test.nested.pick",
                          outputs: [
                            {
                              target: { root: "context", segments: [{ kind: "field", name: "faceId" }] },
                              value: { kind: "path", root: "context", segments: [{ kind: "field", name: "faceId" }] },
                            },
                          ],
                        },
                      ],
                    },
                  ],
                },
              ],
            },
            { name: "committed", final: true },
          ],
        },
        commit: { fromStates: ["committed"], operation: { kind: "action", action: "command.finish", params: {} } },
      })!;
      let interactions = modelDefinitionInteractionRegistry();
      interactions = registerInteractionSpec(interactions, compileInteraction(pickChild));
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(compileInteraction(host), {
        kernel,
        document: { model: new Model(), nodes: [] },
        interactions,
      });
      await rt.send({ kind: "mode.surface" });
      expect(rt.getSnapshot().interactionId).toBe("test.nested.pick");
      rt.cancel();
      const snap = rt.getSnapshot();
      expect(snap.interactionId).toBe("test.nested.host");
      expect(snap.state).toBe("choose_mode");
      expect(snap.context.constructMode).toBeUndefined();
      expect(snap.context.faceId).toBeUndefined();
    });
    it("mergeInteractionCallOutputs maps child context through PathTarget bindings", () => {
      const host: Record<string, unknown> = {};
      mergeInteractionCallOutputs(host, { faceId: "f-42", extra: 1 }, [
        {
          target: { root: "context", segments: [{ kind: "field", name: "faceId" }] },
          value: { kind: "path", root: "context", segments: [{ kind: "field", name: "faceId" }] },
        },
      ]);
      expect(host.faceId).toBe("f-42");
    });
    it("interaction.call supports arbitrarily nested interaction sessions", async () => {
      const grandchild = parseInteractionSpec({
        schema: "spatial.interaction",
        id: "test.pick.grandchild",
        version: "1",
        invocation: "callable",
        machine: {
          initial: "go",
          states: [
            { name: "go", on: [{ event: "confirm", transitions: [{ target: "done" }] }] },
            { name: "done", final: true },
          ],
        },
        commit: {
          fromStates: ["done"],
          operation: { kind: "action", action: "command.finish", params: { commandId: { kind: "const", value: "curve.line" } } },
        },
      })!;
      const child = parseInteractionSpec({
        schema: "spatial.interaction",
        id: "test.pick.child",
        version: "1",
        invocation: "callable",
        machine: {
          initial: "call",
          states: [
            {
              name: "call",
              on: [
                {
                  event: "confirm",
                  transitions: [
                    {
                      target: "done",
                      effects: [
                        {
                          mutation: "interaction.call",
                          interaction: "test.pick.grandchild",
                          outputs: [
                            {
                              target: { root: "context", segments: [{ kind: "field", name: "token" }] },
                              value: { kind: "const", value: "ok" },
                            },
                          ],
                        },
                      ],
                    },
                  ],
                },
              ],
            },
            { name: "done", final: true },
          ],
        },
        commit: {
          fromStates: ["done"],
          operation: { kind: "action", action: "command.finish", params: { commandId: { kind: "const", value: "curve.line" } } },
        },
      })!;
      const host = parseInteractionSpec({
        schema: "spatial.interaction",
        id: "test.pick.host",
        version: "1",
        machine: {
          initial: "call",
          states: [
            {
              name: "call",
              on: [
                {
                  event: "go",
                  transitions: [
                    {
                      target: "done",
                      effects: [
                        {
                          mutation: "interaction.call",
                          interaction: "test.pick.child",
                          outputs: [
                            {
                              target: { root: "context", segments: [{ kind: "field", name: "token" }] },
                              value: { kind: "path", root: "context", segments: [{ kind: "field", name: "token" }] },
                            },
                          ],
                        },
                      ],
                    },
                  ],
                },
              ],
            },
            { name: "done", final: true },
          ],
        },
        commit: {
          fromStates: ["done"],
          operation: { kind: "action", action: "command.finish", params: { commandId: { kind: "const", value: "curve.line" } } },
        },
      })!;
      let reg = modelDefinitionInteractionRegistry();
      reg = registerInteractionSpec(reg, compileInteraction(grandchild));
      reg = registerInteractionSpec(reg, compileInteraction(child));
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(compileInteraction(host), {
        kernel,
        document: { model: new Model(), nodes: [] },
        interactions: reg,
      });
      await rt.send({ kind: "go" });
      let snap = rt.getSnapshot();
      expect(snap.interactionId).toBe("test.pick.child");
      expect(snap.nested?.hostInteractionId).toBe("test.pick.host");
      await rt.send({ kind: "confirm" });
      snap = rt.getSnapshot();
      expect(snap.interactionId).toBe("test.pick.grandchild");
      expect(snap.nested?.hostInteractionId).toBe("test.pick.child");
      await rt.send({ kind: "confirm" });
      snap = rt.getSnapshot();
      expect(snap.interactionId).toBe("test.pick.host");
      expect(snap.state).toBe("done");
      expect(snap.context.token).toBe("ok");
    });
    it("ActionRegistry.withModelDefinitionActions registers declarative model-definition actions only", () => {
      const r = modelDefinitionActionRegistry();
      const ids = new Set(listActionDefs(r).map((d) => d.id));
      expect(ids.has("primitive.createBoxFromCorners")).toBe(true);
      expect(ids.has("box.aabbFromDiagonalCorners")).toBe(true);
      expect(ids.has("command.finish")).toBe(true);
      expect(ids.has("feature.offsetFaces")).toBe(true);
      expect(ids.has("selection.apply")).toBe(true);
      expect(ids.has("selection.selectAll")).toBe(true);
      expect(ids.has("selection.selectVertices")).toBe(true);
      expect(listActionDefs(r).every((def) => def.spec !== undefined && def.run === undefined)).toBe(true);
    });
    it("register replaces a model-definition action id", () => {
      let r = modelDefinitionActionRegistry();
      const before = r.get("measure.faceArea")?.label;
      r = registerActionDef(r, {
        id: "measure.faceArea",
        label: "override",
        run: () => ({ data: 99 }),
      });
      expect(r.get("measure.faceArea")?.label).toBe("override");
      expect(before).not.toBe("override");
    });
    it("InteractionRegistry.withModelDefinitionInteractions get matches buildBoxInteractionSpec", () => {
      const reg = modelDefinitionInteractionRegistry();
      expect(reg.get("primitive.box")).toEqual(buildBoxInteractionSpec());
    });
    it("loadSpatialInteraction resolves callable surface.construct hub from shipped assets", () => {
      const spec = loadSpatialInteraction("surface.construct");
      expect(spec?.id).toBe("surface.construct");
      expect(spec?.invocation).toBe("callable");
      expect(isCallableOnlyInteraction(spec!)).toBe(true);
      expect(loadSpatialInteraction("curve.construct")?.invocation).toBe("callable");
    });
    it("loadSpatialInteraction returns the same compiled instance per interaction id", () => {
      expect(loadSpatialInteraction("primitive.box")).toBe(loadSpatialInteraction("primitive.box"));
    });
    it("createBoxFrom3Points forwards triplet footprint to createBoxFromCorners", async () => {
      class StubKernel extends BrepjsKernel {
        lastInput: { cornerA: Vec3; cornerB: Vec3; height: number } | null = null;
        async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }) {
          this.lastInput = input;
          return { diff: EMPTY_MODEL_DIFF, solid: solidRef("c") };
        }
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const k = new StubKernel();
      const model = new Model();
      const p0: Vec3 = [0, 0, 0];
      const p1: Vec3 = [2, 3, 0];
      const p2: Vec3 = [1, 1, 0];
      await runRegisteredAction(modelDefinitionActionRegistry(),"primitive.createBoxFrom3Points", { p0, p1, p2, __context: {}, __event: { kind: "x" } }, { kernel: k as unknown as SpatialKernel, preview: M, model });
      expect(k.lastInput).toEqual({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 3 });
    });
    it("command.addSelection applies selection modifiers", async () => {
      const actions = modelDefinitionActionRegistry();
      const base = [{ kind: "wire", id: "w0", editable: true }] as const;
      const next = [{ kind: "wire", id: "w1", editable: true }] as const;
      const additive = await runRegisteredAction(actions,
        "command.addSelection",
        { targets: next, __context: { targets: base }, __event: { kind: "selection.changed", modifiers: { shift: true } } },
        { kernel: M as unknown as SpatialKernel, preview: M, model: new Model() },
      );
      expect((additive.patch?.set as { targets?: readonly SelectionTarget[] }).targets).toEqual([...base, ...next]);
      const subtractive = await runRegisteredAction(actions,
        "command.addSelection",
        {
          targets: next,
          __context: { targets: [...base, ...next] },
          __event: { kind: "selection.changed", modifiers: { ctrl: true } },
        },
        { kernel: M as unknown as SpatialKernel, preview: M, model: new Model() },
      );
      expect((subtractive.patch?.set as { targets?: readonly SelectionTarget[] }).targets).toEqual(base);
      const invertive = await runRegisteredAction(actions,
        "command.addSelection",
        {
          targets: [
            { kind: "wire", id: "w0", editable: true },
            { kind: "wire", id: "w2", editable: true },
          ],
          __context: { targets: [...base, ...next] },
          __event: { kind: "selection.changed", modifiers: { shift: true, ctrl: true } },
        },
        { kernel: M as unknown as SpatialKernel, preview: M, model: new Model() },
      );
      expect((invertive.patch?.set as { targets?: readonly SelectionTarget[] }).targets).toEqual([
        { kind: "wire", id: "w1", editable: true },
        { kind: "wire", id: "w2", editable: true },
      ]);
    });
    it("selection.apply runs selectAll, deselectAll, invert, and selectKinds", async () => {
      const actions = modelDefinitionActionRegistry();
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const seed = [{ kind: "vertex", id: Object.keys(model.vertices)[0]!, editable: true }] as const;
      const all = await runRegisteredAction(actions,"selection.apply", { operation: "selectAll", seedTargets: [], __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      const allTargets = selectionTargetsFromActionResult(all);
      expect(allTargets.length).toBeGreaterThan(8);
      const cleared = await runRegisteredAction(actions,"selection.apply", { operation: "deselectAll", seedTargets: allTargets, __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      expect(selectionTargetsFromActionResult(cleared)).toEqual([]);
      const verts = await runRegisteredAction(actions,"selection.apply", { operation: "selectKinds", kinds: ["vertex"], seedTargets: [], __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      const vertTargets = selectionTargetsFromActionResult(verts);
      expect(vertTargets.length).toBe(8);
      expect(vertTargets.every((t) => t.kind === "vertex")).toBe(true);
      const inverted = await runRegisteredAction(actions,"selection.apply", { operation: "invert", seedTargets: vertTargets.slice(0, 1), __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      const invertedTargets = selectionTargetsFromActionResult(inverted);
      expect(invertedTargets.some((t) => t.kind === "vertex")).toBe(true);
      expect(invertedTargets.some((t) => t.kind === "face")).toBe(true);
      expect(invertedTargets.find((t) => t.id === vertTargets[0]!.id)).toBeUndefined();
    });
    it("selection.selectAll returns targets without model diff", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const result = await runRegisteredAction(modelDefinitionActionRegistry(),"selection.selectAll", { seedTargets: [], __context: {}, __event: { kind: "commit" } }, { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model });
      const targets = selectionTargetsFromActionResult(result);
      expect(targets.length).toBeGreaterThan(8);
      expect(targets.some((t) => t.kind === "solid")).toBe(true);
      expect(isEmptyModelDiff(result.diff ?? EMPTY_MODEL_DIFF)).toBe(true);
    });
    it("interactionRecordsDocumentHistory skips selection commands", () => {
      expect(interactionRecordsDocumentHistory("selection.selectAll")).toBe(false);
      expect(interactionRecordsDocumentHistory("measure.distance")).toBe(false);
      expect(interactionRecordsDocumentHistory("primitive.box")).toBe(true);
    });
    it("selection.selectAll headless does not push document history entries", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const hist = new DocumentHistory();
      await runRegisteredAction(modelDefinitionActionRegistry(),"selection.selectAll", { seedTargets: [], __context: {}, __event: { kind: "commit" } }, { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model });
      expect(hist.entries()).toEqual([]);
    });
    it.each(listSelectionOperationsForModelDefinition(defaultModelDefinitionId()))("registers selection command action $id", (defn) => {
      expect(modelDefinitionActionRegistry().get(defn.id)?.spec?.schema).toBe("spatial.action");
    });
    it("selection.invert honors seed targets", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const seed = [{ kind: "solid", id: "e2e-box", editable: true }] as const;
      const result = await runRegisteredAction(modelDefinitionActionRegistry(),"selection.invert", { seedTargets: seed, __context: {}, __event: { kind: "commit" } }, { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model });
      const targets = selectionTargetsFromActionResult(result);
      expect(targets.some((t) => t.kind === "solid" && t.id === "e2e-box")).toBe(false);
      expect(targets.length).toBeGreaterThan(0);
    });
    it("ActionRegistry.run executes selection.apply headless", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const actions = modelDefinitionActionRegistry();
      const result = await runRegisteredAction(actions,
        "selection.apply",
        {
          operation: "selectKinds",
          kinds: ["face"],
          seedTargets: [],
          __context: {},
          __event: { kind: "commit" },
        },
        { kernel: M as unknown as SpatialKernel, preview: M, model },
      );
      const targets = selectionTargetsFromActionResult(result);
      expect(targets.length).toBe(6);
      expect(targets.every((t) => t.kind === "face")).toBe(true);
    });
    it("runSelectionApply matches executeSelectionApply", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const ctx = { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model };
      const params = { operation: "selectAll" as const, seedTargets: [] };
      const direct = executeSelectionApply(params, { model });
      const headless = await runSelectionApply(params, ctx);
      expect(headless).toEqual(direct);
    });
    it.each(listSelectionOperationsForModelDefinition(defaultModelDefinitionId()))("runSelectionApply matches runSelectionOperationInteraction for $id", async (defn) => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const seed = selectionSeedTargetsForOperation(defn.operation);
      const params = selectionApplyParamsForInteraction(defn, seed);
      const headless = await runSelectionApply(params, { kernel, preview: M, model });
      const interactive = await runSelectionOperationInteraction(defn.id, {
        kernel,
        document: { model, nodes: [] },
        seedTargets: seed,
      });
      expect(interactive.targets).toEqual(headless);
    });
    it("selection commands chain selectAll → deselectAll → selectVertices → invert", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const actions = modelDefinitionActionRegistry();
      const run = async (id: string, targets: readonly SelectionTarget[]) => {
        const result = await runRegisteredAction(actions,id, { seedTargets: targets, __context: {}, __event: { kind: "commit" } }, { kernel, preview: M, model });
        return selectionTargetsFromActionResult(result);
      };
      const all = await run("selection.selectAll", []);
      expect(all.length).toBeGreaterThan(8);
      const cleared = await run("selection.deselectAll", all);
      expect(cleared).toEqual([]);
      const verts = await run("selection.selectVertices", cleared);
      expect(verts.length).toBe(8);
      expect(verts.every((t) => t.kind === "vertex")).toBe(true);
      const inverted = await run("selection.invert", verts.slice(0, 1));
      expect(inverted.some((t) => t.kind === "face")).toBe(true);
      expect(inverted.find((t) => t.id === verts[0]!.id)).toBeUndefined();
    });
  });
}
