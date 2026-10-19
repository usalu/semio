/** 📜️ The data-only action documents, typology kits and interpolate curve rollback tests. */
import { listModelDefinitionTypologies } from "@semio-tech/s-spatial-kernel-ts/geometry/typology";
import { isShapeModelDefinition, listModelDefinitionManifests } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { actionOwnedByModelDefinition, listTypologiesForModelDefinition, modelDefinitionIdForInteraction } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { listConstructableTypologiesForModelDefinition, typologyConstructAssetIds, typologyConstructModeActionIds, typologyHasNativeConstructKit } from "../../../../../🧬️typology/🟦️.ts";
import { listModelDefinitionActionSpecs, parseActionSpec } from "../../../../../🎬️actions/🧬️action-contract/🟦️.ts";
import type { SpatialKernel, SpatialPreviewKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { RegistriesTestKit } from "../🧰️kit/🟦️.ts";

/** 🧱 Registers the executable field rejection, model-definition action loading, typology construct kits and the interpolate curve rollback tests. */
export function registerActionDocumentAndTypologyKitTests(kit: RegistriesTestKit): void {
  const { BrepjsKernel, Model, createInteractionRuntime, defaultModelDefinitionId, expect, it, listInteractionSpecs, loadSpatialInteraction, modelDefinitionActionRegistry, modelDefinitionInteractionRegistry, runRegisteredAction } = kit;
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
}
