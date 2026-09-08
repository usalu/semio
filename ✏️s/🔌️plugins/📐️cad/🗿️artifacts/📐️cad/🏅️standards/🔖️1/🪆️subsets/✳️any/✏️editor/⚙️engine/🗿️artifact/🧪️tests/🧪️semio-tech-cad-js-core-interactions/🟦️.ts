type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { CAD_E2E_BUILDING_BOOLEAN_MODEL_SPACE_JSON, CAD_E2E_LOOM_MODEL_SPACE_JSON, CAD_E2E_ROUTES_MODEL_SPACE_JSON, DocumentHistory, EMPTY_MODEL_DIFF, EdgeRef, FaceRef, InteractionEvent, InteractionRuntime, InteractionSpec, Model, ModelDiff, ModelEntityKind, ModelSpace, ModelSpaceJson, ObjectRef, SelectionOperationInteractionDef, SelectionTarget, SolidRef, SpatialKernel, SpatialPreviewKernel, TypologyRef, VertexRef, WireRef, __artifactTestKernel, __artifactTestRuntime, actionOwnedByModelDefinition, applyModelDiff, buildAreaInteractionSpec, buildBoxInteractionSpec, buildDistanceInteractionSpec, clampPointAlongDirection, collectGeometrySelectionTargets, compileInteraction, createInteractionRuntime, defaultModelDefinitionId, emptyMeshTransfer, ensureTypologyObjectFromCreateDiff, executeSelectionApply, interactionControlForState, interactionLengthEntryForState, interactionLengthEntryLiveDistance, interactionNumericEntryCommitEvent, interactionNumericEntryExplicitLockValue, interactionNumericEntryLockedValue, interactionRecordsDocumentHistory, interactionStepFinalizeEvent, isCallableOnlyInteraction, isEmptyModelDiff, isFinalInteractionState, isShapeModelDefinition, listActionDefs, listConstructableTypologiesForModelDefinition, listInteractionSpecs, listModelDefinitionActionSpecs, listModelDefinitionManifests, listModelDefinitionTypologies, listModelObjectsForModelDefinition, listSelectionOperationsForModelDefinition, listSpatialInteractionsForModelDefinition, listTypologiesForModelDefinition, loadSpatialInteraction, loadTypology, mergeInteractionCallOutputs, modelDefinitionActionRegistry, modelDefinitionIdForInteraction, modelDefinitionInteractionRegistry, parseActionSpec, parseInteractionSpec, parseModelJson, pureTsStateEngineProvider, readInteractionContextVec3, registerActionDef, registerInteractionSpec, requireSpatialInteraction, resolveDisplay, runRegisteredAction, runSelectionApply, runSelectionOperationInteraction, selectionApplyParamsForInteraction, selectionSeedTargetsForOperation, selectionTargetsFromActionResult, selectionTargetsFromContext, selectionTargetsPointTransformDiff, solidRef, typologyConstructAssetIds, typologyConstructCommitActionForMode, typologyConstructKitByInteraction, typologyConstructModeActionIds, typologyHasNativeConstructKit, typologyIdForInteractionCommit } = dependencies;
  type InteractionResponse = any;
  type InteractionSnapshot = any;
  type MeshTransfer = any;
  type ShapeNode = any;
  type Vec3 = any;

  __artifactTestRuntime!.bootstrapCadModules();
  const { BrepjsKernel, preciseSpatialKernelMath } = __artifactTestKernel!;
  const geometryLoomFixtureJson = JSON.parse(CAD_E2E_LOOM_MODEL_SPACE_JSON) as ModelSpaceJson;
  const geometryRoutesFixtureJson = JSON.parse(CAD_E2E_ROUTES_MODEL_SPACE_JSON) as ModelSpaceJson;
  const buildingBooleanFixtureJson = JSON.parse(CAD_E2E_BUILDING_BOOLEAN_MODEL_SPACE_JSON) as ModelSpaceJson;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = vitest;

  describe("@semio-tech/cad-js/core interactions", () => {
    async function bootTransformSelection(rt: InteractionRuntime, targets: readonly SelectionTarget[]): Promise<void> {
      await rt.send({ kind: "start", modifiers: {} });
      if (targets.length === 0) return;
      await rt.send({ kind: "selection.changed", targets: [...targets], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
    }

    it("auto-commits curve.arc as one arc edge between start and end", async () => {
      const model = new Model();
      class ArcKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async executeCommandDiff(commandId: string, ctx: Record<string, unknown>) {
          if (commandId !== "curve.arc") return { diff: EMPTY_MODEL_DIFF };
          const center = (Array.isArray(ctx.center) ? ctx.center : [0, 0, 0]) as unknown as Vec3;
          const start = (Array.isArray(ctx.start) ? ctx.start : [1, 0, 0]) as unknown as Vec3;
          const end = M.arcEndOnCircle(center, start, (Array.isArray(ctx.end) ? ctx.end : start) as unknown as Vec3);
          const v0 = "v0" as VertexRef;
          const v1 = "v1" as VertexRef;
          const e = "e0" as EdgeRef;
          const w = "w0" as WireRef;
          return {
            diff: {
              vertices: {
                added: [
                  { id: v0, position: start },
                  { id: v1, position: end },
                ],
              },
              edges: { added: [{ id: e, vertexIds: [v0, v1], curve: { kind: "arc" as const, center } }] },
              wires: { added: [{ id: w, edgeIds: [e] }] },
            },
          };
        }
      }
      const spec = loadSpatialInteraction("curve.arc")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new ArcKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [0, 2, 0] as Vec3, modifiers: {} });
      const snap = rt.getSnapshot();
      expect(snap.state).toBe("committed");
      expect(snap.lastResponse?.ok).toBe(true);
      const edges = Object.values(model.edges);
      expect(edges).toHaveLength(1);
      expect(edges[0]!.curve).toEqual({ kind: "arc", center: [0, 0, 0] });
      expect(Object.keys(model.vertices)).toHaveLength(2);
    });
    it("normalizes commit fromStates to committed for scripted commands without ready", () => {
      const spec = loadSpatialInteraction("curve.arc")!;
      expect(spec.commit.fromStates).toEqual(["committed"]);
    });
    it("transform.move vertical mode changes Z only", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const v0 = Object.keys(model.vertices)[0]!;
      const p0 = model.vertices[v0]!.position;
      class CommandKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async executeCommandDiff() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = loadSpatialInteraction("transform.move")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new CommandKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await bootTransformSelection(rt, [{ kind: "vertex", id: v0, editable: true }]);
      await rt.send({ kind: "pointer.down", point: p0, modifiers: {} });
      await rt.send({ kind: "mode.vertical", modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [p0[0] + 5, p0[1] + 4, p0[2] + 2], modifiers: {} });
      expect(rt.getSnapshot().lastResponse?.ok).toBe(true);
      expect(model.vertices[v0]!.position).toEqual([p0[0], p0[1], p0[2] + 2]);
    });
    it("transform.move confirm without pick uses selection bbox center", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [2, 0, 0], height: 0 }, solidRef("box")));
      const verts = Object.values(model.vertices);
      class CommandKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async executeCommandDiff() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = loadSpatialInteraction("transform.move")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new CommandKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await bootTransformSelection(
        rt,
        verts.map((v) => ({ kind: "vertex" as const, id: v.id, editable: true })),
      );
      expect(rt.getSnapshot().state).toBe("point_to_move_from");
      await rt.send({ kind: "confirm", modifiers: {} });
      const from = rt.getSnapshot().context.from as Vec3;
      expect(from[0]).toBeCloseTo(1, 5);
      expect(from[1]).toBeCloseTo(0, 5);
    });
    it("auto-finalizes transform.move on terminal pointer down without alreadyCommitted", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const v0 = Object.keys(model.vertices)[0]!;
      const p0 = model.vertices[v0]!.position;
      class CommandKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async executeCommandDiff() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = loadSpatialInteraction("transform.move")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new CommandKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await bootTransformSelection(rt, [{ kind: "vertex", id: v0, editable: true }]);
      await rt.send({ kind: "pointer.down", point: p0, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [p0[0] + 2, p0[1] + 1, p0[2]], modifiers: {} });
      const snap = rt.getSnapshot();
      expect(snap.state).toBe("committed");
      expect(snap.lastResponse?.ok).toBe(true);
      expect(snap.lastResponse?.errors).toEqual([]);
      expect(snap.lastResponse?.diff?.vertices?.modified?.length).toBeGreaterThan(0);
      expect(model.vertices[v0]!.position).toEqual([p0[0] + 2, p0[1] + 1, p0[2]]);
    });

    it("transform.copy action constrains vertical delta to Z only", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const actions = modelDefinitionActionRegistry();
      const from: Vec3 = [0, 0, 0];
      const r = await runRegisteredAction(actions,
        "transform.copy",
        {
          targets: [{ kind: "solid", id: "e2e-box", editable: true }],
          from,
          to: [5, 4, 2],
          moveMode: "vertical",
        },
        { model: model, kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M },
      );
      const added = r.diff?.vertices?.added ?? [];
      const originals = Object.values(model.vertices);
      expect(added.length).toBe(8);
      for (const v of added) {
        expect(originals.some((o) => Math.abs(v.position[0] - o.position[0]) < 1e-5 && Math.abs(v.position[1] - o.position[1]) < 1e-5 && Math.abs(v.position[2] - o.position[2] - 2) < 1e-5)).toBe(true);
        expect(originals.some((o) => Math.abs(v.position[0] - o.position[0] - 5) < 1e-5 && Math.abs(v.position[1] - o.position[1] - 4) < 1e-5 && Math.abs(v.position[2] - o.position[2] - 2) < 1e-5)).toBe(false);
      }
    });

    it("transform.copy session keeps vertical moveMode through pick workflow", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const before = Object.keys(model.vertices).length;
      const spec = loadSpatialInteraction("transform.copy")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new BrepjsKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      const from = model.vertices[Object.keys(model.vertices)[0]!]!.position;
      await bootTransformSelection(rt, [{ kind: "solid", id: "e2e-box", editable: true }]);
      await rt.send({ kind: "mode.vertical", modifiers: {} });
      await rt.send({ kind: "pointer.down", point: from, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [from[0] + 5, from[1] + 4, from[2] + 2], modifiers: {} });
      const snap = rt.getSnapshot();
      expect(snap.context.moveMode).toBe("vertical");
      expect(snap.state).toBe("committed");
      expect(snap.lastResponse?.ok).toBe(true);
      expect(Object.keys(model.vertices).length).toBeGreaterThan(before);
    });

    it("transform.copy confirm without from pick uses selection bbox center", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 1 }, solidRef("e2e-box")));
      const spec = loadSpatialInteraction("transform.copy")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new BrepjsKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await bootTransformSelection(rt, [{ kind: "solid", id: "e2e-box", editable: true }]);
      await rt.send({ kind: "confirm", modifiers: {} });
      const from = rt.getSnapshot().context.from as Vec3;
      expect(from[0]).toBeCloseTo(1, 5);
      expect(from[1]).toBeCloseTo(1, 5);
      await rt.send({ kind: "pointer.down", point: [from[0] + 1, from[1], from[2]], modifiers: {} });
      expect(rt.getSnapshot().state).toBe("committed");
      expect(rt.getSnapshot().lastResponse?.ok).toBe(true);
    });
  });
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
  describe("@semio-tech/cad-js/core interaction box", () => {
    it("tracks first-corner cursor on the grid after start", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      let snap = rt.getSnapshot();
      expect(snap.state).toBe("first_corner");
      expect(snap.context.cursor).toEqual([0, 0, 0]);
      expect(snap.display.items.find((i) => i.id === "first-cursor")?.params?.position).toEqual([0, 0, 0]);
      await rt.send({ kind: "pointer.move", point: [-1, 2.5, 0] as Vec3, modifiers: {} });
      snap = rt.getSnapshot();
      expect(snap.context.cursor).toEqual([-1, 2.5, 0]);
      expect(snap.display.items.find((i) => i.id === "first-cursor")?.params?.position).toEqual([-1, 2.5, 0]);
      await rt.send({ kind: "pointer.down", point: [3, 1, 0] as Vec3, modifiers: {} });
      snap = rt.getSnapshot();
      expect(snap.state).toBe("first_corner_other_or_length");
      expect(snap.context.cursor).toBeUndefined();
    });

    it("pushes interaction-local undo snapshot on each non-transient transition", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      expect(rt.getSnapshot().capabilities.canUndo).toBe(false);
      const initial = rt.getSnapshot().state;
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      expect(rt.getSnapshot().capabilities.canUndo).toBe(true);
      await rt.undo();
      expect(rt.getSnapshot().state).toBe(initial);
    });

    it("runs box workflow with a recording kernel stub (no solid modeling in core)", async () => {
      const stubMesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      class RecordingStubKernel {
        readonly id = "recording-stub";
        readonly operations = ["solid.createBox", "entity.tessellate"] as const;
        lastBox: { cornerA: Vec3; cornerB: Vec3; height: number } | null = null;
        constructor() {
          Object.assign(this, M);
        }
        async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef> {
          this.lastBox = input;
          return solidRef("stub-solid");
        }
        async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
          const solid = await this.createBoxFromCorners(input);
          return { diff: M.boxModelDiff(input, solid), solid };
        }
        async volume(): Promise<number> {
          return 0;
        }
        async tessellate(): Promise<MeshTransfer> {
          return stubMesh;
        }
      }
      const spec = buildBoxInteractionSpec();
      const model = new Model();
      const kernel = new RecordingStubKernel();
      const rt = createInteractionRuntime(spec, { kernel: kernel as unknown as SpatialKernel, document: { model: model, nodes: [] } });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.height", value: 4, modifiers: {} });
      expect(rt.getSnapshot().state).toBe("first_corner_height");
      expect(rt.getSnapshot().context.height).toBe(4);
      await rt.send({ kind: "confirm", modifiers: {} });
      const snap = rt.getSnapshot();
      const res = snap.lastResponse!;
      expect(snap.state).toBe("committed");
      expect(res.ok).toBe(true);
      expect(res.data).toEqual({ solid: "stub-solid" });
      expect(res.archiveContext).not.toBeNull();
      expect(res.archiveContext!.origin).toEqual([0, 0, 0]);
      expect(res.archiveContext!.corner).toEqual([2, 3, 0]);
      expect(res.archiveContext!.height).toBe(4);
      expect(Object.keys(model.vertices).length).toBe(8);
      expect(Object.keys(model.edges).length).toBe(12);
      expect(Object.keys(model.wires).length).toBe(6);
      expect(Object.keys(model.faces).length).toBe(6);
      expect(Object.keys(model.shells).length).toBe(1);
      expect(Object.keys(model.solids)).toEqual(["stub-solid"]);
      expect(kernel.lastBox).toEqual({
        cornerA: [0, 0, 0],
        cornerB: [2, 3, 0],
        height: 4,
      });
    });

    it("set.length clamps diagonal from diagA without prior pointer move", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "mode.diagonal" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      expect(rt.getSnapshot().state).toBe("diagonal_rubber");
      await rt.send({ kind: "set.length", value: 5, modifiers: {} });
      const corner = rt.getSnapshot().context.corner as Vec3;
      expect(corner[0]).toBeCloseTo(5, 5);
      expect(corner[1]).toBeCloseTo(0, 5);
    });

    it("scalar axis pointer.move sets height from projection along +Z", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
      expect(rt.getSnapshot().state).toBe("first_corner_height");
      await rt.send({ kind: "pointer.move", point: [9, 9, 4.2] as Vec3, modifiers: {} });
      expect(rt.getSnapshot().context.height).toBeCloseTo(4.2, 5);
      expect(rt.getSnapshot().context.__scalarAxisT).toBeCloseTo(4.2, 5);
    });

    it("resolveDisplay injects height line, cursor, and guide for scalar entry", () => {
      const spec = buildBoxInteractionSpec();
      const ctx: Record<string, unknown> = {
        origin: [0, 0, 0] as Vec3,
        corner: [2, 3, 0] as Vec3,
        height: 3,
        __scalarAxisT: 3,
        __cursorRaw: [5, 6, 4] as Vec3,
      };
      const d = resolveDisplay(spec, "first_corner_height", ctx, M);
      expect(d.items.some((i) => i.id === "first_corner_height-scalar-height")).toBe(true);
      expect(d.items.some((i) => i.id === "first_corner_height-scalar-cursor" && i.role === "cursor")).toBe(true);
      expect(d.items.some((i) => i.id === "first_corner_height-scalar-guide" && i.role === "guide")).toBe(true);
      const heightSeg = d.items.find((i) => i.id === "first_corner_height-scalar-height");
      expect(heightSeg?.kind).toBe("segment");
      if (heightSeg?.kind === "segment") {
        const to = heightSeg.params?.to;
        expect(Array.isArray(to) && typeof to[2] === "number" ? to[2] : NaN).toBeCloseTo(3, 5);
      }
    });

    it("set.height live entry keeps first_corner_height until confirm", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.height", value: 3.5, modifiers: {} });
      expect(rt.getSnapshot().state).toBe("first_corner_height");
      expect(rt.getSnapshot().context.height).toBe(3.5);
    });

    it("set.length clamps rubber-band corner from origin on first footprint edge", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [3, 4, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.length", value: 2.5, modifiers: {} });
      const snap = rt.getSnapshot();
      expect(snap.state).toBe("first_corner_other_or_length");
      const corner = snap.context.corner as Vec3;
      expect(corner[0]).toBeCloseTo(1.5, 5);
      expect(corner[1]).toBeCloseTo(2, 5);
      expect(snap.display.items.some((i) => i.id === "first_corner_other_or_length-length-guide")).toBe(true);
    });
  });
  describe("@semio-tech/cad-js/core interaction length entry", () => {
    it("interactionLengthEntryForState resolves shipped line rubber-band", () => {
      const spec = requireSpatialInteraction("curve.line");
      expect(interactionLengthEntryForState(spec, "end_of_line")).toEqual({
        state: "end_of_line",
        anchor: "points.start",
        field: "cursor",
        control: "stepper",
        min: 0,
        step: 0.1,
        unit: "m",
      });
    });

    it("readInteractionContextVec3 supports points.@last on arrays", () => {
      const ctx = {
        points: [
          [0, 0, 0],
          [1, 2, 3],
        ] as Vec3[],
      };
      expect(readInteractionContextVec3(ctx, "points.@last")).toEqual([1, 2, 3]);
    });

    it("clampPointAlongDirection preserves direction and length", () => {
      expect(clampPointAlongDirection([0, 0, 0], [3, 4, 0], 2.5, M)).toEqual([1.5, 2, 0]);
    });

    it("set.length clamps cursor along anchor direction", async () => {
      class StubKernel extends BrepjsKernel {
        async curveLine() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = requireSpatialInteraction("curve.line");
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [3, 4, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.length", value: 2.5, modifiers: {} });
      const snap = rt.getSnapshot();
      const cursor = snap.context.cursor as Vec3;
      expect(cursor[0]).toBeCloseTo(1.5, 5);
      expect(cursor[1]).toBeCloseTo(2, 5);
      expect(snap.context.__lengthLock).toBe(2.5);
      expect(snap.display.items.some((i) => i.id === "end_of_line-length-guide")).toBe(true);
    });

    it("set.length null unlocks and pointer.move follows cursor again", async () => {
      class StubKernel extends BrepjsKernel {
        async curveLine() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = requireSpatialInteraction("curve.line");
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [3, 4, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.length", value: 2.5, modifiers: {} });
      await rt.send({ kind: "set.length", value: null, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [10, 0, 0] as Vec3, modifiers: {} });
      const cursor = rt.getSnapshot().context.cursor as Vec3;
      expect(cursor).toEqual([10, 0, 0]);
      expect(rt.getSnapshot().context.__lengthLock).toBeNull();
    });

    it("interactionLengthEntryLiveDistance reads Z rod cursor offset for extrusion_distance", () => {
      const spec = loadSpatialInteraction("surface.extrudeCrv")!;
      const entry = interactionLengthEntryForState(spec, "extrusion_distance")!;
      const distance = interactionLengthEntryLiveDistance({ origin: [0, 0, 0] as Vec3, cursor: [0, 0, 1.25] as Vec3, direction: [0, 0, 1] as Vec3 }, entry);
      expect(distance).toBeCloseTo(1.25, 5);
      expect(interactionNumericEntryLockedValue(spec, "extrusion_distance", { origin: [0, 0, 0], cursor: [0, 0, 2], direction: [0, 0, 1] })).toBe(2);
    });

    it("surface.extrudeCrv confirm in extrusion_distance commits solid", async () => {
      const spec = loadSpatialInteraction("surface.extrudeCrv")!;
      const space = ModelSpace.fromJSON(geometryRoutesFixtureJson as ModelSpaceJson);
      const model = space.models["spatial.shape"]!;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "wire", id: "stub-wire" }], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [0, 0, 0.8], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      expect(rt.getSnapshot().state).toBe("committed");
      expect(rt.getSnapshot().lastResponse?.ok).toBe(true);
      expect(Object.keys(model.solids).length).toBeGreaterThan(0);
    });

    it("surface.extrudeCrv start seeds curves from selected interpolate object", async () => {
      const spec = loadSpatialInteraction("surface.extrudeCrv")!;
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
      const typology = "spatial.shape.curve.interpolate-curve";
      const wireId = created.diff.wires?.added?.[0]?.id ?? Object.values(model.wires)[0]?.id;
      expect(wireId).toBeTruthy();
      const objectId = String(ensureTypologyObjectFromCreateDiff(model, typology, created.diff)!);
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({
        kind: "start",
        targets: [{ kind: "object", id: objectId, editable: true }],
        modifiers: {},
      });
      expect(rt.getSnapshot().state).toBe("extrusion_distance");
      expect((rt.getSnapshot().context.curves as { id: string }[])[0]?.id).toBe(wireId);
    });

    it("interactionControlForState resolves stepper for box height and ring for rotate angle", () => {
      const box = loadSpatialInteraction("primitive.box")!;
      const height = interactionControlForState(box, "first_corner_height", { height: 3 });
      expect(height?.kind).toBe("stepper");
      if (height && height.kind !== "ring") {
        expect(height.value).toBe(3);
        expect(height.unit).toBe("m");
      }
      const rotate = loadSpatialInteraction("transform.rotate")!;
      const angle = interactionControlForState(rotate, "angle_or_first_reference_point", { angle: 45 });
      expect(angle?.kind).toBe("ring");
      if (angle?.kind === "ring") {
        expect(angle.options.length).toBeGreaterThan(0);
      }
    });

    it("interactionNumericEntryCommitEvent uses pointer.down for length and confirm for scalar", () => {
      const line = requireSpatialInteraction("curve.line");
      const box = buildBoxInteractionSpec();
      const ctx = { points: { start: [0, 0, 0] as Vec3 }, cursor: [3, 0, 0] as Vec3 };
      expect(interactionNumericEntryCommitEvent(line, "end_of_line", ctx, M)?.kind).toBe("pointer.down");
      expect(interactionNumericEntryCommitEvent(box, "first_corner_height", { height: 2 }, M)?.kind).toBe("confirm");
    });

    it("interactionNumericEntryCommitEvent pointer.down from length lock without field", () => {
      const box = buildBoxInteractionSpec();
      const ctx = { origin: [0, 0, 0] as Vec3, diagA: [0, 0, 0] as Vec3, __lengthLock: 4 };
      const ev = interactionNumericEntryCommitEvent(box, "diagonal_rubber", ctx, M);
      expect(ev?.kind).toBe("pointer.down");
      if (ev?.kind === "pointer.down") {
        const point = ev.point;
        expect(Array.isArray(point) && typeof point[0] === "number" ? point[0] : NaN).toBeCloseTo(4, 5);
        expect(Array.isArray(point) && typeof point[1] === "number" ? point[1] : NaN).toBeCloseTo(0, 5);
      }
    });

    it("Enter commit applies length then pointer.down on line rubber-band", async () => {
      class StubKernel extends BrepjsKernel {
        async curveLine() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = requireSpatialInteraction("curve.line");
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [3, 4, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.length", value: 2.5, modifiers: {} });
      const commitEv = interactionNumericEntryCommitEvent(spec, rt.getSnapshot().state, rt.getSnapshot().context, M);
      expect(commitEv?.kind).toBe("pointer.down");
      await rt.send(commitEv!);
      expect(rt.getSnapshot().state).toBe("committed");
    });

    it("pointer.down while locked commits clamped point", async () => {
      class StubKernel extends BrepjsKernel {
        async curveLine() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = requireSpatialInteraction("curve.line");
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [3, 4, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.length", value: 2.5, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [3, 4, 0] as Vec3, modifiers: {} });
      const snap = rt.getSnapshot();
      const end = (snap.context.points as Record<string, Vec3>).end;
      expect(end[0]).toBeCloseTo(1.5, 5);
      expect(end[1]).toBeCloseTo(2, 5);
      expect(snap.context.__lengthLock).toBeUndefined();
    });
  });
  describe("@semio-tech/cad-js/core stateEngine option", () => {
    it("explicit pure-ts provider matches default interaction snapshots", async () => {
      class StubKernel extends BrepjsKernel {
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
      const spec = buildBoxInteractionSpec();
      const rt0 = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      const rt1 = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
        stateEngine: pureTsStateEngineProvider,
      });
      expect(rt1.getSnapshot().state).toBe(rt0.getSnapshot().state);
      expect(rt1.getSnapshot().context).toEqual(rt0.getSnapshot().context);
      expect(rt1.getSnapshot().capabilities).toEqual(rt0.getSnapshot().capabilities);
    });
  });
  describe("@semio-tech/cad-js/core measure distance", () => {
    it("measure.faceArea action adds face anchor geometry", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("m-area")));
      const fid = Object.keys(model.faces)[0]! as FaceRef;
      const r = await runRegisteredAction(modelDefinitionActionRegistry(),"measure.faceArea", { faceId: fid }, { model: model, kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M });
      expect(r.data).toBeGreaterThan(0);
      expect(r.diff?.anchors?.added?.length).toBe(1);
      expect(r.diff!.anchors!.added![0]!.attachment.kind).toBe("face");
    });

    it("commit returns vertex distance in data", async () => {
      class MeasKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async query(name: string, params: Record<string, unknown>) {
          if (name === "surface.resolveFaces") return [String(params.surfaceId ?? "")];
          return undefined;
        }
        async vertexDistance(a: VertexRef, b: VertexRef, t: Model) {
          const pa = t.vertices[String(a)]?.position;
          const pb = t.vertices[String(b)]?.position;
          if (!pa || !pb) return 0;
          return M.vec3Distance(pa, pb);
        }
      }
      const model = new Model();
      const va = "v0" as VertexRef;
      const vb = "v1" as VertexRef;
      model.vertices[va] = { id: va, position: [0, 0, 0] };
      model.vertices[vb] = { id: vb, position: [3, 4, 0] };
      const spec = buildDistanceInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new MeasKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: va, editable: true }] });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: vb, editable: true }] });
      const res = rt.getSnapshot().lastResponse!;
      expect(res.ok).toBe(true);
      expect(res.data).toBe(5);
      expect(isEmptyModelDiff(res.diff)).toBe(false);
      expect(res.diff.edges?.added?.length).toBe(1);
      expect(res.diff.wires?.added?.length).toBe(1);
      const edge = res.diff.edges!.added![0]!;
      expect(edge.vertexIds).toEqual([va, vb]);
    });

    it("auto-commits when confirm reaches the final state", async () => {
      class MeasKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async vertexDistance(a: VertexRef, b: VertexRef, t: Model) {
          const pa = t.vertices[String(a)]?.position;
          const pb = t.vertices[String(b)]?.position;
          if (!pa || !pb) return 0;
          return M.vec3Distance(pa, pb);
        }
      }
      const model = new Model();
      const va = "v0" as VertexRef;
      const vb = "v1" as VertexRef;
      model.vertices[va] = { id: va, position: [0, 0, 0] };
      model.vertices[vb] = { id: vb, position: [3, 4, 0] };
      const rt = createInteractionRuntime(buildDistanceInteractionSpec(), {
        kernel: new MeasKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: va, editable: true }] });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: vb, editable: true }] });
      const snap = rt.getSnapshot();
      expect(snap.state).toBe("committed");
      expect(snap.capabilities.canCommit).toBe(false);
      expect(snap.capabilities.canCancel).toBe(false);
      expect(snap.lastResponse?.ok).toBe(true);
      expect(snap.lastResponse?.data).toBe(5);
    });
  });
  describe("@semio-tech/cad-js/core measure area", () => {
    it("resolves face picks through surface.resolveFaces before commit", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("area-box")));
      const fid = Object.keys(model.faces)[0]! as FaceRef;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(buildAreaInteractionSpec(), { kernel, document: { model: model, nodes: [] } });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "face", id: fid, editable: true }] });
      expect(rt.getSnapshot().context.resolvedFaceIds).toEqual([fid]);
      await rt.send({ kind: "confirm", modifiers: {} });
      const snap = rt.getSnapshot();
      expect(snap.state).toBe("committed");
      expect(snap.lastResponse?.ok).toBe(true);
      expect(typeof snap.lastResponse?.data).toBe("number");
      expect(isEmptyModelDiff(snap.lastResponse!.diff)).toBe(false);
      expect(snap.lastResponse!.diff.anchors?.added?.length).toBe(1);
    });

    it("commit returns face area in data", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("area-box")));
      const fid = Object.keys(model.faces)[0]! as FaceRef;
      class AreaKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async query(name: string, params: Record<string, unknown>) {
          if (name === "surface.resolveFaces") {
            const sid = String(params.surfaceId ?? "");
            return model.faces[sid as FaceRef] ? [sid] : [];
          }
          return undefined;
        }
        async faceArea(_f: FaceRef, _t: Model) {
          return 2.5;
        }
      }
      const spec = buildAreaInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new AreaKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "face", id: fid, editable: true }] });
      const res = rt.getSnapshot().lastResponse!;
      expect(res.ok).toBe(true);
      expect(res.data).toBe(2.5);
      expect(isEmptyModelDiff(res.diff)).toBe(false);
      expect(res.diff.anchors?.added?.length).toBe(1);
    });
  });
  describe("@semio-tech/cad-js/core document history", () => {
    it("records modifications and undo/redo applies forward and backwards diffs", () => {
      const g = new Model();
      const h = new DocumentHistory();
      const mesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      const d1 = M.meshFaceModelDiff(mesh, "a");
      const inv1 = applyModelDiff(g, d1);
      const res1: InteractionResponse = { ok: true, errors: [], warnings: [], infos: [], diff: d1, data: null, archiveContext: null };
      h.record({ id: "m1", interactionId: "c", label: "A", result: res1, backwardsDiff: inv1 });
      const d2 = M.meshFaceModelDiff(mesh, "b");
      const inv2 = applyModelDiff(g, d2);
      const res2: InteractionResponse = { ok: true, errors: [], warnings: [], infos: [], diff: d2, data: null, archiveContext: null };
      h.record({ id: "m2", interactionId: "c", label: "B", result: res2, backwardsDiff: inv2 });
      expect(Object.keys(g.faces).length).toBe(2);
      expect(h.entries().map((m) => m.id)).toEqual(["m1", "m2"]);
      const doc = { model: g, nodes: [] as ShapeNode[] };
      h.undo(doc);
      expect(Object.keys(g.faces).length).toBe(1);
      expect(h.entries().map((m) => m.id)).toEqual(["m1"]);
      h.undo(doc);
      expect(Object.keys(g.faces).length).toBe(0);
      h.redo(doc);
      expect(Object.keys(g.faces).length).toBe(1);
      h.redo(doc);
      expect(Object.keys(g.faces).length).toBe(2);
      h.clear();
      expect(h.entries()).toEqual([]);
      expect(h.peekUndo()).toBe(null);
    });
  });
  describe("@semio-tech/cad-js/core measure distance history", () => {
    it("interactionRecordsDocumentHistory skips measure interactions", () => {
      expect(interactionRecordsDocumentHistory("measure.distance")).toBe(false);
      expect(interactionRecordsDocumentHistory("primitive.box")).toBe(true);
    });

    it("does not push readonly measure commits onto document history", async () => {
      class MeasKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async query(name: string, params: Record<string, unknown>) {
          if (name === "surface.resolveFaces") return [String(params.surfaceId ?? "")];
          return undefined;
        }
        async vertexDistance(a: VertexRef, b: VertexRef, t: Model) {
          const pa = t.vertices[String(a)]?.position;
          const pb = t.vertices[String(b)]?.position;
          if (!pa || !pb) return 0;
          return M.vec3Distance(pa, pb);
        }
      }
      const hist = new DocumentHistory();
      const model = new Model();
      const va = "v0" as VertexRef;
      const vb = "v1" as VertexRef;
      model.vertices[va] = { id: va, position: [0, 0, 0] };
      model.vertices[vb] = { id: vb, position: [3, 4, 0] };
      const spec = buildDistanceInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new MeasKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
        history: hist,
      });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: va, editable: true }] });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: vb, editable: true }] });
      expect(hist.peekUndo()).toBe(null);
    });
  });
  describe("@semio-tech/cad-js/core interaction session undo redo", () => {
    it("supports redo after undo during an active interaction and clears redo on new branch", async () => {
      class StubKernel extends BrepjsKernel {
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
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      expect(rt.getSnapshot().state).toBe("first_corner");
      expect(rt.getSnapshot().capabilities.canRedo).toBe(false);
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      rt.undo();
      expect(rt.getSnapshot().state).toBe("first_corner");
      expect(rt.getSnapshot().capabilities.canRedo).toBe(true);
      await rt.send({ kind: "pointer.down", point: [1, 0, 0] as Vec3, modifiers: {} });
      expect(rt.getSnapshot().capabilities.canRedo).toBe(false);
    });
  });
  describe("@semio-tech/cad-js/core undo routing", () => {
    it("uses snapshot undo while active and document history when idle", async () => {
      class StubKernel extends BrepjsKernel {
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
      const g = new Model();
      const mesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      const d0 = M.meshFaceModelDiff(mesh, "seed");
      const inv0 = applyModelDiff(g, d0);
      const hist = new DocumentHistory();
      hist.record({
        id: "seed",
        interactionId: "x",
        label: "seed",
        result: { ok: true, errors: [], warnings: [], infos: [], diff: d0, data: null, archiveContext: null },
        backwardsDiff: inv0,
      });
      expect(Object.keys(g.faces).length).toBe(1);
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: g, nodes: [] },
        history: hist,
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      rt.undo();
      expect(rt.getSnapshot().state).toBe("first_corner");
      expect(Object.keys(g.faces).length).toBe(1);
      rt.undo();
      expect(Object.keys(g.faces).length).toBe(0);
    });
  });
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
