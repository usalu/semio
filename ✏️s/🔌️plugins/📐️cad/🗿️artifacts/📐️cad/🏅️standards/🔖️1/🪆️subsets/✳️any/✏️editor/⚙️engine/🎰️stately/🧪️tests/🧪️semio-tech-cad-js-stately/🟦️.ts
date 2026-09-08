type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { Model, NullInspector, __spatialStatelyTestKernel, __spatialStatelyTestRuntime, buildSpatialStatelyMachineCatalogView, buildStatelyMachine, createInteractionRuntime, defaultModelDefinitionId, emptyMeshTransfer, init, isEmptyModelDiff, listSpatialInteractionsForModelDefinition, loadSpatialInteraction, macrostep, makeAdvanceEvent, pureTsStateEngineProvider, solidRef, statelyStateEngineProvider } = dependencies;
  type Command = any;
  type EdgeRef = any;
  type FaceRef = any;
  type InteractionRuntime = any;
  type ModelDiff = any;
  type StatelyMachineSpec = any;
  type Vec3 = any;
  type VertexRef = any;
  type WireRef = any;

  __spatialStatelyTestRuntime!.bootstrapCadModules();
  const { BrepjsKernel } = __spatialStatelyTestKernel!;
  const { describe, expect, it } = vitest;

  class StubKernel extends BrepjsKernel {
    readonly id = "stub-parity";
    readonly operations = ["solid.createBox", "entity.tessellate"] as const;
    lastBox: { cornerA: Vec3; cornerB: Vec3; height: number } | null = null;
    async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }) {
      this.lastBox = input;
      return solidRef("stub-solid");
    }
    async volume() {
      return 0;
    }
    async tessellate() {
      return {
        ...emptyMeshTransfer(),
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        index: new Uint32Array([0, 1, 2]),
      };
    }
  }

  function normalizeModelDiffIds(diff: ModelDiff): ModelDiff {
    const clone = JSON.parse(JSON.stringify(diff)) as ModelDiff;
    const stamp = (added: readonly { id: string }[] | undefined, tag: string) => {
      for (const r of added ?? []) r.id = tag;
    };
    stamp(clone.anchors?.added, "__anchor__");
    stamp(clone.vertices?.added, "__vertex__");
    stamp(clone.edges?.added, "__edge__");
    stamp(clone.wires?.added, "__wire__");
    stamp(clone.faces?.added, "__face__");
    stamp(clone.shells?.added, "__shell__");
    stamp(clone.solids?.added, "__solid__");
    return {
      ...clone,
      wires: clone.wires && {
        ...clone.wires,
        added: clone.wires.added?.map((w) => ({ ...w, edgeIds: w.edgeIds.map(() => "__edge__" as EdgeRef) })),
      },
    };
  }

  async function assertSnapshotsEqual(a: InteractionRuntime, b: InteractionRuntime) {
    const sa = a.getSnapshot();
    const sb = b.getSnapshot();
    expect(sb.state).toBe(sa.state);
    expect(sb.context).toEqual(sa.context);
    expect(sb.capabilities).toEqual(sa.capabilities);
    expect(sb.lastResponse?.ok).toBe(sa.lastResponse?.ok);
    expect(sb.lastResponse?.data).toEqual(sa.lastResponse?.data);
    expect(normalizeModelDiffIds(sb.lastResponse?.diff ?? {})).toEqual(normalizeModelDiffIds(sa.lastResponse?.diff ?? {}));
  }

  class MeasureParityKernel extends BrepjsKernel {
    readonly id = "stub-measure-parity";
    readonly operations = ["surface.resolveFaces", "measure.distance", "measure.area"] as const;
    async createBoxFromCorners() {
      return solidRef("unused");
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
    async vertexDistance(a: VertexRef, b: VertexRef, model: Model) {
      const pa = model.vertices[String(a)]?.position;
      const pb = model.vertices[String(b)]?.position;
      if (!pa || !pb) return 0;
      return Math.hypot(pa[0] - pb[0], pa[1] - pb[1], pa[2] - pb[2]);
    }
    async faceArea(_f: FaceRef, _model: Model) {
      return 42;
    }
  }

  describe("@semio-tech/cad-js/stately", () => {
    it("buildSpatialStatelyMachineCatalogView lists scoped interactions with edges and mermaid", () => {
      const doc = buildSpatialStatelyMachineCatalogView({ modelDefinitionId: defaultModelDefinitionId() });
      expect(doc.kind).toBe("spatial.stately-machine-view/v1");
      expect(doc.machines.length).toBe(listSpatialInteractionsForModelDefinition(defaultModelDefinitionId()).length);
      const box = doc.machines.find((m) => m.interactionId === "primitive.box");
      expect(box?.edges.length).toBeGreaterThan(0);
      expect(box?.mermaid).toContain("primitive_box");
      expect(doc.mermaidCombined.length).toBeGreaterThan(100);
    });

    // 🎓️ Differential test: `xstate` is a devDependency test oracle ONLY (never a production
    // dependency) — asserts `buildStatelyMachine`'s `@semio-tech/machine` definition reaches the
    // same active state as a literal XState v5 chart built from the same `InteractionSpec.machine`,
    // for every `__advance` row of every shipped spatial interaction. Kept so a future edit to
    // `buildStatelyMachine` can't silently drift from the XState semantics it replaced.
    it("buildStatelyMachine matches an XState v5 chart built from the same spec (oracle)", async () => {
      const { createActor, setup } = await import("xstate");
      for (const p of listSpatialInteractionsForModelDefinition(defaultModelDefinitionId())) {
        const spec = loadSpatialInteraction(p.id);
        if (!spec) continue;
        for (const st of spec.machine.states) {
          if (!st.on) continue;
          for (const h of st.on) {
            for (let branch = 0; branch < h.transitions.length; branch++) {
              const tr = h.transitions[branch]!;
              const expected = (tr.target ?? st.name) as string;

              const xstateMachineDef = setup({ types: { events: {} as { type: "__advance"; interactionKind: string; branch: number } } }).createMachine({
                id: `oracle-${spec.id}`,
                initial: st.name,
                states: Object.fromEntries(
                  spec.machine.states.map((s) => [
                    s.name,
                    s.on
                      ? {
                          on: {
                            __advance: s.on.flatMap((row, i) =>
                              row.transitions.map((t, j) => ({
                                guard: ({ event }: { event: { interactionKind: string; branch: number } }) => event.interactionKind === row.event && event.branch === j,
                                target: (t.target ?? s.name) as string,
                                __rowIndex: i,
                              })),
                            ),
                          },
                        }
                      : {},
                  ]),
                ),
              });
              const actor = createActor(xstateMachineDef);
              actor.start();
              actor.send({ type: "__advance", interactionKind: h.event, branch });
              const xstateResult = String(actor.getSnapshot().value);
              actor.stop();

              const ownMachine = buildStatelyMachine(spec, st.name);
              const sink: Command<StatelyMachineSpec>[] = [];
              const snapshot = init(ownMachine, undefined, sink);
              macrostep(ownMachine, snapshot, makeAdvanceEvent(h.event, branch), sink, new NullInspector());
              expect(snapshot.matches(expected)).toBe(true);
              expect(xstateResult).toBe(expected);
            }
          }
        }
      }
    });

    it("matches pure-ts interaction snapshots through box workflow + commit", async () => {
      const spec = loadSpatialInteraction("primitive.box")!;
      const k1 = new StubKernel();
      const k2 = new StubKernel();
      const rtPure = createInteractionRuntime(spec, {
        kernel: k1,
        document: { model: new Model(), nodes: [] },
        stateEngine: pureTsStateEngineProvider,
      });
      const rtSt = createInteractionRuntime(spec, {
        kernel: k2,
        document: { model: new Model(), nodes: [] },
        stateEngine: statelyStateEngineProvider,
      });
      await assertSnapshotsEqual(rtPure, rtSt);
      await rtPure.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rtSt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await assertSnapshotsEqual(rtPure, rtSt);
      await rtPure.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
      await rtSt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
      await assertSnapshotsEqual(rtPure, rtSt);
      await rtPure.send({ kind: "set.height", value: 4, modifiers: {} });
      await rtSt.send({ kind: "set.height", value: 4, modifiers: {} });
      await assertSnapshotsEqual(rtPure, rtSt);
      expect(k1.lastBox).toEqual(k2.lastBox);
    });

    it("matches pure-ts after interaction-local undo", async () => {
      const spec = loadSpatialInteraction("primitive.box")!;
      const k1 = new StubKernel();
      const k2 = new StubKernel();
      const rtPure = createInteractionRuntime(spec, {
        kernel: k1,
        document: { model: new Model(), nodes: [] },
        stateEngine: pureTsStateEngineProvider,
      });
      const rtSt = createInteractionRuntime(spec, {
        kernel: k2,
        document: { model: new Model(), nodes: [] },
        stateEngine: statelyStateEngineProvider,
      });
      await rtPure.send({ kind: "pointer.down", point: [1, 1, 0] as Vec3, modifiers: {} });
      await rtSt.send({ kind: "pointer.down", point: [1, 1, 0] as Vec3, modifiers: {} });
      await assertSnapshotsEqual(rtPure, rtSt);
      rtPure.undo();
      rtSt.undo();
      await assertSnapshotsEqual(rtPure, rtSt);
    });

    it("matches pure-ts distance + area measure commits (response parity)", async () => {
      const distSpec = loadSpatialInteraction("measure.distance")!;
      const areaSpec = loadSpatialInteraction("measure.area")!;
      const mkModel = () => {
        const t = new Model();
        const v0 = "v0" as VertexRef;
        const v1 = "v1" as VertexRef;
        t.vertices[v0] = { id: v0, position: [0, 0, 0] };
        t.vertices[v1] = { id: v1, position: [3, 4, 0] };
        const wf = "w0" as WireRef;
        const e0 = "e0" as EdgeRef;
        const f0 = "f0" as FaceRef;
        t.edges[e0] = { id: e0, vertexIds: [v0, v1] };
        t.wires[wf] = { id: wf, edgeIds: [e0] };
        t.faces[f0] = { id: f0, wireIds: [wf] };
        return t;
      };
      const k1d = new MeasureParityKernel();
      const k2d = new MeasureParityKernel();
      const rtPd = createInteractionRuntime(distSpec, {
        kernel: k1d,
        document: { model: mkModel(), nodes: [] },
        stateEngine: pureTsStateEngineProvider,
      });
      const rtSd = createInteractionRuntime(distSpec, {
        kernel: k2d,
        document: { model: mkModel(), nodes: [] },
        stateEngine: statelyStateEngineProvider,
      });
      await rtPd.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: "v0", editable: true }], modifiers: {} });
      await rtSd.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: "v0", editable: true }], modifiers: {} });
      await rtPd.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: "v1", editable: true }], modifiers: {} });
      await rtSd.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: "v1", editable: true }], modifiers: {} });
      const rd = rtPd.getSnapshot().lastResponse!;
      const sd = rtSd.getSnapshot().lastResponse!;
      expect(rd.data).toBe(5);
      expect(sd.data).toBe(5);
      expect(isEmptyModelDiff(rd.diff)).toBe(false);
      expect(isEmptyModelDiff(sd.diff)).toBe(false);
      expect(rd.diff.edges?.added?.length).toBe(1);
      expect(sd.diff.edges?.added?.length).toBe(1);
      await assertSnapshotsEqual(rtPd, rtSd);

      const k1a = new MeasureParityKernel();
      const k2a = new MeasureParityKernel();
      const rtPa = createInteractionRuntime(areaSpec, {
        kernel: k1a,
        document: { model: mkModel(), nodes: [] },
        stateEngine: pureTsStateEngineProvider,
      });
      const rtSa = createInteractionRuntime(areaSpec, {
        kernel: k2a,
        document: { model: mkModel(), nodes: [] },
        stateEngine: statelyStateEngineProvider,
      });
      await rtPa.send({ kind: "selection.changed", targets: [{ kind: "face", id: "f0", editable: true }], modifiers: {} });
      await rtSa.send({ kind: "selection.changed", targets: [{ kind: "face", id: "f0", editable: true }], modifiers: {} });
      const ra = rtPa.getSnapshot().lastResponse!;
      const sa = rtSa.getSnapshot().lastResponse!;
      expect(ra.data).toBe(42);
      expect(sa.data).toBe(42);
      expect(isEmptyModelDiff(ra.diff)).toBe(false);
      expect(isEmptyModelDiff(sa.diff)).toBe(false);
      expect(ra.diff.anchors?.added?.length).toBe(1);
      expect(sa.diff.anchors?.added?.length).toBe(1);
      await assertSnapshotsEqual(rtPa, rtSa);
    });
  });

}
