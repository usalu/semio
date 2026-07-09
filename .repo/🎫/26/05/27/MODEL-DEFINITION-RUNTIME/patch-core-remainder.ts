#!/usr/bin/env bun
const path = "c:/git/compose/spatial/js/core/index.ts";
let s = await Bun.file(path).text();

s = s.replaceAll("readonly views?: ExtensionViewService", "readonly views?: null");
s = s.replaceAll("views?: ExtensionViewService", "views?: null");
s = s.replaceAll("views?: ExtensionViewService | null", "views?: null");
s = s.replaceAll(", views?: ExtensionViewService,", ", views?: null,");
s = s.replaceAll("ExtensionViewService.forKernel", "(() => null) as unknown as typeof ExtensionViewService.forKernel");

// Remove stale ExtensionViewService type references in send signatures
s = s.replaceAll("actions?: ActionRegistry, views?: null, preview", "actions?: ActionRegistry, views?: null, preview");

function interactionJsonById(id: string): string {
  return `shippedInteractionJsons.find((row) => row.id === ${JSON.stringify(id)})`;
}

s = s.replace(
  `/** @emoji 📦 Parses canonical box asset (\`spatial/assets/interaction/primitive/box.json\`). */
export function buildBoxInteractionSpec(): InteractionSpec {
  const s = parseInteractionSpec(boxInteractionJson);
  if (!s) throw new Error("spatial/assets/interaction/primitive/box.json invalid");
  return compileInteraction(s);
}`,
  `/** @emoji 📦 Parses primitive.box interaction from model-definition assets. */
export function buildBoxInteractionSpec(): InteractionSpec {
  const raw = ${interactionJsonById("primitive.box")};
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) throw new Error("primitive.box interaction missing from modelDefinition assets");
  return compileInteraction(spec);
}`,
);

s = s.replace(
  `/** @emoji 📦 Parses extrude-wire asset (\`spatial/assets/interaction/feature/extrude-wire.json\`). */
export function buildExtrudeInteractionSpec(): InteractionSpec {
  const s = parseInteractionSpec(extrudeWireInteractionJson);
  if (!s) throw new Error("spatial/assets/interaction/feature/extrude-wire.json invalid");
  return compileInteraction(s);
}`,
  `/** @emoji 📦 Parses feature.extrude-wire interaction from model-definition assets. */
export function buildExtrudeInteractionSpec(): InteractionSpec {
  const raw = ${interactionJsonById("feature.extrude-wire")};
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) throw new Error("feature.extrude-wire interaction missing");
  return compileInteraction(spec);
}`,
);

s = s.replace(
  `/** @emoji 📦 Parses offset-surface asset (\`spatial/assets/interaction/feature/offset-surface.json\`). */
export function buildOffsetSurfaceInteractionSpec(): InteractionSpec {
  const s = parseInteractionSpec(offsetSurfaceInteractionJson);
  if (!s) throw new Error("spatial/assets/interaction/feature/offset-surface.json invalid");
  return compileInteraction(s);
}`,
  `/** @emoji 📦 Parses feature.offset-surface interaction from model-definition assets. */
export function buildOffsetSurfaceInteractionSpec(): InteractionSpec {
  const raw = ${interactionJsonById("feature.offset-surface")};
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) throw new Error("feature.offset-surface interaction missing");
  return compileInteraction(spec);
}`,
);

s = s.replace(
  `/** @emoji 📦 Parses distance asset (\`spatial/assets/interaction/measure/length.json\`). */
export function buildDistanceInteractionSpec(): InteractionSpec {
  const s = parseInteractionSpec(distanceInteractionJson);
  if (!s) throw new Error("spatial/assets/interaction/measure/length.json invalid");
  return compileInteraction(s);
}`,
  `/** @emoji 📦 Parses measure.length interaction from model-definition assets. */
export function buildDistanceInteractionSpec(): InteractionSpec {
  const raw = shippedInteractionJsons.find((row) => row.id === "measure.distance" || row.id === "measure.length");
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) throw new Error("measure.length interaction missing");
  return compileInteraction(spec);
}`,
);

s = s.replace(
  `/** @emoji 📦 Parses area asset (\`spatial/assets/interaction/measure/area.json\`). */
export function buildAreaInteractionSpec(): InteractionSpec {
  const s = parseInteractionSpec(areaInteractionJson);
  if (!s) throw new Error("spatial/assets/interaction/measure/area.json invalid");
  return compileInteraction(s);
}`,
  `/** @emoji 📦 Parses measure.area interaction from model-definition assets. */
export function buildAreaInteractionSpec(): InteractionSpec {
  const raw = ${interactionJsonById("measure.area")};
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) throw new Error("measure.area interaction missing");
  return compileInteraction(spec);
}`,
);

s = s.replace(
  `export function buildCreateAnchorInteractionSpec(): InteractionSpec {
  const s = parseInteractionSpec(createAnchorInteractionJson);
  if (!s) throw new Error("entity.createAnchor interaction invalid");
  return compileInteraction(s);
}`,
  ``,
);

// Remove createAnchorInteractionJson block
const anchorStart = "const createAnchorInteractionJson = {";
const anchorEnd = "} as const satisfies BuiltinInteractionFixture;\n\nconst SELECTION_OPERATION";
const aStart = s.indexOf(anchorStart);
const aEnd = s.indexOf(anchorEnd);
if (aStart >= 0 && aEnd > aStart) {
  s = s.slice(0, aStart) + "const SELECTION_OPERATION" + s.slice(aEnd + "const SELECTION_OPERATION".length);
}

// Remove extension views test block
const extViewsTest = `  describe("@spatial/js-core extension views", () => {`;
const extViewsEnd = `  });\n\n  describe("@spatial/js-core interactions", () => {`;
const evStart = s.indexOf(extViewsTest);
const evEnd = s.indexOf(extViewsEnd);
if (evStart >= 0 && evEnd > evStart) {
  s = s.slice(0, evStart) + `  describe("@spatial/js-core interactions", () => {` + s.slice(evEnd + extViewsEnd.length - `  describe("@spatial/js-core interactions", () => {`.length);
}

// Fix selection tests using ExtensionViewService
s = s.replace(
  `      const views = selectionOperationUsesViewObjects(defn) ? ExtensionViewService.forKernel(kernel) : undefined;
      const activeViewId = views ? qualifiedViewId("energy", "energy") : null;
      if (views) await views.refresh(model, activeViewId);
      const params = selectionApplyParamsForInteraction(defn, seed);
      const headless = await runSelectionApply(params, { kernel, preview: M, model: model, views, activeViewId });
      const interactive = await runSelectionOperationInteraction(defn.id, {
        kernel,
        document: { model: model, nodes: [] },
        views,
        activeViewId,
        seedTargets: seed,
      });`,
  `      const params = selectionApplyParamsForInteraction(defn, seed);
      const headless = await runSelectionApply(params, { kernel, preview: M, model: model });
      const interactive = await runSelectionOperationInteraction(defn.id, {
        kernel,
        document: { model: model, nodes: [] },
        seedTargets: seed,
      });`,
);

s = s.replaceAll("ExtensionViewService.forKernel(kernel)", "null");
s = s.replaceAll("listExtensionViews()", "[]");
s = s.replaceAll('qualifiedViewId("energy", "energy")', "null");

// Selection interaction tests -> headless actions
s = s.replace(
  `    it("selection.selectAll commits archived targets without model diff", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const spec = loadSpatialInteraction("selection.selectAll")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new BrepjsKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await rt.send({ kind: "start", targets: [], modifiers: {} });
      const snap = rt.getSnapshot();
      expect(snap.state).toBe("committed");
      expect(snap.lastResponse?.ok).toBe(true);
      expect(isEmptyModelDiff(snap.lastResponse?.diff ?? EMPTY_MODEL_DIFF)).toBe(true);
      const archived = selectionTargetsFromContext(snap.lastResponse?.archiveContext ?? {});
      expect(archived.length).toBeGreaterThan(8);
      expect(archived.some((t) => t.kind === "solid")).toBe(true);
    });`,
  `    it("selection.selectAll returns targets without model diff", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const result = await ActionRegistry.withBuiltins().run(
        "selection.selectAll",
        { seedTargets: [], __context: {}, __event: { kind: "commit" } },
        { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model },
      );
      const targets = selectionTargetsFromActionResult(result);
      expect(targets.length).toBeGreaterThan(8);
      expect(targets.some((t) => t.kind === "solid")).toBe(true);
      expect(isEmptyModelDiff(result.diff ?? EMPTY_MODEL_DIFF)).toBe(true);
    });`,
);

s = s.replace(
  `    it("selection.selectAll does not push document history entries", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const hist = new DocumentHistory();
      const spec = loadSpatialInteraction("selection.selectAll")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new BrepjsKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
        history: hist,
      });
      await rt.send({ kind: "start", targets: [], modifiers: {} });
      expect(rt.getSnapshot().lastResponse?.ok).toBe(true);
      expect(hist.entries()).toEqual([]);
    });`,
  `    it("selection.selectAll headless does not push document history entries", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const hist = new DocumentHistory();
      await ActionRegistry.withBuiltins().run(
        "selection.selectAll",
        { seedTargets: [], __context: {}, __event: { kind: "commit" } },
        { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model },
      );
      expect(hist.entries()).toEqual([]);
    });`,
);

s = s.replace(
  `    it.each(listSelectionOperationInteractionDefs())("resolves selection command key $key → $id", (defn) => {
      expect(resolveSpatialInteractionKey(defn.key)?.id).toBe(defn.id);
      expect(resolveSpatialInteractionKey(defn.id)?.id).toBe(defn.id);
      expect(loadSpatialInteraction(defn.id)?.commit.operation.action).toBe("selection.apply");
    });`,
  `    it.each(listSelectionOperationInteractionDefs())("registers selection command action $id", (defn) => {
      expect(ActionRegistry.withBuiltins().get(defn.id)?.spec?.schema).toBe("spatial.action/v1");
    });`,
);

s = s.replace(
  `    it("compiled selection.invert honors start.targets seed payload", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const spec = loadSpatialInteraction("selection.invert")!;
      expect(spec.machine.initial).toBe("committed");
      const rt = createInteractionRuntime(spec, {
        kernel: new BrepjsKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await rt.send({ kind: "start", targets: [{ kind: "solid", id: "e2e-box", editable: true }], modifiers: {} });
      const archived = selectionTargetsFromContext(rt.getSnapshot().lastResponse?.archiveContext ?? {});
      expect(archived.some((t) => t.kind === "solid" && t.id === "e2e-box")).toBe(false);
      expect(archived.length).toBeGreaterThan(0);
    });`,
  `    it("selection.invert honors seed targets", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const seed = [{ kind: "solid", id: "e2e-box", editable: true }] as const;
      const result = await ActionRegistry.withBuiltins().run(
        "selection.invert",
        { seedTargets: seed, __context: {}, __event: { kind: "commit" } },
        { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model },
      );
      const targets = selectionTargetsFromActionResult(result);
      expect(targets.some((t) => t.kind === "solid" && t.id === "e2e-box")).toBe(false);
      expect(targets.length).toBeGreaterThan(0);
    });`,
);

s = s.replace(
  `    it("selection commands chain selectAll → deselectAll → selectVertices → invert", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const run = async (id: string, targets: readonly SelectionTarget[]) => {
        const spec = loadSpatialInteraction(id)!;
        const rt = createInteractionRuntime(spec, { kernel, document: { model: model, nodes: [] } });
        await rt.send({ kind: "start", targets, modifiers: {} });
        const snap = rt.getSnapshot();
        expect(snap.state).toBe("committed");
        expect(snap.lastResponse?.ok).toBe(true);
        return (snap.lastResponse?.archiveContext?.targets ?? []) as SelectionTarget[];
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
    });`,
  `    it("selection commands chain selectAll → deselectAll → selectVertices → invert", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const actions = ActionRegistry.withBuiltins();
      const run = async (id: string, targets: readonly SelectionTarget[]) => {
        const result = await actions.run(id, { seedTargets: targets, __context: {}, __event: { kind: "commit" } }, { kernel, preview: M, model });
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
    });`,
);

s = s.replaceAll("row.useView ? ExtensionViewService.forKernel(kernel) : undefined", "undefined");

await Bun.write(path, s);
console.log("patched core remainder");
