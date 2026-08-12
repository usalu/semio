// #region 🧲️Header
/** @emoji 🧭️ `@semio-tech/cad-js` — CAD domain module facet. See `cad/AGENTS.md`. */
import { ephemeralBox, ephemeralMap, ephemeralWeakMap } from "@semio-tech/framework";
import type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 } from "@semio-tech/kernel-3d-js";
import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/kernel-3d-js";
// #endregion 🧲️Header


import { AttributeTable, EdgeRef, Expr, FaceRef, InteractionEvent, InteractionSpec, Model, ModelEntityKind, ModelSpace, ModelSpaceJson, ObjectRef, SelectionEvent, SelectionSpec, SelectionTarget, SolidRef, TypologyRef, VertexRef, WireRef, actionAvailableInModelDefinition, actionOwnedByModelDefinition, applyTransformation, buildModelPrimitiveDocument, compileInteraction, computeStat, countViewObjectsForModelDefinition, defaultModelDefinitionId, deletableObjectIdsFromSelection, deleteObjectsFromModel, derivePropertyValue, evalExpr, evalGuard, expandSelectionTargetsForAccept, formatStatOutputValue, hashModelPrimitives, hashModelVertices, hashSolidRecord, hashVertexPosition, isCallableOnlyInteraction, isFinalInteractionState, isShapeModelDefinition, listApplicablePropertyDefinitionsForModelDefinition, listAttributeDefinitionsForModelDefinitionEntity, listModelDefinitionAttributeDefinitions, listModelDefinitionManifests, listModelDefinitionPropertyDefinitions, listModelDefinitionStatDefinitions, listModelDefinitionTypologies, listModelObjectsForModelDefinition, listPropertyDefinitionsForModelDefinition, listSelectionOperationsForModelDefinition, listSpatialInteractionsForModelDefinition, listStatDefinitionsForModelDefinition, listTransformationsFromModelDefinition, listTransformationsIntoModelDefinition, listTypologiesForModelDefinition, loadAttributeDefinition, loadPropertyDefinition, loadStatDefinition, loadTransformation, loadTypology, mergeInteractionCallOutputs, modelDefinitionIdForInteraction, objectMatchesTypologyPrimitives, objectsForStatCompute, parseInteractionSpec, parseModelJson, resolveModelDefinitionScope, resolveTypologyStyle, selectionEventMatches, validateAttributeValue } from "../📐️geometry/🟦️component.ts";
import { ensureTypologyObjectFromCreateDiff, listConstructableTypologiesForModelDefinition, typologyConstructAssetIds, typologyConstructCommitActionForMode, typologyConstructKitByInteraction, typologyConstructModeActionIds, typologyHasNativeConstructKit, typologyIdForInteractionCommit } from "../🧬️typology/🟦️component.ts";
import { EMPTY_MODEL_DIFF, ModelDiff, SpatialKernel, SpatialPreviewKernel, appendCommittedMeshFaceToModel, applyModelDiff, isEmptyModelDiff } from "../🗺️spatial/🟦️component.ts";
import { CAD_GUMBALL_HIDDEN, SelectionOperationInteractionDef, cadGumballConfigVisible, clampPointAlongDirection, collectGeometrySelectionTargets, executeSelectionApply, interactionControlForState, interactionLengthEntryForState, interactionLengthEntryLiveDistance, interactionNumericEntryCommitEvent, interactionNumericEntryExplicitLockValue, interactionNumericEntryLockedValue, interactionRecordsDocumentHistory, interactionStepFinalizeEvent, listActionDefs, listModelDefinitionActionSpecs, modelDefinitionActionRegistry, parseActionSpec, pureTsStateEngineProvider, readInteractionContextVec3, resolveDisplay, runRegisteredAction, runSelectionApply, selectionTargetsFromActionResult, selectionTargetsFromContext, selectionTargetsPointTransformDiff } from "../🎬️actions/🟦️component.ts";
import { DocumentHistory, InteractionResponse, InteractionRuntime, InteractionSnapshot, ShapeNode, buildAreaInteractionSpec, buildBoxInteractionSpec, buildDistanceInteractionSpec, createInteractionRuntime, listInteractionSpecs, loadSpatialInteraction, modelDefinitionInteractionRegistry, runSelectionOperationInteraction, selectionApplyParamsForInteraction, selectionSeedTargetsForOperation } from "../📄️document/🟦️component.ts";



// #region 📦️🧪️tests
// #region 🧪️Tests
const __spatialCoreTestRuntime = import.meta.vitest ? await import("../🏃️runtime/🟦️component.ts") : null;
const __spatialCoreTestKernel = import.meta.vitest ? await import("../📐️brepjs/🟦️component.ts") : null;
//#region 🧪️InteractionE2EFixtures
const CAD_E2E_LOOM_MODEL_SPACE_JSON =
  '{"schema":"spatial.modelspace","revision":1,"models":[{"id":"spatial.shape","model":{"schema":"spatial.model","revision":2,"objects":[{"id":"object-edge-e11","typology":"spatial.shape.kernel.edge","primitives":[{"kind":"vertex","id":"v4","position":[1,1,1.2]},{"kind":"vertex","id":"v9","position":[2.5,0.3,2.4]},{"kind":"curve","slot":"edge","id":"e11","vertexIds":["v4","v9"]}]},{"id":"object-void-cap","typology":"spatial.shape.primitive.box","primitives":[{"kind":"vertex","id":"v0","position":[0,0,0]},{"kind":"vertex","id":"v1","position":[5,0,0]},{"kind":"vertex","id":"v2","position":[5,4,0]},{"kind":"vertex","id":"v3","position":[0,4,0]},{"kind":"curve","id":"e0","vertexIds":["v0","v1"]},{"kind":"curve","id":"e1","vertexIds":["v1","v2"]},{"kind":"curve","id":"e2","vertexIds":["v2","v3"]},{"kind":"curve","id":"e3","vertexIds":["v3","v0"]},{"kind":"curve","id":"w-deck","edgeIds":["e0","e1","e2","e3"]},{"kind":"surface","id":"stub-surface","wireIds":["w-deck"]},{"kind":"shell","id":"shell-deck","faceIds":["stub-surface"]},{"kind":"solid","slot":"solid","id":"void-cap","shellIds":["shell-deck"]}]},{"id":"object-wire-rail-upper","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"v10","position":[4.5,1.8,2.4]},{"kind":"vertex","id":"v11","position":[3.8,3.5,2.4]},{"kind":"vertex","id":"v9","position":[2.5,0.3,2.4]},{"kind":"curve","id":"e10","vertexIds":["v10","v11"]},{"kind":"curve","id":"e9","vertexIds":["v9","v10"]},{"kind":"curve","slot":"wire","id":"rail-upper","edgeIds":["e9","e10"]}]},{"id":"object-wire-stub-wire","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"v4","position":[1,1,1.2]},{"kind":"vertex","id":"v5","position":[3,2.2,1.2]},{"kind":"vertex","id":"v6","position":[4.2,0.8,1.2]},{"kind":"vertex","id":"v7","position":[2.5,3.1,1.2]},{"kind":"vertex","id":"v8","position":[0.8,2.4,1.2]},{"kind":"curve","id":"e4","vertexIds":["v4","v5"]},{"kind":"curve","id":"e5","vertexIds":["v5","v6"]},{"kind":"curve","id":"e6","vertexIds":["v6","v7"]},{"kind":"curve","id":"e7","vertexIds":["v7","v8"]},{"kind":"curve","id":"e8","vertexIds":["v8","v4"]},{"kind":"curve","slot":"wire","id":"stub-wire","edgeIds":["e4","e5","e6","e7","e8"]}]}]}}]}';
const CAD_E2E_ROUTES_MODEL_SPACE_JSON =
  '{"schema":"spatial.modelspace","revision":1,"models":[{"id":"spatial.shape","model":{"schema":"spatial.model","revision":1,"objects":[{"id":"object-wire-orbit-a","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r10","position":[6,0,0.8]},{"kind":"vertex","id":"r11","position":[7.4,0.6,0.8]},{"kind":"vertex","id":"r12","position":[8.8,0.2,0.8]},{"kind":"vertex","id":"r13","position":[9.9,1.1,0.8]},{"kind":"vertex","id":"r14","position":[10.2,2.6,0.8]},{"kind":"vertex","id":"r15","position":[9.4,3.9,0.8]},{"kind":"vertex","id":"r16","position":[7.8,4.4,0.8]},{"kind":"vertex","id":"r17","position":[6.2,4.1,0.8]},{"kind":"curve","id":"re10","vertexIds":["r10","r11"]},{"kind":"curve","id":"re11","vertexIds":["r11","r12"]},{"kind":"curve","id":"re12","vertexIds":["r12","r13"]},{"kind":"curve","id":"re13","vertexIds":["r13","r14"]},{"kind":"curve","id":"re14","vertexIds":["r14","r15"]},{"kind":"curve","id":"re15","vertexIds":["r15","r16"]},{"kind":"curve","id":"re16","vertexIds":["r16","r17"]},{"kind":"curve","id":"re17","vertexIds":["r17","r10"]},{"kind":"curve","slot":"wire","id":"orbit-a","edgeIds":["re10","re11","re12","re13","re14","re15","re16","re17"]}]},{"id":"object-wire-spine-b","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r18","position":[2,6,1.6]},{"kind":"vertex","id":"r19","position":[3.5,6.8,1.6]},{"kind":"vertex","id":"r20","position":[5.2,6.5,1.6]},{"kind":"vertex","id":"r21","position":[6.8,7.2,1.6]},{"kind":"vertex","id":"r22","position":[7.5,8.4,1.6]},{"kind":"vertex","id":"r23","position":[6.1,9.1,1.6]},{"kind":"curve","id":"re18","vertexIds":["r18","r19"]},{"kind":"curve","id":"re19","vertexIds":["r19","r20"]},{"kind":"curve","id":"re20","vertexIds":["r20","r21"]},{"kind":"curve","id":"re21","vertexIds":["r21","r22"]},{"kind":"curve","id":"re22","vertexIds":["r22","r23"]},{"kind":"curve","slot":"wire","id":"spine-b","edgeIds":["re18","re19","re20","re21","re22"]}]},{"id":"object-wire-stub-wire","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r0","position":[0,0,0]},{"kind":"vertex","id":"r1","position":[1.2,0.4,0]},{"kind":"vertex","id":"r2","position":[2.6,0.1,0]},{"kind":"vertex","id":"r3","position":[3.8,0.9,0]},{"kind":"vertex","id":"r4","position":[4.5,2.1,0]},{"kind":"vertex","id":"r5","position":[4.2,3.5,0]},{"kind":"vertex","id":"r6","position":[3.1,4.2,0]},{"kind":"vertex","id":"r7","position":[1.5,4.5,0]},{"kind":"vertex","id":"r8","position":[0.2,3.8,0]},{"kind":"vertex","id":"r9","position":[-0.4,2.2,0]},{"kind":"curve","id":"re0","vertexIds":["r0","r1"]},{"kind":"curve","id":"re1","vertexIds":["r1","r2"]},{"kind":"curve","id":"re2","vertexIds":["r2","r3"]},{"kind":"curve","id":"re3","vertexIds":["r3","r4"]},{"kind":"curve","id":"re4","vertexIds":["r4","r5"]},{"kind":"curve","id":"re5","vertexIds":["r5","r6"]},{"kind":"curve","id":"re6","vertexIds":["r6","r7"]},{"kind":"curve","id":"re7","vertexIds":["r7","r8"]},{"kind":"curve","id":"re8","vertexIds":["r8","r9"]},{"kind":"curve","id":"re9","vertexIds":["r9","r0"]},{"kind":"curve","slot":"wire","id":"stub-wire","edgeIds":["re0","re1","re2","re3","re4","re5","re6","re7","re8","re9"]}]}]}}]}';
const CAD_E2E_BUILDING_BOOLEAN_MODEL_SPACE_JSON =
  '{"schema":"spatial.modelspace","revision":1,"models":[{"id":"spatial.shape","model":{"schema":"spatial.model","revision":1,"objects":[{"id":"object-small-building-cell-123052045","typology":"spatial.shape.primitive.box","primitives":[{"kind":"vertex","id":"small-building-vertex-1027259450","position":[-64.506666,41.273333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-1412665337","position":[-18.586667,41.273333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-200778251","position":[-18.586667,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-354458280","position":[-18.586667,3.553333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-560960189","position":[-64.506666,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-566043311","position":[-64.506666,41.273333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-584100920","position":[-64.506666,3.553333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-834828749","position":[-18.586667,41.273333,-9.407222]},{"kind":"curve","id":"small-building-edge-1660152326","vertexIds":["small-building-vertex-354458280","small-building-vertex-1412665337"]},{"kind":"curve","id":"small-building-edge-1943812986","vertexIds":["small-building-vertex-566043311","small-building-vertex-584100920"]},{"kind":"curve","id":"small-building-edge-2004107109","vertexIds":["small-building-vertex-200778251","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-2075229525","vertexIds":["small-building-vertex-1027259450","small-building-vertex-566043311"]},{"kind":"curve","id":"small-building-edge-224238197","vertexIds":["small-building-vertex-584100920","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-229106015","vertexIds":["small-building-vertex-1027259450","small-building-vertex-834828749"]},{"kind":"curve","id":"small-building-edge-278123677","vertexIds":["small-building-vertex-584100920","small-building-vertex-354458280"]},{"kind":"curve","id":"small-building-edge-278867947","vertexIds":["small-building-vertex-834828749","small-building-vertex-200778251"]},{"kind":"curve","id":"small-building-edge-332998341","vertexIds":["small-building-vertex-200778251","small-building-vertex-354458280"]},{"kind":"curve","id":"small-building-edge-757634469","vertexIds":["small-building-vertex-1412665337","small-building-vertex-834828749"]},{"kind":"curve","id":"small-building-edge-779379499","vertexIds":["small-building-vertex-1412665337","small-building-vertex-566043311"]},{"kind":"curve","id":"small-building-edge-951546977","vertexIds":["small-building-vertex-560960189","small-building-vertex-1027259450"]},{"kind":"curve","id":"small-building-wire-1366152152","edgeIds":["small-building-edge-278123677","small-building-edge-1943812986","small-building-edge-779379499","small-building-edge-1660152326"]},{"kind":"curve","id":"small-building-wire-1559546061","edgeIds":["small-building-edge-2075229525","small-building-edge-229106015","small-building-edge-757634469","small-building-edge-779379499"]},{"kind":"curve","id":"small-building-wire-1742634236","edgeIds":["small-building-edge-278867947","small-building-edge-332998341","small-building-edge-1660152326","small-building-edge-757634469"]},{"kind":"curve","id":"small-building-wire-456551683","edgeIds":["small-building-edge-1943812986","small-building-edge-224238197","small-building-edge-951546977","small-building-edge-2075229525"]},{"kind":"curve","id":"small-building-wire-515231130","edgeIds":["small-building-edge-224238197","small-building-edge-278123677","small-building-edge-332998341","small-building-edge-2004107109"]},{"kind":"curve","id":"small-building-wire-978200956","edgeIds":["small-building-edge-951546977","small-building-edge-2004107109","small-building-edge-278867947","small-building-edge-229106015"]},{"kind":"surface","id":"small-building-face-1071813579","wireIds":["small-building-wire-1559546061"]},{"kind":"surface","id":"small-building-face-1198070201","wireIds":["small-building-wire-456551683"]},{"kind":"surface","id":"small-building-face-1321487947","wireIds":["small-building-wire-1742634236"]},{"kind":"surface","id":"small-building-face-1833451572","wireIds":["small-building-wire-1366152152"]},{"kind":"surface","id":"small-building-face-383803774","wireIds":["small-building-wire-515231130"]},{"kind":"surface","id":"small-building-face-624717229","wireIds":["small-building-wire-978200956"]},{"kind":"shell","id":"small-building-shell-319815043","faceIds":["small-building-face-1198070201","small-building-face-1833451572","small-building-face-383803774","small-building-face-624717229","small-building-face-1071813579","small-building-face-1321487947"]},{"kind":"solid","slot":"solid","id":"small-building-cell-123052045","shellIds":["small-building-shell-319815043"]}]},{"id":"object-small-building-cell-1278694563","typology":"spatial.shape.primitive.box","primitives":[{"kind":"vertex","id":"small-building-vertex-1052483923","position":[-18.586667,-57.126666,-9.407222]},{"kind":"vertex","id":"small-building-vertex-1078954806","position":[-64.506666,-57.126666,-9.407222]},{"kind":"vertex","id":"small-building-vertex-1417750768","position":[-18.586667,3.553333,54.142777]},{"kind":"vertex","id":"small-building-vertex-1487235108","position":[-64.506666,-57.126666,25.852777]},{"kind":"vertex","id":"small-building-vertex-1653716766","position":[-64.506666,3.553333,54.142777]},{"kind":"vertex","id":"small-building-vertex-1928378833","position":[-64.506666,-7.926666,62.752777]},{"kind":"vertex","id":"small-building-vertex-200778251","position":[-18.586667,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-551332595","position":[-18.586667,-7.926666,62.752777]},{"kind":"vertex","id":"small-building-vertex-560960189","position":[-64.506666,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-945778871","position":[-18.586667,-57.126666,25.852777]},{"kind":"curve","id":"small-building-edge-1070453701","vertexIds":["small-building-vertex-945778871","small-building-vertex-1052483923"]},{"kind":"curve","id":"small-building-edge-1108121009","vertexIds":["small-building-vertex-945778871","small-building-vertex-1487235108"]},{"kind":"curve","id":"small-building-edge-1261178177","vertexIds":["small-building-vertex-560960189","small-building-vertex-1653716766"]},{"kind":"curve","id":"small-building-edge-1422159112","vertexIds":["small-building-vertex-1078954806","small-building-vertex-1487235108"]},{"kind":"curve","id":"small-building-edge-1567786765","vertexIds":["small-building-vertex-551332595","small-building-vertex-945778871"]},{"kind":"curve","id":"small-building-edge-1705326756","vertexIds":["small-building-vertex-1928378833","small-building-vertex-551332595"]},{"kind":"curve","id":"small-building-edge-2004107109","vertexIds":["small-building-vertex-200778251","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-2102926252","vertexIds":["small-building-vertex-1928378833","small-building-vertex-1653716766"]},{"kind":"curve","id":"small-building-edge-349583852","vertexIds":["small-building-vertex-1052483923","small-building-vertex-1078954806"]},{"kind":"curve","id":"small-building-edge-354613623","vertexIds":["small-building-vertex-1487235108","small-building-vertex-1928378833"]},{"kind":"curve","id":"small-building-edge-432705901","vertexIds":["small-building-vertex-200778251","small-building-vertex-1052483923"]},{"kind":"curve","id":"small-building-edge-432807106","vertexIds":["small-building-vertex-1417750768","small-building-vertex-200778251"]},{"kind":"curve","id":"small-building-edge-467629952","vertexIds":["small-building-vertex-1078954806","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-49481349","vertexIds":["small-building-vertex-1417750768","small-building-vertex-551332595"]},{"kind":"curve","id":"small-building-edge-508083477","vertexIds":["small-building-vertex-1417750768","small-building-vertex-1653716766"]},{"kind":"curve","id":"small-building-wire-1095357825","edgeIds":["small-building-edge-467629952","small-building-edge-1422159112","small-building-edge-354613623","small-building-edge-2102926252","small-building-edge-1261178177"]},{"kind":"curve","id":"small-building-wire-1399618711","edgeIds":["small-building-edge-349583852","small-building-edge-432705901","small-building-edge-2004107109","small-building-edge-467629952"]},{"kind":"curve","id":"small-building-wire-1676387167","edgeIds":["small-building-edge-1070453701","small-building-edge-432705901","small-building-edge-432807106","small-building-edge-49481349","small-building-edge-1567786765"]},{"kind":"curve","id":"small-building-wire-1755121565","edgeIds":["small-building-edge-2004107109","small-building-edge-432807106","small-building-edge-508083477","small-building-edge-1261178177"]},{"kind":"curve","id":"small-building-wire-285005499","edgeIds":["small-building-edge-349583852","small-building-edge-1070453701","small-building-edge-1108121009","small-building-edge-1422159112"]},{"kind":"curve","id":"small-building-wire-311748032","edgeIds":["small-building-edge-354613623","small-building-edge-1108121009","small-building-edge-1567786765","small-building-edge-1705326756"]},{"kind":"curve","id":"small-building-wire-924227310","edgeIds":["small-building-edge-49481349","small-building-edge-508083477","small-building-edge-2102926252","small-building-edge-1705326756"]},{"kind":"surface","id":"small-building-face-1144073303","wireIds":["small-building-wire-1755121565"]},{"kind":"surface","id":"small-building-face-1382526041","wireIds":["small-building-wire-1095357825"]},{"kind":"surface","id":"small-building-face-1891411219","wireIds":["small-building-wire-1399618711"]},{"kind":"surface","id":"small-building-face-2019874201","wireIds":["small-building-wire-311748032"]},{"kind":"surface","id":"small-building-face-2129815768","wireIds":["small-building-wire-924227310"]},{"kind":"surface","id":"small-building-face-512606747","wireIds":["small-building-wire-285005499"]},{"kind":"surface","id":"small-building-face-816291467","wireIds":["small-building-wire-1676387167"]},{"kind":"shell","id":"small-building-shell-115098816","faceIds":["small-building-face-816291467","small-building-face-512606747","small-building-face-1891411219","small-building-face-1144073303","small-building-face-2129815768","small-building-face-2019874201","small-building-face-1382526041"]},{"kind":"solid","slot":"solid","id":"small-building-cell-1278694563","shellIds":["small-building-shell-115098816"]}]}]}}]}';

//#endregion 🧪️InteractionE2EFixtures

if (import.meta.vitest) {
  __spatialCoreTestRuntime!.bootstrapCadModules();
  const { BrepjsKernel, preciseSpatialKernelMath } = __spatialCoreTestKernel!;
  const geometryLoomFixtureJson = JSON.parse(CAD_E2E_LOOM_MODEL_SPACE_JSON) as ModelSpaceJson;
  const geometryRoutesFixtureJson = JSON.parse(CAD_E2E_ROUTES_MODEL_SPACE_JSON) as ModelSpaceJson;
  const buildingBooleanFixtureJson = JSON.parse(CAD_E2E_BUILDING_BOOLEAN_MODEL_SPACE_JSON) as ModelSpaceJson;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/cad-js/core vec", () => {
    it("adds and distances", () => {
      expect(M.vec3Distance([0, 0, 0], [3, 4, 0])).toBe(5);
    });
  });

  describe("@semio-tech/cad-js/core model definition catalogs", () => {
    it("loads attribute and property definition assets", () => {
      const attributes = listModelDefinitionAttributeDefinitions();
      const properties = listModelDefinitionPropertyDefinitions();
      expect(attributes.length).toBeGreaterThanOrEqual(6);
      expect(properties.some((row) => row.id === "spatial.shape.volume")).toBe(true);
      expect(loadAttributeDefinition("spatial.shape.material")?.field).toBe("material");
      expect(loadPropertyDefinition("spatial.shape.volume")?.unit).toBe("volume");
    });
    it("loads stat definition assets", () => {
      const stats = listModelDefinitionStatDefinitions();
      expect(stats.some((row) => row.id === "spatial.shape.geometry")).toBe(true);
      expect(stats.some((row) => row.id === "energy.demand")).toBe(true);
      expect(stats.some((row) => row.id === "structure.stability")).toBe(true);
      expect(loadStatDefinition("spatial.shape.geometry")?.outputs.some((row) => row.key === "totalVolume")).toBe(true);
      expect(listStatDefinitionsForModelDefinition("aec.building.energy").map((row) => row.id)).toContain("energy.demand");
      expect(resolveModelDefinitionScope(defaultModelDefinitionId()).statDefinitions.map((row) => row.id)).toContain("spatial.shape.geometry");
      expect(formatStatOutputValue(0.456, "percent")).toBe("45.6%");
    });
    it("computes spatial.shape.geometry stats for solid-backed objects", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      model.objects["obj-a"] = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      const defn = loadStatDefinition("spatial.shape.geometry")!;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const out = await computeStat(defn, {
        model,
        kernel,
        modelDefinitionId: defaultModelDefinitionId(),
        scope: "model",
        objects: objectsForStatCompute(model, defaultModelDefinitionId(), defn, "model", []),
      });
      expect(out.objectCount).toBe(1);
      expect(out.solidCount).toBe(1);
      expect(out.totalVolume).toBeCloseTo(1, 3);
      expect(out.sizeX).toBeCloseTo(1, 3);
      expect(out.sizeY).toBeCloseTo(1, 3);
      expect(out.sizeZ).toBeCloseTo(1, 3);
    });
    it("computes energy and structure stats with finite outputs", async () => {
      const energyModel = new Model();
      applyModelDiff(energyModel, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("hull-solid")));
      energyModel.objects["hull"] = {
        id: "hull" as ObjectRef,
        typology: "energy.energy.hull" as TypologyRef,
        primitives: { solid: "hull-solid" },
      };
      const energyDefn = loadStatDefinition("energy.demand")!;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const energyOut = await computeStat(energyDefn, {
        model: energyModel,
        kernel,
        modelDefinitionId: "aec.building.energy",
        scope: "model",
        objects: objectsForStatCompute(energyModel, "aec.building.energy", energyDefn, "model", []),
      });
      expect(Number.isFinite(energyOut.heatedVolume)).toBe(true);
      expect(Number.isFinite(energyOut.annualHeatingDemand)).toBe(true);
      expect(energyOut.heatedVolume).toBeGreaterThan(0);

      const structureModel = new Model();
      applyModelDiff(structureModel, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [0.4, 0.4, 0], height: 3 }, solidRef("column-solid")));
      structureModel.objects["column"] = {
        id: "column" as ObjectRef,
        typology: "structure.structure.reinforcedconcretecolumn" as TypologyRef,
        primitives: { solid: "column-solid" },
      };
      const structureDefn = loadStatDefinition("structure.stability")!;
      const structureOut = await computeStat(structureDefn, {
        model: structureModel,
        kernel,
        modelDefinitionId: "aec.building.structure",
        scope: "model",
        objects: objectsForStatCompute(structureModel, "aec.building.structure", structureDefn, "model", []),
      });
      expect(structureOut.elementCount).toBe(1);
      expect(Number.isFinite(structureOut.estimatedMass)).toBe(true);
      expect(structureOut.stabilityIndex).toBeGreaterThan(0);
      expect(structureOut.stabilityIndex).toBeLessThanOrEqual(1);
    });
    it("loads geometry and AEC typology assets", () => {
      const typologies = listModelDefinitionTypologies();
      expect(typologies.length).toBeGreaterThanOrEqual(27);
      expect(loadTypology("energy.energy.hull")?.properties).toContain("energy.heatedvolume");
      expect(loadTypology("energy.energy.hull")?.properties).toContain("spatial.shape.volume");
    });
    it("assigns primitiveKinds to geometry typologies", () => {
      const box = loadTypology("spatial.shape.primitive.box");
      const line = loadTypology("spatial.shape.curve.line");
      const plane = loadTypology("spatial.shape.surface.plane");
      expect(box?.primitiveKinds).toEqual(["solid"]);
      expect(line?.primitiveKinds).toEqual(["curve"]);
      expect(plane?.primitiveKinds).toEqual(["surface"]);
    });
    it("assigns primitiveKinds to AEC typologies", () => {
      expect(loadTypology("building.building.slab")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("building.building.wall")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("building.building.beam")?.primitiveKinds).toEqual(["curve"]);
      expect(loadTypology("building.building.column")?.primitiveKinds).toEqual(["solid"]);
      expect(loadTypology("energy.energy.externalwall")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("energy.energy.hull")?.primitiveKinds).toEqual(["solid"]);
      expect(loadTypology("structure.structure.onewayreinforcedconcreteslab")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("structure.linefem.lineelement")?.primitiveKinds).toEqual(["curve"]);
      expect(loadTypology("structure.surfacefem.surfaceelement")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("structure.solidfem.solidelement")?.primitiveKinds).toEqual(["solid"]);
    });
    it("resolveTypologyStyle is deterministic and distinct per typology id", () => {
      const a = resolveTypologyStyle("spatial.shape.primitive.box");
      const b = resolveTypologyStyle("spatial.shape.primitive.box");
      const c = resolveTypologyStyle("spatial.shape.curve.line");
      expect(a).toEqual(b);
      expect(a.color).toMatch(/^#[0-9a-f]{6}$/i);
      expect(c.color).not.toBe(a.color);
      expect(a.pattern.kind).toBe("none");
    });
    it("resolveTypologyStyle merges authored typology style overrides", () => {
      const slab = resolveTypologyStyle("structure.structure.onewayreinforcedconcreteslab");
      expect(slab.color).toBe("#8B7355");
      expect(slab.pattern.kind).toBe("hatch");
      expect(slab.pattern.direction).toBe(0);
      const wall = resolveTypologyStyle("structure.structure.reinforcedconcreteexternalwall");
      expect(wall.pattern.kind).toBe("crosshatch");
      expect(wall.edgeColor).toBe("#3D5560");
      const roof = resolveTypologyStyle("energy.energy.roof");
      expect(roof.pattern.kind).toBe("hatch");
      expect(roof.pattern.direction).toBe(90);
    });
    it("derives spatial.shape.volume for solid-backed objects", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      const object = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      const defn = loadPropertyDefinition("spatial.shape.volume")!;
      const kernel = {
        syncSolidsFromModel: async () => {},
        solidVolume: async () => 42,
      } as unknown as SpatialKernel;
      const out = await derivePropertyValue(defn, { model, kernel, object });
      expect(out.volume).toBe(42);
      expect(listApplicablePropertyDefinitionsForModelDefinition(defaultModelDefinitionId(), model, object).map((row) => row.id)).toContain("spatial.shape.volume");
    });
    it("validates object geometry against typology primitiveKinds", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      model.objects["obj-a"] = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-a"]!)).toBe(true);
      model.objects["obj-b"] = {
        id: "obj-b" as ObjectRef,
        typology: "spatial.shape.selection.select-all" as TypologyRef,
        primitives: { solid: solidId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-b"]!)).toBe(false);
      model.objects["obj-c"] = {
        id: "obj-c" as ObjectRef,
        typology: "building.building.slab" as TypologyRef,
        primitives: { solid: solidId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-c"]!)).toBe(false);
      const faceId = Object.keys(model.faces)[0]!;
      model.objects["obj-d"] = {
        id: "obj-d" as ObjectRef,
        typology: "building.building.slab" as TypologyRef,
        primitives: { surface: faceId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-d"]!)).toBe(true);
    });
  });

  describe("@semio-tech/cad-js/core model space and hashing", () => {
    it("hashes vertex positions stably", () => {
      const a = hashVertexPosition([1, 2, 3]);
      const b = hashVertexPosition([1.0000000004, 2, 3]);
      expect(a).toBe(b);
      expect(hashVertexPosition([1, 2, 4])).not.toBe(a);
    });
    it("links models in a model space", () => {
      const space = new ModelSpace();
      const m = new Model();
      applyModelDiff(m, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      space.link("primary", m);
      expect(space.get("primary")).toBe(m);
      const hashes = space.vertexHashesByModel();
      expect(Object.keys(hashes.primary ?? {}).length).toBe(8);
      const geo = space.geometryHashesByModel().primary;
      expect(Object.keys(geo?.solid ?? {}).length).toBeGreaterThanOrEqual(1);
      const roundTrip = ModelSpace.fromJSON(space.toJSON());
      expect(roundTrip.get("primary")?.revision).toBe(m.revision);
      expect(hashModelVertices(roundTrip.get("primary")!)).toEqual(hashes.primary);
    });
    it("hashes edges and solids deterministically", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const hashes = hashModelPrimitives(model);
      const solidId = Object.keys(model.solids)[0]!;
      const edgeId = Object.keys(model.edges)[0]!;
      expect(hashes.solid?.[solidId]).toBeTruthy();
      expect(hashes.edge?.[edgeId]).toBeTruthy();
      expect(hashes.solid?.[solidId]).toBe(hashSolidRecord(model.solids[solidId]!));
    });
  });

  describe("@semio-tech/cad-js/core transformations", () => {
    it("detects visible CAD gumball configs", () => {
      expect(cadGumballConfigVisible(CAD_GUMBALL_HIDDEN)).toBe(false);
      expect(cadGumballConfigVisible({ ...CAD_GUMBALL_HIDDEN, rotate: true })).toBe(true);
      expect(cadGumballConfigVisible(null)).toBe(false);
    });

    it("lists model definition manifests and transformation directions", () => {
      const manifests = listModelDefinitionManifests();
      expect(manifests.some((row) => row.id === "spatial.shape")).toBe(true);
      expect(manifests.some((row) => row.id === "aec.building.energy")).toBe(true);
      expect(listTransformationsIntoModelDefinition("aec.building.energy").some((row) => row.id === "from_geometry")).toBe(true);
      expect(listTransformationsFromModelDefinition("spatial.shape").some((row) => row.target.modelDefinition === "aec.building.energy")).toBe(true);
    });
    it("scopes catalogs to active model definition", () => {
      const shape = resolveModelDefinitionScope(defaultModelDefinitionId());
      const energy = resolveModelDefinitionScope("aec.building.energy");
      expect(shape.interactions.some((row) => row.id === "primitive.box")).toBe(true);
      expect(energy.interactions.length).toBe(5);
      expect(energy.interactions.every((row) => row.id.startsWith("energy.energy.construct"))).toBe(true);
      expect(energy.interactions.some((row) => row.id === "primitive.box")).toBe(false);
      expect(energy.typologies.some((row) => row.id === "energy.energy.hull")).toBe(true);
      expect(energy.selectionEntityKinds).toContain("object");
      expect(energy.selectionEntityKinds).toContain("solid");
      expect(shape.selectionEntityKinds).toContain("vertex");
      expect(shape.selectionEntityKinds).toContain("face");
      const structure = resolveModelDefinitionScope("aec.building.structure");
      expect(structure.selectionEntityKinds).toContain("face");
      expect(structure.selectionEntityKinds).toContain("object");
      expect(listAttributeDefinitionsForModelDefinitionEntity("aec.building.structure", "face").some((row) => row.field === "exposure")).toBe(true);
      expect(listPropertyDefinitionsForModelDefinition("aec.building.energy").some((row) => row.id === "energy.heatedvolume")).toBe(true);
      expect(actionAvailableInModelDefinition("primitive.createBoxFromCorners", defaultModelDefinitionId())).toBe(true);
      expect(actionAvailableInModelDefinition("primitive.createBoxFromCorners", "aec.building.energy")).toBe(false);
      expect(listSelectionOperationsForModelDefinition(defaultModelDefinitionId()).some((row) => row.id === "selection.selectVertices")).toBe(true);
      expect(listSelectionOperationsForModelDefinition("aec.building.energy").length).toBe(0);
    });
    it("buildModelPrimitiveDocument nests shell through vertex under solid", () => {
      const model = new Model();
      const solid = solidRef("box-solid");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      const tree = buildModelPrimitiveDocument(model, String(solid));
      expect(tree?.kind).toBe("solid");
      expect(tree?.children).toHaveLength(1);
      const shell = tree!.children[0]!;
      expect(shell.kind).toBe("shell");
      expect(shell.children.length).toBeGreaterThan(0);
      const face = shell.children[0]!;
      expect(face.kind).toBe("face");
      expect(face.children.some((row) => row.kind === "wire")).toBe(true);
      const wire = face.children.find((row) => row.kind === "wire")!;
      expect(wire.children.some((row) => row.kind === "edge")).toBe(true);
      const edge = wire.children.find((row) => row.kind === "edge")!;
      expect(edge.children.every((row) => row.kind === "vertex")).toBe(true);
      expect(edge.children.length).toBe(2);
    });

    it("listModelObjectsForModelDefinition filters objects by typology ownership", () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["hull"] = { id: "hull" as ObjectRef, typology: "energy.energy.hull", primitives: { solid: String(cell) } };
      model.objects["other"] = { id: "other" as ObjectRef, typology: "spatial.shape.primitive.box", primitives: { solid: String(cell) } };
      expect(listModelObjectsForModelDefinition(model, "aec.building.energy").map((row) => String(row.id))).toEqual(["hull"]);
      expect(countViewObjectsForModelDefinition(model, "aec.building.energy")).toBe(1);
    });

    it("listModelObjectsForModelDefinition includes kernel typology objects on shape model definition", () => {
      const model = new Model();
      const cell = solidRef("imported");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["imported"] = {
        id: "imported" as ObjectRef,
        typology: "spatial.shape.kernel.solid",
        primitives: { solid: String(cell) },
      };
      expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId()).map((row) => String(row.id))).toEqual(["imported"]);
    });
    it("listModelObjectsForModelDefinition lists BIM class objects for aec.building", async () => {
      const { readFile } = await import("node:fs/promises");
      const { resolve } = await import("node:path");
      const fixturePath = resolve(import.meta.dirname, "../../🖼️assets/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json");
      const fixtureJson = JSON.parse(await readFile(fixturePath, "utf8")) as ModelSpaceJson;
      const space = ModelSpace.fromJSON(fixtureJson);
      const building = space.models["aec.building"]!;
      expect(listTypologiesForModelDefinition("aec.building").some((row) => row.id === "building.building.column")).toBe(true);
      expect(listModelObjectsForModelDefinition(building, "aec.building")).toHaveLength(12);
    });
    it("collectGeometrySelectionTargets scopes object rows to model definition typologies", () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["hull"] = { id: "hull" as ObjectRef, typology: "energy.energy.hull", primitives: { solid: String(cell) } };
      model.objects["other"] = { id: "other" as ObjectRef, typology: "spatial.shape.primitive.box", primitives: { solid: String(cell) } };
      const all = collectGeometrySelectionTargets(model, ["object"], "aec.building.energy");
      expect(all.map((row) => row.id)).toEqual(["hull"]);
    });
    it("ActionRegistry rejects out-of-scope actions for active model definition", async () => {
      const actions = modelDefinitionActionRegistry();
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      await expect(
        runRegisteredAction(
          actions,
          "primitive.createBoxFromCorners",
          { cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1, __context: {}, __event: { kind: "test" } },
          { kernel, preview: kernel as unknown as SpatialPreviewKernel, model, activeModelDefinitionId: "aec.building.energy" },
        ),
      ).rejects.toThrow(/not available in model definition aec\.building\.energy/);
    });
    it("loads and applies from_geometry transformation", () => {
      const spec = loadTransformation("aec.building.energy.from_geometry");
      expect(spec?.source.modelDefinition).toBe("spatial.shape");
      expect(spec?.target.modelDefinition).toBe("aec.building.energy");
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      source.objects["geom"] = {
        id: "geom" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "box" },
      };
      const target = applyTransformation(spec!, source, M);
      expect(target.objects["energy.energy.hull"]?.typology).toBe("energy.energy.hull");
      expect(target.objects["energy.energy.hull"]?.primitives).toEqual({ solid: "box" });
      expect(target.solids.box).toBe(source.solids.box);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.roof")).toHaveLength(1);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.baseplate")).toHaveLength(1);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.externalwall")).toHaveLength(4);
      expect(target.objects["energy.energy.roof"]?.primitives.surface).toBe("box-box-face-top");
      expect(target.objects["energy.energy.baseplate"]?.primitives.surface).toBe("box-box-face-bottom");
      const space = new ModelSpace();
      space.link("geometry", source);
      space.transfer("geometry", "energy", spec!, M);
      expect(space.get("energy")?.objects["energy.energy.windows"]).toBeTruthy();
    });
    it("loads and applies from_building transformation with shared solid geometry", () => {
      const spec = loadTransformation("aec.building.structure.from_building");
      expect(spec?.source.modelDefinition).toBe("aec.building");
      expect(spec?.target.modelDefinition).toBe("aec.building.structure");
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [0.4, 0.4, 3], height: 3 }, solidRef("column-solid")));
      source.objects["col"] = {
        id: "col" as ObjectRef,
        typology: "building.building.column" as TypologyRef,
        primitives: { solid: "column-solid" },
      };
      const target = applyTransformation(spec!, source, M);
      expect(listModelObjectsForModelDefinition(target, "aec.building.structure.classic")).toHaveLength(1);
      expect(target.objects["structure.structure.reinforcedconcretecolumn"]?.typology).toBe("structure.structure.reinforcedconcretecolumn");
      expect(target.objects["structure.structure.reinforcedconcretecolumn"]?.primitives.solid).toBe("column-solid");
      expect(target.solids["column-solid"]).toBe(source.solids["column-solid"]);
    });
    it("from_geometry fuses touching shape solids and drops internal faces", () => {
      const spec = loadTransformation("aec.building.energy.from_geometry")!;
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("lower")));
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 1], cornerB: [1, 1, 1], height: 1 }, solidRef("upper")));
      source.objects["lower"] = {
        id: "lower" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "lower" },
      };
      source.objects["upper"] = {
        id: "upper" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "upper" },
      };
      const target = applyTransformation(spec, source, M);
      expect(target.solids["from_geometry-hull"]?.shellIds).toEqual([]);
      expect(target.metadata.get("from_geometry-hull")?.fuseSourceSolidIds).toEqual(["lower", "upper"]);
      expect(target.objects["energy.energy.hull"]?.primitives.solid).toBe("from_geometry-hull");
      expect(target.objects["energy.energy.roof"]?.primitives.surface).toBe("box-upper-face-top");
      expect(target.objects["energy.energy.baseplate"]?.primitives.surface).toBe("box-lower-face-bottom");
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.externalwall")).toHaveLength(4);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.hull" && row.id !== "energy.energy.hull")).toHaveLength(0);
      expect(target.objects["box-lower-face-top"]).toBeUndefined();
      expect(target.objects["box-upper-face-bottom"]).toBeUndefined();
    });
    it("from_geometry hull heated volume is boolean union not vertex AABB", async () => {
      const spec = loadTransformation("aec.building.energy.from_geometry")!;
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("west")));
      applyModelDiff(source, M.boxModelDiff({ cornerA: [3, 0, 0], cornerB: [4, 1, 0], height: 1 }, solidRef("east")));
      source.objects["west"] = {
        id: "west" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "west" },
      };
      source.objects["east"] = {
        id: "east" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "east" },
      };
      const target = applyTransformation(spec, source, M);
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const hull = target.objects["energy.energy.hull"]!;
      const heated = await derivePropertyValue(loadPropertyDefinition("energy.heatedvolume")!, { model: target, kernel, object: hull });
      expect(heated.heatedvolume).toBeCloseTo(2, 3);
    });
  });

  describe("@semio-tech/cad-js/core attribute validation", () => {
    it("validates opening attribute options", () => {
      const defn = loadAttributeDefinition("spatial.shape.opening")!;
      expect(validateAttributeValue(defn, "window")).toBe(true);
      expect(validateAttributeValue(defn, "invalid")).toBe(false);
    });
  });

  describe("@semio-tech/cad-js/core edge and solid geometry", () => {
    it("arcEndOnCircle projects off-circle pick onto arc", () => {
      const end = M.arcEndOnCircle([0, 0, 0], [2, 0, 0], [0, 3, 0]);
      expect(end[0]).toBeCloseTo(0, 5);
      expect(end[1]).toBeCloseTo(2, 5);
      expect(M.vec3Distance([0, 0, 0], end)).toBeCloseTo(2, 5);
    });
    it("arcSamplePoints quarter arc from center start end", () => {
      const pts = M.arcSamplePoints([0, 0, 0], [2, 0, 0], [0, 2, 0], 4);
      expect(pts[0]).toEqual([2, 0, 0]);
      expect(pts[pts.length - 1]![0]).toBeCloseTo(0, 5);
      expect(pts[pts.length - 1]![1]).toBeCloseTo(2, 5);
    });
    it("circleSamplePoints and edgeCurveLength for circle curves", () => {
      const pts = M.circleSamplePoints([0, 0, 0], [0, 0, 1], 2, 64);
      expect(pts.length).toBeGreaterThan(8);
      expect(
        M.edgeCurveLength({ kind: "circle", center: [0, 0, 0], normal: [0, 0, 1], radius: 2 }, [
          [2, 0, 0],
          [2, 0, 0],
        ]),
      ).toBeCloseTo(M.cos(0) * 0 + M.sin(0) * 0 + 4 * 3.141592653589793, 3);
    });
    it("nurbs edge samples through control poles", () => {
      const curve = M.nurbsCurveFromPoles([
        [0, 0, 0],
        [1, 2, 0],
        [3, 0, 0],
      ] as Vec3[])!;
      const v0 = "v0" as VertexRef;
      const v1 = "v1" as VertexRef;
      const verts = {
        [v0]: { id: v0, position: [0, 0, 0] as Vec3 },
        [v1]: { id: v1, position: [3, 0, 0] as Vec3 },
      };
      const edge: EdgeRecord = { id: "e" as EdgeRef, vertexIds: [v0, v1], curve };
      const pts = M.edgeSamplePoints(verts, edge, 24);
      expect(pts.length).toBeGreaterThan(4);
    });
    it("solidPrimitiveAabb sphere bounds", () => {
      const b = M.solidPrimitiveAabb({ kind: "sphere", center: [1, 2, 3], radius: 5 });
      expect(b.min).toEqual([-4, -3, -2]);
      expect(b.max).toEqual([6, 7, 8]);
    });
  });

  describe("@semio-tech/cad-js/core expr", () => {
    it("evaluates numeric fold min expr", () => {
      const e: Expr = {
        kind: "fold",
        operation: "min",
        args: [
          { kind: "const", value: 3 },
          { kind: "const", value: 7 },
        ],
      };
      expect(evalExpr(e, { context: {}, preview: M })).toBe(3);
    });
    it("evaluates guards used by box factory", () => {
      const g: Expr = {
        kind: "all",
        args: [
          { kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "origin" }] } },
          {
            kind: "binop",
            operation: ">",
            left: { kind: "path", root: "context", segments: [{ kind: "field", name: "height" }] },
            right: { kind: "const", value: 0 },
          },
        ],
      };
      expect(evalGuard(g, { context: { origin: [0, 0, 0], height: 2 } })).toBe(true);
    });
  });

  describe("@semio-tech/cad-js/core model json", () => {
    it("materializeInlineObjectPrimitives promotes play fixture wires into model geometry", () => {
      const space = ModelSpace.fromJSON(geometryRoutesFixtureJson as ModelSpaceJson);
      const model = space.models["spatial.shape"]!;
      expect(model.wires["stub-wire"]?.edgeIds.length).toBeGreaterThan(0);
      expect(model.objects["object-wire-stub-wire"]?.primitives.wire).toBe("stub-wire");
    });

    it("parseModelJson fills missing entity arrays with empty lists", () => {
      const model = parseModelJson({
        schema: "spatial.model",
        revision: 1,
        objects: [],
        geometry: {
          vertices: [{ id: "v0", position: [0, 0, 0] }],
          edges: [{ id: "e0", vertexIds: ["v0", "v0"] }],
        },
      });
      expect(model).not.toBeNull();
      expect(Object.keys(model!.anchors).length).toBe(0);
      expect(Object.keys(model!.vertices).length).toBe(1);
      expect(Object.keys(model!.edges).length).toBe(1);
    });
  });

  describe("@semio-tech/cad-js/core model commit mesh", () => {
    it("appendCommittedMeshFaceToModel adds one mesh face from a triangle mesh", () => {
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
      appendCommittedMeshFaceToModel(g, mesh, "t0", M);
      expect(Object.keys(g.faces).length).toBe(1);
      expect(g.revision).toBeGreaterThan(0);
    });
  });

  describe("@semio-tech/cad-js/core metadata", () => {
    it("AttributeTable setField bumps model revision", () => {
      const g = new Model();
      const r0 = g.revision;
      g.metadata.setField("e1", "exposure", "external");
      expect(g.revision).toBeGreaterThan(r0);
      expect(g.metadata.get("e1")?.exposure).toBe("external");
    });

    it("AttributeTable entries and JSON roundtrip", () => {
      const store = new AttributeTable(() => {});
      store.setField("face-1", "exposure", "external");
      store.setField("face-2", "uValue", 0.25);
      const json = store.toJSON();
      expect(json).toHaveLength(2);
      const restored = AttributeTable.fromJSON(json);
      expect(restored.get("face-1")?.exposure).toBe("external");
      expect(restored.get("face-2")?.uValue).toBe(0.25);
    });

    it("ModelJson metadata roundtrip", () => {
      const g = new Model();
      g.metadata.setField("solid-a", "tag", "roof");
      const back = Model.fromJSON(g.toJSON());
      expect(back.metadata.get("solid-a")?.tag).toBe("roof");
    });

    it("AttributeTable getEntityFlags and setEntityFlag roundtrip", () => {
      const g = new Model();
      expect(g.getEntityFlags("obj-1")).toEqual({});
      g.setEntityFlag("obj-1", "hidden", true);
      g.setEntityFlag("face-2", "locked", true);
      expect(g.getEntityFlags("obj-1")).toEqual({ hidden: true });
      expect(g.getEntityFlags("face-2")).toEqual({ locked: true });
      g.setEntityFlag("obj-1", "hidden", false);
      expect(g.getEntityFlags("obj-1")).toEqual({});
      const back = Model.fromJSON(g.toJSON());
      expect(back.getEntityFlags("face-2")).toEqual({ locked: true });
    });
  });


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
      const before = edge.curve?.kind === "nurbs" ? edge.curve.poles.map((pole) => [...pole] as Vec3) : [];
      applyModelDiff(
        model,
        selectionTargetsPointTransformDiff(model, [{ kind: "edge", id: edge.id }], (point) => [point[0] + 1, point[1], point[2]]),
      );
      const updated = model.edges[edge.id]!;
      expect(updated.curve?.kind).toBe("nurbs");
      if (updated.curve?.kind === "nurbs") {
        expect(updated.curve.poles).toEqual(before.map((pole) => [pole[0] + 1, pole[1], pole[2]]));
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
        selectionTargetsPointTransformDiff(model, [{ kind: "vertex", id: startId }], (point) => [point[0], point[1] + 5, point[2]]),
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
                          operation: "assign",
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
                        { operation: "assign", target: { root: "context", segments: [{ kind: "field", name: "constructMode" }] }, value: { kind: "const", value: "surface" } },
                        {
                          operation: "interaction.call",
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
      const interactions = modelDefinitionInteractionRegistry();
      interactions.register(compileInteraction(pickChild));
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
          operation: { kind: "action", action: "command.finish", params: {} },
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
                          operation: "interaction.call",
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
          operation: { kind: "action", action: "command.finish", params: {} },
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
                          operation: "interaction.call",
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
          operation: { kind: "action", action: "command.finish", params: {} },
        },
      })!;
      const reg = modelDefinitionInteractionRegistry();
      reg.register(compileInteraction(grandchild));
      reg.register(compileInteraction(child));
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
      const r = modelDefinitionActionRegistry();
      const before = r.get("measure.faceArea")?.label;
      r.register({
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
      expect(allTargets.every((t) => t.kind !== "surface")).toBe(true);
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
  describe("@semio-tech/cad-js/core model diff", () => {
    it("applyModelDiff then inverse restores counts", () => {
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
      const d = M.meshFaceModelDiff(mesh, "x");
      const inv = applyModelDiff(g, d);
      expect(Object.keys(g.faces).length).toBe(1);
      applyModelDiff(g, inv);
      expect(Object.keys(g.faces).length).toBe(0);
    });

    it("boxModelDiff creates selectable boundary and volume records", () => {
      const g = new Model();
      applyModelDiff(g, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 4 }, solidRef("box-solid")));
      expect(Object.keys(g.vertices).length).toBe(8);
      expect(Object.keys(g.edges).length).toBe(12);
      expect(Object.keys(g.wires).length).toBe(6);
      expect(Object.keys(g.faces).length).toBe(6);
      expect(Object.keys(g.shells).length).toBe(1);
      expect(Object.keys(g.solids)).toEqual(["box-solid"]);
    });

    it("deleteObjectsFromModel removes object rows but keeps geometry primitives", () => {
      const g = new Model();
      applyModelDiff(g, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-solid")));
      g.objects["box-a"] = { id: "box-a" as ObjectRef, typology: "spatial.shape.primitive.box", primitives: { solid: "box-solid" } };
      g.objects["box-b"] = { id: "box-b" as ObjectRef, typology: "spatial.shape.primitive.box", primitives: { solid: "box-solid" } };
      const removed = deleteObjectsFromModel(g, ["box-a", "missing"]);
      expect(removed).toEqual(["box-a"]);
      expect(g.objects["box-a"]).toBeUndefined();
      expect(g.objects["box-b"]).toBeTruthy();
      expect(Object.keys(g.solids)).toEqual(["box-solid"]);
    });

    it("deletableObjectIdsFromSelection keeps only object targets", () => {
      const selection: SelectionTarget[] = [
        { kind: "object", id: "box-a", editable: true },
        { kind: "solid", id: "box-solid", editable: true },
        { kind: "object", id: "box-a", editable: true },
      ];
      expect(deletableObjectIdsFromSelection(selection)).toEqual(["box-a"]);
    });
  });
  describe("@semio-tech/cad-js/core selection filter", () => {
    it("selectionEventMatches rejects kinds outside accept", () => {
      const spec: SelectionSpec = { accept: ["face"], multiple: false };
      const ok: SelectionEvent = {
        kind: "selection.changed",
        targets: [{ kind: "face", id: "f1", editable: true }],
      };
      const bad: SelectionEvent = {
        kind: "selection.changed",
        targets: [{ kind: "surface", id: "s1", editable: false }],
      };
      expect(selectionEventMatches(spec, ok)).toBe(true);
      expect(selectionEventMatches(spec, bad)).toBe(false);
    });

    it("expandSelectionTargetsForAccept maps object picks to wire primitives", () => {
      const model = new Model();
      const typology = "spatial.shape.curve.interpolate-curve";
      model.objects[typology as ObjectRef] = {
        id: typology as ObjectRef,
        typology: typology as TypologyRef,
        primitives: { curve: "w-interp" },
      };
      const spec: SelectionSpec = { accept: ["wire", "edge"], multiple: true };
      const expanded = expandSelectionTargetsForAccept(model, spec, [{ kind: "object", id: typology, editable: true }]);
      expect(expanded).toEqual([{ kind: "wire", id: "w-interp", editable: true }]);
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
        expect(heightSeg.params.to[2]).toBeCloseTo(3, 5);
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
        expect(ev.point[0]).toBeCloseTo(4, 5);
        expect(ev.point[1]).toBeCloseTo(0, 5);
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

  describe("@semio-tech/cad-js/core box display committed", () => {
    it("keeps box-preview visible for committed state", () => {
      const spec = buildBoxInteractionSpec();
      const ctx: Record<string, unknown> = {
        origin: [0, 0, 0] as Vec3,
        corner: [2, 3, 0] as Vec3,
        height: 4,
      };
      const d = resolveDisplay(spec, "committed", ctx, M);
      const prev = d.items.find((i) => i.kind === "box-preview" && i.id === "preview-committed");
      expect(prev?.params?.cornerA).toEqual([0, 0, 0]);
      expect(prev?.params?.cornerB).toEqual([2, 3, 0]);
      expect(prev?.params?.height).toBe(4);
    });
  });

  describe("@semio-tech/cad-js/core interaction e2e fixtures", () => {
    type InteractionE2EFixtureKind = "loom" | "routes" | "building" | "empty";

    const MOD: InteractionEvent["modifiers"] = {};

    const p = (x: number, y: number, z = 0): Vec3 => [x, y, z];

    const sel = (kind: ModelEntityKind, id: string, editable = true): SelectionTarget => ({
      kind,
      id,
      editable: kind === "surface" || kind === "part" ? false : editable,
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
          typology: "spatial.shape.primitive.box",
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
// #endregion 🧪️Tests

// #endregion 📦️🧪️tests
