/** 📏️ The cad-js core length entry and stateEngine option tests. */
import { ModelSpace, ModelSpaceJson } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import { interactionControlForState, interactionLengthEntryLiveDistance, interactionNumericEntryCommitEvent, interactionNumericEntryLockedValue } from "../../../../../🎬️actions/🎰️interaction-state/🟦️.ts";
import type { SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { Vec3 } from "@semio-tech/framework/3d";
import type { BoxLengthTestKit } from "../🧰️box-length-test-kit/🟦️.ts";

/** ⌨ Registers the cad-js core length entry and stateEngine option describe blocks. */
export function registerLengthEntryAndStateEngineTests(kit: BoxLengthTestKit): void {
  const { BrepjsKernel, CAD_E2E_ROUTES_MODEL_SPACE_JSON, EMPTY_MODEL_DIFF, M, Model, applyModelDiff, buildBoxInteractionSpec, clampPointAlongDirection, createInteractionRuntime, defaultModelDefinitionId, describe, emptyMeshTransfer, ensureTypologyObjectFromCreateDiff, expect, interactionLengthEntryForState, it, loadSpatialInteraction, pureTsStateEngineProvider, readInteractionContextVec3, requireSpatialInteraction, solidRef } = kit;
  const geometryRoutesFixtureJson = JSON.parse(CAD_E2E_ROUTES_MODEL_SPACE_JSON) as ModelSpaceJson;
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
