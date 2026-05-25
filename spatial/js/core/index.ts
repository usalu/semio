// #region 🧲Header
/** @emoji 🧭 `@spatial/js-core` — portable interaction spec runtime, `ActionRegistry`, `StateEngine` + `SpatialKernel`, topology graph, derived views. See `spatial/schema/json` and `.repo/✍️/spatial.md`. */
// #endregion 🧲Header

// #region 📥InteractionAssets
import boxInteractionJson from "../../assets/interactions/box.interaction.json" with { type: "json" };
import extrudeWireInteractionJson from "../../assets/interactions/extrude-wire.interaction.json" with { type: "json" };
import offsetSurfaceInteractionJson from "../../assets/interactions/offset-surface.interaction.json" with { type: "json" };
import distanceInteractionJson from "../../assets/interactions/measure-length.interaction.json" with { type: "json" };
import areaInteractionJson from "../../assets/interactions/area.interaction.json" with { type: "json" };
import curveArcInteractionJson from "../../assets/interactions/curve-arc.interaction.json" with { type: "json" };
import curveCircleInteractionJson from "../../assets/interactions/curve-circle.interaction.json" with { type: "json" };
import curveControlPointCurveInteractionJson from "../../assets/interactions/curve-control-point-curve.interaction.json" with { type: "json" };
import curveInterpolateCurveInteractionJson from "../../assets/interactions/curve-interpolate-curve.interaction.json" with { type: "json" };
import curveLineInteractionJson from "../../assets/interactions/curve-line.interaction.json" with { type: "json" };
import curvePolylineInteractionJson from "../../assets/interactions/curve-polyline.interaction.json" with { type: "json" };
import editChamferInteractionJson from "../../assets/interactions/edit-chamfer.interaction.json" with { type: "json" };
import editExplodeInteractionJson from "../../assets/interactions/edit-explode.interaction.json" with { type: "json" };
import editFilletInteractionJson from "../../assets/interactions/edit-fillet.interaction.json" with { type: "json" };
import editJoinInteractionJson from "../../assets/interactions/edit-join.interaction.json" with { type: "json" };
import editSplitInteractionJson from "../../assets/interactions/edit-split.interaction.json" with { type: "json" };
import editTrimInteractionJson from "../../assets/interactions/edit-trim.interaction.json" with { type: "json" };
import solidBooleanDifferenceInteractionJson from "../../assets/interactions/solid-boolean-difference.interaction.json" with { type: "json" };
import solidBooleanIntersectionInteractionJson from "../../assets/interactions/solid-boolean-intersection.interaction.json" with { type: "json" };
import solidBooleanUnionInteractionJson from "../../assets/interactions/solid-boolean-union.interaction.json" with { type: "json" };
import solidCylinderInteractionJson from "../../assets/interactions/solid-cylinder.interaction.json" with { type: "json" };
import solidSphereInteractionJson from "../../assets/interactions/solid-sphere.interaction.json" with { type: "json" };
import surfaceExtrudeCrvInteractionJson from "../../assets/interactions/surface-extrude-crv.interaction.json" with { type: "json" };
import surfaceLoftInteractionJson from "../../assets/interactions/surface-loft.interaction.json" with { type: "json" };
import surfaceNetworkSrfInteractionJson from "../../assets/interactions/surface-network-srf.interaction.json" with { type: "json" };
import surfacePlaneInteractionJson from "../../assets/interactions/surface-plane.interaction.json" with { type: "json" };
import surfaceSweep1InteractionJson from "../../assets/interactions/surface-sweep1.interaction.json" with { type: "json" };
import surfaceSweep2InteractionJson from "../../assets/interactions/surface-sweep2.interaction.json" with { type: "json" };
import transformCopyInteractionJson from "../../assets/interactions/transform-copy.interaction.json" with { type: "json" };
import transformMirrorInteractionJson from "../../assets/interactions/transform-mirror.interaction.json" with { type: "json" };
import transformMoveInteractionJson from "../../assets/interactions/transform-move.interaction.json" with { type: "json" };
import transformRotateInteractionJson from "../../assets/interactions/transform-rotate.interaction.json" with { type: "json" };
import transformScale1dInteractionJson from "../../assets/interactions/transform-scale1d.interaction.json" with { type: "json" };
import transformScale3dInteractionJson from "../../assets/interactions/transform-scale3d.interaction.json" with { type: "json" };
// #endregion 📥InteractionAssets

// #region 🧮Vec
/** @emoji 📐 Column vector `[x,y,z]` used by spatial factories. */
export type Vec3 = readonly [number, number, number];
// #endregion 🧮Vec

// #region 🌀EdgeGeometry
/** @emoji 🌀 OCCT-style edge curve kinds (`Geom_Curve` under a topologic `Edge`). */
export type EdgeCurve =
	| { readonly kind: "line" }
	| { readonly kind: "arc"; readonly center: Vec3 }
	| { readonly kind: "circle"; readonly center: Vec3; readonly normal: Vec3; readonly radius: number }
	| {
			readonly kind: "ellipse";
			readonly center: Vec3;
			readonly normal: Vec3;
			readonly majorAxis: Vec3;
			readonly majorRadius: number;
			readonly minorRadius: number;
	  }
	| {
			readonly kind: "nurbs";
			readonly poles: readonly Vec3[];
			readonly degree: number;
			readonly weights?: readonly number[];
			readonly knots?: readonly number[];
			readonly multiplicities?: readonly number[];
			readonly periodic?: boolean;
			readonly rational?: boolean;
	  };

/** @emoji 🔵 Plane frame for a circular arc through `start` and `end` about `center` (CCW in `u×v`). */
export interface ArcPlaneFrame {
	readonly center: Vec3;
	readonly radius: number;
	readonly normal: Vec3;
	readonly u: Vec3;
	readonly v: Vec3;
}
// #endregion 🌀EdgeGeometry

// #region 🪪Refs
/** @emoji 🪪 Opaque branded string ids for editable topology entities. */
export type AnchorRef = string & { readonly __brand: "AnchorRef" };
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
	| "anchor"
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
	"anchor",
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
	readonly point?: Vec3;
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
	readonly preview: SpatialPreviewKernel;
}

function envWithVars(base: ExprEnv, vars: Record<string, unknown>): ExprEnv {
	return {
		context: base.context,
		event: base.event,
		vars: { ...base.vars, ...vars },
		topology: base.topology,
		metadata: base.metadata,
		derived: base.derived,
		preview: base.preview,
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
			return readTopologyEntityProperty(topo, env.metadata, o.kind, o.id, expr.name, { derived: env.derived, preview: env.preview });
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
			return typeof v === "number" ? env.preview.abs(v) : undefined;
		}
		case "distance": {
			const va = evalExpr(expr.a, env);
			const vb = evalExpr(expr.b, env);
			if (!isVec3(va) || !isVec3(vb)) return undefined;
			return env.preview.vec3Distance(va, vb);
		}
		case "fold":
			return expr.op === "min"
				? env.preview.min2(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env)))
				: env.preview.max2(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env)));
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
			readonly kind: "preview";
			readonly id: string;
			readonly role?: string;
			readonly previewKind: string;
			readonly params?: Record<string, Expr>;
	  }
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

/** @emoji 🎮 Host + viewport hints for spatial picking (declared per interaction). */
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

const COMPILED_INITIAL_CONTEXTS = new WeakMap<InteractionSpec, Record<string, unknown>>();

function initialStartTransition(spec: InteractionSpec): TransitionSpec | null {
	const initial = findState(spec, spec.machine.initial);
	const handler = initial?.on?.find((h) => h.event === "start");
	const transition = handler?.transitions[0];
	return transition?.target ? transition : null;
}

function staticInitialContext(spec: InteractionSpec, transition: TransitionSpec | null): Record<string, unknown> {
	const context: Record<string, unknown> = {};
	if (!transition?.effects) return context;
	const env: ExprEnv = { context, event: { kind: "start" } };
	for (const effect of transition.effects) {
		if (effect.op === "assign") writePathTarget(effect.target, env, evalExpr(effect.value, env));
		else if (effect.op === "clear") clearPathTarget(effect.target, env);
		else if (effect.op === "append") {
			const cur = readPathTarget(effect.target, env);
			const next = Array.isArray(cur) ? [...cur, evalExpr(effect.value, env)] : [evalExpr(effect.value, env)];
			writePathTarget(effect.target, env, next);
		}
	}
	return context;
}

function compileInitialState(spec: InteractionSpec, transition: TransitionSpec | null): InteractionSpec {
	if (!transition?.target) return spec;
	return {
		...spec,
		machine: {
			...spec.machine,
			initial: transition.target,
		},
	};
}

export function initialContextForSpec(spec: InteractionSpec): Record<string, unknown> {
	return structuredClone(COMPILED_INITIAL_CONTEXTS.get(spec) ?? {});
}

/** @emoji 🧭 Normalizes a parsed interaction so runtime sessions begin in the first active state. */
export function compileInteraction(spec: InteractionSpec): InteractionSpec {
	const start = initialStartTransition(spec);
	if (!start) {
		if (!COMPILED_INITIAL_CONTEXTS.has(spec)) COMPILED_INITIAL_CONTEXTS.set(spec, {});
		return spec;
	}
	const compiled = compileInitialState(spec, start);
	COMPILED_INITIAL_CONTEXTS.set(compiled, staticInitialContext(spec, start));
	return compiled;
}
// #endregion 📜Spec

// #region 🧱Topology
/** @emoji 🧱 Vertex payload: point geometry attached to topology. */
export interface VertexRecord {
	readonly id: VertexRef;
	readonly position: Vec3;
}

export type AnchorAttachment =
	| { readonly kind: "vertex"; readonly id: VertexRef }
	| { readonly kind: "edge"; readonly id: EdgeRef; readonly t: number }
	| { readonly kind: "wire"; readonly id: WireRef; readonly t: number }
	| { readonly kind: "face"; readonly id: FaceRef; readonly u: number; readonly v: number }
	| { readonly kind: "cell"; readonly id: CellRef; readonly u: number; readonly v: number; readonly w: number };

/** @emoji 🧱 Anchor payload: parametric point attached to editable topology. */
export interface AnchorRecord {
	readonly id: AnchorRef;
	readonly position: Vec3;
	readonly attachment: AnchorAttachment;
}

/** @emoji 🧱 Edge payload: two boundary vertices; optional `curve` (`Geom_Curve` analog). */
export interface EdgeRecord {
	readonly id: EdgeRef;
	readonly vertexIds: readonly VertexRef[];
	readonly curve?: EdgeCurve;
}

/** @emoji 🧱 Wire payload: ordered boundary edges. */
export interface WireRecord {
	readonly id: WireRef;
	readonly edgeIds: readonly EdgeRef[];
}

/** @emoji 🌊 Face-support geometry (`Geom_Surface` under a topologic `Face`). */
export type FaceSurface =
	| { readonly kind: "plane"; readonly origin: Vec3; readonly normal: Vec3 }
	| { readonly kind: "cylinder"; readonly origin: Vec3; readonly axis: Vec3; readonly radius: number }
	| { readonly kind: "sphere"; readonly center: Vec3; readonly radius: number }
	| { readonly kind: "cone"; readonly apex: Vec3; readonly axis: Vec3; readonly radius: number; readonly semiAngle: number }
	| {
			readonly kind: "nurbs";
			readonly poles: readonly (readonly Vec3[])[];
			readonly uDegree: number;
			readonly vDegree: number;
			readonly uKnots?: readonly number[];
			readonly vKnots?: readonly number[];
	  };

/** @emoji 🧱 Face payload: trimming wires + optional underlying surface. */
export interface FaceRecord {
	readonly id: FaceRef;
	readonly wireIds: readonly WireRef[];
	readonly surface?: FaceSurface;
}

/** @emoji 🧱 Shell payload: connected faces. */
export interface ShellRecord {
	readonly id: ShellRef;
	readonly faceIds: readonly FaceRef[];
}

/** @emoji 🧊 Analytic cell solid (`BRepPrimAPI` / `Geom` analog under topologic `Cell`). */
export type CellSolid =
	| { readonly kind: "box"; readonly cornerA: Vec3; readonly cornerB: Vec3; readonly height: number }
	| { readonly kind: "sphere"; readonly center: Vec3; readonly radius: number }
	| { readonly kind: "cylinder"; readonly base: Vec3; readonly axis: Vec3; readonly radius: number; readonly height: number }
	| { readonly kind: "cone"; readonly base: Vec3; readonly axis: Vec3; readonly radius: number; readonly height: number; readonly radiusTop?: number };

/** @emoji 🧱 Cell payload: bounded volume via closed shells and/or analytic solid. */
export interface CellRecord {
	readonly id: CellRef;
	readonly shellIds: readonly ShellRef[];
	readonly solid?: CellSolid;
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
	readonly anchors: readonly AnchorRecord[];
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
	anchors: Record<string, AnchorRecord> = {};
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
			anchors: sortedRecordValues(this.anchors),
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
		g.anchors = recordsById(j.anchors);
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
	opts?: { readonly derived?: DerivedViewService; readonly preview?: SpatialPreviewKernel },
): unknown {
	const bag = meta?.get(id);
	if (bag && name in bag) return (bag as Record<string, unknown>)[name];
	switch (kind) {
		case "anchor": {
			const anchor = topo.anchors[id];
			if (!anchor) return undefined;
			if (name === "position") return opts?.preview?.evaluateAnchorPosition(topo, anchor) ?? anchor.position;
			return (anchor as unknown as Record<string, unknown>)[name];
		}
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
			const hit = d.computeSurfaces(topo).find((s) => String(s.id) === id);
			if (!hit) return undefined;
			return (hit as unknown as Record<string, unknown>)[name];
		}
		case "part": {
			if (name === "id") return id;
			const d = opts?.derived;
			if (!d) return undefined;
			const hit = d.computeParts(topo).find((p) => String(p.id) === id);
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
	const need = ["anchors", "vertices", "edges", "wires", "faces", "shells", "cells", "cellComplexes", "clusters"] as const;
	for (const k of need) {
		if (!Array.isArray(r[k])) return null;
	}
	return TopologyGraph.fromJSON(raw as TopologyGraphJson);
}
// #endregion 🧱Topology

// #region 🧮Diff
export type AnchorRecordDiff = { readonly id: AnchorRef } & Partial<Pick<AnchorRecord, "position" | "attachment">>;
export type VertexRecordDiff = { readonly id: VertexRef } & Partial<Pick<VertexRecord, "position">>;
export type EdgeRecordDiff = { readonly id: EdgeRef } & Partial<Pick<EdgeRecord, "vertexIds" | "curve">>;
export type WireRecordDiff = { readonly id: WireRef } & Partial<Pick<WireRecord, "edgeIds">>;
export type FaceRecordDiff = { readonly id: FaceRef } & Partial<Pick<FaceRecord, "wireIds" | "surface">>;
export type ShellRecordDiff = { readonly id: ShellRef } & Partial<Pick<ShellRecord, "faceIds">>;
export type CellRecordDiff = { readonly id: CellRef } & Partial<Pick<CellRecord, "shellIds" | "solid">>;
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
	readonly anchors?: EntityDiff<AnchorRecord, AnchorRecordDiff, AnchorRef>;
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
		isEntityDiffEmpty(d.anchors) &&
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
	const aInv: EntityDiff<AnchorRecord, AnchorRecordDiff, AnchorRef> = {};
	const vInv: EntityDiff<VertexRecord, VertexRecordDiff, VertexRef> = {};
	const eInv: EntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef> = {};
	const wInv: EntityDiff<WireRecord, WireRecordDiff, WireRef> = {};
	const fInv: EntityDiff<FaceRecord, FaceRecordDiff, FaceRef> = {};
	const sInv: EntityDiff<ShellRecord, ShellRecordDiff, ShellRef> = {};
	const cInv: EntityDiff<CellRecord, CellRecordDiff, CellRef> = {};
	const ccInv: EntityDiff<CellComplexRecord, CellComplexRecordDiff, CellComplexRef> = {};
	const clInv: EntityDiff<ClusterRecord, ClusterRecordDiff, ClusterRef> = {};
	applyEntityDiff(topo.anchors as Record<string, AnchorRecord>, diff.anchors, aInv);
	applyEntityDiff(topo.vertices as Record<string, VertexRecord>, diff.vertices, vInv);
	applyEntityDiff(topo.edges as Record<string, EdgeRecord>, diff.edges, eInv);
	applyEntityDiff(topo.wires as Record<string, WireRecord>, diff.wires, wInv);
	applyEntityDiff(topo.faces as Record<string, FaceRecord>, diff.faces, fInv);
	applyEntityDiff(topo.shells as Record<string, ShellRecord>, diff.shells, sInv);
	applyEntityDiff(topo.cells as Record<string, CellRecord>, diff.cells, cInv);
	applyEntityDiff(topo.cellComplexes as Record<string, CellComplexRecord>, diff.cellComplexes, ccInv);
	applyEntityDiff(topo.clusters as Record<string, ClusterRecord>, diff.clusters, clInv);
	if (!isEntityDiffEmpty(aInv)) inv.anchors = aInv;
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


// #region 🔌SpatialKernelInterface
export type Aabb = { readonly min: Vec3; readonly max: Vec3 };

/** @emoji ⚡ Fast approximate preview math (sync); subset of `SpatialKernel`. */
export interface SpatialPreviewKernel {
	vec3Add(a: Vec3, b: Vec3): Vec3;
	vec3Sub(a: Vec3, b: Vec3): Vec3;
	vec3Scale(a: Vec3, s: number): Vec3;
	vec3Dot(a: Vec3, b: Vec3): number;
	vec3Cross(a: Vec3, b: Vec3): Vec3;
	vec3Length(a: Vec3): number;
	vec3Distance(a: Vec3, b: Vec3): number;
	vec3Normalize(a: Vec3): Vec3;
	arcPlaneFrame(center: Vec3, start: Vec3, end: Vec3): ArcPlaneFrame | null;
	arcSweepRadians(frame: ArcPlaneFrame, end: Vec3): number;
	arcSamplePoints(center: Vec3, start: Vec3, end: Vec3, segments?: number): readonly Vec3[];
	arcFrameFromRadiusPoint(center: Vec3, onCircle: Vec3): ArcPlaneFrame | null;
	arcEndOnCircle(center: Vec3, start: Vec3, pick: Vec3): Vec3;
	arcEndFromAngle(center: Vec3, start: Vec3, angleDeg: number): Vec3 | null;
	circleSamplePoints(center: Vec3, normal: Vec3, radius: number, segments?: number): readonly Vec3[];
	ellipseSamplePoints(
		center: Vec3,
		normal: Vec3,
		majorAxis: Vec3,
		majorRadius: number,
		minorRadius: number,
		segments?: number,
	): readonly Vec3[];
	nurbsDisplaySamplePoints(poles: readonly Vec3[], segmentsPerSpan?: number): readonly Vec3[];
	polylineLength(points: readonly Vec3[]): number;
	edgeCurveLength(curve: EdgeCurve | undefined, ends: readonly Vec3[]): number;
	edgeSamplePoints(vertices: Readonly<Record<string, VertexRecord>>, edge: EdgeRecord, segments?: number): readonly Vec3[];
	circleFromCenterRadiusPoint(
		center: Vec3,
		radiusPoint: Vec3,
	): { readonly center: Vec3; readonly normal: Vec3; readonly radius: number } | null;
	nurbsCurveFromPoles(poles: readonly Vec3[]): EdgeCurve | null;
	aabbFromPoints(points: readonly Vec3[]): Aabb | null;
	aabbCornerPoints(min: Vec3, max: Vec3): readonly Vec3[];
	aabbIntersect(a: Aabb, b: Aabb): Aabb | null;
	cellSolidAabb(solid: CellSolid): Aabb;
	topologyCellAabb(topo: TopologyGraph, cell: CellRecord): Aabb | null;
	boxTopologyDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }, cell: CellRef): TopologyDiff;
	meshFaceTopologyDiff(mesh: MeshPreview, idTag: string): TopologyDiff;
	evaluateAnchorPosition(topo: TopologyGraph, anchor: AnchorRecord): Vec3;
	computeBoxPreviewLayout(cornerA: Vec3, cornerB: Vec3, height: number): { readonly position: Vec3; readonly scale: Vec3 };
	transformPointsForPreviewKind(previewKind: string, params: Record<string, unknown>): (point: Vec3) => Vec3;
	abs(x: number): number;
	min2(a: number, b: number): number;
	max2(a: number, b: number): number;
	minN(nums: readonly number[]): number;
	maxN(nums: readonly number[]): number;
}

/** @emoji 🔌 Precise BREP kernel: preview math + construction, tessellation, derived views. */
export interface SpatialKernel extends SpatialPreviewKernel {
	readonly id: string;
	readonly operations: readonly string[];
	createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<CellRef>;
	volume(cell: CellRef): Promise<number>;
	tessellate(cell: CellRef, tolerance: number): Promise<MeshPreview>;
	query?(name: string, params: Record<string, unknown>, ctx?: KernelQueryContext): Promise<unknown>;
	computeSurfaceViews(topo: TopologyGraph): SurfaceView[] | Promise<SurfaceView[]>;
	computePartViews(topo: TopologyGraph): PartView[] | Promise<PartView[]>;
	executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: TopologyDiff }>;
	extrudeWire(input: { wireId: string; distance: number; direction: Vec3 }): Promise<CellRef>;
	offsetFaces(input: { faceIds: readonly string[]; distance: number }): Promise<void>;
	createBoxFromCornersDiff(input: {
		cornerA: Vec3;
		cornerB: Vec3;
		height: number;
	}): Promise<{ readonly diff: TopologyDiff; readonly cell: CellRef }>;
	extrudeWireDiff(input: {
		wireId: string;
		distance: number;
		direction: Vec3;
	}): Promise<{ readonly diff: TopologyDiff; readonly cell: CellRef }>;
	offsetFacesDiff(input: { faceIds: readonly string[]; distance: number }): Promise<{ readonly diff: TopologyDiff }>;
	vertexDistance(a: VertexRef, b: VertexRef, topo: TopologyGraph): Promise<number>;
	edgeLength(e: EdgeRef, topo: TopologyGraph): Promise<number>;
	faceArea(f: FaceRef, topo: TopologyGraph): Promise<number>;
	cellVolume(c: CellRef): Promise<number>;
	adjacentCells(cell: CellRef, topo: TopologyGraph): Promise<readonly CellRef[]>;
	sharedFacesBetween(a: CellRef, b: CellRef, topo: TopologyGraph): Promise<readonly FaceRef[]>;
}

/** @emoji 🖼️ Renderer-neutral mesh preview (positions + triangle indices). */
export interface MeshPreview {
	readonly positions: Float32Array;
	readonly indices: Uint32Array;
	readonly normals?: Float32Array;
}

/** @emoji 🧱 Appends a tessellated commit as one mesh `face` on `TopologyGraph` (in-memory scene growth). */
export function appendCommittedMeshFaceToTopology(
	topo: TopologyGraph,
	mesh: MeshPreview,
	idTag: string,
	math: SpatialPreviewKernel,
): void {
	applyTopologyDiff(topo, math.meshFaceTopologyDiff(mesh, idTag));
}

/** @emoji 🔌 Optional query context for derived-view resolution in kernel adapters. */
export interface KernelQueryContext {
	readonly topology: TopologyGraph;
	readonly derived?: DerivedViewService;
}
// #endregion 🔌SpatialKernelInterface

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
	ctx: { readonly kernel: SpatialKernel; readonly preview: SpatialPreviewKernel; readonly topology: TopologyGraph },
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

/** @emoji 🎯 Collects vertex ids reachable from transform/edit selection targets. */
export function collectTargetVertices(topo: TopologyGraph, targets: readonly SelectionTarget[]): Set<string> {
	const out = new Set<string>();
	const walk = (kind: TopologyEntityKind, id: string) => {
		if (kind === "anchor") {
			const anchor = topo.anchors[id];
			if (anchor?.attachment.kind === "vertex") out.add(anchor.attachment.id);
		} else if (kind === "vertex") {
			if (topo.vertices[id]) out.add(id);
		} else if (kind === "edge") {
			const e = topo.edges[id];
			if (e) for (const v of e.vertexIds) walk("vertex", v);
		} else if (kind === "wire") {
			const w = topo.wires[id];
			if (w) for (const e of w.edgeIds) walk("edge", e);
		} else if (kind === "face") {
			const f = topo.faces[id];
			if (f) for (const w of f.wireIds) walk("wire", w);
		} else if (kind === "shell") {
			const s = topo.shells[id];
			if (s) for (const f of s.faceIds) walk("face", f);
		} else if (kind === "cell") {
			const c = topo.cells[id];
			if (c) for (const s of c.shellIds) walk("shell", s);
		} else if (kind === "cellComplex") {
			const cc = topo.cellComplexes[id];
			if (cc) for (const c of cc.cellIds) walk("cell", c);
		}
	};
	for (const t of targets) walk(t.kind, t.id);
	return out;
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
			let cell: CellRef;
			if (kernel.createBoxFromCornersDiff) {
				const r = await kernel.createBoxFromCornersDiff({ cornerA, cornerB, height });
				cell = r.cell;
			} else {
				cell = await kernel.createBoxFromCorners({ cornerA, cornerB, height });
			}
			return { diff: boxTopologyDiff({ cornerA, cornerB, height }, cell) };
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
	const commandAddPoint: ActionDef = {
		id: "command.addPoint",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const field = typeof params.field === "string" ? params.field : "points";
			const key = typeof params.key === "string" ? params.key : null;
			const point = isVec3(params.point) ? params.point : null;
			if (!point) return {};
			const cur = Array.isArray(ctx[field]) ? (ctx[field] as unknown[]) : [];
			const set: Record<string, unknown> = { [field]: [...cur, point], prevPoint: point, cursor: point };
			if (key) set[key] = point;
			return { patch: { set } };
		},
	};
	const commandAddSelection: ActionDef = {
		id: "command.addSelection",
		run: (params) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const field = typeof params.field === "string" ? params.field : "targets";
			const key = typeof params.key === "string" ? params.key : null;
			const targets = Array.isArray(params.targets) ? params.targets : [];
			const cur = Array.isArray(ctx[field]) ? (ctx[field] as unknown[]) : [];
			const set: Record<string, unknown> = { [field]: [...cur, ...targets] };
			const first = targets[0];
			if (key && first && typeof first === "object" && "id" in first) set[key] = String((first as { id: unknown }).id);
			return { patch: { set } };
		},
	};
	const commandFinish: ActionDef = {
		id: "command.finish",
		run: async (params, { kernel }) => {
			const ctx = ctxOf(params as Record<string, unknown>);
			const commandId = String(params.commandId ?? "");
			let diff = EMPTY_TOPOLOGY_DIFF;
			
			if (kernel.executeCommandDiff) {
				const res = await kernel.executeCommandDiff(commandId, ctx);
				if (res && res.diff) diff = res.diff;
			}
			
			return {
				diff,
				data: {
					commandId,
					resultKind: String(params.resultKind ?? "command"),
					context: structuredClone(ctx),
				},
			};
		},
	};

	const entityCreateAnchor: ActionDef = {
		id: "entity.createAnchor",
		run: (params, { topology }) => {
			const hostKind = params.hostKind;
			const hostId = typeof params.hostId === "string" ? params.hostId : "";
			const hitPoint = isVec3(params.hitPoint) ? params.hitPoint : null;
			if (!hitPoint || !hostId) return {};
			if (hostKind !== "vertex" && hostKind !== "edge" && hostKind !== "wire" && hostKind !== "face" && hostKind !== "cell") return {};
			const placement = anchorPlacementFromEntity(topology, hostKind, hostId, hitPoint);
			if (!placement) return {};
			const anchorId = `anchor-${Math.random().toString(36).slice(2, 9)}` as AnchorRef;
			return {
				diff: {
					anchors: {
						added: [{ id: anchorId, position: placement.position, attachment: placement.attachment }],
					},
				},
				data: { anchorId, attachment: placement.attachment, position: placement.position },
			};
		},
	};

	const featureTransformMove: ActionDef = {
		id: "transform.move",
		run: (params, { topology }) => {
			const from = isVec3(params.from) ? params.from : null;
			const to = isVec3(params.to) ? params.to : null;
			const targets = Array.isArray(params.targets) ? (params.targets as SelectionTarget[]) : [];
			if (!from || !to || targets.length === 0) return {};

			const delta = vec3Sub(to, from);
			const vIds = collectTargetVertices(topology, targets);
			const modifiedVertices: VertexRecordDiff[] = [];
			for (const vid of vIds) {
				const v = topology.vertices[vid];
				if (v) {
					modifiedVertices.push({
						id: vid as VertexRef,
						position: vec3Add(v.position, delta),
					});
				}
			}

			return {
				diff: modifiedVertices.length > 0 ? { vertices: { modified: modifiedVertices } } : EMPTY_TOPOLOGY_DIFF,
			};
		},
	};

	const featureTransformRotate: ActionDef = {
		id: "transform.rotate",
		run: (params, { topology }) => {
			const center = isVec3(params.center) ? params.center : null;
			let angle = typeof params.angle === "number" ? params.angle : null;
			if (angle === null) {
				const refA = isVec3(params.referenceA) ? params.referenceA : null;
				const refB = isVec3(params.referenceB) ? params.referenceB : null;
				if (center && refA && refB) {
					const angleA = Math.atan2(refA[1] - center[1], refA[0] - center[0]);
					const angleB = Math.atan2(refB[1] - center[1], refB[0] - center[0]);
					angle = angleB - angleA;
				}
			}
			const targets = Array.isArray(params.targets) ? (params.targets as SelectionTarget[]) : [];
			if (!center || typeof angle !== "number" || targets.length === 0) return {};

			const vIds = collectTargetVertices(topology, targets);
			const modifiedVertices: VertexRecordDiff[] = [];
			const cosA = Math.cos(angle);
			const sinA = Math.sin(angle);
			for (const vid of vIds) {
				const v = topology.vertices[vid];
				if (v) {
					const dx = v.position[0] - center[0];
					const dy = v.position[1] - center[1];
					modifiedVertices.push({
						id: vid as VertexRef,
						position: [
							center[0] + dx * cosA - dy * sinA,
							center[1] + dx * sinA + dy * cosA,
							v.position[2],
						],
					});
				}
			}

			return {
				diff: modifiedVertices.length > 0 ? { vertices: { modified: modifiedVertices } } : EMPTY_TOPOLOGY_DIFF,
			};
		},
	};

	const featureTransformScale3D: ActionDef = {
		id: "transform.scale3d",
		run: (params, { topology }) => {
			const center = isVec3(params.center) ? params.center : null;
			const refA = isVec3(params.referenceA) ? params.referenceA : null;
			const refB = isVec3(params.referenceB) ? params.referenceB : null;
			const targets = Array.isArray(params.targets) ? (params.targets as SelectionTarget[]) : [];
			if (!center || !refA || !refB || targets.length === 0) return {};

			const distA = Math.hypot(refA[0] - center[0], refA[1] - center[1], refA[2] - center[2]);
			const distB = Math.hypot(refB[0] - center[0], refB[1] - center[1], refB[2] - center[2]);
			if (distA < 1e-6) return {};
			const scale = distB / distA;

			const vIds = collectTargetVertices(topology, targets);
			const modifiedVertices: VertexRecordDiff[] = [];
			for (const vid of vIds) {
				const v = topology.vertices[vid];
				if (v) {
					modifiedVertices.push({
						id: vid as VertexRef,
						position: [
							center[0] + (v.position[0] - center[0]) * scale,
							center[1] + (v.position[1] - center[1]) * scale,
							center[2] + (v.position[2] - center[2]) * scale,
						],
					});
				}
			}

			return {
				diff: modifiedVertices.length > 0 ? { vertices: { modified: modifiedVertices } } : EMPTY_TOPOLOGY_DIFF,
			};
		},
	};

	const featureTransformScale1D: ActionDef = {
		id: "transform.scale1d",
		run: (params, { topology }) => {
			const center = isVec3(params.center) ? params.center : null;
			const refA = isVec3(params.referenceA) ? params.referenceA : null;
			const refB = isVec3(params.referenceB) ? params.referenceB : null;
			const targets = Array.isArray(params.targets) ? (params.targets as SelectionTarget[]) : [];
			if (!center || !refA || !refB || targets.length === 0) return {};

			const distA = Math.hypot(refA[0] - center[0], refA[1] - center[1], refA[2] - center[2]);
			const distB = Math.hypot(refB[0] - center[0], refB[1] - center[1], refB[2] - center[2]);
			if (distA < 1e-6) return {};
			const scale = distB / distA;

			const dir = vec3Normalize(vec3Sub(refA, center));

			const vIds = collectTargetVertices(topology, targets);
			const modifiedVertices: VertexRecordDiff[] = [];
			for (const vid of vIds) {
				const v = topology.vertices[vid];
				if (v) {
					const delta = vec3Sub(v.position, center);
					const proj = vec3Dot(delta, dir);
					const ortho = vec3Sub(delta, vec3Scale(dir, proj));
					const newProj = proj * scale;
					
					modifiedVertices.push({
						id: vid as VertexRef,
						position: vec3Add(center, vec3Add(vec3Scale(dir, newProj), ortho)),
					});
				}
			}

			return {
				diff: modifiedVertices.length > 0 ? { vertices: { modified: modifiedVertices } } : EMPTY_TOPOLOGY_DIFF,
			};
		},
	};

	const featureCurveLine: ActionDef = {
		id: "curve.line",
		run: (params) => {
			const p0 = isVec3(params.p0) ? params.p0 : null;
			const p1 = isVec3(params.p1) ? params.p1 : null;
			if (!p0 || !p1) return {};
			const id = () => `id-${Math.random().toString(36).slice(2, 9)}`;
			const v0 = id() as VertexRef;
			const v1 = id() as VertexRef;
			const e = id() as EdgeRef;
			const w = id() as WireRef;
			return {
				diff: {
					vertices: { added: [{ id: v0, position: p0 }, { id: v1, position: p1 }] },
					edges: { added: [{ id: e, vertexIds: [v0, v1] }] },
					wires: { added: [{ id: w, edgeIds: [e] }] },
				},
			};
		},
	};

	const featureCurvePolyline: ActionDef = {
		id: "curve.polyline",
		run: (params) => {
			const points = Array.isArray(params.points) ? params.points.filter(isVec3) : [];
			if (points.length < 2) return {};
			const id = () => `id-${Math.random().toString(36).slice(2, 9)}`;
			const diff: TopologyDiff = { vertices: { added: [] }, edges: { added: [] }, wires: { added: [] } };
			const vIds: VertexRef[] = [];
			for (const p of points) {
				const vid = id() as VertexRef;
				diff.vertices!.added!.push({ id: vid, position: p });
				vIds.push(vid);
			}
			const eIds: EdgeRef[] = [];
			for (let i = 0; i < vIds.length - 1; i++) {
				const eid = id() as EdgeRef;
				diff.edges!.added!.push({ id: eid, vertexIds: [vIds[i]!, vIds[i + 1]!] });
				eIds.push(eid);
			}
			const w = id() as WireRef;
			diff.wires!.added!.push({ id: w, edgeIds: eIds });
			return { diff };
		},
	};
	const featureTransformCopy: ActionDef = {
		id: "transform.copy",
		run: (params, { topology }) => {
			const targets = Array.isArray(params.targets) ? (params.targets as SelectionTarget[]) : [];
			const from = isVec3(params.from) ? params.from : null;
			const to = isVec3(params.to) ? params.to : null;
			if (targets.length === 0 || !from || !to) return {};

			const delta = vec3Sub(to, from);
			
			const idMap = new Map<string, string>();
			const nextId = (kind: string) => `id-${kind}-${Math.random().toString(36).slice(2, 9)}`;
			const getMapped = <T extends string>(id: T, kind: string): T => {
				if (!idMap.has(id)) idMap.set(id, nextId(kind));
				return idMap.get(id) as T;
			};

			const diff: TopologyDiff = {
				vertices: { added: [] },
				edges: { added: [] },
				wires: { added: [] },
				faces: { added: [] },
				shells: { added: [] },
				cells: { added: [] },
			};

			const vertices = new Set<VertexRef>();
			const edges = new Set<EdgeRef>();
			const wires = new Set<WireRef>();
			const faces = new Set<FaceRef>();
			const shells = new Set<ShellRef>();
			const cells = new Set<CellRef>();

			const walk = (kind: TopologyEntityKind, id: string) => {
				if (kind === "vertex") {
					if (topology.vertices[id]) vertices.add(id as VertexRef);
				} else if (kind === "edge") {
					const e = topology.edges[id];
					if (e) { edges.add(id as EdgeRef); for (const v of e.vertexIds) walk("vertex", v); }
				} else if (kind === "wire") {
					const w = topology.wires[id];
					if (w) { wires.add(id as WireRef); for (const e of w.edgeIds) walk("edge", e); }
				} else if (kind === "face") {
					const f = topology.faces[id];
					if (f) { faces.add(id as FaceRef); for (const w of f.wireIds) walk("wire", w); }
				} else if (kind === "shell") {
					const s = topology.shells[id];
					if (s) { shells.add(id as ShellRef); for (const f of s.faceIds) walk("face", f); }
				} else if (kind === "cell") {
					const c = topology.cells[id];
					if (c) { cells.add(id as CellRef); for (const s of c.shellIds) walk("shell", s); }
				}
			};
			for (const t of targets) walk(t.kind, t.id);

			for (const vid of vertices) {
				const v = topology.vertices[vid]!;
				diff.vertices!.added!.push({ id: getMapped(vid, "v"), position: vec3Add(v.position, delta) });
			}
			for (const eid of edges) {
				const e = topology.edges[eid]!;
				diff.edges!.added!.push({ id: getMapped(eid, "e"), vertexIds: e.vertexIds.map(x => getMapped(x, "v")) });
			}
			for (const wid of wires) {
				const w = topology.wires[wid]!;
				diff.wires!.added!.push({ id: getMapped(wid, "w"), edgeIds: w.edgeIds.map(x => getMapped(x, "e")) });
			}
			for (const fid of faces) {
				const f = topology.faces[fid]!;
				diff.faces!.added!.push({ id: getMapped(fid, "f"), wireIds: f.wireIds.map(x => getMapped(x, "w")) });
			}
			for (const sid of shells) {
				const s = topology.shells[sid]!;
				diff.shells!.added!.push({ id: getMapped(sid, "s"), faceIds: s.faceIds.map(x => getMapped(x, "f")) });
			}
			for (const cid of cells) {
				const c = topology.cells[cid]!;
				diff.cells!.added!.push({ id: getMapped(cid, "c"), shellIds: c.shellIds.map(x => getMapped(x, "s")) });
			}

			return { diff };
		},
	};

	return [
		entityCreateAnchor,
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
		commandAddPoint,
		commandAddSelection,
		commandFinish,
		featureTransformMove,
		featureTransformRotate,
		featureTransformScale3D,
		featureTransformCopy,
		featureCurveLine,
		featureCurvePolyline,
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
	/** @emoji 🪞 Pick/display hull when a face is split (subset of the parent face). */
	readonly regionPoints?: readonly Vec3[];
}

/** @emoji 🪞 Semantic `Part` view over one or more cells (overlap classification). */
export interface PartView {
	readonly id: PartRef;
	readonly sourceCellIds: readonly CellRef[];
	readonly overlap: "none" | "difference" | "intersection";
	readonly volume: number;
	/** @emoji 🪞 Pick/display hull for the semantic cell partition (not the whole source cell). */
	readonly regionPoints?: readonly Vec3[];
}
/** @emoji 🪞 Computes derived `SurfaceView` / `PartView` via optional kernel booleans. */
export class DerivedViewService {
	private surfaceRevision = -1;
	private partRevision = -1;
	private surfaces: SurfaceView[] = [];
	private parts: PartView[] = [];
	private refreshGen = 0;

	constructor(private readonly kernel: SpatialKernel) {}

	/** @emoji 🪞 Recomputes surfaces and parts (awaits kernel booleans when present). */
	async refresh(topo: TopologyGraph): Promise<void> {
		const gen = ++this.refreshGen;
		this.surfaces = await Promise.resolve(this.kernel.computeSurfaceViews(topo));
		if (gen !== this.refreshGen) return;
		this.surfaceRevision = topo.revision;
		this.parts = await Promise.resolve(this.kernel.computePartViews(topo));
		if (gen !== this.refreshGen) return;
		this.partRevision = topo.revision;
	}

	/** @emoji 🪞 Returns cached surfaces for `topo.revision`. */
	computeSurfaces(topo: TopologyGraph): SurfaceView[] {
		if (this.surfaceRevision === topo.revision) return this.surfaces;
		return this.surfaces;
	}

	/** @emoji 🪞 Returns cached parts for `topo.revision` (call `refresh` first when kernel parts are async). */
	computeParts(topo: TopologyGraph): PartView[] {
		if (this.partRevision === topo.revision) return this.parts;
		return this.parts;
	}

	resolveSurface(surface: SurfaceRef, topo: TopologyGraph): readonly FaceRef[] {
		const hit = this.computeSurfaces(topo).find((s) => String(s.id) === String(surface));
		return hit ? [...hit.sourceFaceIds] : [];
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
	readonly kernel: SpatialKernel;
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
		kernel?: SpatialKernel,
		topology?: TopologyGraph,
		actions?: ActionRegistry,
		derived?: DerivedViewService,
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

/** @emoji 🧮 Serializes `KernelQueryParams` into the loose record shape expected by `SpatialKernel.query`. */
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
	kernel: SpatialKernel | undefined,
	topology: TopologyGraph,
	actions?: ActionRegistry,
	derived?: DerivedViewService,
): Promise<void> {
	const env: ExprEnv = { context: ctx, event, topology, derived };
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
	} else if (a.op === "kernel.query") {
		const params = kernelQueryParamsToRecord(a.params, env);
		const queryCtx: KernelQueryContext = { topology, derived: env.derived as DerivedViewService | undefined };
		if (a.query === "surface.resolveFaces" && derived) {
			const sid = String(params.surfaceId ?? "");
			writePathTarget(a.assignTo, env, derived.resolveSurface(sid as SurfaceRef, topology));
		} else if (kernel?.query) {
			const res = await kernel.query(a.query, params, queryCtx);
			writePathTarget(a.assignTo, env, res);
		}
	} else if (a.op === "action") {
		const def = reg.get(a.action);
		if (!def) return;
		const paramBag: Record<string, unknown> = { __context: ctx, __event: event };
		for (const [k, ex] of Object.entries(a.params ?? {})) {
			paramBag[k] = evalExpr(ex, env);
		}
		const k = kernel ?? (null as unknown as SpatialKernel);
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
	kernel?: SpatialKernel,
	actions?: ActionRegistry,
	topology?: TopologyGraph,
	derived?: DerivedViewService,
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
			await applyEffectAsync(eff, context, event, kernel, topo, actions, derived);
		}
		let nextState = state;
		if (tr.target) {
			nextState = tr.target;
			if (tr.target === spec.machine.initial && tr.target !== state) {
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

/** @emoji ⎋ Hard-aborts the active interaction session when `capabilities.canCancel`. */
export function abortActiveInteractionSession(rt: InteractionRuntime): boolean {
	if (!rt.getSnapshot().capabilities.canCancel) return false;
	rt.cancel();
	return true;
}

/** @emoji 🎬 Minimal async statechart runner for `InteractionSpec.machine`. */
export class StatechartRuntime implements StateEngine {
	private state: string;
	private context: Record<string, unknown>;

	constructor(private readonly spec: InteractionSpec) {
		this.state = spec.machine.initial;
		this.context = initialContextForSpec(spec);
	}

	getState(): string {
		return this.state;
	}

	getContext(): Record<string, unknown> {
		return this.context;
	}

	reset(): void {
		this.state = this.spec.machine.initial;
		this.context = initialContextForSpec(this.spec);
	}

	/** @emoji 🎬 Restores a prior `state` + `context` snapshot (interaction-local undo). */
	restore(state: string, context: Record<string, unknown>): void {
		this.state = state;
		this.context = context;
	}

	/** @emoji 🎬 Applies one external event; returns whether a transition fired. */
	async send(
		event: InteractionEvent,
		kernel?: SpatialKernel,
		topology?: TopologyGraph,
		actions?: ActionRegistry,
		derived?: DerivedViewService,
	): Promise<StateEngineSendResult> {
		const r = await applyTransition(this.spec, this.state, this.context, event, kernel, actions, topology, derived);
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
			case "preview": {
				const params: Record<string, unknown> = {};
				for (const [k, v] of Object.entries(it.params ?? {})) params[k] = evalExpr(v, env);
				items.push({
					kind: "preview",
					id: it.id,
					...(it.role ? { role: it.role } : {}),
					params: { previewKind: it.previewKind, ...params },
				});
				break;
			}
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

export type SpatialComputeMode = "fast" | "precise";

export interface InteractionRuntimeOptions {
	readonly kernel: SpatialKernel;
	readonly previewKernel?: SpatialPreviewKernel;
	readonly mode?: SpatialComputeMode;
	readonly document: ModelDocument;
	readonly history?: DocumentHistory;
	readonly stateEngine?: StateEngineProvider;
	readonly actions?: ActionRegistry;
	readonly query?: ConstructRunner;
	readonly derived?: DerivedViewService;
}

export function isInteractionSessionActive(spec: InteractionSpec, state: string): boolean {
	return !isFinalInteractionState(spec, state);
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
			return evalGuard(g, this.exprEnv());
		}
		return true;
	}

	private selectionEventFromStart(event: InteractionEvent, spec: SelectionSpec): SelectionEvent | null {
		const rawTargets = Array.isArray(event.targets) ? (event.targets as readonly SelectionTarget[]) : [];
		const selected = filterSelectionTargets(spec, rawTargets);
		if (selected.length === 0) return null;
		return { kind: "selection.changed", targets: spec.multiple ? selected : selected.slice(0, 1) };
	}

	private stateHasEvent(state: string, eventKind: string): boolean {
		return Boolean(findState(this.spec, state)?.on?.some((handler) => handler.event === eventKind));
	}

	private async consumeStartSelection(event: InteractionEvent): Promise<void> {
		const stateBeforeSelection = this.sm.getState();
		const sel = getActiveSelectionSpec(this.spec, stateBeforeSelection);
		if (!sel) return;
		const selectionEvent = this.selectionEventFromStart(event, sel);
		if (!selectionEvent || !selectionEventMatches(sel, selectionEvent)) return;
		const beforeCtx = this.cloneCtx(this.sm.getContext());
		const r = await this.sm.send(selectionEvent, this.opts.kernel, this.opts.document.topology, this.actions, this.opts.derived);
		if (!r.ok) return;
		if (!r.transient) this.snapUndoStack.push({ state: stateBeforeSelection, context: JSON.stringify(beforeCtx) });
		const stateAfterSelection = this.sm.getState();
		if (stateAfterSelection === stateBeforeSelection && this.stateHasEvent(stateAfterSelection, "confirm")) {
			const beforeConfirmCtx = this.cloneCtx(this.sm.getContext());
			const cr = await this.sm.send({ kind: "confirm" }, this.opts.kernel, this.opts.document.topology, this.actions, this.opts.derived);
			if (cr.ok && !cr.transient) this.snapUndoStack.push({ state: stateAfterSelection, context: JSON.stringify(beforeConfirmCtx) });
		}
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
		const pristineInitial = st === this.spec.machine.initial && this.snapUndoStack.length === 0;
		const canUndo = this.snapUndoStack.length > 0 || ((!active || pristineInitial) && Boolean(hist?.peekUndo()));
		const canRedo = this.snapRedoStack.length > 0 || ((!active || pristineInitial) && Boolean(hist?.peekRedo()));
		this.snapshotCache = {
			interactionId: this.spec.id,
			state: st,
			revision: this.revision,
			context: this.cloneCtx(ctx),
			display,
			spatialInteraction,
			capabilities: {
				canCommit: this.canCommit(),
				canCancel: !isFinalInteractionState(this.spec, this.sm.getState()) && (this.sm.getState() !== this.spec.machine.initial || this.snapUndoStack.length > 0),
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
		const r = await this.sm.send(event, this.opts.kernel, this.opts.document.topology, this.actions, this.opts.derived);
		if (!r.ok) return;
		if (!r.transient) {
			this.snapUndoStack.push({ state: beforeState, context: JSON.stringify(beforeCtx) });
			this.snapRedoStack.length = 0;
		}
		if (event.kind === "selection.changed" && this.sm.getState() === beforeState && this.stateHasEvent(this.sm.getState(), "confirm")) {
			const beforeConfirmCtx = this.cloneCtx(this.sm.getContext());
			const cr = await this.sm.send({ kind: "confirm" }, this.opts.kernel, this.opts.document.topology, this.actions, this.opts.derived);
			if (cr.ok && !cr.transient) this.snapUndoStack.push({ state: beforeState, context: JSON.stringify(beforeConfirmCtx) });
		}
		if (event.kind === "start") await this.consumeStartSelection(event);
		if (isFinalInteractionState(this.spec, this.sm.getState())) {
			await this.runCommit(false);
			return;
		}
		if (this.canCommit()) {
			await this.runCommit(true);
			return;
		}
		this.emit();
	}

	undo(): void {
		if (this.inActiveInteraction() && this.snapUndoStack.length > 0) {
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
		if (this.inActiveInteraction() && this.snapRedoStack.length > 0) {
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
		const env: ExprEnv = { context: ctx, preview: this.previewMath() };
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
			const ar = await Promise.resolve(def.run(paramBag, { kernel: k, preview: this.previewMath(), topology: topo }));
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
type BuiltinInteractionFixture = InteractionSpec & { readonly key?: string };

const createAnchorInteractionJson = {
	schema: "spatial.interaction/v1",
	id: "entity.createAnchor",
	version: "1.0.0",
	label: "CreateAnchor",
	key: "cr",
	interaction: {
		spatialGroundPick: false,
		pickDisabledStates: ["committed"],
		groundPointerMoveStates: ["placeAnchor"],
		heightDragStates: [],
		verticalRodStates: [],
		heightConfirmState: null,
	},
	guards: [
		{
			name: "selectionHasPoint",
			expr: { kind: "exists", target: { root: "event", segments: [{ kind: "field", name: "point" }] } },
		},
		{
			name: "hasHostAndHitPoint",
			expr: {
				kind: "all",
				args: [
					{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "hostKind" }] } },
					{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "hostId" }] } },
					{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "hitPoint" }] } },
				],
			},
		},
	],
	machine: {
		initial: "selectHost",
		states: [
			{
				name: "selectHost",
				selection: {
					accept: ["vertex", "edge", "wire", "face", "cell"],
					multiple: false,
					prompt: "Pick anchor host",
				},
				on: [
					{
						event: "selection.changed",
						transitions: [
							{
								target: "committed",
								guard: "selectionHasPoint",
								key: "i",
								label: "Create anchor",
								effects: [
									{ op: "assign", target: { root: "context", segments: [{ kind: "field", name: "hostKind" }] }, value: { kind: "path", root: "event", segments: [{ kind: "field", name: "targets" }, { kind: "index", index: 0 }, { kind: "field", name: "kind" }] } },
									{ op: "assign", target: { root: "context", segments: [{ kind: "field", name: "hostId" }] }, value: { kind: "path", root: "event", segments: [{ kind: "field", name: "targets" }, { kind: "index", index: 0 }, { kind: "field", name: "id" }] } },
									{ op: "assign", target: { root: "context", segments: [{ kind: "field", name: "hitPoint" }] }, value: { kind: "path", root: "event", segments: [{ kind: "field", name: "point" }] } },
								],
							},
							{
								target: "placeAnchor",
								key: "i",
								label: "Select host",
								effects: [
									{ op: "assign", target: { root: "context", segments: [{ kind: "field", name: "hostKind" }] }, value: { kind: "path", root: "event", segments: [{ kind: "field", name: "targets" }, { kind: "index", index: 0 }, { kind: "field", name: "kind" }] } },
									{ op: "assign", target: { root: "context", segments: [{ kind: "field", name: "hostId" }] }, value: { kind: "path", root: "event", segments: [{ kind: "field", name: "targets" }, { kind: "index", index: 0 }, { kind: "field", name: "id" }] } },
								],
							},
						],
					},
					{ event: "cancel", transitions: [{ target: "selectHost", key: "x", label: "Cancel" }] },
				],
			},
			{
				name: "placeAnchor",
				on: [
					{
						event: "pointer.move",
						transitions: [
							{
								transient: true,
								effects: [
									{ op: "assign", target: { root: "context", segments: [{ kind: "field", name: "cursor" }] }, value: { kind: "path", root: "event", segments: [{ kind: "field", name: "point" }] } },
								],
							},
						],
					},
					{
						event: "pointer.down",
						transitions: [
							{
								target: "committed",
								key: "Enter",
								label: "Place anchor",
								effects: [
									{ op: "assign", target: { root: "context", segments: [{ kind: "field", name: "hitPoint" }] }, value: { kind: "path", root: "event", segments: [{ kind: "field", name: "point" }] } },
								],
							},
						],
					},
					{ event: "cancel", transitions: [{ target: "selectHost", key: "x", label: "Cancel" }] },
				],
			},
			{ name: "committed", final: true },
		],
	},
	display: {
		states: [
			{
				state: "placeAnchor",
				items: [{ kind: "point", id: "cursor", role: "cursor", position: { kind: "path", root: "context", segments: [{ kind: "field", name: "cursor" }] } }],
			},
		],
	},
	commit: {
		when: "hasHostAndHitPoint",
		fromStates: ["committed"],
		operation: {
			kind: "action",
			action: "entity.createAnchor",
			params: {
				hostKind: { kind: "path", root: "context", segments: [{ kind: "field", name: "hostKind" }] },
				hostId: { kind: "path", root: "context", segments: [{ kind: "field", name: "hostId" }] },
				hitPoint: { kind: "path", root: "context", segments: [{ kind: "field", name: "hitPoint" }] },
			},
		},
	},
} as const satisfies BuiltinInteractionFixture;

const builtinInteractionJsons = [
	createAnchorInteractionJson,
	boxInteractionJson,
	extrudeWireInteractionJson,
	offsetSurfaceInteractionJson,
	distanceInteractionJson,
	areaInteractionJson,
	curveArcInteractionJson,
	curveCircleInteractionJson,
	curveControlPointCurveInteractionJson,
	curveInterpolateCurveInteractionJson,
	curveLineInteractionJson,
	curvePolylineInteractionJson,
	editChamferInteractionJson,
	editExplodeInteractionJson,
	editFilletInteractionJson,
	editJoinInteractionJson,
	editSplitInteractionJson,
	editTrimInteractionJson,
	solidBooleanDifferenceInteractionJson,
	solidBooleanIntersectionInteractionJson,
	solidBooleanUnionInteractionJson,
	solidCylinderInteractionJson,
	solidSphereInteractionJson,
	surfaceExtrudeCrvInteractionJson,
	surfaceLoftInteractionJson,
	surfaceNetworkSrfInteractionJson,
	surfacePlaneInteractionJson,
	surfaceSweep1InteractionJson,
	surfaceSweep2InteractionJson,
	transformCopyInteractionJson,
	transformMirrorInteractionJson,
	transformMoveInteractionJson,
	transformRotateInteractionJson,
	transformScale1dInteractionJson,
	transformScale3dInteractionJson,
] as readonly BuiltinInteractionFixture[];

function interactionFixtureRow(spec: BuiltinInteractionFixture): SpatialInteraction {
	return { id: spec.id, label: spec.label ?? spec.id, key: typeof spec.key === "string" ? spec.key : spec.id[0] ?? "?" };
}

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
		const xs = builtinInteractionJsons.map((raw) => {
			const spec = parseInteractionSpec(raw);
			return spec ? compileInteraction(spec) : null;
		});
		for (const s of xs) {
			if (s) r.register(s);
		}
		return r;
	}
}

/** @emoji 📦 Parses canonical box asset (`spatial/assets/interactions/box.interaction.json`). */
export function buildBoxInteractionSpec(): InteractionSpec {
	const s = parseInteractionSpec(boxInteractionJson);
	if (!s) throw new Error("spatial/assets/interactions/box.interaction.json invalid");
	return compileInteraction(s);
}

/** @emoji 📦 Parses extrude-wire asset (`spatial/assets/interactions/extrude-wire.interaction.json`). */
export function buildExtrudeInteractionSpec(): InteractionSpec {
	const s = parseInteractionSpec(extrudeWireInteractionJson);
	if (!s) throw new Error("spatial/assets/interactions/extrude-wire.interaction.json invalid");
	return compileInteraction(s);
}

/** @emoji 📦 Parses offset-surface asset (`spatial/assets/interactions/offset-surface.interaction.json`). */
export function buildOffsetSurfaceInteractionSpec(): InteractionSpec {
	const s = parseInteractionSpec(offsetSurfaceInteractionJson);
	if (!s) throw new Error("spatial/assets/interactions/offset-surface.interaction.json invalid");
	return compileInteraction(s);
}

/** @emoji 📦 Parses distance asset (`spatial/assets/interactions/measure-length.interaction.json`). */
export function buildDistanceInteractionSpec(): InteractionSpec {
	const s = parseInteractionSpec(distanceInteractionJson);
	if (!s) throw new Error("spatial/assets/interactions/measure-length.interaction.json invalid");
	return compileInteraction(s);
}

/** @emoji 📦 Parses area asset (`spatial/assets/interactions/area.interaction.json`). */
export function buildAreaInteractionSpec(): InteractionSpec {
	const s = parseInteractionSpec(areaInteractionJson);
	if (!s) throw new Error("spatial/assets/interactions/area.interaction.json invalid");
	return compileInteraction(s);
}

export function buildCreateAnchorInteractionSpec(): InteractionSpec {
	const s = parseInteractionSpec(createAnchorInteractionJson);
	if (!s) throw new Error("entity.createAnchor interaction invalid");
	return compileInteraction(s);
}

/** @emoji 📚 Host-facing built-in interaction row (`spatial/assets/interactions/*.interaction.json`). */
export interface SpatialInteraction {
	readonly id: string;
	readonly label: string;
	/** @emoji ⌨️ Host interaction key; must stay unique and appear in `label` (see `resolveSpatialInteractionKey`). */
	readonly key: string;
}

/** @emoji 📚 Built-in interaction ids for host interaction surfaces (`spatial/assets/interactions/*.interaction.json`). */
export function listSpatialInteractions(): readonly SpatialInteraction[] {
	return builtinInteractionJsons.map(interactionFixtureRow);
}

/** @emoji 🧭 Resolves a typed token to an interaction (`key`, `id`, or compact `label`). */
export function resolveSpatialInteractionKey(token: string): SpatialInteraction | null {
	const t = token.trim().toLowerCase();
	if (!t) return null;
	for (const p of listSpatialInteractions()) {
		if (p.key.toLowerCase() === t) return p;
		if (p.id.toLowerCase() === t) return p;
		const slug = p.label.toLowerCase().replace(/\s+/g, "");
		if (slug === t) return p;
	}
	return null;
}

/** @emoji 📚 Loads a built-in interaction by stable `id` (see `listSpatialInteractions`). */
export function loadSpatialInteraction(interactionId: string): InteractionSpec | null {
	const raw = builtinInteractionJsons.find((spec) => spec.id === interactionId);
	const spec = raw ? parseInteractionSpec(raw) : null;
	return spec ? compileInteraction(spec) : null;
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

	describe("@spatial/js-core edge and cell geometry", () => {
		it("arcEndOnCircle projects off-circle pick onto arc", () => {
			const end = arcEndOnCircle([0, 0, 0], [2, 0, 0], [0, 3, 0]);
			expect(end[0]).toBeCloseTo(0, 5);
			expect(end[1]).toBeCloseTo(2, 5);
			expect(vec3Distance([0, 0, 0], end)).toBeCloseTo(2, 5);
		});
		it("arcSamplePoints quarter arc from center start end", () => {
			const pts = arcSamplePoints([0, 0, 0], [2, 0, 0], [0, 2, 0], 4);
			expect(pts[0]).toEqual([2, 0, 0]);
			expect(pts[pts.length - 1]![0]).toBeCloseTo(0, 5);
			expect(pts[pts.length - 1]![1]).toBeCloseTo(2, 5);
		});
		it("circleSamplePoints and edgeCurveLength for Geom_Circle", () => {
			const pts = circleSamplePoints([0, 0, 0], [0, 0, 1], 2, 64);
			expect(pts.length).toBeGreaterThan(8);
			expect(edgeCurveLength({ kind: "circle", center: [0, 0, 0], normal: [0, 0, 1], radius: 2 }, [[2, 0, 0], [2, 0, 0]])).toBeCloseTo(
				Math.PI * 4,
				3,
			);
		});
		it("nurbs edge samples through control poles", () => {
			const curve = nurbsCurveFromPoles([
				[0, 0, 0],
				[1, 2, 0],
				[3, 0, 0],
			] as Vec3[])!;
			const v0 = "v0" as VertexRef;
			const v1 = "v1" as VertexRef;
			const verts = {
				[v0]: { id: v0, position: [0, 0, 0] as Vec3 },
				[v1]: { id: v1, position: [3, 0, 0] as Vec3 },
			};
			const edge: EdgeRecord = { id: "e" as EdgeRef, vertexIds: [v0, v1], curve };
			const pts = edgeSamplePoints(verts, edge, 24);
			expect(pts.length).toBeGreaterThan(4);
		});
		it("cellSolidAabb sphere bounds", () => {
			const b = cellSolidAabb({ kind: "sphere", center: [1, 2, 3], radius: 5 });
			expect(b.min).toEqual([-4, -3, -2]);
			expect(b.max).toEqual([6, 7, 8]);
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

	describe("@spatial/js-core derived views", () => {
		it("merges coplanar faces into one surface", () => {
			const topo = new TopologyGraph();
			const v0 = "v0" as VertexRef;
			const v1 = "v1" as VertexRef;
			const v2 = "v2" as VertexRef;
			const v3 = "v3" as VertexRef;
			const e0 = "e0" as EdgeRef;
			const e1 = "e1" as EdgeRef;
			const e2 = "e2" as EdgeRef;
			const e3 = "e3" as EdgeRef;
			const w0 = "w0" as WireRef;
			const w1 = "w1" as WireRef;
			const f0 = "f0" as FaceRef;
			const f1 = "f1" as FaceRef;
			topo.vertices[v0] = { id: v0, position: [0, 0, 0] };
			topo.vertices[v1] = { id: v1, position: [1, 0, 0] };
			topo.vertices[v2] = { id: v2, position: [1, 1, 0] };
			topo.vertices[v3] = { id: v3, position: [0, 1, 0] };
			topo.edges[e0] = { id: e0, vertexIds: [v0, v1] };
			topo.edges[e1] = { id: e1, vertexIds: [v1, v2] };
			topo.edges[e2] = { id: e2, vertexIds: [v2, v3] };
			topo.edges[e3] = { id: e3, vertexIds: [v3, v0] };
			topo.wires[w0] = { id: w0, edgeIds: [e0, e1, e2, e3] };
			topo.wires[w1] = { id: w1, edgeIds: [e0, e1, e2, e3] };
			topo.faces[f0] = { id: f0, wireIds: [w0] };
			topo.faces[f1] = { id: f1, wireIds: [w1] };
			const surfaces = computeSurfaceViewsFromTopology(topo);
			expect(surfaces).toHaveLength(1);
			expect(surfaces[0]!.sourceFaceIds.sort()).toEqual([f0, f1].sort());
		});

		it("emits intersection parts for cells sharing a face", () => {
			const topo = new TopologyGraph();
			const f = "fs" as FaceRef;
			topo.faces[f] = { id: f, wireIds: [] };
			const s0 = "s0" as ShellRef;
			const s1 = "s1" as ShellRef;
			topo.shells[s0] = { id: s0, faceIds: [f] };
			topo.shells[s1] = { id: s1, faceIds: [f] };
			topo.cells["c0" as CellRef] = { id: "c0" as CellRef, shellIds: [s0] };
			topo.cells["c1" as CellRef] = { id: "c1" as CellRef, shellIds: [s1] };
			const parts = computePartViewsFromTopology(topo);
			expect(parts.some((p) => p.overlap === "intersection")).toBe(true);
			expect(parts.some((p) => p.overlap === "difference")).toBe(true);
		});

		it("splits overlapping box faces into external and internal surface patches", () => {
			const topo = new TopologyGraph();
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 2 }, cellRef("a")));
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [1, 1, 0], cornerB: [3, 3, 0], height: 2 }, cellRef("b")));
			const surfaces = computeSurfaceViewsFromTopology(topo);
			expect(surfaces.some((s) => s.exposure === "internal")).toBe(true);
			expect(surfaces.some((s) => s.exposure === "external")).toBe(true);
			const internal = surfaces.filter((s) => s.exposure === "internal");
			expect(internal.every((s) => (s.regionPoints?.length ?? 0) >= 4)).toBe(true);
			const topInternal = internal.find((s) => s.regionPoints?.every((p) => Math.abs(p[2] - 2) < 1e-5));
			expect(topInternal).toBeDefined();
			const xs = topInternal!.regionPoints!.map((p) => p[0]);
			expect(Math.max(...xs) - Math.min(...xs)).toBeCloseTo(1, 4);
		});

		it("partitions overlapping box cells by intersection volume", () => {
			const topo = new TopologyGraph();
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 2 }, cellRef("a")));
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [1, 1, 0], cornerB: [3, 3, 0], height: 2 }, cellRef("b")));
			const parts = computePartViewsFromTopology(topo);
			const inter = parts.find((p) => p.overlap === "intersection");
			expect(inter?.volume).toBeCloseTo(2, 4);
			expect(parts.filter((p) => p.overlap === "difference").length).toBe(2);
		});
	});

	describe("@spatial/js-core interactions", () => {
		const DEFAULT_KERNEL: SpatialKernel = {
			id: "default-test",
			operations: [] as const,
			async createBoxFromCorners() {
				return cellRef("c");
			},
			async volume() {
				return 0;
			},
			async tessellate() {
				return { positions: new Float32Array(), indices: new Uint32Array() };
			},
		};

		it("lists stable mnemonic keys for each built-in interaction", () => {
			const ps = listSpatialInteractions();
			expect(ps.slice(0, 5).map((p) => p.key).join("")).toBe("crbeod");
			expect(ps.length).toBeGreaterThanOrEqual(34);
			expect(new Set(ps.map((p) => p.key)).size).toBe(ps.length);
			expect(ps.slice(0, 5).every((p) => p.label.toLowerCase().includes(p.key))).toBe(true);
		});
		it("uses PascalCase interaction labels without spaces", () => {
			for (const row of listSpatialInteractions()) {
				expect(row.label).toMatch(/^[A-Z][A-Za-z0-9]*$/);
				expect(row.label).not.toContain(" ");
			}
		});
		it("commits createAnchor directly from a hit-point selection", async () => {
			class AnchorKernel implements SpatialKernel {
				readonly id = "anchor-test";
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
			}
			const topo = new TopologyGraph();
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cellRef("box")));
			const edgeId = Object.keys(topo.edges)[0]! as EdgeRef;
			const rt = createInteractionRuntime(buildCreateAnchorInteractionSpec(), { kernel: new AnchorKernel(), document: { topology: topo, nodes: [] } });
			await rt.send({
				kind: "selection.changed",
				targets: [{ kind: "edge", id: edgeId, editable: true }],
				point: [0.4, 0, 0],
				modifiers: {},
			});
			const snap = rt.getSnapshot();
			expect(snap.state).toBe("committed");
			expect(snap.lastResponse?.ok).toBe(true);
			const anchors = Object.values(topo.anchors);
			expect(anchors).toHaveLength(1);
			expect(anchors[0]!.attachment).toEqual({ kind: "edge", id: edgeId, t: 0.4 });
			expect(anchors[0]!.position).toEqual([0.4, 0, 0]);
		});
		it("commits createAnchor after selecting a host then placing a point", async () => {
			class AnchorKernel implements SpatialKernel {
				readonly id = "anchor-test";
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
			}
			const topo = new TopologyGraph();
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cellRef("box")));
			const faceId = Object.keys(topo.faces)[0]! as FaceRef;
			const rt = createInteractionRuntime(buildCreateAnchorInteractionSpec(), { kernel: new AnchorKernel(), document: { topology: topo, nodes: [] } });
			await rt.send({
				kind: "selection.changed",
				targets: [{ kind: "face", id: faceId, editable: true }],
				modifiers: {},
			});
			expect(rt.getSnapshot().state).toBe("placeAnchor");
			await rt.send({ kind: "pointer.down", point: [0.25, 0.75, 0], modifiers: {} });
			const snap = rt.getSnapshot();
			expect(snap.state).toBe("committed");
			expect(snap.lastResponse?.ok).toBe(true);
			const anchors = Object.values(topo.anchors);
			expect(anchors).toHaveLength(1);
			expect(anchors[0]!.attachment.kind).toBe("face");
			expect(anchors[0]!.attachment.id).toBe(faceId);
			expect(anchors[0]!.position).toEqual([0.25, 0.75, 0]);
		});
		it("resolves interaction tokens by key, id, and label slug", () => {
			expect(resolveSpatialInteractionKey("b")?.id).toBe("primitive.box");
			expect(resolveSpatialInteractionKey("cr")?.id).toBe("entity.createAnchor");
			expect(resolveSpatialInteractionKey("primitive.box")?.key).toBe("b");
			expect(resolveSpatialInteractionKey("extrudewire")?.id).toBe("feature.extrudeWire");
			expect(resolveSpatialInteractionKey("d")?.id).toBe("measure.distance");
			expect(resolveSpatialInteractionKey("curve.line")?.id).toBe("curve.line");
		});
		it("loads every built-in interaction spec", () => {
			for (const row of listSpatialInteractions()) {
				const spec = loadSpatialInteraction(row.id);
				expect(spec?.id).toBe(row.id);
			}
		});
		it("does not expose finalize or cancel transitions for scripted commands", () => {
			const spec = loadSpatialInteraction("curve.line")!;
			const labels = spec.machine.states.flatMap((state) => state.on?.flatMap((handler) => handler.transitions.map((t) => t.label)) ?? []);
			expect(labels).not.toContain("Finalize");
			expect(labels).not.toContain("Cancel");
		});
		it("auto-finalizes scripted commands when the terminal input is done", async () => {
			class CommandKernel implements SpatialKernel {
				readonly id = "command";
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
				async executeCommandDiff() {
					return { diff: EMPTY_TOPOLOGY_DIFF };
				}
			}
			const spec = loadSpatialInteraction("curve.line")!;
			const rt = createInteractionRuntime(spec, { kernel: new CommandKernel(), document: { topology: new TopologyGraph(), nodes: [] } });
			await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
			await rt.send({ kind: "pointer.down", point: [1, 2, 0] as Vec3, modifiers: {} });
			const snap = rt.getSnapshot();
			expect(snap.state).toBe("committed");
			expect(snap.capabilities.canCommit).toBe(false);
			expect(snap.lastResponse?.ok).toBe(true);
			expect(snap.lastResponse?.diff?.vertices?.added?.length).toBe(2);
		});
		it("abortActiveInteractionSession hard-resets an in-progress session", async () => {
			class CommandKernel implements SpatialKernel {
				readonly id = "command-abort";
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
				async executeCommandDiff() {
					return { diff: EMPTY_TOPOLOGY_DIFF };
				}
			}
			const spec = loadSpatialInteraction("curve.line")!;
			const rt = createInteractionRuntime(spec, { kernel: new CommandKernel(), document: { topology: new TopologyGraph(), nodes: [] } });
			await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
			expect(rt.getSnapshot().capabilities.canCancel).toBe(true);
			expect(abortActiveInteractionSession(rt)).toBe(true);
			const snap = rt.getSnapshot();
			expect(snap.state).toBe(spec.machine.initial);
			expect(snap.capabilities.canCancel).toBe(false);
			expect(abortActiveInteractionSession(rt)).toBe(false);
		});
		it("collectTargetVertices expands face selection to boundary vertices", () => {
			const topo = new TopologyGraph();
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cellRef("box")));
			const faceId = Object.keys(topo.faces)[0]!;
			const vIds = collectTargetVertices(topo, [{ kind: "face", id: faceId }]);
			expect(vIds.size).toBe(4);
		});
		it("uses initial selection to skip selection-first command states", async () => {
			class CommandKernel implements SpatialKernel {
				readonly id = "command-selection";
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
				async executeCommandDiff() {
					return { diff: EMPTY_TOPOLOGY_DIFF };
				}
			}
			const spec = loadSpatialInteraction("transform.move")!;
			const rt = createInteractionRuntime(spec, { kernel: new CommandKernel(), document: { topology: new TopologyGraph(), nodes: [] } });
			await rt.send({
				kind: "selection.changed",
				targets: [{ kind: "cell", id: "c0", editable: true }],
				modifiers: {},
			});
			const snap = rt.getSnapshot();
			expect(snap.state).toBe("point_to_move_from");
			expect((snap.context.targets as SelectionTarget[]).map((target) => target.id)).toEqual(["c0"]);
		});
		it("auto-commits curve.arc as one arc edge between start and end", async () => {
			const topo = new TopologyGraph();
			class ArcKernel implements SpatialKernel {
				readonly id = "arc-command";
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
				async executeCommandDiff(commandId: string, ctx: Record<string, unknown>) {
					if (commandId !== "curve.arc") return { diff: EMPTY_TOPOLOGY_DIFF };
					const center = (Array.isArray(ctx.center) ? ctx.center : [0, 0, 0]) as Vec3;
					const start = (Array.isArray(ctx.start) ? ctx.start : [1, 0, 0]) as Vec3;
					const end = arcEndOnCircle(center, start, (Array.isArray(ctx.end) ? ctx.end : start) as Vec3);
					const v0 = "v0" as VertexRef;
					const v1 = "v1" as VertexRef;
					const e = "e0" as EdgeRef;
					const w = "w0" as WireRef;
					return {
						diff: {
							vertices: { added: [{ id: v0, position: start }, { id: v1, position: end }] },
							edges: { added: [{ id: e, vertexIds: [v0, v1], curve: { kind: "arc" as const, center } }] },
							wires: { added: [{ id: w, edgeIds: [e] }] },
						},
					};
				}
			}
			const spec = loadSpatialInteraction("curve.arc")!;
			const rt = createInteractionRuntime(spec, { kernel: new ArcKernel(), document: { topology: topo, nodes: [] } });
			await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
			await rt.send({ kind: "pointer.down", point: [2, 0, 0] as Vec3, modifiers: {} });
			await rt.send({ kind: "pointer.down", point: [0, 2, 0] as Vec3, modifiers: {} });
			const snap = rt.getSnapshot();
			expect(snap.state).toBe("committed");
			expect(snap.lastResponse?.ok).toBe(true);
			const edges = Object.values(topo.edges);
			expect(edges).toHaveLength(1);
			expect(edges[0]!.curve).toEqual({ kind: "arc", center: [0, 0, 0] });
			expect(Object.keys(topo.vertices)).toHaveLength(2);
		});
		it("auto-finalizes transform.move on terminal pointer down without alreadyCommitted", async () => {
			const topo = new TopologyGraph();
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cellRef("box")));
			const v0 = Object.keys(topo.vertices)[0]!;
			const p0 = topo.vertices[v0]!.position;
			class CommandKernel implements SpatialKernel {
				readonly id = "command-move";
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
				async executeCommandDiff() {
					return { diff: EMPTY_TOPOLOGY_DIFF };
				}
			}
			const spec = loadSpatialInteraction("transform.move")!;
			const rt = createInteractionRuntime(spec, { kernel: new CommandKernel(), document: { topology: topo, nodes: [] } });
			await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: v0, editable: true }], modifiers: {} });
			await rt.send({ kind: "pointer.down", point: p0, modifiers: {} });
			await rt.send({ kind: "pointer.down", point: [p0[0] + 2, p0[1] + 1, p0[2]], modifiers: {} });
			const snap = rt.getSnapshot();
			expect(snap.state).toBe("committed");
			expect(snap.lastResponse?.ok).toBe(true);
			expect(snap.lastResponse?.errors).toEqual([]);
			expect(snap.lastResponse?.diff?.vertices?.modified?.length).toBeGreaterThan(0);
			expect(topo.vertices[v0]!.position).toEqual([p0[0] + 2, p0[1] + 1, p0[2]]);
		});
	});

	describe("@spatial/js-core action and interaction registries", () => {
		it("ActionRegistry.withBuiltins registers known geometry actions", () => {
			const r = ActionRegistry.withBuiltins();
			const ids = new Set(r.list().map((d) => d.id));
			expect(ids.has("primitive.createBoxFromCorners")).toBe(true);
			expect(ids.has("box.aabbFromDiagonalCorners")).toBe(true);
			expect(ids.has("command.finish")).toBe(true);
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
			class StubKernel implements SpatialKernel {
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

		it("boxTopologyDiff creates selectable boundary and volume records", () => {
			const g = new TopologyGraph();
			applyTopologyDiff(g, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 4 }, cellRef("box-cell")));
			expect(Object.keys(g.vertices).length).toBe(8);
			expect(Object.keys(g.edges).length).toBe(12);
			expect(Object.keys(g.wires).length).toBe(6);
			expect(Object.keys(g.faces).length).toBe(6);
			expect(Object.keys(g.shells).length).toBe(1);
			expect(Object.keys(g.cells)).toEqual(["box-cell"]);
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
			class StubKernel implements SpatialKernel {
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
			class StubKernel implements SpatialKernel {
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
			const initial = rt.getSnapshot().state;
			await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
			expect(rt.getSnapshot().capabilities.canUndo).toBe(true);
			await rt.undo();
			expect(rt.getSnapshot().state).toBe(initial);
		});

		it("runs box workflow with a recording kernel stub (no solid modeling in core)", async () => {
			class RecordingStubKernel implements SpatialKernel {
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
			await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
			await rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
			await rt.send({ kind: "set.height", value: 4, modifiers: {} });
			const snap = rt.getSnapshot();
			const res = snap.lastResponse!;
			expect(snap.state).toBe("committed");
			expect(res.ok).toBe(true);
			expect(res.data).toBeNull();
			expect(res.archiveContext).not.toBeNull();
			expect(res.archiveContext!.origin).toEqual([0, 0, 0]);
			expect(res.archiveContext!.corner).toEqual([2, 3, 0]);
			expect(res.archiveContext!.height).toBe(4);
			expect(Object.keys(topo.vertices).length).toBe(8);
			expect(Object.keys(topo.edges).length).toBe(12);
			expect(Object.keys(topo.wires).length).toBe(6);
			expect(Object.keys(topo.faces).length).toBe(6);
			expect(Object.keys(topo.shells).length).toBe(1);
			expect(Object.keys(topo.cells)).toEqual(["stub-cell"]);
			expect(kernel.lastBox).toEqual({
				cornerA: [0, 0, 0],
				cornerB: [2, 3, 0],
				height: 4,
			});
		});
	});

	describe("@spatial/js-core stateEngine option", () => {
		it("explicit pure-ts provider matches default interaction snapshots", async () => {
			class StubKernel implements SpatialKernel {
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
			expect(rt1.getSnapshot().state).toBe(rt0.getSnapshot().state);
			expect(rt1.getSnapshot().context).toEqual(rt0.getSnapshot().context);
			expect(rt1.getSnapshot().capabilities).toEqual(rt0.getSnapshot().capabilities);
		});
	});

	describe("@spatial/js-core measure distance", () => {
		it("commit returns vertex distance in data", async () => {
			class MeasKernel implements SpatialKernel {
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
			const res = rt.getSnapshot().lastResponse!;
			expect(res.ok).toBe(true);
			expect(res.data).toBe(5);
			expect(isEmptyTopologyDiff(res.diff)).toBe(true);
		});

		it("auto-commits when confirm reaches the final state", async () => {
			class MeasKernel implements SpatialKernel {
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
			class AreaKernel implements SpatialKernel {
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
			const res = rt.getSnapshot().lastResponse!;
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
			class MeasKernel implements SpatialKernel {
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
			expect(hist.peekUndo()).toBe(null);
		});
	});

	describe("@spatial/js-core interaction session undo redo", () => {
		it("supports redo after undo during an active interaction and clears redo on new branch", async () => {
			class StubKernel implements SpatialKernel {
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
			expect(rt.getSnapshot().state).toBe("first_corner");
			expect(rt.getSnapshot().capabilities.canRedo).toBe(false);
			await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
			rt.undo();
			expect(rt.getSnapshot().state).toBe("first_corner");
			expect(rt.getSnapshot().capabilities.canRedo).toBe(true);
			await rt.send({ kind: "pointer.down", point: [1, 0, 0] as Vec3, modifiers: {} });
			expect(rt.getSnapshot().capabilities.canRedo).toBe(false);
		});
	});

	describe("@spatial/js-core undo routing", () => {
		it("uses snapshot undo while active and document history when idle", async () => {
			class StubKernel implements SpatialKernel {
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
			await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
			rt.undo();
			expect(rt.getSnapshot().state).toBe("first_corner");
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
