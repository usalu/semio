import { readFileSync, writeFileSync } from "node:fs";

let s = readFileSync("spatial/js/core/index.ts", "utf8");

s = s.replaceAll("KernelAdapter", "SpatialKernel");
s = s.replace(
	"StateEngine` + `KernelAdapter`",
	"StateEngine` + `SpatialKernel`",
);
s = s.replace(
	/export interface ExprEnv \{[\s\S]*?readonly derived\?: DerivedViewService;\n\}/,
	`export interface ExprEnv {
	readonly context: Record<string, unknown>;
	readonly event?: Record<string, unknown>;
	readonly vars?: Record<string, unknown>;
	readonly topology?: TopologyGraph;
	readonly metadata?: EntityMetadataStore;
	readonly derived?: DerivedViewService;
	readonly preview: SpatialPreviewKernel;
}`,
);
s = s.replace(
	"function envWithVars(base: ExprEnv, vars: Record<string, unknown>): ExprEnv {\n\treturn {\n\t\tcontext: base.context,\n\t\tevent: base.event,\n\t\tvars: { ...base.vars, ...vars },\n\t\ttopology: base.topology,\n\t\tmetadata: base.metadata,\n\t\tderived: base.derived,\n\t};\n}",
	`function envWithVars(base: ExprEnv, vars: Record<string, unknown>): ExprEnv {
	return {
		context: base.context,
		event: base.event,
		vars: { ...base.vars, ...vars },
		topology: base.topology,
		metadata: base.metadata,
		derived: base.derived,
		preview: base.preview,
	};
}`,
);
s = s.replace(
	"return typeof v === \"number\" ? Math.abs(v) : undefined;",
	"return typeof v === \"number\" ? env.preview.abs(v) : undefined;",
);
s = s.replace(
	"return vec3Distance(va, vb);",
	"return env.preview.vec3Distance(va, vb);",
);
s = s.replace(
	`return expr.op === "min"
				? Math.min(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env)))
				: Math.max(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env)));`,
	`return expr.op === "min"
				? env.preview.min2(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env)))
				: env.preview.max2(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env)));`,
);
s = s.replace(
	"opts?: { readonly derived?: DerivedViewService }",
	"opts?: { readonly derived?: DerivedViewService; readonly preview?: SpatialPreviewKernel }",
);
s = s.replace(
	"if (name === \"position\") return evaluateAnchorPosition(topo, anchor);",
	"if (name === \"position\") return opts?.preview?.evaluateAnchorPosition(topo, anchor) ?? anchor.position;",
);
s = s.replace(
	`ctx: { readonly kernel: SpatialKernel; readonly topology: TopologyGraph }`,
	`ctx: { readonly kernel: SpatialKernel; readonly preview: SpatialPreviewKernel; readonly topology: TopologyGraph }`,
);
s = s.replace(
	/constructor\(private readonly kernel\?: SpatialKernel\) \{\}/,
	"constructor(private readonly kernel: SpatialKernel) {}",
);
s = s.replace(
	"const sr = this.kernel?.computeSurfaceViews?.(topo);\n\t\tthis.surfaces = sr ? await Promise.resolve(sr) : computeSurfaceViewsFromTopology(topo);",
	"this.surfaces = await Promise.resolve(this.kernel.computeSurfaceViews(topo));",
);
s = s.replace(
	"const pr = this.kernel?.computePartViews?.(topo);\n\t\tthis.parts = pr ? await Promise.resolve(pr) : computePartViewsFromTopology(topo);",
	"this.parts = await Promise.resolve(this.kernel.computePartViews(topo));",
);
s = s.replace(
	`computeSurfaces(topo: TopologyGraph): SurfaceView[] {
		if (this.surfaceRevision === topo.revision) return this.surfaces;
		const r = this.kernel?.computeSurfaceViews?.(topo);
		if (r && typeof (r as Promise<SurfaceView[]>).then === "function") return this.surfaces;
		this.surfaces = Array.isArray(r) ? r : computeSurfaceViewsFromTopology(topo);
		this.surfaceRevision = topo.revision;
		return this.surfaces;
	}`,
	`computeSurfaces(topo: TopologyGraph): SurfaceView[] {
		if (this.surfaceRevision === topo.revision) return this.surfaces;
		return this.surfaces;
	}`,
);
s = s.replace(
	`computeParts(topo: TopologyGraph): PartView[] {
		if (this.partRevision === topo.revision) return this.parts;
		const r = this.kernel?.computePartViews?.(topo);
		if (r && typeof (r as Promise<PartView[]>).then === "function") return this.parts;
		this.parts = Array.isArray(r) ? r : computePartViewsFromTopology(topo);
		this.partRevision = topo.revision;
		return this.parts;
	}`,
	`computeParts(topo: TopologyGraph): PartView[] {
		if (this.partRevision === topo.revision) return this.parts;
		return this.parts;
	}`,
);
s = s.replace(
	`return readTopologyEntityProperty(topo, env.metadata, o.kind, o.id, expr.name, { derived: env.derived });`,
	`return readTopologyEntityProperty(topo, env.metadata, o.kind, o.id, expr.name, { derived: env.derived, preview: env.preview });`,
);

const runtimeHelper = `
	private computeMode(): SpatialComputeMode {
		return this.opts.mode ?? "precise";
	}

	private previewMath(): SpatialPreviewKernel {
		const mode = this.computeMode();
		if (mode === "fast") {
			const pk = this.opts.previewKernel;
			if (!pk) throw new Error("InteractionRuntimeOptions.previewKernel is required when mode is fast");
			return pk;
		}
		return this.opts.kernel;
	}

	private exprEnv(extra?: Partial<ExprEnv>): ExprEnv {
		return {
			context: this.sm.getContext(),
			preview: this.previewMath(),
			...extra,
		};
	}
`;

s = s.replace(
	"private cloneCtx(c: Record<string, unknown>): Record<string, unknown> {",
	runtimeHelper + "\n\tprivate cloneCtx(c: Record<string, unknown>): Record<string, unknown> {",
);

s = s.replace(
	"return evalGuard(g, { context: this.sm.getContext() });",
	"return evalGuard(g, this.exprEnv());",
);
s = s.replace(
	"const env: ExprEnv = { context: ctx };",
	"const env: ExprEnv = { context: ctx, preview: this.previewMath() };",
);
s = s.replace(
	"const ar = await Promise.resolve(def.run(paramBag, { kernel: k, topology: topo }));",
	"const ar = await Promise.resolve(def.run(paramBag, { kernel: k, preview: this.previewMath(), topology: topo }));",
);

writeFileSync("spatial/js/core/index.ts", s);
console.log("fixed refs");
