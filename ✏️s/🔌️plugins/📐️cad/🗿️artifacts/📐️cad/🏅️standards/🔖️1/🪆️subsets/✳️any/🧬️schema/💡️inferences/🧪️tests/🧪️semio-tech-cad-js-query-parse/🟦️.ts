type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { Model, __spatialQueryTestKernel, __spatialQueryTestRuntime, applyModelDiff, emptyMeshTransfer, modelDefinitionActionRegistry, parseConstruct, runConstruct, solidRef } = dependencies;
  type FaceRef = any;
  type ObjectRef = any;
  type SelectionTarget = any;
  type ShellRef = any;
  type SolidRef = any;
  type SpatialKernel = any;
  type TypologyRef = any;
  type Vec3 = any;

  __spatialQueryTestRuntime!.bootstrapCadModules();
  const { BrepjsKernel, preciseSpatialKernelMath } = __spatialQueryTestKernel!;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = vitest;

  class QueryTestKernel extends BrepjsKernel {
    override readonly id = "stub-k";
    override readonly operations = [] as const;
    override async createBoxFromCorners(_input: { cornerA: Vec3; cornerB: Vec3; height: number }) {
      return "c0" as SolidRef;
    }
    override async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }) {
      const cell = await this.createBoxFromCorners(input);
      return { solid: cell, diff: this.boxModelDiff(input, cell) };
    }
    override async volume() {
      return 0;
    }
    override async tessellate() {
      return emptyMeshTransfer();
    }
  }

  function mkKernelStub(): SpatialKernel {
    return new QueryTestKernel();
  }

  function seedSolidShellFaces(model: Model): { solid: string; shell: string; faces: string[] } {
    const f0 = "f0" as FaceRef;
    const f1 = "f1" as FaceRef;
    const sh = "s0" as ShellRef;
    const c0 = "c0" as SolidRef;
    model.faces[f0] = { id: f0, wireIds: [] };
    model.faces[f1] = { id: f1, wireIds: [] };
    model.shells[sh] = { id: sh, faceIds: [f0, f1] };
    model.solids[c0] = { id: c0, shellIds: [sh] };
    return { solid: c0, shell: sh, faces: [f0, f1] };
  }

  describe("@semio-tech/cad-js/query parse", () => {
    it("parses MATCH RETURN with property access", () => {
      const a = parseConstruct("MATCH (f:Object {typology: 'spatial.shape.kernel.face', id: 'f0'}) RETURN f.id");
      expect(a.clauses[0]?.kind).toBe("match");
      expect(a.returnClause?.projections.length).toBe(1);
    });
    it("parses RETURN LIMIT without ORDER BY", () => {
      const a = parseConstruct("MATCH (v:Object {typology: 'spatial.shape.kernel.vertex'}) RETURN v.id LIMIT 3");
      expect(a.returnClause?.limit).toBe(3);
    });
    it("parses CALL with object literal and YIELD", () => {
      const a = parseConstruct("CALL primitive.createBoxFromCorners({ cornerA: [0,0,0], cornerB: [2,3,0], height: 4 }) YIELD diff");
      const c = a.clauses[0];
      expect(c?.kind).toBe("call");
      if (c?.kind === "call") {
        expect(c.actionId).toBe("primitive.createBoxFromCorners");
        expect(c.args.height).toBe(4);
        expect(c.yieldItems[0]?.key).toBe("diff");
      }
    });
    it("parses CALL transformation YIELD with AS alias", () => {
      const a = parseConstruct("CALL aec.building.energy.from_geometry({}) YIELD objects AS energyObjects");
      const c = a.clauses[0];
      expect(c?.kind).toBe("call");
      if (c?.kind === "call") {
        expect(c.actionId).toBe("aec.building.energy.from_geometry");
        expect(c.yieldItems[0]).toEqual({ key: "objects", alias: "energyObjects" });
      }
    });
    it("parses UNWIND with WHERE", () => {
      const a = parseConstruct("UNWIND surfaces AS s WHERE s.exposure = 'external' RETURN s.id");
      expect(a.clauses[0]?.kind).toBe("unwind");
    });
    it("rejects unknown node labels", () => {
      expect(() => parseConstruct("MATCH (f:Face {id: 'f0'}) RETURN f.id")).toThrow(/unknown node label Face/);
      expect(() => parseConstruct("MATCH (m:Model) RETURN m.id")).toThrow(/unknown node label Model/);
      expect(() => parseConstruct("MATCH (s:Surface) RETURN s.id")).toThrow(/unknown node label Surface/);
    });
    it("rejects MATCH on non-shipped typology ids", () => {
      expect(() => parseConstruct("MATCH (o:Object {typology: 'energy.derived.hull'}) RETURN o.id")).toThrow(/not a shipped model typology/);
      expect(() => parseConstruct("MATCH (o:Object {typology: 'unknown.typology.foo'}) RETURN o.id")).toThrow(/unknown typology/);
    });
  });

  describe("@semio-tech/cad-js/query execute", () => {
    it("MATCH solid shell face chain returns face ids", async () => {
      const model = new Model();
      seedSolidShellFaces(model);
      const q = `MATCH (c:Object {typology: 'spatial.shape.kernel.solid'})-[:BOUNDED_BY]->(:Object {typology: 'spatial.shape.kernel.shell'})-[:CONTAINS]->(f:Object {typology: 'spatial.shape.kernel.face'}) RETURN f.id`;
      const res = await runConstruct(q, {
        model: model,
        kernel: mkKernelStub(),
        actions: modelDefinitionActionRegistry(),
      });
      const ids = res.rows.map((r) => r.c0).sort();
      expect(ids).toEqual(["f0", "f1"]);
    });

    it("ADJACENT_TO finds solids sharing a face", async () => {
      const model = new Model();
      const fShared = "fs" as FaceRef;
      const sh0 = "s0" as ShellRef;
      const sh1 = "s1" as ShellRef;
      model.faces[fShared] = { id: fShared, wireIds: [] };
      model.shells[sh0] = { id: sh0, faceIds: [fShared] };
      model.shells[sh1] = { id: sh1, faceIds: [fShared] };
      model.solids["c0" as SolidRef] = { id: "c0" as SolidRef, shellIds: [sh0] };
      model.solids["c1" as SolidRef] = { id: "c1" as SolidRef, shellIds: [sh1] };
      const res = await runConstruct("MATCH (a:Object {typology: 'spatial.shape.kernel.solid'})-[:ADJACENT_TO]-(b:Object {typology: 'spatial.shape.kernel.solid'}) RETURN a.id, b.id", {
        model: model,
        kernel: mkKernelStub(),
        actions: modelDefinitionActionRegistry(),
      });
      expect(res.rows.length).toBeGreaterThan(0);
      const pair = res.rows.find((r) => String(r.c0) === "c0" && String(r.c1) === "c1");
      expect(pair).toBeDefined();
    });

    it("CALL view.* is unknown action", async () => {
      const model = new Model();
      await expect(
        runConstruct("CALL view.energy.energy.hull({}) YIELD data", {
          model: model,
          kernel: new QueryTestKernel(),
          actions: modelDefinitionActionRegistry(),
        }),
      ).rejects.toThrow(/unknown action/i);
    });

    it("unknown CALL action throws", async () => {
      const model = new Model();
      await expect(
        runConstruct("CALL no.such.action({}) YIELD data", {
          model: model,
          kernel: new QueryTestKernel(),
          actions: modelDefinitionActionRegistry(),
        }),
      ).rejects.toThrow(/unknown action/i);
    });

    it("CALL createBoxFromCorners yields diff and data.solid", async () => {
      const model = new Model();
      const res = await runConstruct("CALL primitive.createBoxFromCorners({ cornerA: [0,0,0], cornerB: [2,3,0], height: 4 }) YIELD diff, data.solid AS solid", {
        model: model,
        kernel: new QueryTestKernel(),
        actions: modelDefinitionActionRegistry(),
      });
      expect(res.diff).toBeDefined();
      expect(res.diff?.solids?.added?.length).toBeGreaterThan(0);
      expect(res.rows[0]?.solid).toBeDefined();
    });

    it("CALL selection.selectAll YIELD targets returns every box model kind", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const actions = modelDefinitionActionRegistry();
      expect(actions.get("selection.selectAll")).not.toBeNull();
      const res = await runConstruct("CALL selection.selectAll({}) YIELD targets", {
        model: model,
        kernel: new QueryTestKernel(),
        actions,
      });
      const targets = res.rows[0]?.targets as { kind: string; id: string }[] | undefined;
      expect(Array.isArray(targets)).toBe(true);
      expect(targets!.length).toBeGreaterThan(8);
      expect(targets!.some((t) => t.kind === "solid" && t.id === "box")).toBe(true);
    });

    it("CALL selection.apply invert uses construct selectionTargets seed", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const seed: readonly SelectionTarget[] = [{ kind: "solid", id: "box", editable: true }];
      const res = await runConstruct("CALL selection.apply({ operation: 'invert' }) YIELD data.targets AS targets", {
        model: model,
        kernel: new QueryTestKernel(),
        actions: modelDefinitionActionRegistry(),
        selectionTargets: seed,
      });
      const targets = res.rows[0]?.targets as { kind: string; id: string }[] | undefined;
      expect(targets!.some((t) => t.kind === "solid" && t.id === "box")).toBe(false);
      expect(targets!.length).toBeGreaterThan(0);
    });

    it("CALL selection.selectVertices YIELD targets lists only vertices", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const res = await runConstruct("CALL selection.selectVertices({}) YIELD targets", {
        model: model,
        kernel: new QueryTestKernel(),
        actions: modelDefinitionActionRegistry(),
      });
      const targets = res.rows[0]?.targets as { kind: string; id: string }[] | undefined;
      expect(targets?.length).toBe(8);
      expect(targets?.every((t) => t.kind === "vertex")).toBe(true);
    });

    it("rejects actions outside active model definition", async () => {
      const model = new Model();
      await expect(
        runConstruct("CALL primitive.createBoxFromCorners({ cornerA: [0,0,0], cornerB: [1,1,0], height: 1 })", {
          model: model,
          kernel: new QueryTestKernel(),
          actions: modelDefinitionActionRegistry(),
          activeModelDefinitionId: "aec.building.energy",
        }),
      ).rejects.toThrow(/not available in model definition aec\.building\.energy/);
    });

    it("rejects geometry selection commands under energy model definition", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      await expect(
        runConstruct("CALL selection.selectVertices({}) YIELD targets", {
          model: model,
          kernel: new QueryTestKernel(),
          actions: modelDefinitionActionRegistry(),
          activeModelDefinitionId: "aec.building.energy",
        }),
      ).rejects.toThrow(/not available in model definition aec\.building\.energy/);
    });

    it("rejects geometry typology MATCH under energy model definition", async () => {
      const model = new Model();
      await expect(
        runConstruct("MATCH (o:Object {typology: 'spatial.shape.primitive.box'}) RETURN o.id", {
          model: model,
          kernel: mkKernelStub(),
          actions: modelDefinitionActionRegistry(),
          activeModelDefinitionId: "aec.building.energy",
        }),
      ).rejects.toThrow(/unknown typology spatial\.shape\.primitive\.box for model definition aec\.building\.energy/);
    });

    it("MATCH energy typology resolves to object rows", async () => {
      const model = new Model();
      const solid = solidRef("box");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      model.objects["energy-hull"] = {
        id: "energy-hull" as ObjectRef,
        typology: "energy.energy.hull" as TypologyRef,
        primitives: { solid: String(solid) },
      };
      const res = await runConstruct("MATCH (o:Object {typology: 'energy.energy.hull'}) RETURN o.id AS id", {
        model: model,
        kernel: mkKernelStub(),
        actions: modelDefinitionActionRegistry(),
        activeModelDefinitionId: "aec.building.energy",
      });
      expect(res.rows.some((row) => row.id === "energy-hull")).toBe(true);
    });

    it("MATCH spatial.shape.primitive.box typology resolves to solids", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const res = await runConstruct("MATCH (s:Object {typology: 'spatial.shape.primitive.box'}) RETURN s.id", {
        model: model,
        kernel: mkKernelStub(),
        actions: modelDefinitionActionRegistry(),
      });
      expect(res.rows.length).toBeGreaterThanOrEqual(1);
      expect(res.rows.some((r) => typeof r.c0 === "string")).toBe(true);
    });

    it("CALL selection.selectObjects YIELD targets from model objects", async () => {
      const model = new Model();
      const solid = solidRef("box");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      model.objects["object-box"] = {
        id: "object-box" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: String(solid) },
      };
      const kernel = new QueryTestKernel();
      const res = await runConstruct("CALL selection.selectObjects({}) YIELD targets", {
        model: model,
        kernel,
        actions: modelDefinitionActionRegistry(),
      });
      const targets = res.rows[0]?.targets as { kind: string }[] | undefined;
      expect(targets!.length).toBeGreaterThan(0);
      expect(targets!.every((t) => t.kind === "object")).toBe(true);
    });
  });

}
