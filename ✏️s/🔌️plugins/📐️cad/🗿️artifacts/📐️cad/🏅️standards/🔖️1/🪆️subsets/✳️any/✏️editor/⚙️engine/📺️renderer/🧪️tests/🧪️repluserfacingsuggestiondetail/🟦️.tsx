type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { CAD_WORLD_FORWARD, CAD_WORLD_UP, COMMITTED_MESH_FACE_OPACITY, Model, THREE, WINDOW_SEARCH_USER, __cadRendererTestKernel, __cadRendererTestRuntime, applyCadWorldCoordinateSystem, applyModelDiff, buildGeometryObjectIndex, buildInteractionReplEngagement, buildInteractionReplSearch, buildPlanarFaceMeshTransfer, canvasHoverKeyForSelectionTarget, collectGeometryEdgeSegments, createSolidTypologyStyleResolver, createSpatialPickEvent, createSpatialPickTargets, createTypologyStyledMaterial, defaultInteractionReplChromeState, defaultInteractionSpatialViewTheme, defaultModelDefinitionId, defaultSpatialPrimitiveToggles, defaultSpatialTypologyTogglesForModelDefinition, emptyMeshTransfer, ensureTypologyObjectFromCreateDiff, filterCommittedMeshesForModelDefinition, filterFootprintBoxPreviewDisplayItems, filterSpatialPickTargets, filterSpatialPickTargetsForActiveView, filterSpatialPickTargetsForEntityFlags, filterSpatialPickTargetsForPrimitiveToggles, filterSpatialPickTargetsForTypologyToggles, geometryBuckets, geometryEntityNurbsPoles, geometryEntityWireSegments, historyEntryArchivesBoxFootprint, interactionSpatialGroundPickPlaneEnabled, isRenderableMeshTransfer, kernelGeometry, listFactoryFaceMeshesForModelDefinition, listStatDefinitionsForModelDefinition, loadSpatialInteraction, mergeInteractionSpatial, modelDefinitionPickTargetKinds, modelDefinitionTypologyIds, nurbsPolesFromEdge, pinnedPickTargetKeys, preciseSpatialKernelMath, projectRayToVerticalZLine, projectRayToYzPlaneAtX, pruneSelectionTargetsForEntityFlags, replDisplayedSelectionTargets, replFilterSuggestions, replHostGeometryPickingEnabled, replInteractionIdOnSpace, replIsQueryTypingTarget, replMergeSelectionPickInView, replNormalizeActionText, replRendererSelectionTargets, replShouldRepeatInteractionOnSpace, replUserFacingSuggestionDetail, resetSpatialSceneColorCache, resolveCommittedMeshMaterialProps, resolveSpatialEntityFlags, resolveSpatialPickTargetsToRender, resolveSpatialSceneVisibility, resolveTypologyStyle, revealedObjectIdsFromPickKeys, solidRef, spatialAutoFitShouldRun, spatialHoverKeyAliases, spatialHoverKeysMatch, spatialPickKindTogglesFromTypologyFilteredTargets, spatialPickTargetKey, spatialSceneColors, spatialSceneKindTogglesForModelDefinition, spatialSelectionTarget, spatialToggleCheckboxState, spatialToggleGroupFill, spatialToggleGroupState, spatialTypologyToggleLabel, statDefinitionAppliesToScope, transformGumballMatrixDiff, typologyStyleCacheKey, typologyStyleToMaterialProps, uiDataLabel, visibleSolidRefsForModelDefinition } = dependencies;
  type AnchorRef = any;
  type DisplayModel = any;
  type EdgeRef = any;
  type InteractionReplEngagementInputs = any;
  type InteractionSnapshot = any;
  type MeshTransfer = any;
  type ModelDiff = any;
  type ObjectRef = any;
  type ResolvedTypologyStyle = any;
  type ShellRef = any;
  type SolidRef = any;
  type SpatialInteractionSelectionByState = any;
  type SpatialPickTarget = any;
  type SpatialRendererSelectionByModel = any;
  type TypologyRef = any;
  type Vec3 = any;
  type VertexRef = any;
  type WireRef = any;

  __cadRendererTestRuntime!.bootstrapCadModules();
  const { BrepjsKernel, preciseSpatialKernelMath: M } = __cadRendererTestKernel!;
  const { describe, it, expect } = vitest;

  describe("replUserFacingSuggestionDetail", () => {
    it("keeps short shortcut keys and drops machine ids", () => {
      expect(replUserFacingSuggestionDetail("c")).toBe("c");
      expect(replUserFacingSuggestionDetail("primitive.box")).toBeUndefined();
      expect(replUserFacingSuggestionDetail("confirm")).toBeUndefined();
      expect(replUserFacingSuggestionDetail("action")).toBeUndefined();
    });
  });

  describe("replNormalizeActionText", () => {
    it("PascalCases engagement action text and strips whitespace in aside REPL mode", () => {
      expect(replNormalizeActionText("set height 5", true)).toBe("SetHeight5");
      expect(replNormalizeActionText("box", true)).toBe("Box");
      expect(replNormalizeActionText("Apply Number", false)).toBe("ApplyNumber");
      expect(replNormalizeActionText("b ", false)).toBe("b");
    });
  });

  describe("replFilterSuggestions", () => {
    const no_operation = () => {};

    it("ranks suggestions without optional detail", () => {
      const rows = replFilterSuggestions("sel", [
        { kind: "selection", key: "sel", label: "Select all", detail: undefined, onRun: no_operation },
        { kind: "action", key: "box", label: "Box", onRun: no_operation },
      ]);
      expect(rows.map((row) => row.key)).toEqual(["sel"]);
    });
  });

  describe("replInteractionIdOnSpace", () => {
    it("returns last finalized id only when idle repeat is allowed", () => {
      expect(replInteractionIdOnSpace("", [], [], "primitive.box", false)).toBeNull();
      expect(replInteractionIdOnSpace("", [], [], "primitive.box", true)).toBe("primitive.box");
    });
  });

  describe("replShouldRepeatInteractionOnSpace", () => {
    it("requires no bound interaction id or active session", () => {
      const event = {
        key: " ",
        ctrlKey: false,
        metaKey: false,
        altKey: false,
        defaultPrevented: false,
        isComposing: false,
        target: document.body,
      };
      expect(
        replShouldRepeatInteractionOnSpace(event, {
          interactionId: "primitive.box",
          interactionActive: false,
          cmdTarget: null,
        }),
      ).toBe(false);
      expect(
        replShouldRepeatInteractionOnSpace(event, {
          interactionId: "",
          interactionActive: false,
          cmdTarget: null,
        }),
      ).toBe(true);
    });
  });

  describe("replIsQueryTypingTarget", () => {
    it("treats text inputs and engagement fields as typing targets", () => {
      const input = document.createElement("input");
      input.type = "text";
      expect(replIsQueryTypingTarget(input)).toBe(true);
      const checkbox = document.createElement("input");
      checkbox.type = "checkbox";
      expect(replIsQueryTypingTarget(checkbox)).toBe(false);
      const engagement = document.createElement("div");
      engagement.setAttribute("data-slot", "engagement");
      const nested = document.createElement("input");
      engagement.append(nested);
      expect(replIsQueryTypingTarget(nested)).toBe(true);
    });
  });

  describe("buildInteractionReplEngagement / buildInteractionReplSearch", () => {
    const baseInputs: InteractionReplEngagementInputs = {
      showEngagement: true,
      boundInteractionSession: true,
      interactionId: "primitive.box",
      state: "first_corner",
      lastResponseOk: null,
      lastResponseErrorCount: 0,
      selectionCount: 0,
      cmdLine: "",
      transitions: [{ eventKind: "confirm", key: "c", label: "Confirm" }],
      interactions: [{ id: "primitive.box", key: "b", label: "Box" }],
      onTransition: () => {},
      onStartInteraction: () => {},
      onInputChange: () => {},
      onInputSubmit: () => {},
    };

    it("returns null when engagement is disabled", () => {
      expect(buildInteractionReplEngagement({ ...baseInputs, showEngagement: false })).toBeNull();
      expect(buildInteractionReplSearch({ ...baseInputs, showEngagement: false })).toBeNull();
    });

    it("lists active session transitions and status", () => {
      const transitionRuns: string[] = [];
      const spec = buildInteractionReplEngagement({
        ...baseInputs,
        selectionCount: 2,
        lastResponseOk: true,
        onTransition: (row) => transitionRuns.push(row.key),
      });
      expect(spec?.sessionActive).toBe(true);
      expect(spec?.options?.[0]?.label).toBe("CConfirm");
      expect(spec?.status?.map((row) => row.content)).toEqual(["Step: First Corner", "2 selected", "OK"]);
      spec?.options?.[0]?.onPress?.();
      expect(transitionRuns).toEqual(["c"]);
    });

    it("lists active session action input and possibles", () => {
      const transitionRuns: string[] = [];
      const spec = buildInteractionReplSearch({
        ...baseInputs,
        onTransition: (row) => transitionRuns.push(row.key),
      });
      expect(spec?.sessionActive).toBe(true);
      expect(spec?.input?.placeholder).toBe(WINDOW_SEARCH_USER.actionPlaceholderActive);
      expect(spec?.possibles?.[0]?.label).toBe("CConfirm");
      spec?.possibles?.[0]?.onSelect?.();
      expect(transitionRuns).toEqual(["c"]);
    });

    it("exposes only an action input while idle so a window can start an interaction", () => {
      const submitted: string[] = [];
      const started: string[] = [];
      const spec = buildInteractionReplSearch({
        ...baseInputs,
        boundInteractionSession: false,
        interactionId: "",
        onInputSubmit: (value) => submitted.push(value),
        onStartInteraction: (id) => started.push(id),
      });
      expect(spec?.input?.placeholder).toBe(WINDOW_SEARCH_USER.actionPlaceholder);
      expect(spec?.possibles?.map((row) => row.label)).toEqual(["Box"]);
      spec?.input?.onSubmit?.("box");
      expect(submitted).toEqual(["box"]);
      spec?.possibles?.[0]?.onSelect?.();
      expect(started).toEqual(["primitive.box"]);
    });

    it("returns null when idle with no startable interactions and nothing selected", () => {
      const idleInputs = { ...baseInputs, boundInteractionSession: false, interactionId: "", interactions: [], selectionCount: 0 };
      expect(buildInteractionReplEngagement(idleInputs)).toBeNull();
      expect(buildInteractionReplSearch(idleInputs)).toBeNull();
    });

    it("keeps selection status visible when idle with no interactions", () => {
      const idleInputs = { ...baseInputs, boundInteractionSession: false, interactionId: "", interactions: [], selectionCount: 3 };
      const spec = buildInteractionReplEngagement(idleInputs);
      expect(spec?.options).toBeUndefined();
      expect(spec?.status?.map((row) => row.content)).toEqual(["3 selected"]);
      expect(buildInteractionReplSearch(idleInputs)).toBeNull();
    });

    it("keeps action input with onRepeatLast when idle without startable interactions", () => {
      const repeated: string[] = [];
      const spec = buildInteractionReplSearch({
        ...baseInputs,
        boundInteractionSession: false,
        interactionId: "",
        interactions: [],
        selectionCount: 0,
        onRepeatLast: () => repeated.push("last"),
      });
      expect(spec?.possibles).toBeUndefined();
      expect(spec?.input?.onRepeatLast).toBeTypeOf("function");
      spec?.input?.onRepeatLast?.();
      expect(repeated).toEqual(["last"]);
    });

    it("summarizes failed responses with error counts", () => {
      const spec = buildInteractionReplEngagement({ ...baseInputs, lastResponseOk: false, lastResponseErrorCount: 2 });
      expect(spec?.status?.some((row) => row.content === "Error (2)")).toBe(true);
    });

    it("forwards engagement control when numeric entry is active", () => {
      const changed: number[] = [];
      const spec = buildInteractionReplEngagement({
        ...baseInputs,
        state: "first_corner_height",
        control: {
          kind: "stepper",
          label: uiDataLabel("Height"),
          value: 2,
          min: 0,
          step: 0.1,
          onChange: (value) => changed.push(value),
        },
      });
      expect(spec?.control?.kind).toBe("stepper");
      spec?.control?.kind === "stepper" && spec.control.onChange?.(4);
      expect(changed).toEqual([4]);
    });
  });

  describe("@semio-tech/cad-js/renderer interaction adapter", () => {
    it("uses CAD Y-forward and Z-up instead of Three.js Y-up defaults", () => {
      const camera = new THREE.PerspectiveCamera();
      const scene = new THREE.Scene();
      applyCadWorldCoordinateSystem(camera, scene);
      expect(CAD_WORLD_FORWARD).toEqual([0, 1, 0]);
      expect(camera.up.toArray()).toEqual(CAD_WORLD_UP);
      expect(scene.up.toArray()).toEqual(CAD_WORLD_UP);
    });

    it("replHostGeometryPickingEnabled follows pickDisabledStates while session is active", () => {
      const spec = loadSpatialInteraction("primitive.box");
      expect(spec).not.toBeNull();
      expect(replHostGeometryPickingEnabled("primitive.box", spec!, "first_corner")).toBe(false);
      expect(replHostGeometryPickingEnabled("primitive.box", spec!, "ready")).toBe(true);
      expect(replHostGeometryPickingEnabled("primitive.box", spec!, "committed")).toBe(true);
      expect(replHostGeometryPickingEnabled("primitive.box", spec!, "idle")).toBe(true);
      expect(replHostGeometryPickingEnabled("", spec!, "first_corner")).toBe(true);
    });

    it("surface.extrudeCrv enables host curve picking and disables ground plane during select_curves_to_extrude", () => {
      const spec = loadSpatialInteraction("surface.extrudeCrv");
      expect(spec).not.toBeNull();
      expect(replHostGeometryPickingEnabled("surface.extrudeCrv", spec!, "select_curves_to_extrude")).toBe(true);
      const snapshot = {
        state: "select_curves_to_extrude",
        spatialInteraction: mergeInteractionSpatial(spec!),
      } satisfies Pick<InteractionSnapshot, "state" | "spatialInteraction">;
      expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true)).toBe(false);
    });

    it("surface.extrudeCrv enables ground pick during extrusion_distance for click finalize", () => {
      const spec = loadSpatialInteraction("surface.extrudeCrv");
      expect(spec).not.toBeNull();
      const snapshot = {
        state: "extrusion_distance",
        spatialInteraction: mergeInteractionSpatial(spec!),
      } satisfies Pick<InteractionSnapshot, "state" | "spatialInteraction">;
      expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true)).toBe(true);
    });

    it("projectRayToVerticalZLine locks XY to origin and allows negative Z", () => {
      const origin: Vec3 = [2, 3, 1];
      const ray = new THREE.Ray(new THREE.Vector3(2, 3, -8), new THREE.Vector3(0, 0, 1));
      const vertical = projectRayToVerticalZLine(ray, origin);
      expect(vertical[0]).toBeCloseTo(2, 4);
      expect(vertical[1]).toBeCloseTo(3, 4);
      expect(vertical[2]).toBeCloseTo(-8, 4);
      const oblique = new THREE.Ray(new THREE.Vector3(0, 0, 3), new THREE.Vector3(1, 0, -1.5).normalize());
      const point = projectRayToVerticalZLine(oblique, origin);
      expect(point[0]).toBeCloseTo(2, 4);
      expect(point[2]).toBeLessThan(1);
    });

    it("projectRayToYzPlaneAtX intersects height-drag wall plane", () => {
      const planeX = 4.06;
      const ray = new THREE.Ray(new THREE.Vector3(0, 0, 5), new THREE.Vector3(1, 0, -0.2).normalize());
      const point = projectRayToYzPlaneAtX(ray, planeX);
      expect(point).not.toBeNull();
      expect(point![0]).toBeCloseTo(planeX, 4);
      expect(point![2]).toBeLessThan(5);
    });

    it("enables spatial ground pick plane during rubber-band states regardless of host selection accept", () => {
      const snapshot = {
        state: "first_corner",
        spatialInteraction: {
          spatialGroundPick: true,
          pickDisabledStates: ["idle", "ready", "committed"],
          groundPointerMoveStates: ["first_corner"],
          heightDragStates: [],
          verticalRodStates: [],
          heightConfirmState: null,
          lengthEntry: [],
          scalarEntry: [],
        },
      } satisfies Pick<InteractionSnapshot, "state" | "spatialInteraction">;
      expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true)).toBe(true);
      expect(interactionSpatialGroundPickPlaneEnabled(snapshot, false)).toBe(false);
    });

    it("creates snap and selection metadata for geometry targets", () => {
      const model = new Model();
      model.vertices.v0 = { id: "v0" as VertexRef, position: [1, 2, 3] };
      const targets = createSpatialPickTargets(model);
      expect(targets).toEqual([{ kind: "vertex", geometryKind: "vertex", id: "v0", point: [1, 2, 3], typologyId: undefined }]);
      expect(createSpatialPickEvent("pointer.down", [9, 9, 9], targets[0]!, { shift: true })).toEqual({
        kind: "pointer.down",
        point: [9, 9, 9],
        modifiers: { shift: true },
        snap: { kind: "vertex", id: "v0", point: [1, 2, 3] },
        selection: { kind: "vertex", id: "v0" },
      });
    });

    it("adds typology object picks for non-shape model definitions", async () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["object-c0"] = {
        id: "object-c0" as ObjectRef,
        typology: "energy.energy.hull" as TypologyRef,
        primitives: { solid: String(cell) },
      };
      const activeModelDefinitionId = "aec.building.energy";
      const editTargets = createSpatialPickTargets(model, defaultModelDefinitionId());
      const objectTargets = createSpatialPickTargets(model, activeModelDefinitionId);
      expect(editTargets.some((t) => t.kind === "vertex")).toBe(true);
      expect(objectTargets.some((t) => t.kind === "object" && !t.geometryKind)).toBe(true);
      expect(objectTargets.some((t) => t.geometryKind === "vertex")).toBe(true);
      const structureTargets = createSpatialPickTargets(model, "aec.building.structure");
      expect(structureTargets.some((t) => t.kind === "face")).toBe(true);
      expect(structureTargets.some((t) => t.kind === "object")).toBe(true);
    });

    it("filterSpatialPickTargetsForActiveView scopes by model definition entity kinds", () => {
      const targets: SpatialPickTarget[] = [
        { kind: "vertex", geometryKind: "vertex", id: "v0", point: [0, 0, 0] },
        { kind: "face", geometryKind: "face", id: "f0", point: [0.5, 0.5, 0.5] },
        { kind: "object", id: "energy.energy.hull", point: [0.5, 0.5, 0.5] },
      ];
      expect(filterSpatialPickTargetsForActiveView(targets, defaultModelDefinitionId()).map(spatialPickTargetKey)).toEqual(["vertex:v0", "face:f0"]);
      expect(filterSpatialPickTargetsForActiveView(targets, "aec.building.energy").map(spatialPickTargetKey)).toEqual(["vertex:v0", "face:f0", "object:energy.energy.hull"]);
      expect(filterSpatialPickTargetsForActiveView(targets, "aec.building.structure").map(spatialPickTargetKey)).toEqual(["vertex:v0", "face:f0", "object:energy.energy.hull"]);
    });

    it("polylineWireSegments tessellates nurbs edge samples for factory wireframe", () => {
      const model = new Model();
      const v0 = { id: "v0" as VertexRef, position: [0, 0, 0] as Vec3 };
      const v1 = { id: "v1" as VertexRef, position: [4, 0, 0] as Vec3 };
      const edge = {
        id: "e0" as EdgeRef,
        vertexIds: [v0.id, v1.id] as [VertexRef, VertexRef],
        curve: {
          kind: "nurbs" as const,
          poles: [
            [0, 0, 0],
            [2, 1, 0],
            [4, 0, 0],
          ] as Vec3[],
          degree: 2,
          through: true,
        },
      };
      const wire = { id: "w0" as WireRef, edgeIds: [edge.id] };
      applyModelDiff(model, { vertices: { added: [v0, v1] }, edges: { added: [edge] }, wires: { added: [wire] } });
      const buckets = geometryBuckets(model);
      const segments = collectGeometryEdgeSegments(buckets);
      const edgeSegments = geometryEntityWireSegments(buckets, "edge", edge.id);
      expect(segments.length).toBeGreaterThan(2);
      expect(edgeSegments.length).toBeGreaterThan(2);
      expect(segments[0]).toEqual(edgeSegments[0]);
    });

    it("ensureTypologyObjectFromCreateDiff binds wire primitive for interpolate curve", () => {
      const model = new Model();
      const diff: ModelDiff = {
        wires: { added: [{ id: "w0" as WireRef, edgeIds: ["e0" as EdgeRef] }] },
        edges: { added: [{ id: "e0" as EdgeRef, vertexIds: ["v0" as VertexRef, "v1" as VertexRef] }] },
      };
      ensureTypologyObjectFromCreateDiff(model, "spatial.shape.curve.interpolate-curve", diff);
      expect(model.objects["spatial.shape.curve.interpolate-curve"]?.primitives.curve).toBe("w0");
    });

    it("geometryEntityNurbsPoles returns interpolation poles for through curves", () => {
      const model = new Model();
      const v0 = { id: "v0" as VertexRef, position: [0, 0, 0] as Vec3 };
      const v1 = { id: "v1" as VertexRef, position: [4, 0, 0] as Vec3 };
      const edge = {
        id: "e0" as EdgeRef,
        vertexIds: [v0.id, v1.id] as [VertexRef, VertexRef],
        curve: {
          kind: "nurbs" as const,
          poles: [
            [0, 0, 0],
            [2, 1, 0],
            [4, 0, 0],
          ] as Vec3[],
          degree: 2,
          through: true,
        },
      };
      const wire = { id: "w0" as WireRef, edgeIds: [edge.id] };
      applyModelDiff(model, { vertices: { added: [v0, v1] }, edges: { added: [edge] }, wires: { added: [wire] } });
      const buckets = geometryBuckets(model);
      expect(geometryEntityNurbsPoles(buckets, "edge", edge.id)).toHaveLength(3);
      expect(geometryEntityNurbsPoles(buckets, "wire", wire.id)).toHaveLength(3);
      expect(nurbsPolesFromEdge(edge)).toEqual(edge.curve!.kind === "nurbs" ? edge.curve.poles : null);
    });

    it("geometryEntityNurbsPoles returns control poles for control-point curves", () => {
      const model = new Model();
      const v0 = { id: "v0" as VertexRef, position: [0, 0, 0] as Vec3 };
      const v1 = { id: "v1" as VertexRef, position: [4, 0, 0] as Vec3 };
      const edge = {
        id: "e0" as EdgeRef,
        vertexIds: [v0.id, v1.id] as [VertexRef, VertexRef],
        curve: {
          kind: "nurbs" as const,
          poles: [
            [0, 0, 0],
            [1, 2, 0],
            [3, 1, 0],
            [4, 0, 0],
          ] as Vec3[],
          degree: 3,
          through: false,
        },
      };
      const wire = { id: "w0" as WireRef, edgeIds: [edge.id] };
      applyModelDiff(model, { vertices: { added: [v0, v1] }, edges: { added: [edge] }, wires: { added: [wire] } });
      const buckets = geometryBuckets(model);
      expect(geometryEntityNurbsPoles(buckets, "edge", edge.id)).toHaveLength(4);
      expect(nurbsPolesFromEdge(edge)).toEqual(edge.curve!.kind === "nurbs" ? edge.curve.poles : null);
    });

    it("resolveSpatialSceneVisibility switches edit wireframe vs committed object mesh", () => {
      expect(resolveSpatialSceneVisibility(defaultModelDefinitionId(), { edge: true, face: true })).toEqual({
        showFactoryWireframe: true,
        showCommittedFaces: true,
        showCommittedEdges: true,
      });
      expect(resolveSpatialSceneVisibility("aec.building.energy", { edge: true, face: true, object: true })).toEqual({
        showFactoryWireframe: true,
        showCommittedFaces: true,
        showCommittedEdges: true,
      });
    });

    it("spatialSceneKindTogglesForModelDefinition keeps committed faces on when typology picks are empty", () => {
      const toggles = spatialSceneKindTogglesForModelDefinition("aec.building.energy", defaultSpatialPrimitiveToggles());
      expect(resolveSpatialSceneVisibility("aec.building.energy", toggles).showCommittedFaces).toBe(true);
    });

    it("concrete forest fixture keeps committed face visibility toggles", async () => {
      const { readFileSync } = await import("node:fs");
      const { resolve } = await import("node:path");
      const { ModelSpace } = await import("../../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts");
      const json = JSON.parse(readFileSync(resolve(source.directoryname, "../../../📚️examples/🖼️assets/🎮️play/🔣️.json"), "utf8"));
      const model = (ModelSpace.fromJSON(json).models[defaultModelDefinitionId()] ?? ModelSpace.fromJSON(json).models[""])!;
      const mdId = defaultModelDefinitionId();
      expect(Object.keys(model.solids).length).toBeGreaterThan(0);
      let targets = createSpatialPickTargets(model, mdId);
      targets = filterSpatialPickTargetsForActiveView(targets, mdId);
      targets = filterSpatialPickTargetsForPrimitiveToggles(targets, defaultSpatialPrimitiveToggles());
      targets = filterSpatialPickTargetsForTypologyToggles(targets, defaultSpatialTypologyTogglesForModelDefinition(mdId), modelDefinitionTypologyIds(mdId));
      const toggles = spatialPickKindTogglesFromTypologyFilteredTargets(mdId, targets);
      expect(resolveSpatialSceneVisibility(mdId, toggles).showCommittedFaces).toBe(true);
      const kernel = new BrepjsKernel();
      await kernel.resetDerivedPipelineForTest();
      const solid = Object.keys(model.solids)[0]! as SolidRef;
      const mesh = await kernel.tessellate(solid, 0.02, model);
      expect(isRenderableMeshTransfer(mesh)).toBe(true);
    });

    it("transformGumballMatrixDiff translates solid selection vertices", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const solidId = Object.keys(model.solids)[0]!;
      const diff = transformGumballMatrixDiff(model, [{ kind: "solid", id: solidId, editable: true }], { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [2, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] });
      applyModelDiff(model, diff);
      for (const v of Object.values(model.vertices)) {
        expect(v.position[0]).toBeGreaterThanOrEqual(1.5);
      }
    });

    it("transformGumballMatrixDiff rotates solid selection vertices", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const solidId = Object.keys(model.solids)[0]!;
      const before = Object.values(model.vertices).map((v) => v.position.join(","));
      const diff = transformGumballMatrixDiff(
        model,
        [{ kind: "solid", id: solidId, editable: true }],
        { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
        { position: [0, 0, 0], quaternion: [0, 0, 0.7071068, 0.7071068], scale: [1, 1, 1] },
      );
      applyModelDiff(model, diff);
      const after = Object.values(model.vertices).map((v) => v.position.join(","));
      expect(after.sort().join("|")).not.toBe(before.sort().join("|"));
    });

    it("transformGumballMatrixDiff scales solid selection vertices", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const solidId = Object.keys(model.solids)[0]!;
      const diff = transformGumballMatrixDiff(model, [{ kind: "solid", id: solidId, editable: true }], { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [2, 2, 2] });
      applyModelDiff(model, diff);
      for (const v of Object.values(model.vertices)) {
        expect(v.position[0]).toBeGreaterThanOrEqual(0);
        expect(v.position[0]).toBeLessThanOrEqual(2.01);
      }
    });

    it("defaultInteractionReplChromeState seeds typology and primitive toggles by default", () => {
      const chrome = defaultInteractionReplChromeState();
      expect(chrome.activeModelDefinitionId).toBe(defaultModelDefinitionId());
      expect(chrome.filterTypologyToggles["spatial.shape.primitive.box"]).toBe(true);
      expect(chrome.filterPrimitiveToggles.vertex).toBe(true);
      expect(chrome.filterPrimitiveToggles.solid).toBe(true);
    });

    it("filterFootprintBoxPreviewDisplayItems removes box-preview when model has solids", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const display: DisplayModel = {
        items: [
          { kind: "box-preview", id: "preview-committed", params: { cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 } },
          { kind: "point", id: "p0", params: { position: [0, 0, 0] } },
        ],
      };
      const filtered = filterFootprintBoxPreviewDisplayItems(display, model);
      expect(filtered.items.some((item) => item.kind === "box-preview")).toBe(false);
      expect(filtered.items.some((item) => item.kind === "point")).toBe(true);
    });

    it("historyEntryArchivesBoxFootprint skips transform and measure interactions", () => {
      expect(historyEntryArchivesBoxFootprint("transform.move")).toBe(false);
      expect(historyEntryArchivesBoxFootprint("transform.copy")).toBe(false);
      expect(historyEntryArchivesBoxFootprint("measure.vertexDistance")).toBe(false);
      expect(historyEntryArchivesBoxFootprint("primitive.box")).toBe(true);
    });

    it("filterSpatialPickTargetsForPrimitiveToggles hides primitive picks by kind", () => {
      const model = new Model();
      model.vertices.v0 = { id: "v0" as VertexRef, position: [0, 0, 0] };
      model.edges.e0 = { id: "e0" as EdgeRef, vertexIds: ["v0" as VertexRef, "v0" as VertexRef], curve: { kind: "line" } };
      const targets = createSpatialPickTargets(model);
      const visible = filterSpatialPickTargetsForPrimitiveToggles(targets, { vertex: false });
      expect(visible.some((row) => row.geometryKind === "vertex")).toBe(false);
      expect(visible.some((row) => row.geometryKind === "edge")).toBe(true);
    });

    it("spatialTypologyToggleLabel uses typology label pascal case", () => {
      expect(spatialTypologyToggleLabel("energy.energy.baseplate", "Base Plate")).toBe("BasePlate");
      expect(spatialTypologyToggleLabel("spatial.shape.primitive.box", "Box")).toBe("Box");
    });

    it("filterSpatialPickTargetsForTypologyToggles hides typology object picks", async () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["hull"] = {
        id: "hull" as ObjectRef,
        typology: "energy.energy.hull" as TypologyRef,
        primitives: { solid: String(cell) },
      };
      const targets = createSpatialPickTargets(model, "aec.building.energy");
      const typologyIds = modelDefinitionTypologyIds("aec.building.energy");
      const visible = filterSpatialPickTargetsForTypologyToggles(targets, { "energy.energy.hull": false }, typologyIds);
      expect(visible.some((row) => row.typologyId === "energy.energy.hull")).toBe(false);
    });

    it("scopes displayed selection to activeModelDefinitionId", () => {
      const rendererByModel: SpatialRendererSelectionByModel = {
        [defaultModelDefinitionId()]: [{ kind: "face", id: "f0", editable: true }],
        "aec.building.energy": [
          { kind: "face", id: "f0", editable: true },
          { kind: "object", id: "o0", editable: false },
        ],
      };
      expect(replDisplayedSelectionTargets(false, defaultModelDefinitionId(), "idle", rendererByModel, {})).toEqual([{ kind: "face", id: "f0", editable: true }]);
      expect(replDisplayedSelectionTargets(false, "aec.building.energy", "idle", rendererByModel, {})).toEqual([
        { kind: "face", id: "f0", editable: true },
        { kind: "object", id: "o0", editable: false },
      ]);
      expect(replDisplayedSelectionTargets(false, "aec.building.structure", "idle", rendererByModel, {})).toEqual([]);
    });

    it("creates anchor and shell pick targets for spatial.shape geometry", () => {
      const model = new Model();
      model.anchors["a0"] = { id: "a0" as AnchorRef, position: [0, 0, 0], attachment: { kind: "vertex", id: "v0" as VertexRef } };
      model.vertices["v0"] = { id: "v0" as VertexRef, position: [0, 0, 0] };
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      const faceId = Object.values(model.faces)[0]!.id;
      model.shells["sh0"] = { id: "sh0" as ShellRef, faceIds: [faceId] };
      const targets = createSpatialPickTargets(model, defaultModelDefinitionId());
      expect(targets.some((t) => t.geometryKind === "anchor")).toBe(true);
      expect(targets.some((t) => t.geometryKind === "shell")).toBe(true);
    });

    it("modelDefinitionPickTargetKinds maps primitive entity kinds to pick toggles", () => {
      expect([...modelDefinitionPickTargetKinds(defaultModelDefinitionId())].sort()).toEqual(["edge", "face", "object", "vertex"]);
      expect([...modelDefinitionPickTargetKinds("aec.building.structure")].sort()).toEqual(["edge", "face", "object", "vertex"]);
    });

    it("merges picks within active model definition without clearing other models", () => {
      const rendererByModel: SpatialRendererSelectionByModel = {
        [defaultModelDefinitionId()]: [{ kind: "wire", id: "w0", editable: true }],
        "aec.building.energy": [{ kind: "object", id: "o0", editable: false }],
      };
      expect(replMergeSelectionPickInView(false, defaultModelDefinitionId(), "idle", rendererByModel, {}, [{ kind: "wire", id: "w1", editable: true }], {})).toEqual([{ kind: "wire", id: "w1", editable: true }]);
      expect(replRendererSelectionTargets(rendererByModel, "aec.building.energy")).toEqual([{ kind: "object", id: "o0", editable: false }]);
    });

    it("clears selection on empty background pick in default modifier mode", () => {
      const rendererByModel: SpatialRendererSelectionByModel = {
        [defaultModelDefinitionId()]: [{ kind: "wire", id: "w0", editable: true }],
      };
      expect(replMergeSelectionPickInView(false, defaultModelDefinitionId(), "idle", rendererByModel, {}, [], {})).toEqual([]);
      expect(replMergeSelectionPickInView(false, defaultModelDefinitionId(), "idle", rendererByModel, {}, [], { shift: true })).toEqual([{ kind: "wire", id: "w0", editable: true }]);
    });

    it("maps selection target keys to pick target keys for highlights", () => {
      const keys = pinnedPickTargetKeys(new Set(["shell:sh0" as string]));
      expect(keys.has("shell:sh0")).toBe(true);
      expect(keys.has("face:sh0")).toBe(true);
    });

    it("spatialHoverKeyAliases links object and solid geometry pick keys", () => {
      expect(spatialHoverKeyAliases("solid:foo").has("object:foo")).toBe(true);
      expect(spatialHoverKeyAliases("object:foo").has("solid:foo")).toBe(true);
      expect(spatialHoverKeysMatch("object:foo", "solid:foo")).toBe(true);
    });

    it("canvasHoverKeyForSelectionTarget maps primitive picks to typology object hover keys", () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["object-c0"] = {
        id: "object-c0" as ObjectRef,
        typology: "energy.energy.hull" as TypologyRef,
        primitives: { solid: String(cell) },
      };
      expect(canvasHoverKeyForSelectionTarget(model, "aec.building.energy", { kind: "solid", id: String(cell), editable: true })).toBe("object:object-c0");
      expect(canvasHoverKeyForSelectionTarget(model, "aec.building.energy", { kind: "object", id: "object-c0", editable: true })).toBe("object:object-c0");
    });

    it("resolveCommittedMeshMaterialProps resolves selection, hover, and style fallback materials", () => {
      const palette = spatialSceneColors();
      const solidId = solidRef("s0");

      // Test default state
      const defaultProps = resolveCommittedMeshMaterialProps(undefined, undefined, solidId, null, null, null);
      expect(defaultProps.color).toBe(palette.committed);
      expect(defaultProps.emissive).toBe(palette.committedEmissive);
      expect(defaultProps.opacity).toBe(COMMITTED_MESH_FACE_OPACITY);

      // Test hovered state (via targetKey solid:s0 matching object:s0 / solid:s0)
      const hoveredPropsObj = resolveCommittedMeshMaterialProps(undefined, undefined, solidId, "object:s0", null, null);
      expect(hoveredPropsObj.color).toBe(palette.hovered);
      expect(hoveredPropsObj.emissive).toBe(palette.hoveredEmissive);
      expect(hoveredPropsObj.opacity).toBe(0.28);

      const hoveredPropsSolid = resolveCommittedMeshMaterialProps(undefined, undefined, solidId, "solid:s0", null, null);
      expect(hoveredPropsSolid.color).toBe(palette.hovered);

      // Test selected state
      const selectedProps = resolveCommittedMeshMaterialProps(undefined, undefined, solidId, null, "object:s0", null);
      expect(selectedProps.color).toBe(palette.selected);
      expect(selectedProps.emissive).toBe(palette.selectedEmissive);
      expect(selectedProps.opacity).toBe(0.34);

      // Test selected keys set state
      const selectedKeysProps = resolveCommittedMeshMaterialProps(undefined, undefined, solidId, null, null, new Set(["solid:s0"]));
      expect(selectedKeysProps.color).toBe(palette.selected);

      // Test custom style overrides when not selected/hovered
      const style: ResolvedTypologyStyle = {
        color: "#ff00ff",
        edgeColor: "#00ffff",
        opacity: 0.5,
        pattern: { kind: "none", direction: 0, spacing: 0.35, lineWidth: 0.03, color: "#000000" },
      };
      const styleProps = resolveCommittedMeshMaterialProps(style, undefined, solidId, null, null, null);
      expect(styleProps.color).toBe("#ff00ff");
      expect(styleProps.opacity).toBe(0.5);
    });

    it("keeps interaction selection isolated per state", () => {
      const interactionByState: SpatialInteractionSelectionByState = {
        first_corner: [{ kind: "vertex", id: "v0", editable: true }],
        second_corner: [{ kind: "vertex", id: "v1", editable: true }],
      };
      expect(replDisplayedSelectionTargets(true, defaultModelDefinitionId(), "first_corner", {}, interactionByState)).toEqual([{ kind: "vertex", id: "v0", editable: true }]);
      expect(replMergeSelectionPickInView(true, defaultModelDefinitionId(), "second_corner", {}, interactionByState, [{ kind: "edge", id: "e0", editable: true }], { shift: true })).toEqual([
        { kind: "vertex", id: "v1", editable: true },
        { kind: "edge", id: "e0", editable: true },
      ]);
    });

    it("spatialToggleGroupState reports all, none, and partial chrome groups", () => {
      expect(spatialToggleGroupState(["a", "b"], { a: true, b: true })).toBe("all");
      expect(spatialToggleGroupState(["a", "b"], { a: false, b: false })).toBe("none");
      expect(spatialToggleGroupState(["a", "b"], { a: true, b: false })).toBe("partial");
      expect(spatialToggleGroupFill(["a", "b"], true)).toEqual({ a: true, b: true });
      expect(spatialToggleGroupFill(["a", "b"], false)).toEqual({ a: false, b: false });
      expect(spatialToggleCheckboxState("all")).toBe(true);
      expect(spatialToggleCheckboxState("none")).toBe(false);
      expect(spatialToggleCheckboxState("partial")).toBe("indeterminate");
    });

    it("filterSpatialPickTargets matches primitive geometryKind in selection accept", () => {
      const targets: SpatialPickTarget[] = [
        { kind: "face", geometryKind: "shell", id: "sh0", point: [0, 0, 0] },
        { kind: "vertex", geometryKind: "anchor", id: "a0", point: [1, 0, 0] },
      ];
      expect(filterSpatialPickTargets(targets, ["shell"], {}).map(spatialPickTargetKey)).toEqual(["face:sh0"]);
      expect(filterSpatialPickTargets(targets, ["anchor"], {}).map(spatialPickTargetKey)).toEqual(["vertex:a0"]);
    });

    it("filterSpatialPickTargetsForEntityFlags excludes hidden and locked targets", () => {
      const targets: SpatialPickTarget[] = [
        { kind: "object", id: "visible", point: [0, 0, 0] },
        { kind: "object", id: "hidden", point: [0, 0, 0] },
        { kind: "face", geometryKind: "face", id: "locked", point: [0, 0, 0] },
      ];
      const flagsForId = (id: string) => ({ ...(id === "hidden" ? { hidden: true } : {}), ...(id === "locked" ? { locked: true } : {}) });
      expect(filterSpatialPickTargetsForEntityFlags(targets, flagsForId).map((row) => row.id)).toEqual(["visible"]);
      expect(pruneSelectionTargetsForEntityFlags(targets.map(spatialSelectionTarget), flagsForId).map((row) => row.id)).toEqual(["visible"]);
    });

    it("resolveSpatialPickTargetsToRender skips hidden unless pinned", () => {
      const targets: SpatialPickTarget[] = [
        { kind: "object", id: "hidden", point: [0, 0, 0] },
        { kind: "object", id: "visible", point: [1, 0, 0] },
      ];
      const flagsForId = (id: string) => ({ ...(id === "hidden" ? { hidden: true } : {}) });
      expect(resolveSpatialPickTargetsToRender(targets, {}, new Set(), flagsForId).map((row) => row.id)).toEqual(["visible"]);
      expect(
        resolveSpatialPickTargetsToRender(targets, {}, new Set(["object:hidden"]), flagsForId)
          .map((row) => row.id)
          .sort(),
      ).toEqual(["hidden", "visible"]);
    });

    it("resolveSpatialPickTargetsToRender draws all enabled kinds", () => {
      const targets: SpatialPickTarget[] = [
        { kind: "vertex", geometryKind: "vertex", id: "v0", point: [0, 0, 0] },
        {
          kind: "edge",
          geometryKind: "edge",
          id: "e0",
          point: [0, 0, 0],
          points: [
            [0, 0, 0],
            [1, 0, 0],
          ],
        },
      ];
      expect(resolveSpatialPickTargetsToRender(targets, { edge: false }).map(spatialPickTargetKey)).toEqual(["vertex:v0"]);
      expect(resolveSpatialPickTargetsToRender(targets, {}).map(spatialPickTargetKey).sort()).toEqual(["edge:e0", "vertex:v0"]);
    });

    it("resolveSpatialPickTargetsToRender draws factory primitives without hover reveal", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const solidId = Object.keys(model.solids)[0]!;
      model.objects["box-obj"] = {
        id: "box-obj" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      const targets = createSpatialPickTargets(model, defaultModelDefinitionId());
      expect(resolveSpatialPickTargetsToRender(targets, {}, new Set(), () => ({})).some((row) => row.kind === "vertex")).toBe(true);
    });

    it("revealedObjectIdsFromPickKeys expands solid and member picks to the owning object", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const solidId = Object.keys(model.solids)[0]!;
      const vertexId = Object.keys(model.vertices)[0]!;
      model.objects["box-obj"] = {
        id: "box-obj" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      const objectIndex = buildGeometryObjectIndex(model, defaultModelDefinitionId());
      expect([...revealedObjectIdsFromPickKeys(objectIndex, `vertex:${vertexId}`, new Set())]).toEqual(["box-obj"]);
      expect([...revealedObjectIdsFromPickKeys(objectIndex, `object:${solidId}`, new Set())]).toEqual(["box-obj"]);
    });

    it("spatialSceneColors exposes a single product-aligned palette", () => {
      resetSpatialSceneColorCache();
      const palette = spatialSceneColors();
      expect(palette.accent).toBeTruthy();
      expect(palette.construction).toBe(palette.accent);
      expect(palette.selected).toBeTruthy();
      expect(palette.groundPlane).not.toBe(palette.accent);
      resetSpatialSceneColorCache();
    });

    it("visibleSolidRefsForModelDefinition scopes building solids to visible objects only", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("solid-a")));
      applyModelDiff(model, M.boxModelDiff({ cornerA: [2, 0, 0], cornerB: [3, 1, 0], height: 1 }, solidRef("solid-b")));
      model.objects["obj-a"] = { id: "obj-a" as ObjectRef, typology: "building.building.column" as TypologyRef, primitives: { solid: "solid-a" } };
      model.objects["obj-b"] = { id: "obj-b" as ObjectRef, typology: "building.building.beam" as TypologyRef, primitives: { solid: "solid-b" } };
      model.objects["orphan"] = { id: "orphan" as ObjectRef, typology: "spatial.shape.kernel.solid" as TypologyRef, primitives: { solid: "solid-a" } };
      const flagsForId = (id: string) => model.getEntityFlags(id);
      expect([...visibleSolidRefsForModelDefinition(model, "aec.building", { flagsForId })].sort()).toEqual(["solid-a", "solid-b"]);
      model.setEntityFlag("obj-a", "hidden", true);
      expect([...visibleSolidRefsForModelDefinition(model, "aec.building", { flagsForId })].sort()).toEqual(["solid-b"]);
      model.setEntityFlag("obj-b", "hidden", true);
      expect([...visibleSolidRefsForModelDefinition(model, "aec.building", { flagsForId })]).toEqual([]);
    });

    it("buildPlanarFaceMeshTransfer shades typology surface primitives for energy models", () => {
      const model = new Model();
      const v0 = { id: "v0" as VertexRef, position: [0, 0, 0] as Vec3 };
      const v1 = { id: "v1" as VertexRef, position: [1, 0, 0] as Vec3 };
      const v2 = { id: "v2" as VertexRef, position: [0, 1, 0] as Vec3 };
      const e0 = { id: "e0" as EdgeRef, vertexIds: [v0.id, v1.id] as [VertexRef, VertexRef] };
      const e1 = { id: "e1" as EdgeRef, vertexIds: [v1.id, v2.id] as [VertexRef, VertexRef] };
      const e2 = { id: "e2" as EdgeRef, vertexIds: [v2.id, v0.id] as [VertexRef, VertexRef] };
      const wireId = "w0" as WireRef;
      const faceId = "f0" as kernelGeometry.FaceRef;
      applyModelDiff(model, {
        vertices: { added: [v0, v1, v2] },
        edges: { added: [e0, e1, e2] },
        wires: { added: [{ id: wireId, edgeIds: [e0.id, e1.id, e2.id] }] },
        faces: {
          added: [
            {
              id: faceId,
              wireIds: [wireId],
              surface: { kind: "plane", origin: [0, 0, 0], normal: [0, 0, 1] },
            },
          ],
        },
      });
      model.objects["baseplate"] = {
        id: "baseplate" as ObjectRef,
        typology: "energy.energy.baseplate" as TypologyRef,
        primitives: { surface: faceId },
      };
      const mesh = buildPlanarFaceMeshTransfer(model, faceId);
      expect(mesh).not.toBeNull();
      expect(isRenderableMeshTransfer(mesh!)).toBe(true);
      expect(mesh!.index.length).toBe(3);
      const rows = listFactoryFaceMeshesForModelDefinition(model, "aec.building.energy");
      expect(rows).toHaveLength(1);
      expect(rows[0]?.faceId).toBe(faceId);
      expect(rows[0]?.style?.color).toBeTruthy();
    });

    it("filterCommittedMeshesForModelDefinition drops meshes when all objects are hidden", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("solid-a")));
      model.objects["obj-a"] = { id: "obj-a" as ObjectRef, typology: "building.building.column" as TypologyRef, primitives: { solid: "solid-a" } };
      model.setEntityFlag("obj-a", "hidden", true);
      const mesh: MeshTransfer = {
        ...emptyMeshTransfer(),
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 0, 0, 0, 0, 0, 0, 0]),
        index: new Uint32Array([0, 1, 2]),
      };
      const filtered = filterCommittedMeshesForModelDefinition(model, "aec.building", [{ solid: solidRef("solid-a"), mesh }], {
        flagsForId: (id) => model.getEntityFlags(id),
      });
      expect(filtered).toEqual([]);
    });

    it("resolveSpatialEntityFlags inherits hidden state from owning object", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("solid-a")));
      const solidId = Object.keys(model.solids)[0]!;
      const faceId = Object.keys(model.faces)[0]!;
      model.objects["obj-a"] = { id: "obj-a" as ObjectRef, typology: "building.building.column" as TypologyRef, primitives: { solid: solidId } };
      model.setEntityFlag("obj-a", "hidden", true);
      expect(resolveSpatialEntityFlags(model, "aec.building", faceId).hidden).toBe(true);
      expect(resolveSpatialEntityFlags(model, "aec.building", solidId).hidden).toBe(true);
    });

    it("typologyStyleToMaterialProps and typologyStyleCacheKey reflect resolved style", () => {
      const style = resolveTypologyStyle("structure.structure.onewayreinforcedconcreteslab");
      const props = typologyStyleToMaterialProps(style);
      expect(props.color).toBe("#8B7355");
      expect(props.opacity).toBe(0.78);
      expect(typologyStyleCacheKey(style)).toContain("hatch");
    });

    it("createTypologyStyledMaterial injects self-contained world position for patterns", () => {
      const material = createTypologyStyledMaterial({
        color: "#c0ffee",
        edgeColor: "#102030",
        opacity: 0.8,
        pattern: { kind: "hatch", direction: 30, spacing: 0.4, lineWidth: 0.02, color: "#203040" },
      });
      const shader = {
        uniforms: {},
        vertexShader: "#include <common>\nvoid main() {\nvec3 transformed = vec3(position);\n#include <worldpos_vertex>\n}",
        fragmentShader: "#include <common>\nvoid main() {\nvec3 outgoingLight = vec3(1.0);\nvec3 normal = vec3(0.0, 0.0, 1.0);\n#include <output_fragment>\n}",
      };
      material.onBeforeCompile(shader as never, {} as never);
      expect(shader.vertexShader).toContain("vec4 typologyWorldPosition = vec4(transformed, 1.0);");
      expect(shader.vertexShader).toContain("typologyWorldPosition = instanceMatrix * typologyWorldPosition;");
      expect(shader.vertexShader).not.toContain("vTypologyWorldPos = worldPosition.xyz;");
    });

    it("createSolidTypologyStyleResolver maps solids to their object typology style", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("slab-solid")));
      const solidId = Object.keys(model.solids)[0]!;
      model.objects["slab-obj"] = {
        id: "slab-obj" as ObjectRef,
        typology: "structure.structure.onewayreinforcedconcreteslab" as TypologyRef,
        primitives: { solid: solidId },
      };
      const resolveStyle = createSolidTypologyStyleResolver(model, "aec.building.structure.classic");
      const style = resolveStyle(solidId as SolidRef);
      expect(style?.pattern.kind).toBe("hatch");
      expect(style?.color).toBe("#8B7355");
    });

    it("defaultInteractionSpatialViewTheme hides the factory ground plane tint", () => {
      expect(defaultInteractionSpatialViewTheme.groundPlaneOpacity).toBe(0);
    });

    it("spatialAutoFitShouldRun keeps initial fit pane-local across mesh reloads", () => {
      expect(spatialAutoFitShouldRun("initial", "mesh-key", "mesh-key", true)).toBe(false);
      expect(spatialAutoFitShouldRun("changes", "mesh-key", "mesh-key", true)).toBe(false);
      expect(spatialAutoFitShouldRun("changes", "next-key", "mesh-key", true)).toBe(true);
    });
  });

  describe("ModelStatsPane", () => {
    it("resolves shape stat labels for the active model definition", () => {
      const definitions = listStatDefinitionsForModelDefinition(defaultModelDefinitionId());
      const geometry = definitions.find((row) => row.id === "spatial.shape.geometry");
      expect(geometry?.label).toBe("Geometry KPIs");
      expect(geometry?.outputs.map((row) => row.label)).toContain("Total volume");
      expect(statDefinitionAppliesToScope(geometry!, "model")).toBe(true);
    });
  });

}
