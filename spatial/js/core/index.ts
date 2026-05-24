// #region 🧲Header
/** @emoji 🧭 `@spatial/js-core` — portable interaction spec runtime, `ActionRegistry`, `StateEngine` + `KernelAdapter`, topology graph, derived views. See `spatial/schema/json` and `.repo/✍️/spatial.md`. */
// #endregion 🧲Header

// #region 📥Fixtures
import areaInteractionJson from "../../fixtures/area.interaction.json" with { type: "json" };
import boxInteractionJson from "../../fixtures/box.interaction.json" with { type: "json" };
import distanceInteractionJson from "../../fixtures/distance.interaction.json" with { type: "json" };
import extrudeWireInteractionJson from "../../fixtures/extrude-wire.interaction.json" with { type: "json" };
import offsetSurfaceInteractionJson from "../../fixtures/offset-surface.interaction.json" with { type: "json" };
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

// #region 🎮InteractionEvent
/** @emoji 🧭 Interaction input envelope; `kind` selects `machine.states[*].on` keys. */
export type InteractionEvent = { readonly kind: string; readonly [k: string]: unknown };
// #endregion 🎮InteractionEvent

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
export interface SelectionEvent extends InteractionEvent {
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
	spec: {
		readonly machine: {
			readonly states: readonly { readonly name: string; readonly selection?: SelectionSpec }[];
		};
	},
	state: string,
): SelectionSpec | null {
	const st = spec.machine.states.find((s) => s.name === state);
	const s = st?.selection;
	return s ?? null;
}
// #endregion 🪪Selection

// #region 🗺️Paths
/** @emoji 🧭 Root object for segmented path reads (`context` vs `event`). */
export type PathRoot = "context" | "event";

/** @emoji 🧭 One navigation step: object field or array index (no dynamic JSON keys). */
export type PathSegment =
	| { readonly kind: "field"; readonly name: string }
	| { readonly kind: "index"; readonly index: number };

/** @emoji 🧭 Absolute path into `context` or `event` payloads. */
export interface PathTarget {
	readonly root: PathRoot;
	readonly segments: readonly PathSegment[];
}

/** @emoji 🧭 Reads `segments` from `root` (object/array chain). */
export function readPathSegments(root: unknown, segments: readonly PathSegment[]): unknown {
	let cur: unknown = root;
	for (const seg of segments) {
		if (cur === null || cur === undefined) return undefined;
		if (seg.kind === "field") {
			if (typeof cur !== "object" || Array.isArray(cur)) return undefined;
			cur = (cur as Record<string, unknown>)[seg.name];
		} else {
			if (!Array.isArray(cur)) return undefined;
			cur = cur[seg.index];
		}
	}
	return cur;
}

/** @emoji 🧭 Resolves a `PathTarget` against `ExprEnv`. */
export function readPathTarget(t: PathTarget, env: ExprEnv): unknown {
	const root = t.root === "context" ? env.context : env.event;
	return readPathSegments(root, t.segments);
}

/** @emoji 🧭 Writes `value` at `segments` under `root` (creates object/array shells). */
export function writePathSegments(root: Record<string, unknown>, segments: readonly PathSegment[], value: unknown): void {
	if (segments.length === 0) return;
	let cur: Record<string, unknown> | unknown[] = root;
	for (let i = 0; i < segments.length - 1; i++) {
		const seg = segments[i]!;
		const next = segments[i + 1]!;
		const nextIsIndex = next.kind === "index";
		if (seg.kind === "field") {
			const o = cur as Record<string, unknown>;
			let child = o[seg.name];
			if (child === undefined || child === null || typeof child !== "object") {
				o[seg.name] = nextIsIndex ? ([] as unknown[]) : {};
				child = o[seg.name];
			}
			cur = child as Record<string, unknown> | unknown[];
		} else {
			const arr = cur as unknown[];
			let child = arr[seg.index];
			if (child === undefined || child === null || typeof child !== "object") {
				arr[seg.index] = nextIsIndex ? ([] as unknown[]) : {};
				child = arr[seg.index];
			}
			cur = child as Record<string, unknown> | unknown[];
		}
	}
	const last = segments[segments.length - 1]!;
	if (last.kind === "field") {
		(cur as Record<string, unknown>)[last.name] = value;
	} else {
		(cur as unknown[])[last.index] = value;
	}
}

/** @emoji 🧭 Writes into `env.context` using a context-rooted path. */
export function writePathTarget(t: PathTarget, env: ExprEnv, value: unknown): void {
	if (t.root !== "context") return;
	writePathSegments(env.context, t.segments, value);
}

/** @emoji 🧭 Clears the value at `segments` (deletes final field or sets array slot to `undefined`). */
export function clearPathSegments(root: Record<string, unknown>, segments: readonly PathSegment[]): void {
	if (segments.length === 0) return;
	if (segments.length === 1 && segments[0]!.kind === "field") {
		delete root[segments[0]!.name];
		return;
	}
	let cur: unknown = root;
	for (let i = 0; i < segments.length - 1; i++) {
		const seg = segments[i]!;
		if (cur === null || cur === undefined) return;
		if (seg.kind === "field") cur = (cur as Record<string, unknown>)[seg.name];
		else cur = (cur as unknown[])[seg.index];
	}
	const last = segments[segments.length - 1]!;
	const parent = cur as Record<string, unknown> | unknown[];
	if (last.kind === "field") delete (parent as Record<string, unknown>)[last.name];
	else (parent as unknown[])[last.index] = undefined;
}

/** @emoji 🧭 Clears `target` on `env.context`. */
export function clearPathTarget(t: PathTarget, env: ExprEnv): void {
	if (t.root !== "context") return;
	clearPathSegments(env.context, t.segments);
}
// #endregion 🗺️Paths

// #region 🏷️Metadata
/** @emoji 🏷️ Sidecar semantic fields keyed by topology or derived entity id; each write bumps hosting `TopologyGraph.revision`. */
export class EntityMetadataStore {
	private readonly byId = new Map<string, Record<string, unknown>>();

	constructor(private readonly bumpRevision: () => void) {}

	get(id: string): Readonly<Record<string, unknown>> | undefined {
		const r = this.byId.get(id);
		return r ? r : undefined;
	}

	setField(id: string, key: string, value: unknown): void {
		let r = this.byId.get(id);
		if (!r) {
			r = {};
			this.byId.set(id, r);
		}
		r[key] = value;
		this.bumpRevision();
	}

	deleteEntity(id: string): void {
		if (this.byId.delete(id)) this.bumpRevision();
	}
}

/** @emoji 🪪 `evalExpr` `field` target: a bound topology row entity (`kind` + `id`). */
export interface TopologyEntityRef {
	readonly kind: TopologyEntityKind;
	readonly id: string;
}
// #endregion 🏷️Metadata

// #region 🗺️Expr
/** @emoji 🗺️ Tagged declarative expression evaluated by `evalExpr` (`spatial/schema/json/expression.json`). */
export type Expr =
	| ExprPath
	| ExprConst
	| ExprVar
	| ExprField
	| ExprLet
	| ExprExists
	| ExprNotEmpty
	| ExprAll
	| ExprAny
	| ExprNot
	| ExprAbs
	| ExprDistance
	| ExprBinop
	| ExprFold;

export interface ExprPath {
	readonly kind: "path";
	readonly root: PathRoot;
	readonly segments: readonly PathSegment[];
}
export interface ExprConst {
	readonly kind: "const";
	readonly value: unknown;
}
export interface ExprVar {
	readonly kind: "var";
	readonly name: string;
}
export interface ExprField {
	readonly kind: "field";
	readonly object: Expr;
	readonly name: string;
}
export interface ExprLet {
	readonly kind: "let";
	readonly bindings: readonly { readonly name: string; readonly value: Expr }[];
	readonly in: Expr;
}
export interface ExprExists {
	readonly kind: "exists";
	readonly target: PathTarget;
}
export interface ExprNotEmpty {
	readonly kind: "notEmpty";
	readonly target: PathTarget;
}
export interface ExprAll {
	readonly kind: "all";
	readonly args: readonly Expr[];
}
export interface ExprAny {
	readonly kind: "any";
	readonly args: readonly Expr[];
}
export interface ExprNot {
	readonly kind: "not";
	readonly arg: Expr;
}
export interface ExprAbs {
	readonly kind: "abs";
	readonly arg: Expr;
}
export interface ExprDistance {
	readonly kind: "distance";
	readonly a: Expr;
	readonly b: Expr;
}
export interface ExprBinop {
	readonly kind: "binop";
	readonly op: "==" | "!=" | ">" | "<" | ">=" | "<=" | "+" | "-" | "*" | "/";
	readonly left: Expr;
	readonly right: Expr;
}
export interface ExprFold {
	readonly kind: "fold";
	readonly op: "min" | "max";
	readonly args: readonly [Expr, Expr];
}

export interface ExprEnv {
	readonly context: Record<string, unknown>;
	readonly event?: Record<string, unknown>;
	readonly vars?: Record<string, unknown>;
	readonly topology?: TopologyGraph;
	readonly metadata?: EntityMetadataStore;
	readonly derived?: DerivedViewService;
}

function envWithVars(base: ExprEnv, vars: Record<string, unknown>): ExprEnv {
	return {
		context: base.context,
		event: base.event,
		vars: { ...base.vars, ...vars },
		topology: base.topology,
		metadata: base.metadata,
		derived: base.derived,
	};
}

function isVec3(v: unknown): v is Vec3 {
	return Array.isArray(v) && v.length === 3 && v.every((n) => typeof n === "number");
}

function isTopologyEntityRef(v: unknown): v is TopologyEntityRef {
	if (!v || typeof v !== "object") return false;
	const o = v as Record<string, unknown>;
	return typeof o.kind === "string" && typeof o.id === "string";
}

/** @emoji 🧮 Evaluates a tagged `Expr` against `ExprEnv` (guards + action values). */
export function evalExpr(expr: Expr, env: ExprEnv): unknown {
	switch (expr.kind) {
		case "const":
			return expr.value;
		case "path":
			return readPathTarget({ root: expr.root, segments: expr.segments }, env);
		case "var":
			return env.vars ? env.vars[expr.name] : undefined;
		case "field": {
			const o = evalExpr(expr.object, env);
			const topo = env.topology;
			if (!topo || !isTopologyEntityRef(o)) return undefined;
			return readTopologyEntityProperty(topo, env.metadata, o.kind, o.id, expr.name, { derived: env.derived });
		}
		case "let": {
			const next: Record<string, unknown> = {};
			for (const b of expr.bindings) {
				next[b.name] = evalExpr(b.value, env);
			}
			return evalExpr(expr.in, envWithVars(env, next));
		}
		case "exists": {
			const v = readPathTarget(expr.target, env);
			return v !== undefined && v !== null;
		}
		case "notEmpty": {
			const v = readPathTarget(expr.target, env);
			if (v === undefined || v === null) return false;
			if (Array.isArray(v)) return v.length > 0;
			if (typeof v === "string") return v.length > 0;
			return true;
		}
		case "all":
			return expr.args.every((x) => Boolean(evalExpr(x, env)));
		case "any":
			return expr.args.some((x) => Boolean(evalExpr(x, env)));
		case "not":
			return !Boolean(evalExpr(expr.arg, env));
		case "abs": {
			const v = evalExpr(expr.arg, env);
			return typeof v === "number" ? Math.abs(v) : undefined;
		}
		case "distance": {
			const va = evalExpr(expr.a, env);
			const vb = evalExpr(expr.b, env);
			if (!isVec3(va) || !isVec3(vb)) return undefined;
			return vec3Distance(va, vb);
		}
		case "fold":
			return expr.op === "min"
				? Math.min(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env)))
				: Math.max(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env)));
		case "binop": {
			const left = evalExpr(expr.left, env);
			const right = evalExpr(expr.right, env);
			switch (expr.op) {
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
		default:
			return undefined;
	}
}

/** @emoji 🧭 Coerces `evalExpr` output to strict boolean guard result. */
export function evalGuard(expr: Expr, env: ExprEnv): boolean {
	return Boolean(evalExpr(expr, env));
}
// #endregion 🗺️Expr

// #region 📜Spec
/** @emoji 📜 Declared interaction-local context slots (`spatial.interaction/v1` `context`). */
export interface ContextFieldDecl {
	readonly name: string;
	readonly kind: "string" | "number" | "boolean" | "vec3" | "stringArray" | "unknown";
	readonly enumValues?: readonly string[];
}

/** @emoji 📜 Named guard binding (`guards[]`). */
export interface NamedGuard {
	readonly name: string;
	readonly expr: Expr;
}

export interface TransitionSpec {
	readonly target?: string;
	readonly guard?: string;
	readonly transient?: boolean;
	readonly effects?: readonly EffectSpec[];
	readonly key?: string;
	readonly label?: string;
}

export type KernelQueryParams = {
	readonly kind: "surface.resolveFaces";
	readonly surfaceId: Expr;
};

export type EffectSpec =
	| { readonly op: "assign"; readonly target: PathTarget; readonly value: Expr }
	| { readonly op: "clear"; readonly target: PathTarget }
	| { readonly op: "append"; readonly target: PathTarget; readonly value: Expr }
	| { readonly op: "emit"; readonly event: { readonly kind: string } }
	| { readonly op: "raise"; readonly event: string }
	| { readonly op: "openTransaction" }
	| { readonly op: "commitTransaction" }
	| { readonly op: "rollbackTransaction" }
	| { readonly op: "requestPreview" }
	| { readonly op: "kernel.query"; readonly query: string; readonly assignTo: PathTarget; readonly params: KernelQueryParams }
	| { readonly op: "resolveEditable" }
	| { readonly op: "setDiagnostic"; readonly severity: "info" | "warning" | "error"; readonly code: string; readonly message: string }
	| { readonly op: "clearDiagnostic"; readonly code: string }
	| { readonly op: "action"; readonly action: string; readonly params?: Record<string, Expr> };

export interface EventHandlerSpec {
	readonly event: string;
	readonly transitions: readonly TransitionSpec[];
}

export interface StateDefSpec {
	readonly name: string;
	readonly final?: boolean;
	readonly selection?: SelectionSpec;
	readonly on?: readonly EventHandlerSpec[];
}

export type DisplayItemSpec =
	| { readonly kind: "point"; readonly id: string; readonly role?: string; readonly position: Expr }
	| { readonly kind: "label"; readonly id: string; readonly role?: string; readonly text: string; readonly position: Expr }
	| { readonly kind: "segment"; readonly id: string; readonly role?: string; readonly from: Expr; readonly to: Expr }
	| { readonly kind: "linear-handle"; readonly id: string; readonly role?: string; readonly axis: Vec3; readonly origin: Expr }
	| { readonly kind: "box-preview"; readonly id: string; readonly role?: string; readonly cornerA: Expr; readonly cornerB: Expr; readonly height: Expr }
	| {
			readonly kind: "entity-highlight";
			readonly id: string;
			readonly role?: string;
			readonly topologyEntityKind: TopologyEntityKind;
			readonly entityId: Expr;
	  }
	| { readonly kind: "curve"; readonly id: string; readonly role?: string }
	| { readonly kind: "mesh"; readonly id: string; readonly role?: string };

export type CommitOperationSpec = {
	readonly kind: "action";
	readonly action: string;
	readonly params?: Record<string, Expr>;
};

/** @emoji 📜 Parsed static interaction document (`spatial.interaction/v1`). */
export interface InteractionSpec {
	readonly schema: "spatial.interaction/v1";
	readonly id: string;
	readonly version: string;
	readonly label?: string;
	readonly context?: { readonly fields: readonly ContextFieldDecl[] };
	readonly requires?: Record<string, unknown>;
	readonly guards?: readonly NamedGuard[];
	readonly machine: {
		readonly initial: string;
		readonly states: readonly StateDefSpec[];
	};
	readonly display?: {
		readonly states?: readonly { readonly state: string; readonly items: readonly DisplayItemSpec[] }[];
	};
	readonly interaction?: InteractionSpatialConfig;
	readonly commit: {
		readonly when?: string;
		readonly fromStates?: readonly string[];
		readonly outputDataPath?: PathTarget;
		readonly operation: CommitOperationSpec;
	};
}

/** @emoji 🎮 Host + viewport hints for spatial picking (declared per interaction preset). */
export interface InteractionSpatialConfig {
	readonly spatialGroundPick?: boolean;
	readonly pickDisabledStates?: readonly string[];
	readonly groundPointerMoveStates?: readonly string[];
	readonly heightDragStates?: readonly string[];
	readonly verticalRodStates?: readonly string[];
	readonly heightConfirmState?: string | null;
}

function guardNames(spec: InteractionSpec): Set<string> {
	return new Set((spec.guards ?? []).map((g) => g.name));
}

function findState(spec: InteractionSpec, name: string): StateDefSpec | undefined {
	return spec.machine.states.find((s) => s.name === name);
}

function isFinalInteractionState(spec: InteractionSpec, state: string): boolean {
	return Boolean(findState(spec, state)?.final);
}

function listFinalInteractionStates(spec: InteractionSpec): string[] {
	return spec.machine.states.filter((s) => s.final).map((s) => s.name);
}

/** @emoji 🧾 Validates and returns an `InteractionSpec` or `null` when malformed. */
export function parseInteractionSpec(raw: unknown): InteractionSpec | null {
	if (!raw || typeof raw !== "object") return null;
	const r = structuredClone(raw) as Record<string, unknown>;
	if (r.schema !== "spatial.interaction/v1") return null;
	if (typeof r.id !== "string" || typeof r.version !== "string") return null;
	const machine = r.machine;
	if (!machine || typeof machine !== "object") return null;
	const m = machine as Record<string, unknown>;
	if (typeof m.initial !== "string" || !Array.isArray(m.states)) return null;
	const states = m.states as unknown[];
	const names = new Set<string>();
	for (const st of states) {
		if (!st || typeof st !== "object") return null;
		const s = st as Record<string, unknown>;
		if (typeof s.name !== "string" || s.name.length === 0) return null;
		if (names.has(s.name)) return null;
		names.add(s.name);
		const sel = s.selection;
		if (sel) {
			if (typeof sel !== "object") return null;
			const acc = (sel as Record<string, unknown>).accept;
			if (!Array.isArray(acc) || acc.length === 0) return null;
			for (const k of acc) {
				if (typeof k !== "string" || !TOPOLOGY_ENTITY_KINDS.has(k)) return null;
			}
		}
		const on = s.on;
		if (on !== undefined) {
			if (!Array.isArray(on)) return null;
			for (const h of on) {
				if (!h || typeof h !== "object") return null;
				const he = h as Record<string, unknown>;
				if (typeof he.event !== "string" || !Array.isArray(he.transitions)) return null;
				for (const tr of he.transitions as unknown[]) {
					if (!tr || typeof tr !== "object") return null;
					const t = tr as Record<string, unknown>;
					if (t.effects !== undefined && !Array.isArray(t.effects)) return null;
					if (t.actions !== undefined) return null;
				}
			}
		}
	}
	if (!names.has(m.initial as string)) return null;
	const commit = r.commit;
	if (!commit || typeof commit !== "object") return null;
	const c = commit as Record<string, unknown>;
	const op = c.operation;
	if (!op || typeof op !== "object") return null;
	const o = op as Record<string, unknown>;
	if (o.kind !== "action" || typeof o.action !== "string") return null;
	const spec = r as unknown as InteractionSpec;
	const gn = guardNames(spec);
	if (c.when !== undefined && typeof c.when === "string" && !gn.has(c.when)) return null;
	for (const st of spec.machine.states) {
		for (const h of st.on ?? []) {
			for (const tr of h.transitions) {
				if (tr.guard !== undefined && typeof tr.guard === "string" && !gn.has(tr.guard)) return null;
			}
		}
	}
	return spec;
}

/** @emoji 🧭 Normalizes a parsed interaction (currently identity). */
export function compileInteraction(spec: InteractionSpec): InteractionSpec {
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
	readonly vertices: readonly VertexRecord[];
	readonly edges: readonly EdgeRecord[];
	readonly wires: readonly WireRecord[];
	readonly faces: readonly FaceRecord[];
	readonly shells: readonly ShellRecord[];
	readonly cells: readonly CellRecord[];
	readonly cellComplexes: readonly CellComplexRecord[];
	readonly clusters: readonly ClusterRecord[];
}

function recordsById<T extends { id: string }>(xs: readonly T[]): Record<string, T> {
	const o: Record<string, T> = {};
	for (const x of xs) o[x.id] = x;
	return o;
}

function sortedRecordValues<T extends { id: string }>(bucket: Record<string, T>): T[] {
	return Object.keys(bucket)
		.sort()
		.map((k) => bucket[k]!);
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
	readonly metadata: EntityMetadataStore = new EntityMetadataStore(() => this.bump());

	/** @emoji 🧭 Serializes to `TopologyGraphJson` (stable id-sorted arrays). */
	toJSON(): TopologyGraphJson {
		return {
			schema: "spatial.topology/v1",
			revision: this.revision,
			vertices: sortedRecordValues(this.vertices),
			edges: sortedRecordValues(this.edges),
			wires: sortedRecordValues(this.wires),
			faces: sortedRecordValues(this.faces),
			shells: sortedRecordValues(this.shells),
			cells: sortedRecordValues(this.cells),
			cellComplexes: sortedRecordValues(this.cellComplexes),
			clusters: sortedRecordValues(this.clusters),
		};
	}

	/** @emoji 🧭 Hydrates from `TopologyGraphJson`. */
	static fromJSON(j: TopologyGraphJson): TopologyGraph {
		const g = new TopologyGraph();
		g.revision = j.revision;
		g.vertices = recordsById(j.vertices);
		g.edges = recordsById(j.edges);
		g.wires = recordsById(j.wires);
		g.faces = recordsById(j.faces);
		g.shells = recordsById(j.shells);
		g.cells = recordsById(j.cells);
		g.cellComplexes = recordsById(j.cellComplexes);
		g.clusters = recordsById(j.clusters);
		return g;
	}

	bump(): void {
		this.revision += 1;
	}
}

/** @emoji 🧭 Reads `name` from metadata, then topology records, then optional `DerivedViewService` for `surface` / `part`. */
export function readTopologyEntityProperty(
	topo: TopologyGraph,
	meta: EntityMetadataStore | undefined,
	kind: TopologyEntityKind,
	id: string,
	name: string,
	opts?: { readonly derived?: DerivedViewService },
): unknown {
	const bag = meta?.get(id);
	if (bag && name in bag) return (bag as Record<string, unknown>)[name];
	switch (kind) {
		case "vertex":
			return (topo.vertices[id] as unknown as Record<string, unknown> | undefined)?.[name];
		case "edge":
			return (topo.edges[id] as unknown as Record<string, unknown> | undefined)?.[name];
		case "wire":
			return (topo.wires[id] as unknown as Record<string, unknown> | undefined)?.[name];
		case "face":
			return (topo.faces[id] as unknown as Record<string, unknown> | undefined)?.[name];
		case "shell":
			return (topo.shells[id] as unknown as Record<string, unknown> | undefined)?.[name];
		case "cell":
			return (topo.cells[id] as unknown as Record<string, unknown> | undefined)?.[name];
		case "cellComplex":
			return (topo.cellComplexes[id] as unknown as Record<string, unknown> | undefined)?.[name];
		case "cluster":
			return (topo.clusters[id] as unknown as Record<string, unknown> | undefined)?.[name];
		case "surface": {
			if (name === "id") return id;
			const d = opts?.derived;
			if (!d) return undefined;
			const hit = d.computeSurfaces(topo.revision, topo.faces).find((s) => String(s.id) === id);
			if (!hit) return undefined;
			return (hit as unknown as Record<string, unknown>)[name];
		}
		case "part": {
			if (name === "id") return id;
			const d = opts?.derived;
			if (!d) return undefined;
			const hit = d.computeParts(topo.revision, topo.cells).find((p) => String(p.id) === id);
			if (!hit) return undefined;
			return (hit as unknown as Record<string, unknown>)[name];
		}
		default:
			return undefined;
	}
}

/** @emoji 🧾 Parses `spatial.topology/v1` JSON into a graph or returns `null`. */
export function parseTopologyGraphJson(raw: unknown): TopologyGraph | null {
	if (!raw || typeof raw !== "object") return null;
	const r = raw as Record<string, unknown>;
	if (r.schema !== "spatial.topology/v1") return null;
	const need = ["vertices", "edges", "wires", "faces", "shells", "cells", "cellComplexes", "clusters"] as const;
	for (const k of need) {
		if (!Array.isArray(r[k])) return null;
	}
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

/** @emoji 🧮 Forward patch bucket for one topology table (`added` / `modified` / `removed` arrays). */
export interface EntityDiff<TRec, TDiff, TId extends string> {
	readonly added?: readonly TRec[];
	readonly modified?: readonly TDiff[];
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
	const a = e.added?.length ?? 0;
	const m = e.modified?.length ?? 0;
	const r = e.removed?.length ?? 0;
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
			if (!inverse.added) inverse.added = [];
			(inverse.added as T[]).push(cloneRec(cur));
			delete bucket[id];
		}
	}
	if (section.added) {
		for (const rec of section.added) {
			const id = rec.id;
			bucket[id] = cloneRec(rec as T);
			if (!inverse.removed) inverse.removed = [];
			(inverse.removed as string[]).push(id);
		}
	}
	if (section.modified) {
		for (const md of section.modified) {
			const id = md.id;
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
			if (!inverse.modified) inverse.modified = [];
			(inverse.modified as TDiff[]).push(back as TDiff);
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
			added: [
				{ id: v0, position: [ctr[0] + eps, ctr[1], ctr[2]] },
				{ id: v1, position: [ctr[0], ctr[1] + eps, ctr[2]] },
				{ id: v2, position: [ctr[0], ctr[1], ctr[2] + eps] },
			],
		},
		edges: {
			added: [
				{ id: e0, vertexIds: [v0, v1] },
				{ id: e1, vertexIds: [v1, v2] },
				{ id: e2, vertexIds: [v2, v0] },
			],
		},
		wires: { added: [{ id: wireId, edgeIds: [e0, e1, e2] }] },
		faces: {
			added: [{ id: faceId, wireIds: [wireId] }],
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
	adjacentCells?(cell: CellRef, topo: TopologyGraph): Promise<readonly CellRef[]>;
	sharedFacesBetween?(a: CellRef, b: CellRef, topo: TopologyGraph): Promise<readonly FaceRef[]>;
}
// #endregion 🔌Kernel

// #region 🧮ActionRegistry
/** @emoji 🧩 Serializable context patch applied after pure box geometry actions (`set` keys merged; `del` removes top-level context keys). */
export interface ActionContextPatch {
	readonly set?: Record<string, unknown>;
	readonly del?: readonly string[];
}

/** @emoji 🧩 Pure action output: optional topology `diff`, scalar `data`, or context `patch` for interactive preview fields. */
export interface ActionResult<TData = unknown> {
	readonly diff?: TopologyDiff;
	readonly data?: TData;
	readonly patch?: ActionContextPatch;
}

export type ActionFn<TParams = Record<string, unknown>, TData = unknown> = (
	params: TParams,
	ctx: { readonly kernel: KernelAdapter; readonly topology: TopologyGraph },
) => Promise<ActionResult<TData>> | ActionResult<TData>;

/** @emoji 🧩 Registerable pure spatial action (`id` is stable registry key). */
export interface ActionDef<TParams = Record<string, unknown>, TData = unknown> {
	readonly id: string;
	readonly label?: string;
	readonly run: ActionFn<TParams, TData>;
}

function applyActionPatchToContext(ctx: Record<string, unknown>, patch: ActionContextPatch | undefined): void {
	if (!patch) return;
	if (patch.set) Object.assign(ctx, patch.set);
	if (patch.del) for (const k of patch.del) delete ctx[k];
}

/** @emoji 🧭 Runtime registry for pure `ActionDef` entries (built-ins + host overrides). */
export class ActionRegistry {
	private readonly defs = new Map<string, ActionDef>();

	register(def: ActionDef): void {
		this.defs.set(def.id, def);
	}

	get(id: string): ActionDef | null {
		return this.defs.get(id) ?? null;
	}

	list(): readonly ActionDef[] {
		return [...this.defs.values()];
	}

	static withBuiltins(): ActionRegistry {
		const r = new ActionRegistry();
		for (const d of builtinActionDefs()) r.register(d);
		return r;
	}
}

function builtinActionDefs(): ActionDef[] {
	const ctxOf = (p: Record<string, unknown>) => p.__context as Record<string, unknown>;
	const boxAabbFromDiagonalCorners: ActionDef = {
		id: "box.aabbFromDiagonalCorners",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const ev = params.__event as InteractionEvent;
			const pt = (ev as { point?: unknown }).point;
			const P = isVec3(pt) ? pt : null;
			const a = ctx.diagA;
			if (!isVec3(a) || !P) return {};
			const z = a[2];
			return {
				patch: {
					set: {
						origin: [Math.min(a[0], P[0]), Math.min(a[1], P[1]), z] as Vec3,
						corner: [Math.max(a[0], P[0]), Math.max(a[1], P[1]), z] as Vec3,
					},
					del: ["diagA"],
				},
			};
		},
	};
	const boxTripletRubber: ActionDef = {
		id: "box.tripletRubber",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const ev = params.__event as InteractionEvent;
			const P = isVec3((ev as { point?: unknown }).point) ? ((ev as { point: Vec3 }).point as Vec3) : null;
			const p0 = ctx.p0;
			const p1 = ctx.p1;
			if (!isVec3(p0) || !isVec3(p1) || !P) return {};
			const z = p0[2];
			return {
				patch: {
					set: {
						previewA: [Math.min(p0[0], p1[0], P[0]), Math.min(p0[1], p1[1], P[1]), z] as Vec3,
						previewB: [Math.max(p0[0], p1[0], P[0]), Math.max(p0[1], p1[1], P[1]), z] as Vec3,
					},
				},
			};
		},
	};
	const boxTripletCommit: ActionDef = {
		id: "box.tripletCommit",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const ev = params.__event as InteractionEvent;
			const P = isVec3((ev as { point?: unknown }).point) ? ((ev as { point: Vec3 }).point as Vec3) : null;
			const p0 = ctx.p0;
			const p1 = ctx.p1;
			if (!isVec3(p0) || !isVec3(p1) || !P) return {};
			const z = p0[2];
			return {
				patch: {
					set: {
						origin: [Math.min(p0[0], p1[0], P[0]), Math.min(p0[1], p1[1], P[1]), z] as Vec3,
						corner: [Math.max(p0[0], p1[0], P[0]), Math.max(p0[1], p1[1], P[1]), z] as Vec3,
					},
					del: ["p0", "p1", "previewA", "previewB"],
				},
			};
		},
	};
	const boxSnapSquareFootprint: ActionDef = {
		id: "box.snapSquareFootprint",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const ev = params.__event as InteractionEvent;
			const P = isVec3((ev as { point?: unknown }).point) ? ((ev as { point: Vec3 }).point as Vec3) : null;
			const o = ctx.origin;
			if (!isVec3(o) || !P) return {};
			const dx = P[0] - o[0];
			const dy = P[1] - o[1];
			const s = Math.max(Math.abs(dx), Math.abs(dy), 1e-9);
			const sx = dx >= 0 ? 1 : -1;
			const sy = dy >= 0 ? 1 : -1;
			return { patch: { set: { corner: [o[0] + sx * s, o[1] + sy * s, o[2]] as Vec3 } } };
		},
	};
	const boxSetCubeHeightFromFootprint: ActionDef = {
		id: "box.setCubeHeightFromFootprint",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const o = ctx.origin;
			const c = ctx.corner;
			if (!isVec3(o) || !isVec3(c)) return {};
			const dx = Math.abs(c[0] - o[0]);
			const dy = Math.abs(c[1] - o[1]);
			return { patch: { set: { height: Math.max(dx, dy, 0.01) } } };
		},
	};
	const boxRubberCornerFromCenter: ActionDef = {
		id: "box.rubberCornerFromCenter",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const ev = params.__event as InteractionEvent;
			const P = isVec3((ev as { point?: unknown }).point) ? ((ev as { point: Vec3 }).point as Vec3) : null;
			const c = ctx.rectCenter;
			if (!isVec3(c) || !P) return {};
			return {
				patch: {
					set: {
						origin: [Math.min(2 * c[0] - P[0], P[0]), Math.min(2 * c[1] - P[1], P[1]), c[2]] as Vec3,
						corner: [Math.max(2 * c[0] - P[0], P[0]), Math.max(2 * c[1] - P[1], P[1]), c[2]] as Vec3,
					},
				},
			};
		},
	};
	const boxRubberSquareFromCenter: ActionDef = {
		id: "box.rubberSquareFromCenter",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const ev = params.__event as InteractionEvent;
			const P = isVec3((ev as { point?: unknown }).point) ? ((ev as { point: Vec3 }).point as Vec3) : null;
			const c = ctx.rectCenter;
			if (!isVec3(c) || !P) return {};
			const ox = Math.min(2 * c[0] - P[0], P[0]);
			const oy = Math.min(2 * c[1] - P[1], P[1]);
			const cx = Math.max(2 * c[0] - P[0], P[0]);
			const cy = Math.max(2 * c[1] - P[1], P[1]);
			const w = cx - ox;
			const d = cy - oy;
			const s = Math.max(w, d, 1e-9);
			return {
				patch: {
					set: {
						origin: [c[0] - s / 2, c[1] - s / 2, c[2]] as Vec3,
						corner: [c[0] + s / 2, c[1] + s / 2, c[2]] as Vec3,
					},
				},
			};
		},
	};
	const boxVerticalFinalizeFootprint: ActionDef = {
		id: "box.verticalFinalizeFootprint",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const ev = params.__event as InteractionEvent;
			const P = isVec3((ev as { point?: unknown }).point) ? ((ev as { point: Vec3 }).point as Vec3) : null;
			const o = ctx.origin;
			const pk = ctx.peak;
			if (!isVec3(o) || !isVec3(pk) || !P) return {};
			return {
				patch: {
					set: { corner: [P[0], P[1], o[2]] as Vec3, height: Math.max(0.01, Math.abs(pk[2] - o[2])) },
					del: ["peak"],
				},
			};
		},
	};
	const boxInitPeakAboveOrigin: ActionDef = {
		id: "box.initPeakAboveOrigin",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const o = ctx.origin;
			if (!isVec3(o)) return {};
			return { patch: { set: { peak: [o[0], o[1], o[2] + 0.25] as Vec3 } } };
		},
	};
	const boxPeakFromOriginZ: ActionDef = {
		id: "box.peakFromOriginZ",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const ev = params.__event as InteractionEvent;
			const P = isVec3((ev as { point?: unknown }).point) ? ((ev as { point: Vec3 }).point as Vec3) : null;
			const o = ctx.origin;
			if (!isVec3(o) || !P) return {};
			return { patch: { set: { peak: [o[0], o[1], P[2]] as Vec3 } } };
		},
	};
	const boxVerticalRubberCorner: ActionDef = {
		id: "box.verticalRubberCorner",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const ev = params.__event as InteractionEvent;
			const P = isVec3((ev as { point?: unknown }).point) ? ((ev as { point: Vec3 }).point as Vec3) : null;
			const o = ctx.origin;
			if (!isVec3(o) || !P) return {};
			return { patch: { set: { corner: [P[0], P[1], o[2]] as Vec3 } } };
		},
	};
	const boxCornerFromLengthWidth: ActionDef = {
		id: "box.cornerFromLengthWidth",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const ev = params.__event as InteractionEvent;
			const val = (ev as { value?: unknown }).value;
			const o = ctx.origin;
			if (!isVec3(o) || val === null || typeof val !== "object") return {};
			const rec = val as Record<string, unknown>;
			const L = Number(rec.length);
			const W = Number(rec.width);
			if (!Number.isFinite(L) || !Number.isFinite(W)) return {};
			return { patch: { set: { corner: [o[0] + L, o[1] + W, o[2]] as Vec3 } } };
		},
	};
	const primitiveCreateBoxFromCorners: ActionDef = {
		id: "primitive.createBoxFromCorners",
		run: async (params, { kernel }) => {
			const cornerA = params.cornerA as Vec3;
			const cornerB = params.cornerB as Vec3;
			const height = Number(params.height);
			let diff: TopologyDiff = EMPTY_TOPOLOGY_DIFF;
			if (kernel.createBoxFromCornersDiff) {
				const r = await kernel.createBoxFromCornersDiff({ cornerA, cornerB, height });
				diff = r.diff;
			} else {
				const cell = await kernel.createBoxFromCorners({ cornerA, cornerB, height });
				const preview = await kernel.tessellate(cell, 1e-3);
				diff = meshFaceTopologyDiff(preview, `f-${kernel.id}`);
			}
			return { diff };
		},
	};
	const primitiveCreateBoxFrom3Points: ActionDef = {
		id: "primitive.createBoxFrom3Points",
		run: async (params, ctx) => {
			const p0 = params.p0 as Vec3;
			const p1 = params.p1 as Vec3;
			const p2 = params.p2 as Vec3;
			if (!isVec3(p0) || !isVec3(p1) || !isVec3(p2)) return {};
			const z = p0[2];
			const cornerA: Vec3 = [Math.min(p0[0], p1[0], p2[0]), Math.min(p0[1], p1[1], p2[1]), z];
			const cornerB: Vec3 = [Math.max(p0[0], p1[0], p2[0]), Math.max(p0[1], p1[1], p2[1]), z];
			const dx = Math.abs(cornerB[0] - cornerA[0]);
			const dy = Math.abs(cornerB[1] - cornerA[1]);
			const height = Math.max(dx, dy, 0.01);
			return await Promise.resolve(primitiveCreateBoxFromCorners.run({ cornerA, cornerB, height }, ctx));
		},
	};
	const featureExtrudeWireToCell: ActionDef = {
		id: "feature.extrudeWireToCell",
		run: async (params, { kernel }) => {
			const input = {
				wireId: String(params.wireId),
				distance: Number(params.distance),
				direction: params.direction as Vec3,
			};
			let diff: TopologyDiff = EMPTY_TOPOLOGY_DIFF;
			if (kernel.extrudeWireDiff) diff = (await kernel.extrudeWireDiff(input)).diff;
			else {
				const cell = (await kernel.extrudeWire?.(input)) ?? null;
				if (cell) {
					const preview = await kernel.tessellate(cell, 1e-3);
					diff = meshFaceTopologyDiff(preview, `f${kernel.id}`);
				}
			}
			return { diff };
		},
	};
	const featureOffsetFaces: ActionDef = {
		id: "feature.offsetFaces",
		run: async (params, { kernel }) => {
			const faceIdsRaw = params.faceIds;
			const faceIds = Array.isArray(faceIdsRaw) ? (faceIdsRaw as unknown[]).map(String) : [];
			const diff =
				(await kernel.offsetFacesDiff?.({ faceIds, distance: Number(params.distance) }))?.diff ?? EMPTY_TOPOLOGY_DIFF;
			return { diff };
		},
	};
	const measureVertexDistance: ActionDef = {
		id: "measure.vertexDistance",
		run: async (params, { kernel, topology }) => {
			const a = params.a as VertexRef;
			const b = params.b as VertexRef;
			if (!kernel.vertexDistance) throw new Error("kernel.vertexDistance required");
			const data = await kernel.vertexDistance(a, b, topology);
			return { data };
		},
	};
	const measureFaceArea: ActionDef = {
		id: "measure.faceArea",
		run: async (params, { kernel, topology }) => {
			const fid = params.faceId as FaceRef;
			if (!kernel.faceArea) throw new Error("kernel.faceArea required");
			const data = await kernel.faceArea(fid, topology);
			return { data };
		},
	};
	const measureCellVolume: ActionDef = {
		id: "measure.cellVolume",
		run: async (params, { kernel }) => {
			const cid = params.cellId as CellRef;
			const data = await (kernel.cellVolume?.(cid) ?? kernel.volume(cid));
			return { data };
		},
	};
	return [
		boxAabbFromDiagonalCorners,
		boxTripletRubber,
		boxTripletCommit,
		boxSnapSquareFootprint,
		boxSetCubeHeightFromFootprint,
		boxRubberCornerFromCenter,
		boxRubberSquareFromCenter,
		boxVerticalFinalizeFootprint,
		boxInitPeakAboveOrigin,
		boxPeakFromOriginZ,
		boxVerticalRubberCorner,
		boxCornerFromLengthWidth,
		primitiveCreateBoxFromCorners,
		primitiveCreateBoxFrom3Points,
		featureExtrudeWireToCell,
		featureOffsetFaces,
		measureVertexDistance,
		measureFaceArea,
		measureCellVolume,
	];
}
// #endregion 🧮ActionRegistry

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

// #region 🔍ConstructQuery
/** @emoji 🔍 One named column in a `construct` result row. */
export type ConstructQueryRow = Readonly<Record<string, unknown>>;

/** @emoji 🔍 `construct` runner output (`rows` always materialized; optional `data`/`diff` from CALL). */
export interface ConstructQueryResult {
	readonly rows: readonly ConstructQueryRow[];
	readonly data?: unknown;
	readonly diff?: TopologyDiff;
}

/** @emoji 🔍 Host wiring for `InteractionRuntime.query` (`@spatial/js-query` supplies the default runner). */
export interface ConstructQueryContext {
	readonly topology: TopologyGraph;
	readonly kernel: KernelAdapter;
	readonly actions: ActionRegistry;
	readonly derived?: DerivedViewService;
}

/** @emoji 🔍 Async bridge so core never imports `@spatial/js-query`. */
export type ConstructRunner = (text: string, ctx: ConstructQueryContext) => Promise<ConstructQueryResult>;
// #endregion 🔍ConstructQuery

// #region 🎬Statechart
/** @emoji 🎭 Result of `StateEngine.send` / `applyTransition` (`transient` skips interaction-local undo). */
export interface StateEngineSendResult {
	readonly ok: boolean;
	readonly transient?: boolean;
}

/** @emoji 🎭 `applyTransition` output: next factory state + disambiguation index for XState routing. */
export interface ApplyTransitionResult extends StateEngineSendResult {
	readonly nextState: string;
	readonly branchIndex: number;
}

/** @emoji 🎭 Pluggable state backend for `InteractionRuntime` (pure TS, XState, …). */
export interface StateEngine {
	getState(): string;
	getContext(): Record<string, unknown>;
	reset(): void;
	restore(state: string, context: Record<string, unknown>): void;
	send(
		event: InteractionEvent,
		kernel?: KernelAdapter,
		topology?: TopologyGraph,
		actions?: ActionRegistry,
	): Promise<StateEngineSendResult>;
}

/** @emoji 🎭 Instantiates a `StateEngine` for a compiled `InteractionSpec`. */
export interface StateEngineProvider {
	readonly id: string;
	create(spec: InteractionSpec): StateEngine;
}

function lookupGuard(spec: InteractionSpec, name: string): Expr | undefined {
	return spec.guards?.find((g) => g.name === name)?.expr;
}

/** @emoji 🧮 Serializes `KernelQueryParams` into the loose record shape expected by `KernelAdapter.query`. */
function kernelQueryParamsToRecord(p: KernelQueryParams, env: ExprEnv): Record<string, unknown> {
	if (p.kind === "surface.resolveFaces") {
		return { surfaceId: evalExpr(p.surfaceId, env) };
	}
	return {};
}

/** @emoji 🎬 Applies one declarative transition `EffectSpec` (async kernel queries + registered `ActionRegistry` calls). */
export async function applyEffectAsync(
	a: EffectSpec,
	ctx: Record<string, unknown>,
	event: InteractionEvent,
	kernel: KernelAdapter | undefined,
	topology: TopologyGraph,
	actions?: ActionRegistry,
): Promise<void> {
	const env: ExprEnv = { context: ctx, event };
	const reg = actions ?? ActionRegistry.withBuiltins();
	if (a.op === "assign") {
		const v = evalExpr(a.value, env);
		writePathTarget(a.target, env, v);
	} else if (a.op === "clear") {
		clearPathTarget(a.target, env);
	} else if (a.op === "append") {
		const cur = readPathTarget(a.target, env);
		const v = evalExpr(a.value, env);
		if (Array.isArray(cur)) {
			const next = [...cur, v];
			writePathTarget(a.target, env, next);
		}
	} else if (a.op === "kernel.query" && kernel?.query) {
		const params = kernelQueryParamsToRecord(a.params, env);
		const res = await kernel.query(a.query, params);
		writePathTarget(a.assignTo, env, res);
	} else if (a.op === "action") {
		const def = reg.get(a.action);
		if (!def) return;
		const paramBag: Record<string, unknown> = { __context: ctx, __event: event };
		for (const [k, ex] of Object.entries(a.params ?? {})) {
			paramBag[k] = evalExpr(ex, env);
		}
		const k = kernel ?? (null as unknown as KernelAdapter);
		const r = await Promise.resolve(def.run(paramBag, { kernel: k, topology }));
		if (r.patch) applyActionPatchToContext(ctx, r.patch);
	}
}

/** @emoji 🎬 First matching transition for `event` from `state`; mutates `context` in place. */
export async function applyTransition(
	spec: InteractionSpec,
	state: string,
	context: Record<string, unknown>,
	event: InteractionEvent,
	kernel?: KernelAdapter,
	actions?: ActionRegistry,
	topology?: TopologyGraph,
): Promise<ApplyTransitionResult> {
	const topo = topology ?? new TopologyGraph();
	const st = findState(spec, state);
	const handler = st?.on?.find((h) => h.event === event.kind);
	if (!handler) return { ok: false, nextState: state, branchIndex: -1 };
	const choices = handler.transitions;
	if (choices.length === 0) return { ok: false, nextState: state, branchIndex: -1 };
	for (let i = 0; i < choices.length; i++) {
		const tr = choices[i]!;
		if (tr.guard) {
			const g = lookupGuard(spec, tr.guard);
			if (!g || !evalGuard(g, { context, event })) continue;
		}
		for (const eff of tr.effects ?? []) {
			await applyEffectAsync(eff, context, event, kernel, topo, actions);
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

/** @emoji ⌨️ Resolved spatial host hints (defaults disable ground picking). */
export interface InteractionSpatialResolved {
	readonly spatialGroundPick: boolean;
	readonly pickDisabledStates: readonly string[];
	readonly groundPointerMoveStates: readonly string[];
	readonly heightDragStates: readonly string[];
	readonly verticalRodStates: readonly string[];
	readonly heightConfirmState: string | null;
}

/** @emoji ⌨️ Merges `spec.interaction` with safe defaults for hosts and `InteractionSpatialView`. */
export function mergeInteractionSpatial(spec: InteractionSpec): InteractionSpatialResolved {
	const i = spec.interaction;
	const basePickDisabled = [...new Set([spec.machine.initial, "ready", ...listFinalInteractionStates(spec)])];
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
const HOST_KEYBIND_EXCLUDED_KINDS = new Set(["pointer.move", "pointer.down", "selection.changed"]);

export interface InteractionKeybindRow {
	readonly eventKind: string;
	readonly key: string;
	readonly label: string;
}

/** @emoji ⌨️ Lists keyed transitions for the active state (excludes pointer + selection). */
export function listKeyedInteractionTransitions(spec: InteractionSpec, state: string): readonly InteractionKeybindRow[] {
	const st = findState(spec, state);
	if (!st?.on) return [];
	const out: InteractionKeybindRow[] = [];
	for (const h of st.on) {
		if (HOST_KEYBIND_EXCLUDED_KINDS.has(h.event)) continue;
		for (const tr of h.transitions) {
			if (tr.transient) continue;
			const key = tr.key;
			const label = tr.label;
			if (typeof key !== "string" || key.length === 0) continue;
			if (typeof label !== "string" || label.length === 0) continue;
			out.push({ eventKind: h.event, key, label });
		}
	}
	return out;
}

/** @emoji 🎬 Minimal async statechart runner for `InteractionSpec.machine`. */
export class StatechartRuntime implements StateEngine {
	private state: string;
	private context: Record<string, unknown> = {};

	constructor(private readonly spec: InteractionSpec) {
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

	/** @emoji 🎬 Restores a prior `state` + `context` snapshot (interaction-local undo). */
	restore(state: string, context: Record<string, unknown>): void {
		this.state = state;
		this.context = context;
	}

	/** @emoji 🎬 Applies one external event; returns whether a transition fired. */
	async send(
		event: InteractionEvent,
		kernel?: KernelAdapter,
		topology?: TopologyGraph,
		actions?: ActionRegistry,
	): Promise<StateEngineSendResult> {
		const r = await applyTransition(this.spec, this.state, this.context, event, kernel, actions, topology);
		if (r.ok) this.state = r.nextState;
		return { ok: r.ok, transient: r.transient };
	}
}

/** @emoji 🎭 Default in-process engine (no XState); same semantics as `applyTransition`. */
export const pureTsStateEngineProvider: StateEngineProvider = {
	id: "pure-ts",
	create(spec: InteractionSpec): StateEngine {
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
export function resolveDisplay(spec: InteractionSpec, state: string, context: Record<string, unknown>): DisplayModel {
	const env: ExprEnv = { context };
	const section = spec.display?.states?.find((s) => s.state === state);
	const raw = section?.items ?? [];
	const items: DisplayItem[] = [];
	for (const it of raw) {
		switch (it.kind) {
			case "point":
				items.push({
					kind: "point",
					id: it.id,
					...(it.role ? { role: it.role } : {}),
					params: { position: evalExpr(it.position, env) },
				});
				break;
			case "label":
				items.push({
					kind: "label",
					id: it.id,
					...(it.role ? { role: it.role } : {}),
					params: { text: it.text, position: evalExpr(it.position, env) },
				});
				break;
			case "segment":
				items.push({
					kind: "segment",
					id: it.id,
					...(it.role ? { role: it.role } : {}),
					params: { from: evalExpr(it.from, env), to: evalExpr(it.to, env) },
				});
				break;
			case "linear-handle":
				items.push({
					kind: "linear-handle",
					id: it.id,
					...(it.role ? { role: it.role } : {}),
					params: { axis: [...it.axis], origin: evalExpr(it.origin, env) },
				});
				break;
			case "box-preview":
				items.push({
					kind: "box-preview",
					id: it.id,
					...(it.role ? { role: it.role } : {}),
					params: {
						cornerA: evalExpr(it.cornerA, env),
						cornerB: evalExpr(it.cornerB, env),
						height: evalExpr(it.height, env),
					},
				});
				break;
			case "entity-highlight": {
				const idVal = evalExpr(it.entityId, env);
				items.push({
					kind: "entity-highlight",
					id: it.id,
					...(it.role ? { role: it.role } : {}),
					params: { entity: { kind: it.topologyEntityKind, id: String(idVal ?? "") } },
				});
				break;
			}
			case "curve":
				items.push({ kind: "curve", id: it.id, ...(it.role ? { role: it.role } : {}) });
				break;
			case "mesh":
				items.push({ kind: "mesh", id: it.id, ...(it.role ? { role: it.role } : {}) });
				break;
			default:
				break;
		}
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

/** @emoji 📄 Working document: topology + committed shape nodes + command stack. */
export interface ModelDocument {
	readonly topology: TopologyGraph;
	nodes: ShapeNode[];
}

// #endregion 📄Document

// #region 📨Response
/** @emoji 📨 Portable command outcome envelope (`diff` + `data` + messages). */
export interface InteractionMessage {
	readonly code: string;
	readonly message: string;
	readonly path?: string;
}

/** @emoji 📨 Result returned by `InteractionRuntime.commit` (read/write topology + scalar `data`). */
export interface InteractionResponse<TData = unknown> {
	readonly ok: boolean;
	readonly errors: readonly InteractionMessage[];
	readonly warnings: readonly InteractionMessage[];
	readonly infos: readonly InteractionMessage[];
	readonly diff: TopologyDiff;
	readonly data: TData | null;
	/** @emoji 📦 Context clone immediately before the post-commit `confirm` transition; null when commit aborted before confirm. */
	readonly archiveContext: Record<string, unknown> | null;
}

/** @emoji 📨 Default empty success payload for guards and early returns. */
export const EMPTY_INTERACTION_RESPONSE: InteractionResponse<null> = {
	ok: true,
	errors: [],
	warnings: [],
	infos: [],
	diff: EMPTY_TOPOLOGY_DIFF,
	data: null,
	archiveContext: null,
};

/** @emoji 📄 One committed topology change plus inverse diff for document-level undo/redo. */
export interface Modification {
	readonly id: string;
	readonly interactionId: string;
	readonly label: string;
	readonly result: InteractionResponse;
	readonly backwardsDiff: TopologyDiff;
}

/** @emoji 📄 Two-stack modification history (undo / redo) keyed by topology diffs. */
export class DocumentHistory {
	private undoStack: Modification[] = [];
	private redoStack: Modification[] = [];

	record(mod: Modification): void {
		if (isEmptyTopologyDiff(mod.result.diff)) return;
		this.undoStack.push(mod);
		this.redoStack = [];
	}

	peekUndo(): Modification | null {
		const n = this.undoStack.length;
		return n ? this.undoStack[n - 1]! : null;
	}

	peekRedo(): Modification | null {
		const n = this.redoStack.length;
		return n ? this.redoStack[n - 1]! : null;
	}

	/** @emoji 📚 Committed undo stack in document order for renderer views. */
	entries(): readonly Modification[] {
		return [...this.undoStack];
	}

	/** @emoji 🧹 Drops undo and redo stacks when the host swaps the base document. */
	clear(): void {
		this.undoStack = [];
		this.redoStack = [];
	}

	undo(doc: ModelDocument): Modification | null {
		const mod = this.undoStack.pop();
		if (!mod) return null;
		applyTopologyDiff(doc.topology, mod.backwardsDiff);
		this.redoStack.push(mod);
		return mod;
	}

	redo(doc: ModelDocument): Modification | null {
		const mod = this.redoStack.pop();
		if (!mod) return null;
		applyTopologyDiff(doc.topology, mod.result.diff);
		this.undoStack.push(mod);
		return mod;
	}
}
// #endregion 📨Response

// #region 📜Interaction
/** @emoji 🩺 Non-fatal runtime diagnostic surfaced in snapshots. */
export interface Diagnostic {
	readonly severity: "info" | "warning" | "error";
	readonly code: string;
	readonly message: string;
}

/** @emoji 📜 Serializable interaction snapshot for hosts and renderers. */
export interface InteractionSnapshot {
	readonly interactionId: string;
	readonly state: string;
	readonly revision: number;
	readonly context: Record<string, unknown>;
	readonly display: DisplayModel;
	readonly spatialInteraction: InteractionSpatialResolved;
	readonly capabilities: { readonly canCommit: boolean; readonly canCancel: boolean; readonly canUndo: boolean; readonly canRedo: boolean };
	readonly diagnostics: readonly Diagnostic[];
	readonly lastResponse: InteractionResponse | null;
}

export interface InteractionRuntimeOptions {
	readonly kernel: KernelAdapter;
	readonly document: ModelDocument;
	readonly history?: DocumentHistory;
	readonly stateEngine?: StateEngineProvider;
	readonly actions?: ActionRegistry;
	readonly query?: ConstructRunner;
	readonly derived?: DerivedViewService;
}

/** @emoji 🧭 True while the statechart is between `machine.initial` and a declared final state. */
export function isInteractionSessionActive(spec: InteractionSpec, state: string): boolean {
	return state !== spec.machine.initial && !isFinalInteractionState(spec, state);
}

/** @emoji 📜 Headless + interactive interaction controller (`send`, `commit`, `undo`). */
export class InteractionRuntime {
	private readonly sm: StateEngine;
	private readonly actions: ActionRegistry;
	private revision = 0;
	private readonly listeners = new Set<() => void>();
	private readonly snapUndoStack: { state: string; context: string }[] = [];
	private readonly snapRedoStack: { state: string; context: string }[] = [];
	private snapshotCache: InteractionSnapshot | null = null;
	private lastResponse: InteractionResponse | null = null;
	private readonly pendingSnapshotInfos: InteractionMessage[] = [];

	constructor(
		private readonly spec: InteractionSpec,
		private readonly opts: InteractionRuntimeOptions,
	) {
		this.sm = (opts.stateEngine ?? pureTsStateEngineProvider).create(spec);
		this.actions = opts.actions ?? ActionRegistry.withBuiltins();
	}

	private cloneCtx(c: Record<string, unknown>): Record<string, unknown> {
		return JSON.parse(JSON.stringify(c)) as Record<string, unknown>;
	}

	private inActiveInteraction(): boolean {
		return isInteractionSessionActive(this.spec, this.sm.getState());
	}

	private canCommit(): boolean {
		return this.canCommitFromState(this.sm.getState());
	}

	private canCommitFromState(st: string): boolean {
		const allowed = this.spec.commit.fromStates ?? ["ready"];
		if (!allowed.includes(st)) return false;
		const w = this.spec.commit.when;
		if (w) {
			const g = lookupGuard(this.spec, w);
			if (!g) return false;
			return evalGuard(g, { context: this.sm.getContext() });
		}
		return true;
	}

	/** @emoji 🧭 Accepted topology kinds for the active machine state (`[]` when none). */
	listActiveSelectionAccept(): readonly TopologyEntityKind[] {
		return getActiveSelectionSpec(this.spec, this.sm.getState())?.accept ?? [];
	}

	/** @emoji 🔍 Executes a `construct` script via `opts.query` (host registers `@spatial/js-query`). */
	async query(text: string): Promise<ConstructQueryResult> {
		const runner = this.opts.query;
		if (!runner) throw new Error("InteractionRuntime.query requires InteractionRuntimeOptions.query");
		return runner(text, {
			topology: this.opts.document.topology,
			kernel: this.opts.kernel,
			actions: this.actions,
			derived: this.opts.derived,
		});
	}

	getSnapshot(): InteractionSnapshot {
		if (this.snapshotCache) return this.snapshotCache;
		const ctx = this.sm.getContext();
		const st = this.sm.getState();
		const display = resolveDisplay(this.spec, st, ctx);
		const spatialInteraction = mergeInteractionSpatial(this.spec);
		const flushed = this.pendingSnapshotInfos.splice(0, this.pendingSnapshotInfos.length);
		const infoDiags: Diagnostic[] = flushed.map((m) => ({ severity: "info" as const, code: m.code, message: m.message }));
		const hist = this.opts.history;
		const active = this.inActiveInteraction();
		const canUndo = this.snapUndoStack.length > 0 || (!active && Boolean(hist?.peekUndo()));
		const canRedo = this.snapRedoStack.length > 0 || (!active && Boolean(hist?.peekRedo()));
		this.snapshotCache = {
			interactionId: this.spec.id,
			state: st,
			revision: this.revision,
			context: this.cloneCtx(ctx),
			display,
			spatialInteraction,
			capabilities: {
				canCommit: this.canCommit(),
				canCancel: this.inActiveInteraction(),
				canUndo,
				canRedo,
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

	/** @emoji 📜 Dispatches a typed interaction event through the statechart + optional kernel queries. */
	async send(event: InteractionEvent): Promise<void> {
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
		const beforeCanCommit = this.canCommitFromState(beforeState);
		const r = await this.sm.send(event, this.opts.kernel, this.opts.document.topology, this.actions);
		if (!r.ok) return;
		if (!r.transient) {
			this.snapUndoStack.push({ state: beforeState, context: JSON.stringify(beforeCtx) });
			this.snapRedoStack.length = 0;
		}
		if (beforeCanCommit && isFinalInteractionState(this.spec, this.sm.getState())) {
			await this.runCommit(false);
			return;
		}
		this.emit();
	}

	undo(): void {
		if (this.inActiveInteraction()) {
			const snap = this.snapUndoStack.pop();
			if (!snap) return;
			const curState = this.sm.getState();
			const curCtx = JSON.stringify(this.cloneCtx(this.sm.getContext()));
			this.snapRedoStack.push({ state: curState, context: curCtx });
			const o = JSON.parse(snap.context) as Record<string, unknown>;
			this.sm.restore(snap.state, o);
			this.emit();
			return;
		}
		if (this.snapUndoStack.length > 0) {
			const snap = this.snapUndoStack.pop();
			if (!snap) return;
			const curState = this.sm.getState();
			const curCtx = JSON.stringify(this.cloneCtx(this.sm.getContext()));
			this.snapRedoStack.push({ state: curState, context: curCtx });
			const o = JSON.parse(snap.context) as Record<string, unknown>;
			this.sm.restore(snap.state, o);
			this.emit();
			return;
		}
		const h = this.opts.history;
		if (h) h.undo(this.opts.document);
		this.emit();
	}

	redo(): void {
		if (this.inActiveInteraction()) {
			const snap = this.snapRedoStack.pop();
			if (!snap) return;
			const curState = this.sm.getState();
			const curCtx = JSON.stringify(this.cloneCtx(this.sm.getContext()));
			this.snapUndoStack.push({ state: curState, context: curCtx });
			const o = JSON.parse(snap.context) as Record<string, unknown>;
			this.sm.restore(snap.state, o);
			this.emit();
			return;
		}
		if (this.snapRedoStack.length > 0) {
			const snap = this.snapRedoStack.pop();
			if (!snap) return;
			const curState = this.sm.getState();
			const curCtx = JSON.stringify(this.cloneCtx(this.sm.getContext()));
			this.snapUndoStack.push({ state: curState, context: curCtx });
			const o = JSON.parse(snap.context) as Record<string, unknown>;
			this.sm.restore(snap.state, o);
			this.emit();
			return;
		}
		const h = this.opts.history;
		if (h) h.redo(this.opts.document);
		this.emit();
	}

	cancel(): void {
		this.snapUndoStack.length = 0;
		this.snapRedoStack.length = 0;
		this.sm.reset();
		this.emit();
	}

	private async runCommit(advanceToFinalState: boolean): Promise<InteractionResponse> {
		const fail = (code: string, message: string): InteractionResponse => {
			const res: InteractionResponse = {
				ok: false,
				errors: [{ code, message }],
				warnings: [],
				infos: [],
				diff: EMPTY_TOPOLOGY_DIFF,
				data: null,
				archiveContext: null,
			};
			this.lastResponse = res;
			this.emit();
			return res;
		};
		const st = this.sm.getState();
		if (advanceToFinalState && isFinalInteractionState(this.spec, st)) {
			return fail("interaction.alreadyCommitted", "Interaction already finalized.");
		}
		if (advanceToFinalState && !this.canCommit()) return fail("interaction.cannotCommit", "Commit guard or fromStates rejected this commit.");
		const ctx = this.sm.getContext();
		const op = this.spec.commit.operation;
		const env: ExprEnv = { context: ctx };
		const k = this.opts.kernel;
		const topo = this.opts.document.topology;
		let diff: TopologyDiff = EMPTY_TOPOLOGY_DIFF;
		let data: unknown = null;
		try {
			const def = this.actions.get(op.action);
			if (!def) throw new Error(`Unknown commit action: ${op.action}`);
			const paramBag: Record<string, unknown> = { __context: ctx, __event: { kind: "commit" } };
			for (const [key, ex] of Object.entries(op.params ?? {})) {
				paramBag[key] = evalExpr(ex, env);
			}
			const ar = await Promise.resolve(def.run(paramBag, { kernel: k, topology: topo }));
			diff = ar.diff ?? EMPTY_TOPOLOGY_DIFF;
			data = ar.data ?? null;
		} catch (e) {
			const msg = e instanceof Error ? e.message : String(e);
			return fail("interaction.commitFailed", msg);
		}
		const outPath = this.spec.commit.outputDataPath;
		if (outPath) {
			const ctx2 = this.sm.getContext();
			writePathTarget(outPath, { context: ctx2, event: undefined }, data);
			data = readPathTarget(outPath, { context: ctx2, event: undefined }) ?? data;
		}
		const inverse = applyTopologyDiff(topo, diff);
		const archiveContext = this.cloneCtx(this.sm.getContext());
		if (advanceToFinalState) await this.sm.send({ kind: "confirm" }, k, topo, this.actions);
		const res: InteractionResponse = { ok: true, errors: [], warnings: [], infos: [], diff, data, archiveContext };
		this.lastResponse = res;
		this.snapUndoStack.length = 0;
		this.snapRedoStack.length = 0;
		const hist = this.opts.history;
		if (hist && !isEmptyTopologyDiff(diff)) {
			hist.record({
				id: `cmd-${this.spec.id}-${this.revision}`,
				interactionId: this.spec.id,
				label: this.spec.label ?? this.spec.id,
				result: res,
				backwardsDiff: inverse,
			});
		}
		this.emit();
		return res;
	}

	/** @emoji 📜 Executes `commit.operation` against `kernel`, applies `diff` to `document.topology`, records history. */
	async commit(): Promise<InteractionResponse> {
		return this.runCommit(true);
	}
}

/** @emoji 📜 Constructs a `InteractionRuntime` from a compiled `InteractionSpec`. */
export function createInteractionRuntime(spec: InteractionSpec, opts: InteractionRuntimeOptions): InteractionRuntime {
	return new InteractionRuntime(compileInteraction(spec), opts);
}
// #endregion 📜Interaction

// #region 📦Interactions
/** @emoji 🧭 Built-in `InteractionSpec` registry (fixtures + host `register`). */
export class InteractionRegistry {
	private readonly specs = new Map<string, InteractionSpec>();

	register(spec: InteractionSpec): void {
		this.specs.set(spec.id, spec);
	}

	get(id: string): InteractionSpec | null {
		return this.specs.get(id) ?? null;
	}

	list(): readonly InteractionSpec[] {
		return [...this.specs.values()];
	}

	static withBuiltins(): InteractionRegistry {
		const r = new InteractionRegistry();
		const xs = [
			parseInteractionSpec(boxInteractionJson),
			parseInteractionSpec(extrudeWireInteractionJson),
			parseInteractionSpec(offsetSurfaceInteractionJson),
			parseInteractionSpec(distanceInteractionJson),
			parseInteractionSpec(areaInteractionJson),
		];
		for (const s of xs) {
			if (s) r.register(s);
		}
		return r;
	}
}

/** @emoji 📦 Parses canonical box fixture (`spatial/fixtures/box.interaction.json`). */
export function buildBoxInteractionSpec(): InteractionSpec {
	const s = parseInteractionSpec(boxInteractionJson);
	if (!s) throw new Error("spatial/fixtures/box.interaction.json invalid");
	return s;
}

/** @emoji 📦 Parses extrude-wire fixture (`spatial/fixtures/extrude-wire.interaction.json`). */
export function buildExtrudeInteractionSpec(): InteractionSpec {
	const s = parseInteractionSpec(extrudeWireInteractionJson);
	if (!s) throw new Error("spatial/fixtures/extrude-wire.interaction.json invalid");
	return s;
}

/** @emoji 📦 Parses offset-surface fixture (`spatial/fixtures/offset-surface.interaction.json`). */
export function buildOffsetSurfaceInteractionSpec(): InteractionSpec {
	const s = parseInteractionSpec(offsetSurfaceInteractionJson);
	if (!s) throw new Error("spatial/fixtures/offset-surface.interaction.json invalid");
	return s;
}

/** @emoji 📦 Parses distance fixture (`spatial/fixtures/distance.interaction.json`). */
export function buildDistanceInteractionSpec(): InteractionSpec {
	const s = parseInteractionSpec(distanceInteractionJson);
	if (!s) throw new Error("spatial/fixtures/distance.interaction.json invalid");
	return s;
}

/** @emoji 📦 Parses area fixture (`spatial/fixtures/area.interaction.json`). */
export function buildAreaInteractionSpec(): InteractionSpec {
	const s = parseInteractionSpec(areaInteractionJson);
	if (!s) throw new Error("spatial/fixtures/area.interaction.json invalid");
	return s;
}

/** @emoji 📚 Host-facing interaction preset row (`spatial/fixtures/*.interaction.json`). */
export interface SpatialInteractionPreset {
	readonly id: string;
	readonly label: string;
	/** @emoji ⌨️ Single-stroke host interaction key; must stay unique among presets (see `resolveSpatialInteractionPresetKey`). */
	readonly key: string;
}

/** @emoji 📚 Built-in interaction preset ids for host interaction surfaces (`spatial/fixtures/*.interaction.json`). */
export function listSpatialInteractionPresets(): readonly SpatialInteractionPreset[] {
	return [
		{ id: "primitive.box", label: "Box", key: "q" },
		{ id: "feature.extrudeWire", label: "Extrude wire", key: "j" },
		{ id: "feature.offsetSurface", label: "Offset surface", key: "k" },
		{ id: "measure.distance", label: "Distance", key: "d" },
		{ id: "measure.area", label: "Area", key: "a" },
	];
}

/** @emoji 🧭 Resolves a typed token to a preset (`key`, `id`, or compact `label`). */
export function resolveSpatialInteractionPresetKey(token: string): SpatialInteractionPreset | null {
	const t = token.trim().toLowerCase();
	if (!t) return null;
	for (const p of listSpatialInteractionPresets()) {
		if (p.key.toLowerCase() === t) return p;
		if (p.id.toLowerCase() === t) return p;
		const slug = p.label.toLowerCase().replace(/\s+/g, "");
		if (slug === t) return p;
	}
	return null;
}

/** @emoji 📚 Loads a built-in interaction preset by stable `id` (see `listSpatialInteractionPresets`). */
export function loadSpatialInteractionPreset(presetId: string): InteractionSpec | null {
	const raw =
		presetId === "primitive.box"
			? boxInteractionJson
			: presetId === "feature.extrudeWire"
				? extrudeWireInteractionJson
				: presetId === "feature.offsetSurface"
					? offsetSurfaceInteractionJson
					: presetId === "measure.distance"
						? distanceInteractionJson
						: presetId === "measure.area"
							? areaInteractionJson
							: null;
	if (!raw) return null;
	return parseInteractionSpec(raw as unknown);
}
// #endregion 📦Interactions

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@spatial/js-core vec", () => {
		it("adds and distances", () => {
			expect(vec3Distance([0, 0, 0], [3, 4, 0])).toBe(5);
		});
	});

	describe("@spatial/js-core expr", () => {
		it("evaluates numeric fold min expr", () => {
			const e: Expr = {
				kind: "fold",
				op: "min",
				args: [
					{ kind: "const", value: 3 },
					{ kind: "const", value: 7 },
				],
			};
			expect(evalExpr(e, { context: {} })).toBe(3);
		});
		it("evaluates guards used by box factory", () => {
			const g: Expr = {
				kind: "all",
				args: [
					{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "origin" }] } },
					{
						kind: "binop",
						op: ">",
						left: { kind: "path", root: "context", segments: [{ kind: "field", name: "height" }] },
						right: { kind: "const", value: 0 },
					},
				],
			};
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

	describe("@spatial/js-core metadata", () => {
		it("EntityMetadataStore setField bumps topology revision", () => {
			const g = new TopologyGraph();
			const r0 = g.revision;
			g.metadata.setField("e1", "exposure", "external");
			expect(g.revision).toBeGreaterThan(r0);
			expect(g.metadata.get("e1")?.exposure).toBe("external");
		});
	});

	describe("@spatial/js-core interaction presets", () => {
		it("lists stable keys for each built-in interaction preset", () => {
			const ps = listSpatialInteractionPresets();
			expect(ps.map((p) => p.key).join("")).toBe("qjkda");
			expect(new Set(ps.map((p) => p.key)).size).toBe(ps.length);
		});
		it("resolves interaction preset tokens by key, id, and label slug", () => {
			expect(resolveSpatialInteractionPresetKey("q")?.id).toBe("primitive.box");
			expect(resolveSpatialInteractionPresetKey("primitive.box")?.key).toBe("q");
			expect(resolveSpatialInteractionPresetKey("extrudewire")?.id).toBe("feature.extrudeWire");
			expect(resolveSpatialInteractionPresetKey("d")?.id).toBe("measure.distance");
		});
	});

	describe("@spatial/js-core action and interaction registries", () => {
		it("ActionRegistry.withBuiltins registers known geometry actions", () => {
			const r = ActionRegistry.withBuiltins();
			const ids = new Set(r.list().map((d) => d.id));
			expect(ids.has("primitive.createBoxFromCorners")).toBe(true);
			expect(ids.has("box.aabbFromDiagonalCorners")).toBe(true);
		});
		it("register replaces a built-in action id", () => {
			const r = ActionRegistry.withBuiltins();
			const before = r.get("measure.faceArea")?.label;
			r.register({
				id: "measure.faceArea",
				label: "override",
				run: () => ({ data: 99 }),
			});
			expect(r.get("measure.faceArea")?.label).toBe("override");
			expect(before).not.toBe("override");
		});
		it("InteractionRegistry.withBuiltins get matches buildBoxInteractionSpec", () => {
			const reg = InteractionRegistry.withBuiltins();
			expect(reg.get("primitive.box")).toEqual(buildBoxInteractionSpec());
		});
		it("createBoxFrom3Points forwards triplet footprint to createBoxFromCorners", async () => {
			class StubKernel implements KernelAdapter {
				readonly id = "stub-3pt";
				readonly operations = [] as const;
				lastInput: { cornerA: Vec3; cornerB: Vec3; height: number } | null = null;
				async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }) {
					this.lastInput = input;
					return { diff: EMPTY_TOPOLOGY_DIFF, cell: cellRef("c") };
				}
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
			const k = new StubKernel();
			const topo = new TopologyGraph();
			const def = ActionRegistry.withBuiltins().get("primitive.createBoxFrom3Points")!;
			const p0: Vec3 = [0, 0, 0];
			const p1: Vec3 = [2, 3, 0];
			const p2: Vec3 = [1, 1, 0];
			await def.run({ p0, p1, p2, __context: {}, __event: { kind: "x" } }, { kernel: k, topology: topo });
			expect(k.lastInput).toEqual({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 3 });
		});
	});
	describe("@spatial/js-core topology diff", () => {
		it("applyTopologyDiff then inverse restores counts", () => {
			const g = new TopologyGraph();
			const mesh = {
				positions: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
				indices: new Uint32Array([0, 1, 2]),
			};
			const d = meshFaceTopologyDiff(mesh, "x");
			const inv = applyTopologyDiff(g, d);
			expect(Object.keys(g.faces).length).toBe(1);
			applyTopologyDiff(g, inv);
			expect(Object.keys(g.faces).length).toBe(0);
		});
	});
	describe("@spatial/js-core selection filter", () => {
		it("selectionEventMatches rejects kinds outside accept", () => {
			const spec: SelectionSpec = { accept: ["face"], multiple: false };
			const ok: SelectionEvent = {
				kind: "selection.changed",
				targets: [{ kind: "face", id: "f1", editable: true }],
			};
			const bad: SelectionEvent = {
				kind: "selection.changed",
				targets: [{ kind: "surface", id: "s1", editable: false }],
			};
			expect(selectionEventMatches(spec, ok)).toBe(true);
			expect(selectionEventMatches(spec, bad)).toBe(false);
		});
	});
	describe("@spatial/js-core interaction box", () => {
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
			const spec = buildBoxInteractionSpec();
			const rt = createInteractionRuntime(spec, {
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

		it("pushes interaction-local undo snapshot on each non-transient transition", async () => {
			class StubKernel implements KernelAdapter {
				readonly id = "stub-undo";
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
			const spec = buildBoxInteractionSpec();
			const rt = createInteractionRuntime(spec, {
				kernel: new StubKernel(),
				document: { topology: new TopologyGraph(), nodes: [] },
			});
			expect(rt.getSnapshot().capabilities.canUndo).toBe(false);
			await rt.send({ kind: "start" });
			expect(rt.getSnapshot().capabilities.canUndo).toBe(true);
			const afterStart = rt.getSnapshot().state;
			await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
			expect(rt.getSnapshot().capabilities.canUndo).toBe(true);
			await rt.undo();
			expect(rt.getSnapshot().state).toBe(afterStart);
			await rt.undo();
			expect(rt.getSnapshot().state).toBe(spec.machine.initial);
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
			const spec = buildBoxInteractionSpec();
			const topo = new TopologyGraph();
			const kernel = new RecordingStubKernel();
			const rt = createInteractionRuntime(spec, { kernel, document: { topology: topo, nodes: [] } });
			await rt.send({ kind: "start" });
			await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
			await rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
			await rt.send({ kind: "set.height", value: 4, modifiers: {} });
			const res = await rt.commit();
			expect(res.ok).toBe(true);
			expect(res.data).toBeNull();
			expect(res.archiveContext).not.toBeNull();
			expect(res.archiveContext!.origin).toEqual([0, 0, 0]);
			expect(res.archiveContext!.corner).toEqual([2, 3, 0]);
			expect(res.archiveContext!.height).toBe(4);
			expect(Object.keys(topo.faces).length).toBeGreaterThan(0);
			expect(kernel.lastBox).toEqual({
				cornerA: [0, 0, 0],
				cornerB: [2, 3, 0],
				height: 4,
			});
		});
	});

	describe("@spatial/js-core stateEngine option", () => {
		it("explicit pure-ts provider matches default interaction snapshots", async () => {
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
			const spec = buildBoxInteractionSpec();
			const rt0 = createInteractionRuntime(spec, { kernel: new StubKernel(), document: { topology: new TopologyGraph(), nodes: [] } });
			const rt1 = createInteractionRuntime(spec, {
				kernel: new StubKernel(),
				document: { topology: new TopologyGraph(), nodes: [] },
				stateEngine: pureTsStateEngineProvider,
			});
			await rt0.send({ kind: "start" });
			await rt1.send({ kind: "start" });
			expect(rt1.getSnapshot().state).toBe(rt0.getSnapshot().state);
			expect(rt1.getSnapshot().context).toEqual(rt0.getSnapshot().context);
			expect(rt1.getSnapshot().capabilities).toEqual(rt0.getSnapshot().capabilities);
		});
	});

	describe("@spatial/js-core measure distance", () => {
		it("commit returns vertex distance in data", async () => {
			class MeasKernel implements KernelAdapter {
				readonly id = "meas";
				readonly operations = [] as const;
				async createBoxFromCorners() {
					return cellRef("c");
				}
				async volume() {
					return 0;
				}
				async tessellate() {
					return { positions: new Float32Array(), indices: new Uint32Array() };
				}
				async query(name: string, params: Record<string, unknown>) {
					if (name === "surface.resolveFaces") return [String(params.surfaceId ?? "")];
					return undefined;
				}
				async vertexDistance(a: VertexRef, b: VertexRef, t: TopologyGraph) {
					const pa = t.vertices[String(a)]?.position;
					const pb = t.vertices[String(b)]?.position;
					if (!pa || !pb) return 0;
					return vec3Distance(pa, pb);
				}
			}
			const topo = new TopologyGraph();
			const va = "v0" as VertexRef;
			const vb = "v1" as VertexRef;
			topo.vertices[va] = { id: va, position: [0, 0, 0] };
			topo.vertices[vb] = { id: vb, position: [3, 4, 0] };
			const spec = buildDistanceInteractionSpec();
			const rt = createInteractionRuntime(spec, { kernel: new MeasKernel(), document: { topology: topo, nodes: [] } });
			await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: va, editable: true }] });
			await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: vb, editable: true }] });
			const res = await rt.commit();
			expect(res.ok).toBe(true);
			expect(res.data).toBe(5);
			expect(isEmptyTopologyDiff(res.diff)).toBe(true);
		});

		it("auto-commits when confirm reaches the final state", async () => {
			class MeasKernel implements KernelAdapter {
				readonly id = "meas-auto";
				readonly operations = [] as const;
				async createBoxFromCorners() {
					return cellRef("c");
				}
				async volume() {
					return 0;
				}
				async tessellate() {
					return { positions: new Float32Array(), indices: new Uint32Array() };
				}
				async vertexDistance(a: VertexRef, b: VertexRef, t: TopologyGraph) {
					const pa = t.vertices[String(a)]?.position;
					const pb = t.vertices[String(b)]?.position;
					if (!pa || !pb) return 0;
					return vec3Distance(pa, pb);
				}
			}
			const topo = new TopologyGraph();
			const va = "v0" as VertexRef;
			const vb = "v1" as VertexRef;
			topo.vertices[va] = { id: va, position: [0, 0, 0] };
			topo.vertices[vb] = { id: vb, position: [3, 4, 0] };
			const rt = createInteractionRuntime(buildDistanceInteractionSpec(), { kernel: new MeasKernel(), document: { topology: topo, nodes: [] } });
			await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: va, editable: true }] });
			await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: vb, editable: true }] });
			await rt.send({ kind: "confirm" });
			const snap = rt.getSnapshot();
			expect(snap.state).toBe("committed");
			expect(snap.capabilities.canCommit).toBe(false);
			expect(snap.capabilities.canCancel).toBe(false);
			expect(snap.lastResponse?.ok).toBe(true);
			expect(snap.lastResponse?.data).toBe(5);
		});
	});

	describe("@spatial/js-core measure area", () => {
		it("commit returns face area in data", async () => {
			class AreaKernel implements KernelAdapter {
				readonly id = "area";
				readonly operations = [] as const;
				async createBoxFromCorners() {
					return cellRef("c");
				}
				async volume() {
					return 0;
				}
				async tessellate() {
					return { positions: new Float32Array(), indices: new Uint32Array() };
				}
				async query(name: string) {
					if (name === "surface.resolveFaces") return ["f0"];
					return undefined;
				}
				async faceArea(_f: FaceRef, _t: TopologyGraph) {
					return 2.5;
				}
			}
			const topo = new TopologyGraph();
			const spec = buildAreaInteractionSpec();
			const rt = createInteractionRuntime(spec, { kernel: new AreaKernel(), document: { topology: topo, nodes: [] } });
			await rt.send({ kind: "selection.changed", targets: [{ kind: "face", id: "f0", editable: true }] });
			const res = await rt.commit();
			expect(res.ok).toBe(true);
			expect(res.data).toBe(2.5);
			expect(isEmptyTopologyDiff(res.diff)).toBe(true);
		});
	});

	describe("@spatial/js-core document history", () => {
		it("records modifications and undo/redo applies forward and backwards diffs", () => {
			const g = new TopologyGraph();
			const h = new DocumentHistory();
			const mesh = { positions: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]), indices: new Uint32Array([0, 1, 2]) };
			const d1 = meshFaceTopologyDiff(mesh, "a");
			const inv1 = applyTopologyDiff(g, d1);
			const res1: InteractionResponse = { ok: true, errors: [], warnings: [], infos: [], diff: d1, data: null, archiveContext: null };
			h.record({ id: "m1", interactionId: "c", label: "A", result: res1, backwardsDiff: inv1 });
			const d2 = meshFaceTopologyDiff(mesh, "b");
			const inv2 = applyTopologyDiff(g, d2);
			const res2: InteractionResponse = { ok: true, errors: [], warnings: [], infos: [], diff: d2, data: null, archiveContext: null };
			h.record({ id: "m2", interactionId: "c", label: "B", result: res2, backwardsDiff: inv2 });
			expect(Object.keys(g.faces).length).toBe(2);
			expect(h.entries().map((m) => m.id)).toEqual(["m1", "m2"]);
			const doc = { topology: g, nodes: [] as ShapeNode[] };
			h.undo(doc);
			expect(Object.keys(g.faces).length).toBe(1);
			expect(h.entries().map((m) => m.id)).toEqual(["m1"]);
			h.undo(doc);
			expect(Object.keys(g.faces).length).toBe(0);
			h.redo(doc);
			expect(Object.keys(g.faces).length).toBe(1);
			h.redo(doc);
			expect(Object.keys(g.faces).length).toBe(2);
			h.clear();
			expect(h.entries()).toEqual([]);
			expect(h.peekUndo()).toBe(null);
		});
	});

	describe("@spatial/js-core measure distance history", () => {
		it("does not push readonly measure commits onto document history", async () => {
			class MeasKernel implements KernelAdapter {
				readonly id = "meas-h";
				readonly operations = [] as const;
				async createBoxFromCorners() {
					return cellRef("c");
				}
				async volume() {
					return 0;
				}
				async tessellate() {
					return { positions: new Float32Array(), indices: new Uint32Array() };
				}
				async query(name: string, params: Record<string, unknown>) {
					if (name === "surface.resolveFaces") return [String(params.surfaceId ?? "")];
					return undefined;
				}
				async vertexDistance(a: VertexRef, b: VertexRef, t: TopologyGraph) {
					const pa = t.vertices[String(a)]?.position;
					const pb = t.vertices[String(b)]?.position;
					if (!pa || !pb) return 0;
					return vec3Distance(pa, pb);
				}
			}
			const hist = new DocumentHistory();
			const topo = new TopologyGraph();
			const va = "v0" as VertexRef;
			const vb = "v1" as VertexRef;
			topo.vertices[va] = { id: va, position: [0, 0, 0] };
			topo.vertices[vb] = { id: vb, position: [3, 4, 0] };
			const spec = buildDistanceInteractionSpec();
			const rt = createInteractionRuntime(spec, { kernel: new MeasKernel(), document: { topology: topo, nodes: [] }, history: hist });
			await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: va, editable: true }] });
			await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: vb, editable: true }] });
			await rt.commit();
			expect(hist.peekUndo()).toBe(null);
		});
	});

	describe("@spatial/js-core interaction session undo redo", () => {
		it("supports redo after undo during an active interaction and clears redo on new branch", async () => {
			class StubKernel implements KernelAdapter {
				readonly id = "stub-s";
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
			const spec = buildBoxInteractionSpec();
			const rt = createInteractionRuntime(spec, { kernel: new StubKernel(), document: { topology: new TopologyGraph(), nodes: [] } });
			await rt.send({ kind: "start" });
			expect(rt.getSnapshot().state).toBe("first_corner");
			expect(rt.getSnapshot().capabilities.canRedo).toBe(false);
			rt.undo();
			expect(rt.getSnapshot().state).toBe("idle");
			expect(rt.getSnapshot().capabilities.canRedo).toBe(true);
			await rt.send({ kind: "start" });
			expect(rt.getSnapshot().capabilities.canRedo).toBe(false);
		});
	});

	describe("@spatial/js-core undo routing", () => {
		it("uses snapshot undo while active and document history when idle", async () => {
			class StubKernel implements KernelAdapter {
				readonly id = "stub-r";
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
			const g = new TopologyGraph();
			const mesh = { positions: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]), indices: new Uint32Array([0, 1, 2]) };
			const d0 = meshFaceTopologyDiff(mesh, "seed");
			const inv0 = applyTopologyDiff(g, d0);
			const hist = new DocumentHistory();
			hist.record({
				id: "seed",
				interactionId: "x",
				label: "seed",
				result: { ok: true, errors: [], warnings: [], infos: [], diff: d0, data: null, archiveContext: null },
				backwardsDiff: inv0,
			});
			expect(Object.keys(g.faces).length).toBe(1);
			const spec = buildBoxInteractionSpec();
			const rt = createInteractionRuntime(spec, { kernel: new StubKernel(), document: { topology: g, nodes: [] }, history: hist });
			await rt.send({ kind: "start" });
			rt.undo();
			expect(rt.getSnapshot().state).toBe("idle");
			expect(Object.keys(g.faces).length).toBe(1);
			rt.undo();
			expect(Object.keys(g.faces).length).toBe(0);
		});
	});

	describe("@spatial/js-core box display committed", () => {
		it("keeps box-preview visible for committed state", () => {
			const spec = buildBoxInteractionSpec();
			const ctx: Record<string, unknown> = {
				origin: [0, 0, 0] as Vec3,
				corner: [2, 3, 0] as Vec3,
				height: 4,
			};
			const d = resolveDisplay(spec, "committed", ctx);
			const prev = d.items.find((i) => i.kind === "box-preview" && i.id === "preview-committed");
			expect(prev?.params?.cornerA).toEqual([0, 0, 0]);
			expect(prev?.params?.cornerB).toEqual([2, 3, 0]);
			expect(prev?.params?.height).toBe(4);
		});
	});
}
// #endregion 🧪Tests

