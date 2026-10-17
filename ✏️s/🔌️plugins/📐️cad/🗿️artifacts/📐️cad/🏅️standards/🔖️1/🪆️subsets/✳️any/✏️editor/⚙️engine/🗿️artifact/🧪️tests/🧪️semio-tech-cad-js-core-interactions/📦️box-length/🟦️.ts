/** 📦️ Box construction, length entry and the stateEngine option. */
import { ModelSpace, ModelSpaceJson } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import { interactionControlForState, interactionLengthEntryLiveDistance, interactionNumericEntryCommitEvent, interactionNumericEntryLockedValue } from "../../../../🎬️actions/🎰️interaction-state/🟦️.ts";
import { preciseSpatialKernelMath } from "@semio-tech/s-spatial-kernel-ts/preview";
import type { ArtifactTestDependencies } from "../../../🟦️.ts";
import type { SolidRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { ModelDiff, SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { MeshTransfer, Vec3 } from "@semio-tech/framework/3d";

export async function registerBoxLengthEntryTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ArtifactTestDependencies): Promise<void> {
  const { CAD_E2E_ROUTES_MODEL_SPACE_JSON, EMPTY_MODEL_DIFF, Model, __artifactTestKernel, applyModelDiff, buildBoxInteractionSpec, clampPointAlongDirection, createInteractionRuntime, defaultModelDefinitionId, emptyMeshTransfer, ensureTypologyObjectFromCreateDiff, interactionLengthEntryForState, loadSpatialInteraction, pureTsStateEngineProvider, readInteractionContextVec3, requireSpatialInteraction, resolveDisplay, solidRef } = dependencies;
  const { BrepjsKernel } = __artifactTestKernel!;
  const geometryRoutesFixtureJson = JSON.parse(CAD_E2E_ROUTES_MODEL_SPACE_JSON) as ModelSpaceJson;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = vitest;

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
}
