/** 📏️ Measure distance and measure area interactions. */
import { FaceRef, VertexRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { preciseSpatialKernelMath } from "@semio-tech/s-spatial-kernel-ts/preview";
import type { ArtifactTestDependencies } from "../../../🎒️test-dependencies/🟦️.ts";
import type { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import type { SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";

export async function registerMeasureTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ArtifactTestDependencies): Promise<void> {
  const { Model, __artifactTestKernel, applyModelDiff, buildAreaInteractionSpec, buildDistanceInteractionSpec, createInteractionRuntime, emptyMeshTransfer, isEmptyModelDiff, modelDefinitionActionRegistry, runRegisteredAction, solidRef } = dependencies;
  const { BrepjsKernel } = __artifactTestKernel!;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = vitest;

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
}
