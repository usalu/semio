// #region 🧲Header
/** @emoji 🧭 `@spatial/js-core` — portable command spec runtime, `StateEngine` + `KernelAdapter` contracts, topology graph, derived views. See `spatial/schema/json` and `.repo/✍️/spatial.md`. */
// #endregion 🧲Header

// #region 📥Fixtures
import areaCommandJson from "../../fixtures/area.command.json" with { type: "json" };
import boxCommandJson from "../../fixtures/box.command.json" with { type: "json" };
import distanceCommandJson from "../../fixtures/distance.command.json" with { type: "json" };
import extrudeWireCommandJson from "../../fixtures/extrude-wire.command.json" with { type: "json" };
import offsetSurfaceCommandJson from "../../fixtures/offset-surface.command.json" with { type: "json" };
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

// #region 🎮CommandEvent
/** @emoji 🧭 Command input envelope; `kind` selects `machine.states[*].on` keys. */
export type CommandEvent = { readonly kind: string; readonly [k: string]: unknown };
// #endregion 🎮CommandEvent

// #region 🪪Selection
const TOPOLOGY_ENTITY_KINDS = new Set<string>([
	"vertex",
	"edge",
	"wire",
	"face",
	"shell",
	"cell",
	"cellComplex",
	"cluster",
	"surface",
	"part",
]);

/** @emoji 🪪 One picked topology or derived view target for `selection.changed`. */
export interface SelectionTarget {
	readonly kind: TopologyEntityKind;
	readonly id: string;
	readonly editable: boolean;
	readonly derivedFrom?: readonly { kind: EditableEntityKind; id: string }[];
}

/** @emoji 🪪 Host selection payload; `targets` filtered by `SelectionSpec.accept`. */
export interface SelectionEvent extends CommandEvent {
	readonly kind: "selection.changed";
	readonly targets: readonly SelectionTarget[];
}

/** @emoji 🪪 Per-state declarative filter for raw vs analytic picking. */
export interface SelectionSpec {
	readonly accept: readonly TopologyEntityKind[];
	readonly multiple?: boolean;
	readonly prompt?: string;
}

/** @emoji 🧭 Returns `targets` whose `kind` is listed in `spec.accept`. */
export function filterSelectionTargets(spec: SelectionSpec, targets: readonly SelectionTarget[]): SelectionTarget[] {
	return targets.filter((t) => spec.accept.includes(t.kind));
}

/** @emoji 🧭 True when every target is accepted (and at least one target exists). */
export function selectionEventMatches(spec: SelectionSpec, ev: SelectionEvent): boolean {
	if (!ev.targets || ev.targets.length === 0) return false;
	const xs = filterSelectionTargets(spec, ev.targets);
	if (xs.length !== ev.targets.length) return false;
	if (!spec.multiple && xs.length > 1) return false;
	return true;
}

/** @emoji 🧭 Active `selection` block for `state`, or `null` when unrestricted. */
export function getActiveSelectionSpec(
	spec: { readonly machine: { readonly states: Record<string, { readonly selection?: SelectionSpec }> } },
	state: string,
): SelectionSpec | null {
	const st = spec.machine.states[state];
	const s = st?.selection;
	return s ?? null;
}
// #endregion 🪪Selection

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
	if ("min" in expr) {
		const pair = (expr as { min: [Expr, Expr] }).min;
		return Math.min(Number(evalExpr(pair[0], env)), Number(evalExpr(pair[1], env)));
	}
	if ("max" in expr) {
		const pair = (expr as { max: [Expr, Expr] }).max;
		return Math.max(Number(evalExpr(pair[0], env)), Number(evalExpr(pair[1], env)));
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
/** @emoji 📜 Parsed static command document (`spatial.command/v1`). */
export interface CommandSpec {
	readonly schema: "spatial.command/v1";
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
				readonly selection?: SelectionSpec;
				readonly on?: Record<string, TransitionSpec | readonly TransitionSpec[]>;
			}
		>;
	};
	readonly display?: {
		readonly states?: Record<string, readonly DisplayItemSpec[]>;
	};
	readonly interaction?: CommandSpatialInteractionConfig;
	readonly commit: {
		readonly when?: string;
		readonly fromStates?: readonly string[];
		readonly outputDataPath?: string;
		readonly operation: { readonly kind: string; readonly params: Record<string, unknown> };
	};
}

/** @emoji 🎮 Host + viewport hints for spatial picking (declared per command preset). */
export interface CommandSpatialInteractionConfig {
	readonly spatialGroundPick?: boolean;
	readonly pickDisabledStates?: readonly string[];
	readonly groundPointerMoveStates?: readonly string[];
	readonly heightDragStates?: readonly string[];
	readonly verticalRodStates?: readonly string[];
	readonly heightConfirmState?: string | null;
}

export interface TransitionSpec {
	readonly target?: string;
	readonly guard?: string;
	readonly transient?: boolean;
	readonly actions?: readonly ActionSpec[];
	readonly key?: string;
	readonly label?: string;
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

/** @emoji 🧾 Validates and returns a `CommandSpec` or `null` when malformed. */
export function parseCommandSpec(raw: unknown): CommandSpec | null {
	if (!raw || typeof raw !== "object") return null;
	const r = raw as Record<string, unknown>;
	if (r.schema !== "spatial.command/v1") return null;
	if (typeof r.id !== "string" || typeof r.version !== "string") return null;
	const machine = r.machine;
	if (!machine || typeof machine !== "object") return null;
	const m = machine as Record<string, unknown>;
	if (typeof m.initial !== "string" || !m.states || typeof m.states !== "object") return null;
	const states = m.states as Record<string, unknown>;
	for (const st of Object.values(states)) {
		if (!st || typeof st !== "object") return null;
		const sel = (st as Record<string, unknown>).selection;
		if (!sel) continue;
		if (typeof sel !== "object") return null;
		const acc = (sel as Record<string, unknown>).accept;
		if (!Array.isArray(acc) || acc.length === 0) return null;
		for (const k of acc) {
			if (typeof k !== "string" || !TOPOLOGY_ENTITY_KINDS.has(k)) return null;
		}
	}
	const commit = r.commit;
	if (!commit || typeof commit !== "object") return null;
	const c = commit as Record<string, unknown>;
	const op = c.operation;
	if (!op || typeof op !== "object") return null;
	const o = op as Record<string, unknown>;
	if (typeof o.kind !== "string" || !o.params || typeof o.params !== "object") return null;
	return r as unknown as CommandSpec;
}

/** @emoji 🧭 Normalizes a parsed command (currently identity). */
export function compileCommand(spec: CommandSpec): CommandSpec {
	return spec;
}
// #endregion 📜Spec

// #region 🧱Topology
/** @emoji 🧱 Vertex payload: point geometry attached to topology. */
export interface VertexRecord {
	readonly id: VertexRef;
	readonly position: Vec3;
}

/** @emoji 🧱 Edge payload: references one or two boundary vertices. */
export interface EdgeRecord {
	readonly id: EdgeRef;
	readonly vertexIds: readonly VertexRef[];
}

/** @emoji 🧱 Wire payload: ordered boundary edges. */
export interface WireRecord {
	readonly id: WireRef;
	readonly edgeIds: readonly EdgeRef[];
}

/** @emoji 🧱 Face payload: boundary wires. */
export interface FaceRecord {
	readonly id: FaceRef;
	readonly wireIds: readonly WireRef[];
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

/** @emoji 🧱 Cell complex payload: member cells. */
export interface CellComplexRecord {
	readonly id: CellComplexRef;
	readonly cellIds: readonly CellRef[];
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

// #region 🧮Diff
export type VertexRecordDiff = { readonly id: VertexRef } & Partial<Pick<VertexRecord, "position">>;
export type EdgeRecordDiff = { readonly id: EdgeRef } & Partial<Pick<EdgeRecord, "vertexIds">>;
export type WireRecordDiff = { readonly id: WireRef } & Partial<Pick<WireRecord, "edgeIds">>;
export type FaceRecordDiff = { readonly id: FaceRef } & Partial<Pick<FaceRecord, "wireIds">>;
export type ShellRecordDiff = { readonly id: ShellRef } & Partial<Pick<ShellRecord, "faceIds">>;
export type CellRecordDiff = { readonly id: CellRef } & Partial<Pick<CellRecord, "shellIds">>;
export type CellComplexRecordDiff = { readonly id: CellComplexRef } & Partial<Pick<CellComplexRecord, "cellIds">>;
export type ClusterRecordDiff = { readonly id: ClusterRef } & Partial<Pick<ClusterRecord, "memberIds">>;

/** @emoji 🧮 Forward patch bucket for one topology table (`added` / `modified` / `removed`). */
export interface EntityDiff<TRec, TDiff, TId extends string> {
	readonly added?: Readonly<Record<TId, TRec>>;
	readonly modified?: Readonly<Record<TId, TDiff>>;
	readonly removed?: readonly TId[];
}

/** @emoji 🧮 Serializable topology delta applied by `applyTopologyDiff`. */
export interface TopologyDiff {
	readonly vertices?: EntityDiff<VertexRecord, VertexRecordDiff, VertexRef>;
	readonly edges?: EntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef>;
	readonly wires?: EntityDiff<WireRecord, WireRecordDiff, WireRef>;
	readonly faces?: EntityDiff<FaceRecord, FaceRecordDiff, FaceRef>;
	readonly shells?: EntityDiff<ShellRecord, ShellRecordDiff, ShellRef>;
	readonly cells?: EntityDiff<CellRecord, CellRecordDiff, CellRef>;
	readonly cellComplexes?: EntityDiff<CellComplexRecord, CellComplexRecordDiff, CellComplexRef>;
	readonly clusters?: EntityDiff<ClusterRecord, ClusterRecordDiff, ClusterRef>;
}

export const EMPTY_TOPOLOGY_DIFF: TopologyDiff = {};

function isEntityDiffEmpty<TRec, TDiff, TId extends string>(e: EntityDiff<TRec, TDiff, TId> | undefined): boolean {
	if (!e) return true;
	const a = e.added ? Object.keys(e.added).length : 0;
	const m = e.modified ? Object.keys(e.modified).length : 0;
	const r = e.removed ? e.removed.length : 0;
	return a === 0 && m === 0 && r === 0;
}

/** @emoji 🧮 True when `diff` has no topology mutations. */
export function isEmptyTopologyDiff(d: TopologyDiff | undefined): boolean {
	if (!d) return true;
	return (
		isEntityDiffEmpty(d.vertices) &&
		isEntityDiffEmpty(d.edges) &&
		isEntityDiffEmpty(d.wires) &&
		isEntityDiffEmpty(d.faces) &&
		isEntityDiffEmpty(d.shells) &&
		isEntityDiffEmpty(d.cells) &&
		isEntityDiffEmpty(d.cellComplexes) &&
		isEntityDiffEmpty(d.clusters)
	);
}

function cloneRec<T>(r: T): T {
	return JSON.parse(JSON.stringify(r)) as T;
}

function applyEntityDiff<T extends { id: string }, TDiff extends { id: string }>(
	bucket: Record<string, T>,
	section: EntityDiff<T, TDiff, string> | undefined,
	inverse: EntityDiff<T, TDiff, string>,
): void {
	if (!section) return;
	if (section.removed) {
		for (const id of section.removed) {
			const cur = bucket[id];
			if (!cur) continue;
			if (!inverse.added) inverse.added = {} as Record<string, T>;
			(inverse.added as Record<string, T>)[id] = cloneRec(cur);
			delete bucket[id];
		}
	}
	if (section.added) {
		for (const [id, rec] of Object.entries(section.added)) {
			bucket[id] = cloneRec(rec as T);
			if (!inverse.removed) inverse.removed = [];
			inverse.removed.push(id);
		}
	}
	if (section.modified) {
		for (const [id, md] of Object.entries(section.modified)) {
			const cur = bucket[id];
			if (!cur) continue;
			const back: Record<string, unknown> = { id };
			const curO = cur as Record<string, unknown>;
			const mdO = md as Record<string, unknown>;
			for (const fk of Object.keys(mdO)) {
				if (fk === "id") continue;
				back[fk] = curO[fk];
				curO[fk] = mdO[fk];
			}
			if (!inverse.modified) inverse.modified = {} as Record<string, TDiff>;
			(inverse.modified as Record<string, TDiff>)[id] = back as TDiff;
		}
	}
}

/** @emoji 🧮 Applies `diff` to `topo` in place; returns an inverse `TopologyDiff` for `applyTopologyDiff` again. */
export function applyTopologyDiff(topo: TopologyGraph, diff: TopologyDiff): TopologyDiff {
	const inv: TopologyDiff = {};
	const vInv: EntityDiff<VertexRecord, VertexRecordDiff, VertexRef> = {};
	const eInv: EntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef> = {};
	const wInv: EntityDiff<WireRecord, WireRecordDiff, WireRef> = {};
	const fInv: EntityDiff<FaceRecord, FaceRecordDiff, FaceRef> = {};
	const sInv: EntityDiff<ShellRecord, ShellRecordDiff, ShellRef> = {};
	const cInv: EntityDiff<CellRecord, CellRecordDiff, CellRef> = {};
	const ccInv: EntityDiff<CellComplexRecord, CellComplexRecordDiff, CellComplexRef> = {};
	const clInv: EntityDiff<ClusterRecord, ClusterRecordDiff, ClusterRef> = {};
	applyEntityDiff(topo.vertices as Record<string, VertexRecord>, diff.vertices, vInv);
	applyEntityDiff(topo.edges as Record<string, EdgeRecord>, diff.edges, eInv);
	applyEntityDiff(topo.wires as Record<string, WireRecord>, diff.wires, wInv);
	applyEntityDiff(topo.faces as Record<string, FaceRecord>, diff.faces, fInv);
	applyEntityDiff(topo.shells as Record<string, ShellRecord>, diff.shells, sInv);
	applyEntityDiff(topo.cells as Record<string, CellRecord>, diff.cells, cInv);
	applyEntityDiff(topo.cellComplexes as Record<string, CellComplexRecord>, diff.cellComplexes, ccInv);
	applyEntityDiff(topo.clusters as Record<string, ClusterRecord>, diff.clusters, clInv);
	if (!isEntityDiffEmpty(vInv)) inv.vertices = vInv;
	if (!isEntityDiffEmpty(eInv)) inv.edges = eInv;
	if (!isEntityDiffEmpty(wInv)) inv.wires = wInv;
	if (!isEntityDiffEmpty(fInv)) inv.faces = fInv;
	if (!isEntityDiffEmpty(sInv)) inv.shells = sInv;
	if (!isEntityDiffEmpty(cInv)) inv.cells = cInv;
	if (!isEntityDiffEmpty(ccInv)) inv.cellComplexes = ccInv;
	if (!isEntityDiffEmpty(clInv)) inv.clusters = clInv;
	if (!isEmptyTopologyDiff(diff)) topo.bump();
	return inv;
}

/** @emoji 🧮 One tessellated triangle mesh as a single `FaceRecord` plus boundary topology. */
export function meshFaceTopologyDiff(mesh: MeshPreview, idTag: string): TopologyDiff {
	const pos = mesh.positions;
	const ind = mesh.indices;
	if (ind.length < 3 || pos.length < 9) return {};
	const verts: Vec3[] = [];
	for (let k = 0; k < pos.length; k += 3) {
		verts.push([pos[k]!, pos[k + 1]!, pos[k + 2]!]);
	}
	const tris: [number, number, number][] = [];
	for (let k = 0; k < ind.length; k += 3) {
		tris.push([ind[k]!, ind[k + 1]!, ind[k + 2]!]);
	}
	const i0 = ind[0]!;
	const i1 = ind[1]!;
	const i2 = ind[2]!;
	const a = [pos[i0 * 3]!, pos[i0 * 3 + 1]!, pos[i0 * 3 + 2]!] as Vec3;
	const b = [pos[i1 * 3]!, pos[i1 * 3 + 1]!, pos[i1 * 3 + 2]!] as Vec3;
	const c = [pos[i2 * 3]!, pos[i2 * 3 + 1]!, pos[i2 * 3 + 2]!] as Vec3;
	const ctr: Vec3 = [(a[0] + b[0] + c[0]) / 3, (a[1] + b[1] + c[1]) / 3, (a[2] + b[2] + c[2]) / 3];
	const eps = 0.04;
	const pfx = `cm-${idTag}`;
	const v0 = `${pfx}-w0` as VertexRef;
	const v1 = `${pfx}-w1` as VertexRef;
	const v2 = `${pfx}-w2` as VertexRef;
	const e0 = `${pfx}-e0` as EdgeRef;
	const e1 = `${pfx}-e1` as EdgeRef;
	const e2 = `${pfx}-e2` as EdgeRef;
	const wireId = `${pfx}-wire` as WireRef;
	const faceId = `${pfx}-face` as FaceRef;
	return {
		vertices: {
			added: {
				[v0]: { id: v0, position: [ctr[0] + eps, ctr[1], ctr[2]] },
				[v1]: { id: v1, position: [ctr[0], ctr[1] + eps, ctr[2]] },
				[v2]: { id: v2, position: [ctr[0], ctr[1], ctr[2] + eps] },
			},
		},
		edges: {
			added: {
				[e0]: { id: e0, vertexIds: [v0, v1] },
				[e1]: { id: e1, vertexIds: [v1, v2] },
				[e2]: { id: e2, vertexIds: [v2, v0] },
			},
		},
		wires: { added: { [wireId]: { id: wireId, edgeIds: [e0, e1, e2] } } },
		faces: {
			added: {
				[faceId]: {
					id: faceId,
					wireIds: [wireId],
				},
			},
		},
	};
}

// #endregion 🧮Diff

// #region 🔌Kernel
/** @emoji 🖼️ Renderer-neutral mesh preview (positions + triangle indices). */
export interface MeshPreview {
	readonly positions: Float32Array;
	readonly indices: Uint32Array;
	readonly normals?: Float32Array;
}

/** @emoji 🧱 Appends a tessellated commit as one mesh `face` on `TopologyGraph` (in-memory scene growth). */
export function appendCommittedMeshFaceToTopology(topo: TopologyGraph, mesh: MeshPreview, idTag: string): void {
	applyTopologyDiff(topo, meshFaceTopologyDiff(mesh, idTag));
}

/** @emoji 🔌 Kernel capability surface executed by command commits. */
export interface KernelAdapter {
	readonly id: string;
	readonly operations: readonly string[];
	createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<CellRef>;
	volume(cell: CellRef): Promise<number>;
	tessellate(cell: CellRef, tolerance: number): Promise<MeshPreview>;
	query?(name: string, params: Record<string, unknown>): Promise<unknown>;
	extrudeWire?(input: { wireId: string; distance: number; direction: Vec3 }): Promise<CellRef>;
	offsetFaces?(input: { faceIds: readonly string[]; distance: number }): Promise<void>;
	createBoxFromCornersDiff?(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: TopologyDiff; readonly cell: CellRef }>;
	extrudeWireDiff?(input: { wireId: string; distance: number; direction: Vec3 }): Promise<{ readonly diff: TopologyDiff; readonly cell: CellRef }>;
	offsetFacesDiff?(input: { faceIds: readonly string[]; distance: number }): Promise<{ readonly diff: TopologyDiff }>;
	vertexDistance?(a: VertexRef, b: VertexRef, topo: TopologyGraph): Promise<number>;
	edgeLength?(e: EdgeRef, topo: TopologyGraph): Promise<number>;
	faceArea?(f: FaceRef, topo: TopologyGraph): Promise<number>;
	cellVolume?(c: CellRef): Promise<number>;
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
			out.push({
				id: `surface-${f.id}` as SurfaceRef,
				sourceFaceIds: [f.id],
				exposure: "external",
				stance: "vertical",
				area: 0,
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
/** @emoji 🎭 Result of `StateEngine.send` / `applyTransition` (`transient` skips command-local undo). */
export interface StateEngineSendResult {
	readonly ok: boolean;
	readonly transient?: boolean;
}

/** @emoji 🎭 `applyTransition` output: next factory state + disambiguation index for XState routing. */
export interface ApplyTransitionResult extends StateEngineSendResult {
	readonly nextState: string;
	readonly branchIndex: number;
}

/** @emoji 🎭 Pluggable state backend for `CommandRuntime` (pure TS, XState, …). */
export interface StateEngine {
	getState(): string;
	getContext(): Record<string, unknown>;
	reset(): void;
	restore(state: string, context: Record<string, unknown>): void;
	send(event: CommandEvent, kernel?: KernelAdapter): Promise<StateEngineSendResult>;
}

/** @emoji 🎭 Instantiates a `StateEngine` for a compiled `CommandSpec`. */
export interface StateEngineProvider {
	readonly id: string;
	create(spec: CommandSpec): StateEngine;
}

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

export function expandMachineTransitions(
	raw: TransitionSpec | readonly TransitionSpec[] | undefined,
): readonly TransitionSpec[] {
	if (raw === undefined) return [];
	return (Array.isArray(raw) ? raw : [raw]) as readonly TransitionSpec[];
}

const HOST_KEYBIND_EXCLUDED_KINDS = new Set(["pointer.move", "pointer.down", "selection.changed"]);

/** @emoji ⌨️ Resolved spatial host hints (defaults disable ground picking). */
export interface CommandSpatialInteractionResolved {
	readonly spatialGroundPick: boolean;
	readonly pickDisabledStates: readonly string[];
	readonly groundPointerMoveStates: readonly string[];
	readonly heightDragStates: readonly string[];
	readonly verticalRodStates: readonly string[];
	readonly heightConfirmState: string | null;
}

/** @emoji ⌨️ Merges `spec.interaction` with safe defaults for hosts and `CommandSpatialView`. */
export function mergeCommandSpatialInteraction(spec: CommandSpec): CommandSpatialInteractionResolved {
	const i = spec.interaction;
	const basePickDisabled = ["idle", "ready", "committed"] as const;
	return {
		spatialGroundPick: Boolean(i?.spatialGroundPick),
		pickDisabledStates: i?.pickDisabledStates ?? [...basePickDisabled],
		groundPointerMoveStates: i?.groundPointerMoveStates ?? [],
		heightDragStates: i?.heightDragStates ?? [],
		verticalRodStates: i?.verticalRodStates ?? [],
		heightConfirmState: i?.heightConfirmState === undefined ? null : i.heightConfirmState,
	};
}

/** @emoji ⌨️ One host-triggerable transition row for palette + command input (see `TransitionSpec.key`). */
export interface CommandKeybindRow {
	readonly eventKind: string;
	readonly key: string;
	readonly label: string;
}

/** @emoji ⌨️ Lists keyed transitions for the active state (excludes pointer + selection). */
export function listKeyedCommandTransitions(spec: CommandSpec, state: string): readonly CommandKeybindRow[] {
	const st = spec.machine.states[state];
	if (!st?.on) return [];
	const out: CommandKeybindRow[] = [];
	for (const [eventKind, raw] of Object.entries(st.on)) {
		if (HOST_KEYBIND_EXCLUDED_KINDS.has(eventKind)) continue;
		for (const tr of expandMachineTransitions(raw)) {
			if (tr.transient) continue;
			const key = tr.key;
			const label = tr.label;
			if (typeof key !== "string" || key.length === 0) continue;
			if (typeof label !== "string" || label.length === 0) continue;
			out.push({ eventKind, key, label });
		}
	}
	return out;
}

/** @emoji 📦 Applies imperative `box.*` footprint helpers used by `spatial/fixtures/box.command.json`. */
function applyBoxGeometryOp(ctx: Record<string, unknown>, event: CommandEvent, op: string): void {
	const pt = (event as { point?: unknown }).point;
	const P = isVec3(pt) ? pt : null;
	const val = (event as { value?: unknown }).value;
	if (op === "box.aabbFromDiagonalCorners") {
		const a = ctx.diagA;
		if (!isVec3(a) || !P) return;
		const z = a[2];
		ctx.origin = [Math.min(a[0], P[0]), Math.min(a[1], P[1]), z] as unknown as Vec3;
		ctx.corner = [Math.max(a[0], P[0]), Math.max(a[1], P[1]), z] as unknown as Vec3;
		delete ctx.diagA;
		return;
	}
	if (op === "box.tripletRubber") {
		const p0 = ctx.p0;
		const p1 = ctx.p1;
		if (!isVec3(p0) || !isVec3(p1) || !P) return;
		const z = p0[2];
		ctx.previewA = [Math.min(p0[0], p1[0], P[0]), Math.min(p0[1], p1[1], P[1]), z] as unknown as Vec3;
		ctx.previewB = [Math.max(p0[0], p1[0], P[0]), Math.max(p0[1], p1[1], P[1]), z] as unknown as Vec3;
		return;
	}
	if (op === "box.tripletCommit") {
		const p0 = ctx.p0;
		const p1 = ctx.p1;
		if (!isVec3(p0) || !isVec3(p1) || !P) return;
		const z = p0[2];
		ctx.origin = [Math.min(p0[0], p1[0], P[0]), Math.min(p0[1], p1[1], P[1]), z] as unknown as Vec3;
		ctx.corner = [Math.max(p0[0], p1[0], P[0]), Math.max(p0[1], p1[1], P[1]), z] as unknown as Vec3;
		delete ctx.p0;
		delete ctx.p1;
		delete ctx.previewA;
		delete ctx.previewB;
		return;
	}
	if (op === "box.snapSquareFootprint") {
		const o = ctx.origin;
		if (!isVec3(o) || !P) return;
		const dx = P[0] - o[0];
		const dy = P[1] - o[1];
		const s = Math.max(Math.abs(dx), Math.abs(dy), 1e-9);
		const sx = dx >= 0 ? 1 : -1;
		const sy = dy >= 0 ? 1 : -1;
		ctx.corner = [o[0] + sx * s, o[1] + sy * s, o[2]] as unknown as Vec3;
		return;
	}
	if (op === "box.setCubeHeightFromFootprint") {
		const o = ctx.origin;
		const c = ctx.corner;
		if (!isVec3(o) || !isVec3(c)) return;
		const dx = Math.abs(c[0] - o[0]);
		const dy = Math.abs(c[1] - o[1]);
		ctx.height = Math.max(dx, dy, 0.01);
		return;
	}
	if (op === "box.rubberCornerFromCenter") {
		const c = ctx.rectCenter;
		if (!isVec3(c) || !P) return;
		ctx.origin = [Math.min(2 * c[0] - P[0], P[0]), Math.min(2 * c[1] - P[1], P[1]), c[2]] as unknown as Vec3;
		ctx.corner = [Math.max(2 * c[0] - P[0], P[0]), Math.max(2 * c[1] - P[1], P[1]), c[2]] as unknown as Vec3;
		return;
	}
	if (op === "box.rubberSquareFromCenter") {
		const c = ctx.rectCenter;
		if (!isVec3(c) || !P) return;
		const ox = Math.min(2 * c[0] - P[0], P[0]);
		const oy = Math.min(2 * c[1] - P[1], P[1]);
		const cx = Math.max(2 * c[0] - P[0], P[0]);
		const cy = Math.max(2 * c[1] - P[1], P[1]);
		const w = cx - ox;
		const d = cy - oy;
		const s = Math.max(w, d, 1e-9);
		ctx.origin = [c[0] - s / 2, c[1] - s / 2, c[2]] as unknown as Vec3;
		ctx.corner = [c[0] + s / 2, c[1] + s / 2, c[2]] as unknown as Vec3;
		return;
	}
	if (op === "box.verticalFinalizeFootprint") {
		const o = ctx.origin;
		const pk = ctx.peak;
		if (!isVec3(o) || !isVec3(pk) || !P) return;
		ctx.corner = [P[0], P[1], o[2]] as unknown as Vec3;
		ctx.height = Math.max(0.01, Math.abs(pk[2] - o[2]));
		delete ctx.peak;
		return;
	}
	if (op === "box.initPeakAboveOrigin") {
		const o = ctx.origin;
		if (!isVec3(o)) return;
		ctx.peak = [o[0], o[1], o[2] + 0.25] as unknown as Vec3;
		return;
	}
	if (op === "box.peakFromOriginZ") {
		const o = ctx.origin;
		if (!isVec3(o) || !P) return;
		ctx.peak = [o[0], o[1], P[2]] as unknown as Vec3;
		return;
	}
	if (op === "box.verticalRubberCorner") {
		const o = ctx.origin;
		if (!isVec3(o) || !P) return;
		ctx.corner = [P[0], P[1], o[2]] as unknown as Vec3;
		return;
	}
	if (op === "box.cornerFromLengthWidth") {
		const o = ctx.origin;
		if (!isVec3(o) || val === null || typeof val !== "object") return;
		const rec = val as Record<string, unknown>;
		const L = Number(rec.length);
		const W = Number(rec.width);
		if (!Number.isFinite(L) || !Number.isFinite(W)) return;
		ctx.corner = [o[0] + L, o[1] + W, o[2]] as unknown as Vec3;
		return;
	}
}

/** @emoji 🎬 Applies one declarative or `box.*` action (async kernel queries). */
export async function applyActionAsync(
	a: ActionSpec,
	ctx: Record<string, unknown>,
	event: CommandEvent,
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
	} else if (typeof a.op === "string" && a.op.startsWith("box.")) {
		applyBoxGeometryOp(ctx, event, a.op);
	}
}

/** @emoji 🎬 First matching transition for `event` from `state`; mutates `context` in place. */
export async function applyTransition(
	spec: CommandSpec,
	state: string,
	context: Record<string, unknown>,
	event: CommandEvent,
	kernel?: KernelAdapter,
): Promise<ApplyTransitionResult> {
	const st = spec.machine.states[state];
	if (!st?.on) return { ok: false, nextState: state, branchIndex: -1 };
	const raw = st.on[event.kind];
	const choices = expandMachineTransitions(raw);
	if (choices.length === 0) return { ok: false, nextState: state, branchIndex: -1 };
	for (let i = 0; i < choices.length; i++) {
		const tr = choices[i]!;
		if (tr.guard) {
			const g = spec.guards?.[tr.guard];
			if (!g || !evalGuard(g, { context, event })) continue;
		}
		for (const a of tr.actions ?? []) {
			await applyActionAsync(a, context, event, kernel);
		}
		let nextState = state;
		if (tr.target) {
			nextState = tr.target;
			if (tr.target === spec.machine.initial) {
				for (const k of Object.keys(context)) delete context[k];
			}
		}
		return { ok: true, transient: Boolean(tr.transient), nextState, branchIndex: i };
	}
	return { ok: false, nextState: state, branchIndex: -1 };
}

/** @emoji 🎬 Minimal async statechart runner for `CommandSpec.machine`. */
export class StatechartRuntime implements StateEngine {
	private state: string;
	private context: Record<string, unknown> = {};

	constructor(private readonly spec: CommandSpec) {
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
	async send(event: CommandEvent, kernel?: KernelAdapter): Promise<StateEngineSendResult> {
		const r = await applyTransition(this.spec, this.state, this.context, event, kernel);
		if (r.ok) this.state = r.nextState;
		return { ok: r.ok, transient: r.transient };
	}
}

/** @emoji 🎭 Default in-process engine (no XState); same semantics as `applyTransition`. */
export const pureTsStateEngineProvider: StateEngineProvider = {
	id: "pure-ts",
	create(spec: CommandSpec): StateEngine {
		return new StatechartRuntime(spec);
	},
};
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
export function resolveDisplay(spec: CommandSpec, state: string, context: Record<string, unknown>): DisplayModel {
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

// #region 📨Response
/** @emoji 📨 Portable command outcome envelope (`diff` + `data` + messages). */
export interface CommandMessage {
	readonly code: string;
	readonly message: string;
	readonly path?: string;
}

/** @emoji 📨 Result returned by `CommandRuntime.commit` (read/write topology + scalar `data`). */
export interface CommandResponse<TData = unknown> {
	readonly ok: boolean;
	readonly errors: readonly CommandMessage[];
	readonly warnings: readonly CommandMessage[];
	readonly infos: readonly CommandMessage[];
	readonly diff: TopologyDiff;
	readonly data: TData | null;
}

/** @emoji 📨 Default empty success payload for guards and early returns. */
export const EMPTY_COMMAND_RESPONSE: CommandResponse<null> = {
	ok: true,
	errors: [],
	warnings: [],
	infos: [],
	diff: EMPTY_TOPOLOGY_DIFF,
	data: null,
};
// #endregion 📨Response

// #region 📜Command
/** @emoji 🩺 Non-fatal runtime diagnostic surfaced in snapshots. */
export interface Diagnostic {
	readonly severity: "info" | "warning" | "error";
	readonly code: string;
	readonly message: string;
}

/** @emoji 📜 Serializable command snapshot for hosts and renderers. */
export interface CommandSnapshot {
	readonly commandId: string;
	readonly state: string;
	readonly revision: number;
	readonly context: Record<string, unknown>;
	readonly display: DisplayModel;
	readonly spatialInteraction: CommandSpatialInteractionResolved;
	readonly capabilities: { readonly canCommit: boolean; readonly canCancel: boolean; readonly canUndo: boolean; readonly canRedo: boolean };
	readonly diagnostics: readonly Diagnostic[];
	readonly lastResponse: CommandResponse | null;
}

export interface CommandRuntimeOptions {
	readonly kernel: KernelAdapter;
	readonly document: ModelDocument;
	readonly history?: DocumentHistory;
	readonly stateEngine?: StateEngineProvider;
}

/** @emoji 📜 Headless + interactive command controller (`send`, `commit`, `undo`). */
export class CommandRuntime {
	private readonly sm: StateEngine;
	private revision = 0;
	private readonly listeners = new Set<() => void>();
	private readonly snapStack: { state: string; context: string }[] = [];
	private snapshotCache: CommandSnapshot | null = null;
	private lastResponse: CommandResponse | null = null;
	private readonly pendingSnapshotInfos: CommandMessage[] = [];

	constructor(
		private readonly spec: CommandSpec,
		private readonly opts: CommandRuntimeOptions,
	) {
		this.sm = (opts.stateEngine ?? pureTsStateEngineProvider).create(spec);
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
		const allowed = this.spec.commit.fromStates ?? ["ready"];
		if (!allowed.includes(st)) return false;
		const w = this.spec.commit.when;
		if (w) {
			const g = this.spec.guards?.[w];
			if (!g) return false;
			return evalGuard(g, { context: this.sm.getContext() });
		}
		return true;
	}

	/** @emoji 🧭 Accepted topology kinds for the active machine state (`[]` when none). */
	listActiveSelectionAccept(): readonly TopologyEntityKind[] {
		return getActiveSelectionSpec(this.spec, this.sm.getState())?.accept ?? [];
	}

	getSnapshot(): CommandSnapshot {
		if (this.snapshotCache) return this.snapshotCache;
		const ctx = this.sm.getContext();
		const st = this.sm.getState();
		const display = resolveDisplay(this.spec, st, ctx);
		const spatialInteraction = mergeCommandSpatialInteraction(this.spec);
		const flushed = this.pendingSnapshotInfos.splice(0, this.pendingSnapshotInfos.length);
		const infoDiags: Diagnostic[] = flushed.map((m) => ({ severity: "info" as const, code: m.code, message: m.message }));
		this.snapshotCache = {
			commandId: this.spec.id,
			state: st,
			revision: this.revision,
			context: this.cloneCtx(ctx),
			display,
			spatialInteraction,
			capabilities: {
				canCommit: this.canCommit(),
				canCancel: st !== "committed",
				canUndo: this.snapStack.length > 0,
				canRedo: false,
			},
			diagnostics: infoDiags,
			lastResponse: this.lastResponse,
		};
		return this.snapshotCache;
	}

	subscribe(fn: () => void): () => void {
		this.listeners.add(fn);
		return () => this.listeners.delete(fn);
	}

	private emit(): void {
		this.revision += 1;
		this.snapshotCache = null;
		for (const l of this.listeners) l();
	}

	/** @emoji 📜 Dispatches a typed command event through the statechart + optional kernel queries. */
	async send(event: CommandEvent): Promise<void> {
		if (event.kind === "selection.changed") {
			const sel = getActiveSelectionSpec(this.spec, this.sm.getState());
			const sev = event as SelectionEvent;
			if (sel && !selectionEventMatches(sel, sev)) {
				this.pendingSnapshotInfos.push({
					code: "selection.filtered",
					message: "Selection did not match this state's accept list.",
				});
				this.emit();
				return;
			}
		}
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

	/** @emoji 📜 Executes `commit.operation` against `kernel`, applies `diff` to `document.topology`, records history. */
	async commit(): Promise<CommandResponse> {
		const fail = (code: string, message: string): CommandResponse => {
			const res: CommandResponse = {
				ok: false,
				errors: [{ code, message }],
				warnings: [],
				infos: [],
				diff: EMPTY_TOPOLOGY_DIFF,
				data: null,
			};
			this.lastResponse = res;
			this.emit();
			return res;
		};
		const st = this.sm.getState();
		if (st === "committed") return fail("command.alreadyCommitted", "Command already committed.");
		if (!this.canCommit()) return fail("command.cannotCommit", "Commit guard or fromStates rejected this commit.");
		const ctx = this.sm.getContext();
		const op = this.spec.commit.operation;
		const params = resolveTemplate(op.params, { context: ctx }) as Record<string, unknown>;
		const k = this.opts.kernel;
		const topo = this.opts.document.topology;
		let diff: TopologyDiff = EMPTY_TOPOLOGY_DIFF;
		let data: unknown = null;
		try {
			if (op.kind === "cell.createBox") {
				const cornerA = params.cornerA as Vec3;
				const cornerB = params.cornerB as Vec3;
				const height = Number(params.height);
				if (k.createBoxFromCornersDiff) {
					const r = await k.createBoxFromCornersDiff({ cornerA, cornerB, height });
					diff = r.diff;
				} else {
					const cell = await k.createBoxFromCorners({ cornerA, cornerB, height });
					const preview = await k.tessellate(cell, 1e-3);
					diff = meshFaceTopologyDiff(preview, `f${this.spec.id}-${this.revision}`);
				}
			} else if (op.kind === "wire.extrudeToCell") {
				const input = {
					wireId: String(params.wireId),
					distance: Number(params.distance),
					direction: params.direction as Vec3,
				};
				if (k.extrudeWireDiff) {
					diff = (await k.extrudeWireDiff(input)).diff;
				} else {
					const cell = (await k.extrudeWire?.(input)) ?? null;
					if (cell) {
						const preview = await k.tessellate(cell, 1e-3);
						diff = meshFaceTopologyDiff(preview, `f${this.spec.id}-${this.revision}`);
					}
				}
			} else if (op.kind === "face.offset") {
				diff = (await k.offsetFacesDiff?.({ faceIds: params.faceIds as string[], distance: Number(params.distance) }))?.diff ?? EMPTY_TOPOLOGY_DIFF;
			} else if (op.kind === "measure.distance") {
				const a = params.a as VertexRef;
				const b = params.b as VertexRef;
				if (!k.vertexDistance) throw new Error("kernel.vertexDistance required");
				data = await k.vertexDistance(a, b, topo);
			} else if (op.kind === "measure.area") {
				const fid = params.faceId as FaceRef;
				if (!k.faceArea) throw new Error("kernel.faceArea required");
				data = await k.faceArea(fid, topo);
			} else if (op.kind === "measure.volume") {
				const cid = params.cellId as CellRef;
				data = await (k.cellVolume?.(cid) ?? k.volume(cid));
			}
		} catch (e) {
			const msg = e instanceof Error ? e.message : String(e);
			return fail("command.commitFailed", msg);
		}
		const outPath = this.spec.commit.outputDataPath;
		if (outPath) {
			const ctx2 = this.sm.getContext();
			setPath(ctx2 as Record<string, unknown>, outPath, data);
			data = getPath(ctx2, outPath) ?? data;
		}
		const inverse = applyTopologyDiff(topo, diff);
		const hist = this.opts.history;
		if (hist && !isEmptyTopologyDiff(diff)) {
			const id = `cmd-${this.spec.id}-${this.revision}`;
			const forward = diff;
			hist.recordCommand({
				id,
				label: this.spec.label ?? this.spec.id,
				do: async (doc) => {
					applyTopologyDiff(doc.topology, forward);
				},
				undo: async (doc) => {
					applyTopologyDiff(doc.topology, inverse);
				},
			});
		}
		await this.sm.send({ kind: "confirm" }, k);
		const res: CommandResponse = { ok: true, errors: [], warnings: [], infos: [], diff, data };
		this.lastResponse = res;
		this.emit();
		return res;
	}
}

/** @emoji 📜 Constructs a `CommandRuntime` from a compiled `CommandSpec`. */
export function createCommandRuntime(spec: CommandSpec, opts: CommandRuntimeOptions): CommandRuntime {
	return new CommandRuntime(compileCommand(spec), opts);
}
// #endregion 📜Command

// #region 📦Factories
/** @emoji 📦 Parses canonical box fixture (`spatial/fixtures/factory.json`). */
export function buildBoxCommandSpec(): CommandSpec {
	const s = parseCommandSpec(boxFactoryJson);
	if (!s) throw new Error("spatial/fixtures/factory.json invalid");
	return s;
}

/** @emoji 📦 Parses extrude-wire fixture (`spatial/fixtures/extrude.factory.json`). */
export function buildExtrudeCommandSpec(): CommandSpec {
	const s = parseCommandSpec(extrudeFactoryJson);
	if (!s) throw new Error("spatial/fixtures/extrude.factory.json invalid");
	return s;
}

/** @emoji 📦 Parses offset-surface fixture (`spatial/fixtures/offset-surface.factory.json`). */
export function buildOffsetSurfaceCommandSpec(): CommandSpec {
	const s = parseCommandSpec(offsetSurfaceFactoryJson);
	if (!s) throw new Error("spatial/fixtures/offset-surface.factory.json invalid");
	return s;
}

/** @emoji 📚 Host-facing factory preset row (`spatial/fixtures/*.factory.json`). */
export interface SpatialFactoryPreset {
	readonly id: string;
	readonly label: string;
	/** @emoji ⌨️ Single-stroke host command key; must stay unique among presets (see `resolveSpatialFactoryPresetKey`). */
	readonly key: string;
}

/** @emoji 📚 Built-in factory preset ids for host command surfaces (`spatial/fixtures/*.factory.json`). */
export function listSpatialFactoryPresets(): readonly SpatialFactoryPreset[] {
	return [
		{ id: "primitive.box", label: "Box", key: "q" },
		{ id: "feature.extrudeWire", label: "Extrude wire", key: "j" },
		{ id: "feature.offsetSurface", label: "Offset surface", key: "k" },
	];
}

/** @emoji 🧭 Resolves a typed token to a preset (`key`, `id`, or compact `label`). */
export function resolveSpatialFactoryPresetKey(token: string): SpatialFactoryPreset | null {
	const t = token.trim().toLowerCase();
	if (!t) return null;
	for (const p of listSpatialFactoryPresets()) {
		if (p.key.toLowerCase() === t) return p;
		if (p.id.toLowerCase() === t) return p;
		const slug = p.label.toLowerCase().replace(/\s+/g, "");
		if (slug === t) return p;
	}
	return null;
}

/** @emoji 📚 Loads a built-in factory preset by stable `id` (see `listSpatialFactoryPresets`). */
export function loadSpatialFactoryPreset(presetId: string): CommandSpec | null {
	const raw =
		presetId === "primitive.box"
			? boxFactoryJson
			: presetId === "feature.extrudeWire"
				? extrudeFactoryJson
				: presetId === "feature.offsetSurface"
					? offsetSurfaceFactoryJson
					: null;
	if (!raw) return null;
	return parseCommandSpec(raw as unknown);
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
		it("evaluates numeric min expr", () => {
			const e = { min: [{ const: 3 }, { const: 7 }] } as Expr;
			expect(evalExpr(e, { context: {} })).toBe(3);
		});
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

	describe("@spatial/js-core topology commit mesh", () => {
		it("appendCommittedMeshFaceToTopology adds one mesh face from a triangle mesh", () => {
			const g = new TopologyGraph();
			const mesh = {
				positions: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
				indices: new Uint32Array([0, 1, 2]),
			};
			appendCommittedMeshFaceToTopology(g, mesh, "t0");
			expect(Object.keys(g.faces).length).toBe(1);
			expect(g.revision).toBeGreaterThan(0);
		});
	});

	describe("@spatial/js-core factory presets", () => {
		it("lists stable keys for each built-in factory preset", () => {
			const ps = listSpatialFactoryPresets();
			expect(ps.map((p) => p.key).join("")).toBe("qjk");
			expect(new Set(ps.map((p) => p.key)).size).toBe(ps.length);
		});
		it("resolves factory preset tokens by key, id, and label slug", () => {
			expect(resolveSpatialFactoryPresetKey("q")?.id).toBe("primitive.box");
			expect(resolveSpatialFactoryPresetKey("primitive.box")?.key).toBe("q");
			expect(resolveSpatialFactoryPresetKey("extrudewire")?.id).toBe("feature.extrudeWire");
		});
	});
	describe("@spatial/js-core factory box", () => {
		it("tracks first-corner cursor on the grid after start", async () => {
			class StubKernel implements KernelAdapter {
				readonly id = "stub";
				readonly operations = ["cell.createBox", "entity.tessellate"] as const;
				async createBoxFromCorners() {
					return cellRef("stub");
				}
				async volume() {
					return 0;
				}
				async tessellate() {
					return { positions: new Float32Array(), indices: new Uint32Array() };
				}
			}
			const spec = buildBoxCommandSpec();
			const rt = createCommandRuntime(spec, {
				kernel: new StubKernel(),
				document: { topology: new TopologyGraph(), nodes: [] },
			});
			await rt.send({ kind: "start" });
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
					return {
						positions: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
						indices: new Uint32Array([0, 1, 2]),
					};
				}
			}
			const spec = buildBoxCommandSpec();
			const topo = new TopologyGraph();
			const kernel = new RecordingStubKernel();
			const rt = createCommandRuntime(spec, { kernel, document: { topology: topo, nodes: [] } });
			await rt.send({ kind: "start" });
			await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
			await rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
			await rt.send({ kind: "set.height", value: 4 });
			const cell = await rt.commit();
			expect(cell).toBe("stub-cell");
			expect(Object.keys(topo.faces).length).toBeGreaterThan(0);
			expect(kernel.lastBox).toEqual({
				cornerA: [0, 0, 0],
				cornerB: [2, 3, 0],
				height: 4,
			});
		});
	});

	describe("@spatial/js-core stateEngine option", () => {
		it("explicit pure-ts provider matches default factory snapshots", async () => {
			class StubKernel implements KernelAdapter {
				readonly id = "stub-opt";
				readonly operations = ["cell.createBox", "entity.tessellate"] as const;
				async createBoxFromCorners() {
					return cellRef("c");
				}
				async volume() {
					return 0;
				}
				async tessellate() {
					return { positions: new Float32Array(), indices: new Uint32Array() };
				}
			}
			const spec = buildBoxCommandSpec();
			const rt0 = createCommandRuntime(spec, { kernel: new StubKernel(), document: { topology: new TopologyGraph(), nodes: [] } });
			const rt1 = createCommandRuntime(spec, {
				kernel: new StubKernel(),
				document: { topology: new TopologyGraph(), nodes: [] },
				stateEngine: pureTsStateEngineProvider,
			});
			await rt0.send({ kind: "start" });
			await rt1.send({ kind: "start" });
			expect(rt1.getSnapshot().state).toBe(rt0.getSnapshot().state);
			expect(rt1.getSnapshot().context).toEqual(rt0.getSnapshot().context);
		});
	});
}
// #endregion 🧪Tests

