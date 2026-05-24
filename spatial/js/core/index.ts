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

// #region 🗺️Expr
/** @emoji 🗺️ Tagged declarative expression evaluated by `evalExpr` (`spatial/schema/json/expression.json`). */
export type Expr =
	| ExprPath
	| ExprConst
	| ExprVar
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
}

function envWithVars(base: ExprEnv, vars: Record<string, unknown>): ExprEnv {
	return { context: base.context, event: base.event, vars: { ...base.vars, ...vars } };
}

function isVec3(v: unknown): v is Vec3 {
	return Array.isArray(v) && v.length === 3 && v.every((n) => typeof n === "number");
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
/** @emoji 📜 Declared command-local context slots (`spatial.command/v1` `context`). */
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
	readonly actions?: readonly ActionSpec[];
	readonly key?: string;
	readonly label?: string;
}

export type KernelQueryParams = {
	readonly kind: "surface.resolveFaces";
	readonly surfaceId: Expr;
};

export type ActionSpec =
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
	| {
			readonly op: "box.transform";
			readonly transform:
				| "aabbFromDiagonalCorners"
				| "tripletRubber"
				| "tripletCommit"
				| "snapSquareFootprint"
				| "setCubeHeightFromFootprint"
				| "rubberCornerFromCenter"
				| "rubberSquareFromCenter"
				| "verticalFinalizeFootprint"
				| "initPeakAboveOrigin"
				| "peakFromOriginZ"
				| "verticalRubberCorner"
				| "cornerFromLengthWidth";
	  };

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

export type CommitOperationSpec =
	| { readonly kind: "cell.createBox"; readonly cornerA: Expr; readonly cornerB: Expr; readonly height: Expr }
	| { readonly kind: "wire.extrudeToCell"; readonly wireId: Expr; readonly distance: Expr; readonly direction: Expr }
	| { readonly kind: "face.offset"; readonly faceIds: Expr; readonly distance: Expr }
	| { readonly kind: "measure.distance"; readonly a: Expr; readonly b: Expr }
	| { readonly kind: "measure.area"; readonly faceId: Expr }
	| { readonly kind: "measure.volume"; readonly cellId: Expr };

/** @emoji 📜 Parsed static command document (`spatial.command/v1`). */
export interface CommandSpec {
	readonly schema: "spatial.command/v1";
	readonly id: string;
	readonly version: string;
	readonly label?: string;
	readonly context?: { readonly fields: readonly ContextFieldDecl[] };
	readonly requires?: Record<string, unknown>;
	readonly guards?: readonly NamedGuard[];
	readonly history?: { excludeEvents?: readonly string[] };
	readonly machine: {
		readonly initial: string;
		readonly states: readonly StateDefSpec[];
	};
	readonly display?: {
		readonly states?: readonly { readonly state: string; readonly items: readonly DisplayItemSpec[] }[];
	};
	readonly interaction?: CommandSpatialInteractionConfig;
	readonly commit: {
		readonly when?: string;
		readonly fromStates?: readonly string[];
		readonly outputDataPath?: PathTarget;
		readonly operation: CommitOperationSpec;
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

function guardNames(spec: CommandSpec): Set<string> {
	return new Set((spec.guards ?? []).map((g) => g.name));
}

function findState(spec: CommandSpec, name: string): StateDefSpec | undefined {
	return spec.machine.states.find((s) => s.name === name);
}

/** @emoji 🧾 Rewrites legacy `spatial.command/v1` JSON (`states` map, `guards` map, `on` map) into `StateDefSpec[]` + `NamedGuard[]` + `EventHandlerSpec[]`. */
function legacyPathToTarget(path: string): PathTarget {
	const segs = path
		.split(".")
		.filter(Boolean)
		.map((name) => ({ kind: "field" as const, name }));
	return { root: "context", segments: segs };
}

function migrateLegacyActionObject(act: unknown): unknown {
	if (!act || typeof act !== "object") return act;
	const a = act as Record<string, unknown>;
	const op = a.op;
	if (typeof op === "string" && op.startsWith("box.") && op !== "box.transform") {
		return { op: "box.transform", transform: op.slice(4) };
	}
	if (op === "assign" && typeof a.path === "string" && a.target === undefined) {
		const out = { ...a };
		delete out.path;
		out.target = legacyPathToTarget(a.path as string);
		return out;
	}
	if (op === "clear" && typeof a.path === "string" && a.target === undefined) {
		return { op: "clear", target: legacyPathToTarget(a.path as string) };
	}
	return act;
}

function migrateLegacyMachineTransitionActions(m: Record<string, unknown>): void {
	const statesVal = m.states;
	if (!Array.isArray(statesVal)) return;
	for (const st of statesVal as Record<string, unknown>[]) {
		const on = st.on as unknown[] | undefined;
		if (!Array.isArray(on)) continue;
		for (const h of on) {
			if (!h || typeof h !== "object") continue;
			const trs = (h as Record<string, unknown>).transitions as unknown[] | undefined;
			if (!Array.isArray(trs)) continue;
			for (const tr of trs) {
				if (!tr || typeof tr !== "object") continue;
				const acts = (tr as Record<string, unknown>).actions as unknown[] | undefined;
				if (!Array.isArray(acts)) continue;
				(tr as Record<string, unknown>).actions = acts.map((x) => migrateLegacyActionObject(x));
			}
		}
	}
}

function normalizeLegacyDisplay(r: Record<string, unknown>): void {
	const disp = r.display;
	if (!disp || typeof disp !== "object") return;
	const d = disp as Record<string, unknown>;
	const st = d.states;
	if (!st || typeof st !== "object" || Array.isArray(st)) return;
	d.states = Object.entries(st as Record<string, unknown>).map(([state, items]) => ({
		state,
		items: Array.isArray(items) ? items : [],
	}));
}

function normalizeLegacyCommandDocument(r: Record<string, unknown>): void {
	const machine = r.machine;
	if (!machine || typeof machine !== "object") return;
	const m = machine as Record<string, unknown>;
	const statesVal = m.states;
	if (!statesVal || typeof statesVal !== "object") return;
	if (Array.isArray(statesVal)) return;
	const out: unknown[] = [];
	for (const [stateName, stateBody] of Object.entries(statesVal as Record<string, unknown>)) {
		if (!stateBody || typeof stateBody !== "object") continue;
		const sb = { ...(stateBody as Record<string, unknown>) };
		const legacyOn = sb.on;
		if (legacyOn !== undefined && legacyOn !== null && typeof legacyOn === "object" && !Array.isArray(legacyOn)) {
			const handlers: unknown[] = [];
			for (const [event, rawTr] of Object.entries(legacyOn as Record<string, unknown>)) {
				const transitions = (Array.isArray(rawTr) ? rawTr : [rawTr]).filter((x) => x !== null && typeof x === "object");
				if (transitions.length > 0) {
					handlers.push({ event, transitions });
				}
			}
			sb.on = handlers;
		}
		out.push({ name: stateName, ...sb });
	}
	m.states = out;

	const g = r.guards;
	if (g !== undefined && g !== null && typeof g === "object" && !Array.isArray(g)) {
		r.guards = Object.entries(g as Record<string, unknown>).map(([name, expr]) => ({ name, expr }));
	}

	normalizeLegacyDisplay(r);
	migrateLegacyMachineTransitionActions(m);
}

/** @emoji 🧾 Validates and returns a `CommandSpec` or `null` when malformed. */
export function parseCommandSpec(raw: unknown): CommandSpec | null {
	if (!raw || typeof raw !== "object") return null;
	const r = structuredClone(raw) as Record<string, unknown>;
	if (r.schema !== "spatial.command/v1") return null;
	normalizeLegacyCommandDocument(r);
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
	if (typeof o.kind !== "string") return null;
	const spec = r as unknown as CommandSpec;
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

function lookupGuard(spec: CommandSpec, name: string): Expr | undefined {
	return spec.guards?.find((g) => g.name === name)?.expr;
}

/** @emoji 🧮 Serializes `KernelQueryParams` into the loose record shape expected by `KernelAdapter.query`. */
function kernelQueryParamsToRecord(p: KernelQueryParams, env: ExprEnv): Record<string, unknown> {
	if (p.kind === "surface.resolveFaces") {
		return { surfaceId: evalExpr(p.surfaceId, env) };
	}
	return {};
}

function applyBoxGeometryTransform(ctx: Record<string, unknown>, event: CommandEvent, transform: ActionSpec & { op: "box.transform" }): void {
	const pt = (event as { point?: unknown }).point;
	const P = isVec3(pt) ? pt : null;
	const val = (event as { value?: unknown }).value;
	const op = transform.transform;
	if (op === "aabbFromDiagonalCorners") {
		const a = ctx.diagA;
		if (!isVec3(a) || !P) return;
		const z = a[2];
		ctx.origin = [Math.min(a[0], P[0]), Math.min(a[1], P[1]), z] as unknown as Vec3;
		ctx.corner = [Math.max(a[0], P[0]), Math.max(a[1], P[1]), z] as unknown as Vec3;
		delete ctx.diagA;
		return;
	}
	if (op === "tripletRubber") {
		const p0 = ctx.p0;
		const p1 = ctx.p1;
		if (!isVec3(p0) || !isVec3(p1) || !P) return;
		const z = p0[2];
		ctx.previewA = [Math.min(p0[0], p1[0], P[0]), Math.min(p0[1], p1[1], P[1]), z] as unknown as Vec3;
		ctx.previewB = [Math.max(p0[0], p1[0], P[0]), Math.max(p0[1], p1[1], P[1]), z] as unknown as Vec3;
		return;
	}
	if (op === "tripletCommit") {
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
	if (op === "snapSquareFootprint") {
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
	if (op === "setCubeHeightFromFootprint") {
		const o = ctx.origin;
		const c = ctx.corner;
		if (!isVec3(o) || !isVec3(c)) return;
		const dx = Math.abs(c[0] - o[0]);
		const dy = Math.abs(c[1] - o[1]);
		ctx.height = Math.max(dx, dy, 0.01);
		return;
	}
	if (op === "rubberCornerFromCenter") {
		const c = ctx.rectCenter;
		if (!isVec3(c) || !P) return;
		ctx.origin = [Math.min(2 * c[0] - P[0], P[0]), Math.min(2 * c[1] - P[1], P[1]), c[2]] as unknown as Vec3;
		ctx.corner = [Math.max(2 * c[0] - P[0], P[0]), Math.max(2 * c[1] - P[1], P[1]), c[2]] as unknown as Vec3;
		return;
	}
	if (op === "rubberSquareFromCenter") {
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
	if (op === "verticalFinalizeFootprint") {
		const o = ctx.origin;
		const pk = ctx.peak;
		if (!isVec3(o) || !isVec3(pk) || !P) return;
		ctx.corner = [P[0], P[1], o[2]] as unknown as Vec3;
		ctx.height = Math.max(0.01, Math.abs(pk[2] - o[2]));
		delete ctx.peak;
		return;
	}
	if (op === "initPeakAboveOrigin") {
		const o = ctx.origin;
		if (!isVec3(o)) return;
		ctx.peak = [o[0], o[1], o[2] + 0.25] as unknown as Vec3;
		return;
	}
	if (op === "peakFromOriginZ") {
		const o = ctx.origin;
		if (!isVec3(o) || !P) return;
		ctx.peak = [o[0], o[1], P[2]] as unknown as Vec3;
		return;
	}
	if (op === "verticalRubberCorner") {
		const o = ctx.origin;
		if (!isVec3(o) || !P) return;
		ctx.corner = [P[0], P[1], o[2]] as unknown as Vec3;
		return;
	}
	if (op === "cornerFromLengthWidth") {
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

/** @emoji 🎬 Applies one declarative action (async kernel queries + `box.transform`). */
export async function applyActionAsync(
	a: ActionSpec,
	ctx: Record<string, unknown>,
	event: CommandEvent,
	kernel?: KernelAdapter,
): Promise<void> {
	const env: ExprEnv = { context: ctx, event };
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
	} else if (a.op === "box.transform") {
		applyBoxGeometryTransform(ctx, event, a);
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
		for (const act of tr.actions ?? []) {
			await applyActionAsync(act, context, event, kernel);
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
const HOST_KEYBIND_EXCLUDED_KINDS = new Set(["pointer.move", "pointer.down", "selection.changed"]);

export interface CommandKeybindRow {
	readonly eventKind: string;
	readonly key: string;
	readonly label: string;
}

/** @emoji ⌨️ Lists keyed transitions for the active state (excludes pointer + selection). */
export function listKeyedCommandTransitions(spec: CommandSpec, state: string): readonly CommandKeybindRow[] {
	const st = findState(spec, state);
	if (!st?.on) return [];
	const out: CommandKeybindRow[] = [];
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
		const env: ExprEnv = { context: ctx };
		const k = this.opts.kernel;
		const topo = this.opts.document.topology;
		let diff: TopologyDiff = EMPTY_TOPOLOGY_DIFF;
		let data: unknown = null;
		try {
			if (op.kind === "cell.createBox") {
				const cornerA = evalExpr(op.cornerA, env) as Vec3;
				const cornerB = evalExpr(op.cornerB, env) as Vec3;
				const height = Number(evalExpr(op.height, env));
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
					wireId: String(evalExpr(op.wireId, env)),
					distance: Number(evalExpr(op.distance, env)),
					direction: evalExpr(op.direction, env) as Vec3,
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
				const faceIdsRaw = evalExpr(op.faceIds, env);
				const faceIds = Array.isArray(faceIdsRaw) ? (faceIdsRaw as unknown[]).map(String) : [];
				diff =
					(await k.offsetFacesDiff?.({ faceIds, distance: Number(evalExpr(op.distance, env)) }))?.diff ?? EMPTY_TOPOLOGY_DIFF;
			} else if (op.kind === "measure.distance") {
				const a = evalExpr(op.a, env) as VertexRef;
				const b = evalExpr(op.b, env) as VertexRef;
				if (!k.vertexDistance) throw new Error("kernel.vertexDistance required");
				data = await k.vertexDistance(a, b, topo);
			} else if (op.kind === "measure.area") {
				const fid = evalExpr(op.faceId, env) as FaceRef;
				if (!k.faceArea) throw new Error("kernel.faceArea required");
				data = await k.faceArea(fid, topo);
			} else if (op.kind === "measure.volume") {
				const cid = evalExpr(op.cellId, env) as CellRef;
				data = await (k.cellVolume?.(cid) ?? k.volume(cid));
			}
		} catch (e) {
			const msg = e instanceof Error ? e.message : String(e);
			return fail("command.commitFailed", msg);
		}
		const outPath = this.spec.commit.outputDataPath;
		if (outPath) {
			const ctx2 = this.sm.getContext();
			writePathTarget(outPath, { context: ctx2, event: undefined }, data);
			data = readPathTarget(outPath, { context: ctx2, event: undefined }) ?? data;
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

// #region 📦Commands
/** @emoji 📦 Parses canonical box fixture (`spatial/fixtures/box.command.json`). */
export function buildBoxCommandSpec(): CommandSpec {
	const s = parseCommandSpec(boxCommandJson);
	if (!s) throw new Error("spatial/fixtures/box.command.json invalid");
	return s;
}

/** @emoji 📦 Parses extrude-wire fixture (`spatial/fixtures/extrude-wire.command.json`). */
export function buildExtrudeCommandSpec(): CommandSpec {
	const s = parseCommandSpec(extrudeWireCommandJson);
	if (!s) throw new Error("spatial/fixtures/extrude-wire.command.json invalid");
	return s;
}

/** @emoji 📦 Parses offset-surface fixture (`spatial/fixtures/offset-surface.command.json`). */
export function buildOffsetSurfaceCommandSpec(): CommandSpec {
	const s = parseCommandSpec(offsetSurfaceCommandJson);
	if (!s) throw new Error("spatial/fixtures/offset-surface.command.json invalid");
	return s;
}

/** @emoji 📦 Parses distance fixture (`spatial/fixtures/distance.command.json`). */
export function buildDistanceCommandSpec(): CommandSpec {
	const s = parseCommandSpec(distanceCommandJson);
	if (!s) throw new Error("spatial/fixtures/distance.command.json invalid");
	return s;
}

/** @emoji 📦 Parses area fixture (`spatial/fixtures/area.command.json`). */
export function buildAreaCommandSpec(): CommandSpec {
	const s = parseCommandSpec(areaCommandJson);
	if (!s) throw new Error("spatial/fixtures/area.command.json invalid");
	return s;
}

/** @emoji 📚 Host-facing command preset row (`spatial/fixtures/*.command.json`). */
export interface SpatialCommandPreset {
	readonly id: string;
	readonly label: string;
	/** @emoji ⌨️ Single-stroke host command key; must stay unique among presets (see `resolveSpatialCommandPresetKey`). */
	readonly key: string;
}

/** @emoji 📚 Built-in command preset ids for host command surfaces (`spatial/fixtures/*.command.json`). */
export function listSpatialCommandPresets(): readonly SpatialCommandPreset[] {
	return [
		{ id: "primitive.box", label: "Box", key: "q" },
		{ id: "feature.extrudeWire", label: "Extrude wire", key: "j" },
		{ id: "feature.offsetSurface", label: "Offset surface", key: "k" },
		{ id: "measure.distance", label: "Distance", key: "d" },
		{ id: "measure.area", label: "Area", key: "a" },
	];
}

/** @emoji 🧭 Resolves a typed token to a preset (`key`, `id`, or compact `label`). */
export function resolveSpatialCommandPresetKey(token: string): SpatialCommandPreset | null {
	const t = token.trim().toLowerCase();
	if (!t) return null;
	for (const p of listSpatialCommandPresets()) {
		if (p.key.toLowerCase() === t) return p;
		if (p.id.toLowerCase() === t) return p;
		const slug = p.label.toLowerCase().replace(/\s+/g, "");
		if (slug === t) return p;
	}
	return null;
}

/** @emoji 📚 Loads a built-in command preset by stable `id` (see `listSpatialCommandPresets`). */
export function loadSpatialCommandPreset(presetId: string): CommandSpec | null {
	const raw =
		presetId === "primitive.box"
			? boxCommandJson
			: presetId === "feature.extrudeWire"
				? extrudeWireCommandJson
				: presetId === "feature.offsetSurface"
					? offsetSurfaceCommandJson
					: presetId === "measure.distance"
						? distanceCommandJson
						: presetId === "measure.area"
							? areaCommandJson
							: null;
	if (!raw) return null;
	return parseCommandSpec(raw as unknown);
}
// #endregion 📦Commands

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

	describe("@spatial/js-core command presets", () => {
		it("lists stable keys for each built-in command preset", () => {
			const ps = listSpatialCommandPresets();
			expect(ps.map((p) => p.key).join("")).toBe("qjkda");
			expect(new Set(ps.map((p) => p.key)).size).toBe(ps.length);
		});
		it("resolves command preset tokens by key, id, and label slug", () => {
			expect(resolveSpatialCommandPresetKey("q")?.id).toBe("primitive.box");
			expect(resolveSpatialCommandPresetKey("primitive.box")?.key).toBe("q");
			expect(resolveSpatialCommandPresetKey("extrudewire")?.id).toBe("feature.extrudeWire");
			expect(resolveSpatialCommandPresetKey("d")?.id).toBe("measure.distance");
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
	describe("@spatial/js-core command box", () => {
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
			await rt.send({ kind: "set.height", value: 4, modifiers: {} });
			const res = await rt.commit();
			expect(res.ok).toBe(true);
			expect(res.data).toBeNull();
			expect(Object.keys(topo.faces).length).toBeGreaterThan(0);
			expect(kernel.lastBox).toEqual({
				cornerA: [0, 0, 0],
				cornerB: [2, 3, 0],
				height: 4,
			});
		});
	});

	describe("@spatial/js-core stateEngine option", () => {
		it("explicit pure-ts provider matches default command snapshots", async () => {
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
			const spec = buildDistanceCommandSpec();
			const rt = createCommandRuntime(spec, { kernel: new MeasKernel(), document: { topology: topo, nodes: [] } });
			await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: va, editable: true }] });
			await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: vb, editable: true }] });
			const res = await rt.commit();
			expect(res.ok).toBe(true);
			expect(res.data).toBe(5);
			expect(isEmptyTopologyDiff(res.diff)).toBe(true);
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
			const spec = buildAreaCommandSpec();
			const rt = createCommandRuntime(spec, { kernel: new AreaKernel(), document: { topology: topo, nodes: [] } });
			await rt.send({ kind: "selection.changed", targets: [{ kind: "face", id: "f0", editable: true }] });
			const res = await rt.commit();
			expect(res.ok).toBe(true);
			expect(res.data).toBe(2.5);
			expect(isEmptyTopologyDiff(res.diff)).toBe(true);
		});
	});
}
// #endregion 🧪Tests

