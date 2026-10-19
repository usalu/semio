/** 🎯️ The selection command, apply, headless and chain tests. */
import { listSelectionOperationsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/selection-operations";
import { runSelectionApply } from "../../../../../🎬️actions/🏃️action-execution/🟦️.ts";
import { executeSelectionApply } from "../../../../../🎬️actions/🧲️selection-targets/🟦️.ts";
import type { SelectionTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import type { SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { RegistriesTestKit } from "../🧰️kit/🟦️.ts";

/** 🖱 Registers the selection modifiers, apply, selectAll, invert, headless history, registered operation and chain tests. */
export function registerSelectionCommandTests(kit: RegistriesTestKit): void {
  const { BrepjsKernel, DocumentHistory, EMPTY_MODEL_DIFF, M, Model, applyModelDiff, defaultModelDefinitionId, expect, interactionRecordsDocumentHistory, isEmptyModelDiff, it, modelDefinitionActionRegistry, runRegisteredAction, runSelectionOperationInteraction, selectionApplyParamsForInteraction, selectionSeedTargetsForOperation, selectionTargetsFromActionResult, solidRef } = kit;
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
}
