/** 📚️ The model-definition registry registration, spatial interaction loading and box footprint tests. */
import { isCallableOnlyInteraction } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import { listActionDefs, registerActionDef } from "../../../../../🎬️actions/🏃️action-execution/🟦️.ts";
import type { SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { Vec3 } from "@semio-tech/framework/3d";
import type { RegistriesTestKit } from "../🧰️kit/🟦️.ts";

/** 🗃 Registers the model-definition action and interaction registration, the spatial interaction loading and the box triplet footprint tests. */
export function registerRegistryLoadingAndBoxFootprintTests(kit: RegistriesTestKit): void {
  const { BrepjsKernel, EMPTY_MODEL_DIFF, M, Model, buildBoxInteractionSpec, emptyMeshTransfer, expect, it, loadSpatialInteraction, modelDefinitionActionRegistry, modelDefinitionInteractionRegistry, runRegisteredAction, solidRef } = kit;
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
}
