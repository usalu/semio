// #region 🧲Header
/** @emoji 🧭 `@spatial/js-core` — portable factory spec runtime, topology graph, `KernelAdapter` contract, derived views. See `spatial/schema/json` and `.repo/✍️/spatial.md`. */
// #endregion 🧲Header

// #region 📥Fixtures
import boxFactoryJson from "../../fixtures/factory.json" with { type: "json" };
import extrudeFactoryJson from "../../fixtures/extrude.factory.json" with { type: "json" };
import offsetSurfaceFactoryJson from "../../fixtures/offset-surface.factory.json" with { type: "json" };
// #endregion 📥Fixtures

// #region 🧮Vec
/** @emoji 📐 Column vector `[x,y,z]` used by spatial factories. */
export type Vec3 = readonly [number, number, number];

/** @emoji 📏 `a+b` component-wise for `Vec3`. */
export function vec3Add(a: Vec3, b: Vec3): Vec3 {
	return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

/** @emoji 📏 `a-b` component-wise for `Vec3`. */
export function vec3Sub(a: Vec3, b: Vec3): Vec3 {
	return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

/** @emoji 📏 Scales a `Vec3` by scalar `s`. */
export function vec3Scale(a: Vec3, s: number): Vec3 {
	return [a[0] * s, a[1] * s, a[2] * s];
}

/** @emoji 📏 Dot product of two `Vec3`. */
export function vec3Dot(a: Vec3, b: Vec3): number {
	return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

/** @emoji 📏 Cross product `a×b`. */
export function vec3Cross(a: Vec3, b: Vec3): Vec3 {
	return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
}

/** @emoji 📏 Euclidean length of `Vec3`. */
export function vec3Length(a: Vec3): number {
	return Math.hypot(a[0], a[1], a[2]);
}

/** @emoji 📏 Euclidean distance between two `Vec3`. */
export function vec3Distance(a: Vec3, b: Vec3): number {
	return vec3Length(vec3Sub(b, a));
}

/** @emoji 📏 Normalizes to unit length when non-zero; otherwise returns `[0,0,1]`. */
export function vec3Normalize(a: Vec3): Vec3 {
	const l = vec3Length(a);
	if (l < 1e-12) return [0, 0, 1];
	return [a[0] / l, a[1] / l, a[2] / l];
}
// #endregion 🧮Vec

// #region 🪪Refs
/** @emoji 🪪 Opaque branded string ids for editable topology entities. */
export type VertexRef = string & { readonly __brand: "VertexRef" };
export type EdgeRef = string & { readonly __brand: "EdgeRef" };
export type WireRef = string & { readonly __brand: "WireRef" };
export type FaceRef = string & { readonly __brand: "FaceRef" };
export type ShellRef = string & { readonly __brand: "ShellRef" };
export type CellRef = string & { readonly __brand: "CellRef" };
export type CellComplexRef = string & { readonly __brand: "CellComplexRef" };
export type ClusterRef = string & { readonly __brand: "ClusterRef" };

/** @emoji 🪞 Derived semantic ids (`Surface`, `Part`) — never directly mutated by factories. */
export type SurfaceRef = string & { readonly __brand: "SurfaceRef" };
export type PartRef = string & { readonly __brand: "PartRef" };

/** @emoji 🧱 Editable topology kinds from `spatial/AGENTS.md`. */
export type EditableEntityKind =
	| "vertex"
	| "edge"
	| "wire"
	| "face"
	| "shell"
	| "cell"
	| "cellComplex"
	| "cluster";

/** @emoji 🪞 Derived topology kinds. */
export type DerivedEntityKind = "surface" | "part";

/** @emoji 🧭 Any addressable topology or derived view kind. */
export type TopologyEntityKind = EditableEntityKind | DerivedEntityKind;

/** @emoji 🪪 Builds a branded `CellRef` from an opaque id string. */
export function cellRef(id: string): CellRef {
	return id as CellRef;
}
// #endregion 🪪Refs

// #region 🗺️Expr
/** @emoji 🗺️ JSON-serializable expression AST evaluated by `evalExpr`. */
export type Expr = Record<string, unknown>;

export interface ExprEnv {
	readonly context: Record<string, unknown>;
	readonly event?: Record<string, unknown>;
	readonly vars?: Record<string, unknown>;
}

function envWithVars(base: ExprEnv, vars: Record<string, unknown>): ExprEnv {
	return { context: base.context, event: base.event, vars: { ...base.vars, ...vars } };
}

/** @emoji 🧭 Reads `path` like `a.b.0` from a nested plain object / array value `root`. */
export function getPath(root: unknown, path: string): unknown {
	const segs = path.split(".").filter(Boolean);
	let cur: unknown = root;
	for (const s of segs) {
		if (cur === null || cur === undefined) return undefined;
		if (Array.isArray(cur)) {
			const i = Number(s);
			if (!Number.isInteger(i)) return undefined;
			cur = cur[i];
			continue;
		}
		if (typeof cur === "object") {
			cur = (cur as Record<string, unknown>)[s];
			continue;
		}
		return undefined;
	}
	return cur;
}

/** @emoji 🧭 Writes `value` at `path` (creates object shells as needed). */
export function setPath(root: Record<string, unknown>, path: string, value: unknown): void {
	const segs = path.split(".").filter(Boolean);
	if (segs.length === 0) return;
	let cur: Record<string, unknown> | unknown[] = root;
	for (let i = 0; i < segs.length - 1; i++) {
		const s = segs[i]!;
		const next = segs[i + 1]!;
		const isNextIndex = Array.isArray(cur) ? Number.isInteger(Number(next)) : /^\d+$/.test(next);
		if (Array.isArray(cur)) {
			const idx = Number(s);
			const child = cur[idx];
			if (child === undefined || child === null) {
				const slot: Record<string, unknown> = {};
				cur[idx] = isNextIndex ? ([] as unknown[]) : slot;
			}
			cur = cur[idx] as Record<string, unknown> | unknown[];
			continue;
		}
		const o = cur as Record<string, unknown>;
		let child = o[s];
		if (child === undefined || child === null || typeof child !== "object") {
			o[s] = isNextIndex ? ([] as unknown[]) : {};
			child = o[s];
		}
		cur = child as Record<string, unknown> | unknown[];
	}
	const last = segs[segs.length - 1]!;
	if (Array.isArray(cur)) {
		cur[Number(last)] = value;
		return;
	}
	(cur as Record<string, unknown>)[last] = value;
}

/** @emoji 🧮 Evaluates a declarative `Expr` against `ExprEnv` (guards + action values). */
export function evalExpr(expr: Expr, env: ExprEnv): unknown {
	if (expr === null || typeof expr !== "object") return expr as unknown;
	if ("const" in expr) return (expr as { const: unknown }).const;
	if ("path" in expr) {
		const p = (expr as { path: string }).path;
		return getPath(env.context, p);
	}
	if ("$event" in expr) {
		const k = (expr as { $event: string }).$event;
		return env.event ? getPath(env.event, k) : undefined;
	}
	if ("var" in expr) {
		const k = (expr as { var: string }).var;
		return env.vars ? env.vars[k] : undefined;
	}
	if ("let" in expr && "in" in expr) {
		const { let: bindings, in: inner } = expr as { let: Record<string, Expr>; in: Expr };
		const next: Record<string, unknown> = {};
		for (const [k, v] of Object.entries(bindings)) {
			next[k] = evalExpr(v, env);
		}
		return evalExpr(inner, envWithVars(env, next));
	}
	if ("exists" in expr) {
		const inner = (expr as { exists: { path: string } }).exists;
		const v = getPath(env.context, inner.path);
		return v !== undefined && v !== null;
	}
	if ("notEmpty" in expr) {
		const inner = (expr as { notEmpty: { path: string } }).notEmpty;
		const v = getPath(env.context, inner.path);
		if (v === undefined || v === null) return false;
		if (Array.isArray(v)) return v.length > 0;
		if (typeof v === "string") return v.length > 0;
		return true;
	}
	if ("all" in expr) {
		const xs = (expr as { all: Expr[] }).all;
		return xs.every((x) => Boolean(evalExpr(x, env)));
	}
	if ("any" in expr) {
		const xs = (expr as { any: Expr[] }).any;
		return xs.some((x) => Boolean(evalExpr(x, env)));
	}
	if ("not" in expr) {
		return !Boolean(evalExpr((expr as { not: Expr }).not, env));
	}
	if ("abs" in expr) {
		const v = evalExpr((expr as { abs: Expr }).abs, env);
		return typeof v === "number" ? Math.abs(v) : undefined;
	}
	if ("distance" in expr) {
		const { a, b } = (expr as { distance: { a: Expr; b: Expr } }).distance;
		const va = evalExpr(a, env);
		const vb = evalExpr(b, env);
		if (!isVec3(va) || !isVec3(vb)) return undefined;
		return vec3Distance(va, vb);
	}
	const binKeys = ["==", "!=", ">", "<", ">=", "<=", "+", "-", "*", "/"] as const;
	for (const k of binKeys) {
		if (k in expr) {
			const pair = (expr as Record<string, [Expr, Expr]>)[k];
			const left = evalExpr(pair[0], env);
			const right = evalExpr(pair[1], env);
			switch (k) {
				case "==":
					return left === right;
				case "!=":
					return left !== right;
				case ">":
					return Number(left) > Number(right);
				case "<":
					return Number(left) < Number(right);
				case ">=":
					return Number(left) >= Number(right);
				case "<=":
					return Number(left) <= Number(right);
				case "+":
					return Number(left) + Number(right);
				case "-":
					return Number(left) - Number(right);
				case "*":
					return Number(left) * Number(right);
				case "/":
					return Number(right) === 0 ? undefined : Number(left) / Number(right);
				default:
					return undefined;
			}
		}
	}
	return undefined;
}

function isVec3(v: unknown): v is Vec3 {
	return Array.isArray(v) && v.length === 3 && v.every((n) => typeof n === "number");
}

/** @emoji 🧭 Coerces `evalExpr` output to strict boolean guard result. */
export function evalGuard(expr: Expr, env: ExprEnv): boolean {
	return Boolean(evalExpr(expr, env));
}
// #endregion 🗺️Expr

// #region 📜Spec
/** @emoji 📜 Parsed static factory document (`spatial.factory/v1`). */
export interface FactorySpec {
	readonly schema: "spatial.factory/v1";
	readonly id: string;
	readonly version: string;
	readonly label?: string;
	readonly requires?: Record<string, unknown>;
	readonly guards?: Record<string, Expr>;
	readonly history?: { excludeEvents?: readonly string[] };
	readonly machine: {
		readonly initial: string;
		readonly states: Record<
			string,
			{
				readonly final?: boolean;
				readonly on?: Record<string, TransitionSpec>;
			}
		>;
	};
	readonly display?: {
		readonly states?: Record<string, readonly DisplayItemSpec[]>;
	};
	readonly commit: {
		readonly when?: string;
		readonly operation: { readonly kind: string; readonly params: Record<string, unknown> };
	};
}

export interface TransitionSpec {
	readonly target?: string;
	readonly guard?: string;
	readonly transient?: boolean;
	readonly actions?: readonly ActionSpec[];
}

export interface ActionSpec {
	readonly op: string;
	readonly path?: string;
	readonly value?: unknown;
	readonly query?: string;
	readonly assignTo?: string;
	readonly params?: Record<string, unknown>;
	readonly severity?: string;
	readonly code?: string;
	readonly message?: string;
}

export interface DisplayItemSpec {
	readonly kind: string;
	readonly id: string;
	readonly role?: string;
	readonly params?: Record<string, unknown>;
}

/** @emoji 🧾 Validates and returns a `FactorySpec` or `null` when malformed. */
export function parseFactorySpec(raw: unknown): FactorySpec | null {
	if (!raw || typeof raw !== "object") return null;
	const r = raw as Record<string, unknown>;
	if (r.schema !== "spatial.factory/v1") return null;
	if (typeof r.id !== "string" || typeof r.version !== "string") return null;
	const machine = r.machine;
	if (!machine || typeof machine !== "object") return null;
	const m = machine as Record<string, unknown>;
	if (typeof m.initial !== "string" || !m.states || typeof m.states !== "object") return null;
	const commit = r.commit;
	if (!commit || typeof commit !== "object") return null;
	const c = commit as Record<string, unknown>;
	const op = c.operation;
	if (!op || typeof op !== "object") return null;
	const o = op as Record<string, unknown>;
	if (typeof o.kind !== "string" || !o.params || typeof o.params !== "object") return null;
	return r as unknown as FactorySpec;
}

/** @emoji 🧭 Normalizes a parsed factory (currently identity). */
export function compileFactory(spec: FactorySpec): FactorySpec {
	return spec;
}
// #endregion 📜Spec

// #region 🧱Topology
/** @emoji 🧱 Vertex payload: point geometry attached to topology. */
export interface VertexRecord {
	readonly id: VertexRef;
	readonly position: Vec3;
}

/** @emoji 🧱 Edge payload: two vertices plus curve geometry (Topologic-style). */
export interface EdgeRecord {
	readonly id: EdgeRef;
	readonly vertexA: VertexRef;
	readonly vertexB: VertexRef;
	readonly curve: { readonly kind: "line" | "polyline" | "bezier"; readonly controls: readonly Vec3[] };
}

/** @emoji 🧱 Wire payload: ordered edges and closure flag. */
export interface WireRecord {
	readonly id: WireRef;
	readonly edgeIds: readonly EdgeRef[];
	readonly closed: boolean;
}

/** @emoji 🧱 Face payload: outer wire, optional holes, planar or mesh surface geometry. */
export interface FaceRecord {
	readonly id: FaceRef;
	readonly outerWireId: WireRef;
	readonly holeWireIds?: readonly WireRef[];
	readonly surface:
		| { readonly kind: "planar"; readonly normal: Vec3; readonly origin: Vec3 }
		| { readonly kind: "mesh"; readonly vertices: readonly Vec3[]; readonly triangles: readonly [number, number, number][] };
}

/** @emoji 🧱 Shell payload: connected faces. */
export interface ShellRecord {
	readonly id: ShellRef;
	readonly faceIds: readonly FaceRef[];
}

/** @emoji 🧱 Cell payload: bounded volume via closed shells. */
export interface CellRecord {
	readonly id: CellRef;
	readonly shellIds: readonly ShellRef[];
}

/** @emoji 🧱 Cell complex payload: cells plus shared-face bookkeeping. */
export interface CellComplexRecord {
	readonly id: CellComplexRef;
	readonly cellIds: readonly CellRef[];
	readonly sharedFaceIds: readonly FaceRef[];
}

/** @emoji 🧱 Cluster payload: arbitrary nested membership. */
export interface ClusterRecord {
	readonly id: ClusterRef;
	readonly memberIds: readonly string[];
}

/** @emoji 🗺️ Serializable topology graph (`spatial.topology/v1`). */
export interface TopologyGraphJson {
	readonly schema: "spatial.topology/v1";
	readonly revision: number;
	readonly vertices: Record<string, VertexRecord>;
	readonly edges: Record<string, EdgeRecord>;
	readonly wires: Record<string, WireRecord>;
	readonly faces: Record<string, FaceRecord>;
	readonly shells: Record<string, ShellRecord>;
	readonly cells: Record<string, CellRecord>;
	readonly cellComplexes: Record<string, CellComplexRecord>;
	readonly clusters: Record<string, ClusterRecord>;
}

/** @emoji 🧱 Mutable in-memory topology graph with revision counter. */
export class TopologyGraph {
	revision = 0;
	vertices: Record<string, VertexRecord> = {};
	edges: Record<string, EdgeRecord> = {};
	wires: Record<string, WireRecord> = {};
	faces: Record<string, FaceRecord> = {};
	shells: Record<string, ShellRecord> = {};
	cells: Record<string, CellRecord> = {};
	cellComplexes: Record<string, CellComplexRecord> = {};
	clusters: Record<string, ClusterRecord> = {};

	/** @emoji 🧭 Serializes to `TopologyGraphJson`. */
	toJSON(): TopologyGraphJson {
		return {
			schema: "spatial.topology/v1",
			revision: this.revision,
			vertices: { ...this.vertices },
			edges: { ...this.edges },
			wires: { ...this.wires },
			faces: { ...this.faces },
			shells: { ...this.shells },
			cells: { ...this.cells },
			cellComplexes: { ...this.cellComplexes },
			clusters: { ...this.clusters },
		};
	}

	/** @emoji 🧭 Hydrates from `TopologyGraphJson`. */
	static fromJSON(j: TopologyGraphJson): TopologyGraph {
		const g = new TopologyGraph();
		g.revision = j.revision;
		g.vertices = { ...j.vertices };
		g.edges = { ...j.edges };
		g.wires = { ...j.wires };
		g.faces = { ...j.faces };
		g.shells = { ...j.shells };
		g.cells = { ...j.cells };
		g.cellComplexes = { ...j.cellComplexes };
		g.clusters = { ...j.clusters };
		return g;
	}

	bump(): void {
		this.revision += 1;
	}
}

/** @emoji 🧾 Parses `spatial.topology/v1` JSON into a graph or returns `null`. */
export function parseTopologyGraphJson(raw: unknown): TopologyGraph | null {
	if (!raw || typeof raw !== "object") return null;
	const r = raw as Record<string, unknown>;
	if (r.schema !== "spatial.topology/v1") return null;
	return TopologyGraph.fromJSON(raw as TopologyGraphJson);
}
// #endregion 🧱Topology

// #region 🔌Kernel
/** @emoji 🖼️ Renderer-neutral mesh preview (positions + triangle indices). */
export interface MeshPreview {
	readonly positions: Float32Array;
	readonly indices: Uint32Array;
	readonly normals?: Float32Array;
}

/** @emoji 🔌 Kernel capability surface executed by factory commits. */
export interface KernelAdapter {
	readonly id: string;
	readonly operations: readonly string[];
	createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<CellRef>;
	volume(cell: CellRef): Promise<number>;
	tessellate(cell: CellRef, tolerance: number): Promise<MeshPreview>;
	query?(name: string, params: Record<string, unknown>): Promise<unknown>;
	extrudeWire?(input: { wireId: string; distance: number; direction: Vec3 }): Promise<CellRef>;
	offsetFaces?(input: { faceIds: readonly string[]; distance: number }): Promise<void>;
}
// #endregion 🔌Kernel

// #region 🪞DerivedViews
/** @emoji 🪞 Semantic `Surface` view over one or more faces (exposure × stance). */
export interface SurfaceView {
	readonly id: SurfaceRef;
	readonly sourceFaceIds: readonly FaceRef[];
	readonly exposure: "external" | "internal";
	readonly stance: "horizontal" | "vertical";
	readonly area: number;
}

/** @emoji 🪞 Semantic `Part` view over one or more cells (overlap classification). */
export interface PartView {
	readonly id: PartRef;
	readonly sourceCellIds: readonly CellRef[];
	readonly overlap: "none" | "difference" | "intersection";
	readonly volume: number;
}

/** @emoji 🪞 Computes derived `SurfaceView` / `PartView` projections from faces/cells. */
export class DerivedViewService {
	private surfaceRevision = -1;
	private partRevision = -1;
	private surfaces: SurfaceView[] = [];
	private parts: PartView[] = [];

	/** @emoji 🪞 Returns cached surfaces, recomputing when `topologyRevision` changes. */
	computeSurfaces(topologyRevision: number, faces: Record<string, FaceRecord>): SurfaceView[] {
		if (this.surfaceRevision === topologyRevision) return this.surfaces;
		const out: SurfaceView[] = [];
		for (const f of Object.values(faces)) {
			const n = f.surface.kind === "planar" ? f.surface.normal : [0, 0, 1];
			const stance = Math.abs(n[2]) > 0.707 ? ("horizontal" as const) : ("vertical" as const);
			const area = f.surface.kind === "planar" ? 1 : (f.surface.vertices.length > 0 ? 1 : 0);
			out.push({
				id: `surface-${f.id}` as SurfaceRef,
				sourceFaceIds: [f.id],
				exposure: "external",
				stance,
				area,
			});
		}
		this.surfaces = out;
		this.surfaceRevision = topologyRevision;
		return out;
	}

	/** @emoji 🪞 Returns cached parts for the given cells. */
	computeParts(topologyRevision: number, cells: Record<string, { id: CellRef }>): PartView[] {
		if (this.partRevision === topologyRevision) return this.parts;
		const out: PartView[] = [];
		for (const c of Object.values(cells)) {
			out.push({ id: `part-${c.id}` as PartRef, sourceCellIds: [c.id], overlap: "none", volume: 0 });
		}
		this.parts = out;
		this.partRevision = topologyRevision;
		return out;
	}

	async resolveSurface(surface: SurfaceRef, faces: Record<string, FaceRecord>): Promise<FaceRef[]> {
		const id = String(surface).replace(/^surface-/, "");
		if (faces[id]) return [faces[id]!.id];
		return [];
	}
}
// #endregion 🪞DerivedViews

// #region 🎬Statechart
/** @emoji 🧭 Factory input envelope; `kind` selects `machine.states[*].on` keys. */
export type FactoryEvent = { readonly kind: string; readonly [k: string]: unknown };

function resolveTemplate(value: unknown, env: ExprEnv): unknown {
	if (!value || typeof value !== "object") return value;
	const o = value as Record<string, unknown>;
	if ("path" in o && typeof o.path === "string") return getPath(env.context, o.path);
	if ("const" in o) return o.const;
	if ("$event" in o && typeof o.$event === "string") return env.event ? getPath(env.event, o.$event) : undefined;
	if ("let" in o && "in" in o) return evalExpr(o as Expr, env);
	const binKeys = ["==", "!=", ">", "<", ">=", "<=", "+", "-", "*", "/", "all", "any", "not", "exists", "notEmpty", "abs", "distance"] as const;
	for (const k of binKeys) {
		if (k in o) return evalExpr(o as Expr, env);
	}
	if (Array.isArray(value)) {
		return value.map((x) => resolveTemplate(x, env));
	}
	const out: Record<string, unknown> = {};
	for (const [k, v] of Object.entries(o)) {
		out[k] = resolveTemplate(v, env);
	}
	return out;
}

async function applyActionAsync(
	a: ActionSpec,
	ctx: Record<string, unknown>,
	event: FactoryEvent,
	kernel?: KernelAdapter,
): Promise<void> {
	if (a.op === "assign" && a.path) {
		const v = resolveTemplate(a.value, { context: ctx, event });
		setPath(ctx, a.path, v);
	} else if (a.op === "clear" && a.path) {
		delete ctx[a.path];
	} else if (a.op === "kernel.query" && a.query && a.assignTo && kernel?.query) {
		const params = resolveTemplate(a.params ?? {}, { context: ctx, event }) as Record<string, unknown>;
		const res = await kernel.query(a.query, params);
		setPath(ctx, a.assignTo, res);
	}
}

/** @emoji 🎬 Minimal async statechart runner for `FactorySpec.machine`. */
export class StatechartRuntime {
	private state: string;
	private context: Record<string, unknown> = {};

	constructor(private readonly spec: FactorySpec) {
		this.state = spec.machine.initial;
	}

	getState(): string {
		return this.state;
	}

	getContext(): Record<string, unknown> {
		return this.context;
	}

	reset(): void {
		this.state = this.spec.machine.initial;
		this.context = {};
	}

	/** @emoji 🎬 Restores a prior `state` + `context` snapshot (factory-local undo). */
	restore(state: string, context: Record<string, unknown>): void {
		this.state = state;
		this.context = context;
	}

	/** @emoji 🎬 Applies one external event; returns whether a transition fired. */
	async send(event: FactoryEvent, kernel?: KernelAdapter): Promise<{ ok: boolean; transient?: boolean }> {
		const st = this.spec.machine.states[this.state];
		if (!st?.on) return { ok: false };
		const tr = st.on[event.kind];
		if (!tr) return { ok: false };
		if (tr.guard) {
			const g = this.spec.guards?.[tr.guard];
			if (!g || !evalGuard(g, { context: this.context, event })) return { ok: false };
		}
		for (const a of tr.actions ?? []) {
			await applyActionAsync(a, this.context, event, kernel);
		}
		if (tr.target) this.state = tr.target;
		return { ok: true, transient: Boolean(tr.transient) };
	}
}
// #endregion 🎬Statechart

// #region 🖼️Display
/** @emoji 🖼️ Resolved display primitive for renderer adapters. */
export interface DisplayItem {
	readonly kind: string;
	readonly id: string;
	readonly role?: string;
	readonly params?: Record<string, unknown>;
}

/** @emoji 🖼️ Renderer-neutral snapshot slice consumed by `@spatial/js-renderer-r3f`. */
export interface DisplayModel {
	readonly prompt?: string;
	readonly items: readonly DisplayItem[];
}

/** @emoji 🖼️ Instantiates `display.states[state]` templates using current `context`. */
export function resolveDisplay(spec: FactorySpec, state: string, context: Record<string, unknown>): DisplayModel {
	const raw = spec.display?.states?.[state] ?? [];
	const items: DisplayItem[] = [];
	for (const it of raw) {
		items.push({
			kind: it.kind,
			id: it.id,
			...(it.role ? { role: it.role } : {}),
			...(it.params ? { params: resolveTemplate(it.params, { context }) as Record<string, unknown> } : {}),
		});
	}
	return { items };
}
// #endregion 🖼️Display

// #region 📄Document
/** @emoji 📄 Single committed modeling operation node. */
export interface ShapeNode {
	readonly id: string;
	readonly operationKind: string;
	readonly cellRef?: CellRef;
}

/** @emoji 📄 Undoable document command applied after a factory commits. */
export interface DocumentCommand {
	readonly id: string;
	readonly label: string;
	readonly do: (doc: ModelDocument, kernel: KernelAdapter) => Promise<void>;
	readonly undo: (doc: ModelDocument, kernel: KernelAdapter) => Promise<void>;
}

/** @emoji 📄 Working document: topology + committed shape nodes + command stack. */
export interface ModelDocument {
	readonly topology: TopologyGraph;
	nodes: ShapeNode[];
}

/** @emoji 📄 Two-tier history: factory-local snapshots + document commands. */
export class DocumentHistory {
	private docStack: ModelDocument[] = [];
	private cmdStack: DocumentCommand[] = [];
	private redoStack: DocumentCommand[] = [];

	pushSnapshot(doc: ModelDocument): void {
		this.docStack.push({
			topology: TopologyGraph.fromJSON(doc.topology.toJSON()),
			nodes: [...doc.nodes],
		});
	}

	async undoDocument(kernel: KernelAdapter, current: ModelDocument): Promise<boolean> {
		const cmd = this.cmdStack.pop();
		if (!cmd) return false;
		await cmd.undo(current, kernel);
		this.redoStack.push(cmd);
		return true;
	}

	async redoDocument(kernel: KernelAdapter, current: ModelDocument): Promise<boolean> {
		const cmd = this.redoStack.pop();
		if (!cmd) return false;
		await cmd.do(current, kernel);
		this.cmdStack.push(cmd);
		return true;
	}

	recordCommand(cmd: DocumentCommand): void {
		this.cmdStack.push(cmd);
		this.redoStack = [];
	}
}
// #endregion 📄Document

// #region 🏭Factory
/** @emoji 🩺 Non-fatal runtime diagnostic surfaced in snapshots. */
export interface Diagnostic {
	readonly severity: "info" | "warning" | "error";
	readonly code: string;
	readonly message: string;
}

/** @emoji 🏭 Serializable factory snapshot for hosts and renderers. */
export interface FactorySnapshot {
	readonly factoryId: string;
	readonly state: string;
	readonly revision: number;
	readonly context: Record<string, unknown>;
	readonly display: DisplayModel;
	readonly capabilities: { readonly canCommit: boolean; readonly canCancel: boolean; readonly canUndo: boolean; readonly canRedo: boolean };
	readonly diagnostics: readonly Diagnostic[];
}

export interface FactoryRuntimeOptions {
	readonly kernel: KernelAdapter;
	readonly document: ModelDocument;
	readonly history?: DocumentHistory;
}

/** @emoji 🏭 Headless + interactive factory controller (`send`, `commit`, `undo`). */
export class FactoryRuntime {
	private readonly sm: StatechartRuntime;
	private revision = 0;
	private readonly listeners = new Set<() => void>();
	private readonly snapStack: { state: string; context: string }[] = [];
	private committedCell: CellRef | null = null;

	constructor(
		private readonly spec: FactorySpec,
		private readonly opts: FactoryRuntimeOptions,
	) {
		this.sm = new StatechartRuntime(spec);
	}

	private cloneCtx(c: Record<string, unknown>): Record<string, unknown> {
		return JSON.parse(JSON.stringify(c)) as Record<string, unknown>;
	}

	private excludeFromHistory(kind: string): boolean {
		const xs = this.spec.history?.excludeEvents ?? [];
		return xs.includes(kind);
	}

	private canCommit(): boolean {
		const st = this.sm.getState();
		if (st !== "ready") return false;
		const w = this.spec.commit.when;
		if (w) {
			const g = this.spec.guards?.[w];
			if (!g) return false;
			return evalGuard(g, { context: this.sm.getContext() });
		}
		return true;
	}

	getSnapshot(): FactorySnapshot {
		const ctx = this.sm.getContext();
		const st = this.sm.getState();
		const display = resolveDisplay(this.spec, st, ctx);
		return {
			factoryId: this.spec.id,
			state: st,
			revision: this.revision,
			context: this.cloneCtx(ctx),
			display,
			capabilities: {
				canCommit: this.canCommit() && st === "ready",
				canCancel: st !== "committed" && st !== "cancelled",
				canUndo: this.snapStack.length > 0,
				canRedo: false,
			},
			diagnostics: [],
		};
	}

	subscribe(fn: () => void): () => void {
		this.listeners.add(fn);
		return () => this.listeners.delete(fn);
	}

	private emit(): void {
		this.revision += 1;
		for (const l of this.listeners) l();
	}

	/** @emoji 🏭 Dispatches a typed factory event through the statechart + optional kernel queries. */
	async send(event: FactoryEvent): Promise<void> {
		const beforeState = this.sm.getState();
		const beforeCtx = this.cloneCtx(this.sm.getContext());
		const r = await this.sm.send(event, this.opts.kernel);
		if (!r.ok) return;
		if (!r.transient && !this.excludeFromHistory(event.kind)) {
			this.snapStack.push({ state: beforeState, context: JSON.stringify(beforeCtx) });
		}
		this.emit();
	}

	undo(): void {
		const snap = this.snapStack.pop();
		if (!snap) return;
		const o = JSON.parse(snap.context) as Record<string, unknown>;
		this.sm.restore(snap.state, o);
		this.emit();
	}

	cancel(): void {
		this.sm.reset();
		this.emit();
	}

	/** @emoji 🏭 Executes `commit.operation` against `kernel` and records a `DocumentCommand`. */
	async commit(): Promise<CellRef | null> {
		const st = this.sm.getState();
		if (st !== "ready" && st !== "committed") return null;
		if (!this.canCommit()) return null;
		const ctx = this.sm.getContext();
		const op = this.spec.commit.operation;
		const params = resolveTemplate(op.params, { context: ctx }) as Record<string, unknown>;
		let cell: CellRef | null = null;
		if (op.kind === "cell.createBox") {
			const cornerA = params.cornerA as Vec3;
			const cornerB = params.cornerB as Vec3;
			const height = Number(params.height);
			cell = await this.opts.kernel.createBoxFromCorners({ cornerA, cornerB, height });
		} else if (op.kind === "wire.extrudeToCell") {
			cell = (await this.opts.kernel.extrudeWire?.({
				wireId: String(params.wireId),
				distance: Number(params.distance),
				direction: params.direction as Vec3,
			})) ?? null;
		} else if (op.kind === "face.offset") {
			await this.opts.kernel.offsetFaces?.({
				faceIds: params.faceIds as string[],
				distance: Number(params.distance),
			});
		}
		this.committedCell = cell;
		const hist = this.opts.history;
		if (hist && cell) {
			const id = `cmd-${this.revision}`;
			hist.recordCommand({
				id,
				label: this.spec.label ?? this.spec.id,
				do: async (doc, _kernel) => {
					doc.nodes.push({ id, operationKind: op.kind, cellRef: cell! });
				},
				undo: async (doc, _kernel) => {
					doc.nodes = doc.nodes.filter((n) => n.id !== id);
				},
			});
		}
		await this.sm.send({ kind: "confirm" }, this.opts.kernel);
		this.emit();
		return cell;
	}
}

/** @emoji 🏭 Constructs a `FactoryRuntime` from a compiled `FactorySpec`. */
export function createFactoryRuntime(spec: FactorySpec, opts: FactoryRuntimeOptions): FactoryRuntime {
	return new FactoryRuntime(compileFactory(spec), opts);
}
// #endregion 🏭Factory

// #region 📦Factories
/** @emoji 📦 Parses canonical box fixture (`spatial/fixtures/factory.json`). */
export function buildBoxFactorySpec(): FactorySpec {
	const s = parseFactorySpec(boxFactoryJson);
	if (!s) throw new Error("spatial/fixtures/factory.json invalid");
	return s;
}

/** @emoji 📦 Parses extrude-wire fixture (`spatial/fixtures/extrude.factory.json`). */
export function buildExtrudeFactorySpec(): FactorySpec {
	const s = parseFactorySpec(extrudeFactoryJson);
	if (!s) throw new Error("spatial/fixtures/extrude.factory.json invalid");
	return s;
}

/** @emoji 📦 Parses offset-surface fixture (`spatial/fixtures/offset-surface.factory.json`). */
export function buildOffsetSurfaceFactorySpec(): FactorySpec {
	const s = parseFactorySpec(offsetSurfaceFactoryJson);
	if (!s) throw new Error("spatial/fixtures/offset-surface.factory.json invalid");
	return s;
}
// #endregion 📦Factories

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@spatial/js-core vec", () => {
		it("adds and distances", () => {
			expect(vec3Distance([0, 0, 0], [3, 4, 0])).toBe(5);
		});
	});

	describe("@spatial/js-core expr", () => {
		it("evaluates guards used by box factory", () => {
			const g = {
				all: [
					{ exists: { path: "origin" } },
					{ ">": [{ path: "height" }, 0] },
				],
			} as Expr;
			expect(evalGuard(g, { context: { origin: [0, 0, 0], height: 2 } })).toBe(true);
		});
	});

	describe("@spatial/js-core factory box", () => {
		it("runs box workflow with a recording kernel stub (no solid modeling in core)", async () => {
			class RecordingStubKernel implements KernelAdapter {
				readonly id = "recording-stub";
				readonly operations = ["cell.createBox", "entity.tessellate"] as const;
				lastBox: { cornerA: Vec3; cornerB: Vec3; height: number } | null = null;
				async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<CellRef> {
					this.lastBox = input;
					return cellRef("stub-cell");
				}
				async volume(): Promise<number> {
					return 0;
				}
				async tessellate(): Promise<MeshPreview> {
					return { positions: new Float32Array(), indices: new Uint32Array() };
				}
			}
			const spec = buildBoxFactorySpec();
			const topo = new TopologyGraph();
			const kernel = new RecordingStubKernel();
			const rt = createFactoryRuntime(spec, { kernel, document: { topology: topo, nodes: [] } });
			await rt.send({ kind: "start" });
			await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
			await rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
			await rt.send({ kind: "set.height", value: 4 });
			const cell = await rt.commit();
			expect(cell).toBe("stub-cell");
			expect(kernel.lastBox).toEqual({
				cornerA: [0, 0, 0],
				cornerB: [2, 3, 0],
				height: 4,
			});
		});
	});
}
// #endregion 🧪Tests

