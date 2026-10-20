/** 📦️ The cad-js core box interaction tests. */
import type { SolidRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { ModelDiff, SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { MeshTransfer, Vec3 } from "@semio-tech/framework/3d";
import type { BoxLengthTestKit } from "../🧰️box-length-test-kit/🟦️.ts";

/** 🧊 Registers the cad-js core box interaction describe block. */
export function registerBoxInteractionTests(kit: BoxLengthTestKit): void {
  const { BrepjsKernel, M, Model, buildBoxInteractionSpec, createInteractionRuntime, describe, emptyMeshTransfer, expect, it, resolveDisplay, solidRef } = kit;
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
}
