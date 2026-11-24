/** 🧱️ Gives the flat drawings of a person that put the three views out of step their real thickness, as repair ops.
 *
 * A front or plan shape without a `solid` is a plate without depth that its bone carries along, and a `solid` swept over a millimetre is the same plate: a swung limb is foreshortened to a sliver in the view that looks along it. For every instant the three-views sweep reports, the tool finds the shape that sets the disagreeing edge and restates the plates of that bone, in the view that lacks the extent, as prisms: the authored outline stays the profile (expressed in the two bone axes across the viewing direction at rest) and is swept along the third bone axis over the range the side elevation draws for the same part (the twin of the same id, else every side shape of the bone). A flat side-elevation shape of such a bone is swept laterally over the range its front or plan twin draws. Where the outline of a part does not lie along its bone (a shoe on a pitched foot bone), the second way is taken: the side profile of the twin swept laterally over the width of the old outline. A change is kept only when the reported edge comes closer and the sweep of the scale reports no more findings than before.
 *
 * Usage: `bun prism-3.ts <id> --file <candidate document json, rewritten in place> --ops <ops json, written>` `[--only <view>:<bone>[:side][,…]]` (restate these and stop) `[--skip <view>:<bone>[,…]]` `[--side <view>:<bone>[,…]]` (take the second way first for these) `[--one-side]` (leave the mirrored bone alone)
 * The ops are `{ op: "shape", scale, view, id, shape }`, the form both repair chains of the human wave apply.
 * @see ./sweep-2.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { restFrames } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { VIEWS, authoredScales, extentOf, queries, sweep, toleranceM } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

type View = (typeof VIEWS)[number];
type Vec = readonly [number, number, number];
type Shape = { id: string; bone: string; kind: string; role: string; points?: [number, number][]; solid?: { kind: string; sweep?: "u" | "v" | "w"; z0?: number; z1?: number; zTip?: [number, number] }; followTip?: boolean; [key: string]: unknown };
type Frame = { o: Vec; u: Vec; v: Vec; w: Vec };
type Mode = "through" | "side";

const THIN = 0.012;
const TURNED = Math.cos((15 * Math.PI) / 180);
const dot = (a: Vec, b: Vec): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const round = (n: number): number => Math.round(n * 1e5) / 1e5;
const mirrorOf = (bone: string): string | undefined => (bone.endsWith("-l") ? `${bone.slice(0, -2)}-r` : bone.endsWith("-r") ? `${bone.slice(0, -2)}-l` : undefined);

/** 🧮️ The restating of the shapes of one asset document. */
export class Restater {
  readonly asset: any;
  readonly frames: Record<string, { frame: Frame }>;
  readonly changed = new Map<string, { scale: string; view: View; id: string; shape: Shape }>();
  private readonly states = new Map<string, ReturnType<typeof evaluate>>();

  constructor(asset: any) {
    this.asset = asset;
    this.frames = restFrames(asset.rig) as never;
  }

  shapes(scale: number, view: View): Shape[] {
    return this.asset.representations[String(scale)].geometry[view];
  }

  /** 📏️ The range a side-elevation shape draws along the bone axis `u` or `v`, or undefined when the shape does not say. */
  private sideRange(shape: Shape, axis: "u" | "v"): [number, number] | undefined {
    if (shape.kind !== "poly" || shape.points === undefined) return undefined;
    const first = shape.points.map((p) => p[0]);
    const second = shape.points.map((p) => p[1]);
    const span = (values: number[]): [number, number] => [Math.min(...values), Math.max(...values)];
    if (shape.solid === undefined || (shape.solid.kind === "extruded" && shape.solid.sweep === "w")) return span(axis === "u" ? first : second);
    if (shape.solid.kind !== "extruded") return undefined;
    if (shape.solid.sweep === axis) return [shape.solid.z0!, shape.solid.z1!];
    return shape.solid.sweep === "u" ? span(first) : axis === "u" ? span(first) : undefined;
  }

  /** 📏️ The lateral range (rest position in the asset) a front or plan shape draws. */
  private lateralRange(shape: Shape, view: View, frame: Frame): [number, number] | undefined {
    if (shape.kind !== "poly" || shape.points === undefined) return undefined;
    if (shape.solid === undefined) {
      const values = shape.points.map((p) => (view === "front" ? p[0] : p[1]));
      return [Math.min(...values), Math.max(...values)];
    }
    if (shape.solid.kind !== "extruded") return undefined;
    if (shape.solid.sweep === "w") return [shape.solid.z0!, shape.solid.z1!];
    const values = shape.points.map((p) => p[1] + frame.o[2]);
    return [Math.min(...values), Math.max(...values)];
  }

  private reference<T>(scale: number, views: readonly View[], shape: Shape, read: (candidate: Shape, view: View) => T | undefined): T[] {
    for (const view of views) {
      const twins = this.shapes(scale, view).filter((candidate) => candidate.bone === shape.bone && candidate.id === shape.id);
      const found = twins.map((twin) => read(twin, view)).filter((value): value is T => value !== undefined);
      if (found.length > 0) return found;
    }
    for (const view of views) {
      const found = this.shapes(scale, view).filter((candidate) => candidate.bone === shape.bone && candidate.role !== "ground").map((candidate) => read(candidate, view)).filter((value): value is T => value !== undefined);
      if (found.length > 0) return found;
    }
    return [];
  }

  /** 🧱️ The shape restated as a prism of real thickness, or the reason why it stays. */
  restate(scale: number, view: View, shape: Shape, mode: Mode = "through"): Shape | string {
    if (shape.kind !== "poly" || shape.points === undefined) return `kind ${shape.kind}`;
    if (shape.role === "ground") return "ground";
    const frame = this.frames[shape.bone]?.frame;
    if (frame === undefined) return "no bone";
    if (mode === "side") {
      if (view === "elevation") return "is the side view";
      if (shape.solid !== undefined && shape.solid.kind !== "extruded") return "revolved";
      if (shape.solid !== undefined && shape.solid.zTip === undefined && shape.solid.z1! - shape.solid.z0! > THIN) return "is a prism";
      const twin = this.shapes(scale, "elevation").find((candidate) => candidate.bone === shape.bone && candidate.id === shape.id);
      if (twin === undefined || twin.kind !== "poly" || !(twin.solid === undefined || (twin.solid.kind === "extruded" && twin.solid.sweep === "w"))) return "no flat side twin";
      const own = this.lateralRange(shape, view, frame);
      const other = this.reference(scale, [view === "front" ? "plan" : "front"], shape, (candidate, from) => this.lateralRange(candidate, from, frame));
      const lateral = own !== undefined && own[1] - own[0] > THIN ? own : other.length > 0 ? ([Math.min(...other.map((r) => r[0])), Math.max(...other.map((r) => r[1]))] as [number, number]) : undefined;
      if (lateral === undefined || lateral[1] - lateral[0] <= THIN) return "no lateral width";
      const { followTip: _stretch, ...kept } = shape;
      return { ...kept, points: twin.points!.map((p): [number, number] => [p[0], p[1]]), closed: twin.closed, solid: { kind: "extruded", sweep: "w", z0: round(lateral[0]), z1: round(lateral[1]) } };
    }
    const merge = (ranges: [number, number][]): [number, number] => [round(Math.min(...ranges.map((r) => r[0]))), round(Math.max(...ranges.map((r) => r[1])))];
    const { followTip: _dropped, ...rest } = shape;
    if (shape.solid !== undefined) {
      if (shape.solid.kind !== "extruded") return "revolved";
      const thickness = Math.min(shape.solid.z1! - shape.solid.z0!, shape.solid.zTip === undefined ? Infinity : shape.solid.zTip[1] - shape.solid.zTip[0]);
      if (thickness > THIN && shape.solid.zTip === undefined) return "is a prism";
      if (view !== "elevation" && (shape.solid.sweep === "w" || shape.solid.zTip !== undefined)) {
        const outline = this.outlineAt(scale, view, shape, this.asset.presentation.defaultPose, {});
        return outline === undefined || outline.rings !== 1 ? "no single rest outline" : this.through(scale, view, { ...rest, closed: true }, outline.points, outline.frame);
      }
      if (shape.solid.sweep === "w") {
        const ranges = this.reference(scale, view === "elevation" ? ["front", "plan"] : ["front", "plan"].filter((other) => other !== view) as View[], shape, (candidate, other) => this.lateralRange(candidate, other, frame));
        if (ranges.length === 0) return "no lateral reference";
        const [z0, z1] = merge(ranges);
        return z1 - z0 > THIN ? { ...rest, solid: { kind: "extruded", sweep: "w", z0, z1 } } : "lateral reference is thin";
      }
      const ranges = this.reference(scale, ["elevation"], shape, (candidate) => this.sideRange(candidate, shape.solid!.sweep as "u" | "v"));
      if (ranges.length === 0) return "no side reference";
      const [z0, z1] = merge(ranges);
      return z1 - z0 > THIN ? { ...rest, solid: { kind: "extruded", sweep: shape.solid.sweep, z0, z1 } } : "side reference is thin";
    }
    if (view === "elevation") {
      const ranges = this.reference(scale, ["front", "plan"], shape, (candidate, other) => this.lateralRange(candidate, other, frame));
      if (ranges.length === 0) return "no lateral reference";
      const [z0, z1] = merge(ranges);
      return z1 - z0 > THIN ? { ...rest, solid: { kind: "extruded", sweep: "w", z0, z1 } } : "lateral reference is thin";
    }
    const posed = this.outlineAt(scale, view, shape, this.asset.presentation.defaultPose);
    if (posed !== undefined && Math.min(dot(posed.frame.u, frame.u), dot(posed.frame.v, frame.v)) < TURNED) return posed.rings === 1 ? this.through(scale, view, { ...rest, closed: true }, posed.points, posed.frame) : "no single outline in the default pose";
    return this.through(scale, view, rest, shape.points, frame);
  }

  /** 🔦️ The outline a shape draws in a view in a pose (with `bones` given: that pose with these offsets instead of its own), and the frame of its bone there. */
  private outlineAt(scale: number, view: View, shape: Shape, pose: string, bones?: object): { points: [number, number][]; frame: Frame; rings: number } | undefined {
    const probe = bones === undefined ? this.asset : { ...this.asset, poses: [...this.asset.poses, { ...this.asset.poses.find((candidate: { id: string }) => candidate.id === pose), id: "rest-probe", bones }] };
    const key = `${scale}|${bones === undefined ? pose : "rest-probe"}`;
    const state = this.states.get(key) ?? evaluate(probe, { kind: "pose", pose: bones === undefined ? pose : "rest-probe" }, { kind: "scale", scale: { denominator: scale } });
    this.states.set(key, state);
    const evaluated = state.shapes[view].filter((candidate) => candidate.shape.id === shape.id && candidate.shape.bone === shape.bone);
    if (evaluated.length !== 1 || evaluated[0]!.rings.length === 0 || evaluated[0]!.shape.solid !== shape.solid) return undefined;
    return { points: evaluated[0]!.rings[0]!.map((p): [number, number] => [p.x, p.y]), rings: evaluated[0]!.rings.length, frame: Object.fromEntries(Object.entries(state.bones[shape.bone]!.frame).map(([axis, value]) => [axis, [value.x, value.y, value.z]])) as never };
  }

  /** 🧱️ An outline of a front or plan view as the profile of a prism swept along the bone axis nearest to the viewing direction, over the range the side elevation draws. */
  private through(scale: number, view: View, rest: Shape, outline: [number, number][], frame: Frame): Shape | string {
    const shape = rest;
    const merge = (ranges: [number, number][]): [number, number] => [round(Math.min(...ranges.map((r) => r[0]))), round(Math.max(...ranges.map((r) => r[1])))];
    const direction = view === "front" ? 0 : 1;
    const along = Math.abs(frame.u[direction]) >= Math.abs(frame.v[direction]) ? "u" : "v";
    if (Math.abs(frame.w[direction]) > Math.abs(frame[along][direction])) return "bone lies across";
    const ranges = this.reference(scale, ["elevation"], shape, (candidate) => this.sideRange(candidate, along));
    if (ranges.length === 0) return "no side reference";
    const [z0, z1] = merge(ranges);
    if (z1 - z0 <= THIN) return "side reference is thin";
    const middle = (z0 + z1) / 2;
    const A = frame[along];
    const B = along === "u" ? frame.v : frame.u;
    const C = frame.w;
    const [i, j] = view === "front" ? [2, 1] : [0, 2];
    const determinant = B[i]! * C[j]! - C[i]! * B[j]!;
    if (Math.abs(determinant) < 0.5) return "outline plane is edge-on";
    const points = outline.map((p): [number, number] => {
      const a = p[0] - frame.o[i]! - middle * A[i]!;
      const b = p[1] - frame.o[j]! - middle * A[j]!;
      return [round((a * C[j]! - C[i]! * b) / determinant), round((B[i]! * b - a * B[j]!) / determinant)];
    });
    return { ...rest, points, solid: { kind: "extruded", sweep: along, z0, z1 } };
  }

  /** ✏️ Restates every plate of a bone in a view; returns what it changed and what it left. */
  restateBone(scale: number, view: View, bone: string, mode: Mode = "through"): { done: string[]; left: string[]; undo: () => void } {
    const list = this.shapes(scale, view);
    const before = list.slice();
    const keys = new Map(this.changed);
    const done: string[] = [];
    const left: string[] = [];
    list.forEach((shape, index) => {
      if (shape.bone !== bone) return;
      const result = this.restate(scale, view, shape, mode);
      if (typeof result === "string") { left.push(`${shape.id} (${result})`); return; }
      if (list.filter((other) => other.id === shape.id).length !== 1) { left.push(`${shape.id} (id not unique)`); return; }
      list[index] = result;
      this.changed.set(`${scale}|${view}|${shape.id}`, { scale: String(scale), view, id: shape.id, shape: result });
      done.push(shape.id);
    });
    return { done, left, undo: () => { list.splice(0, list.length, ...before); this.changed.clear(); for (const [key, value] of keys) this.changed.set(key, value); } };
  }

  ops(): object[] {
    return [...this.changed.values()].map((entry) => ({ op: "shape", ...entry }));
  }
}

type Edge = { readonly check: string; readonly gap: number; readonly lacking: View; readonly exceeding: View; readonly bone: string; readonly shape: string; readonly name: string };

/** 🔎️ The disagreeing edges of every instant of a scale, widest first. */
export function edges(asset: any, scale: number): Edge[] {
  const tolerance = toleranceM(asset);
  const found: Edge[] = [];
  for (const { name, query } of queries(asset)) {
    let state: ReturnType<typeof evaluate>;
    try { state = evaluate(asset, query, { kind: "scale", scale: { denominator: scale } }); } catch { continue; }
    if (!Object.values(state.bones).some((bone) => Math.abs(bone.offset.abductionDeg) > 1e-9 || Math.abs(bone.offset.twistDeg) > 1e-9)) continue;
    const edge = (view: View, pick: "minX" | "maxX" | "minY" | "maxY"): { value: number; bone: string; shape: string } => {
      const sign = pick.startsWith("min") ? -1 : 1;
      let best = { value: -Infinity, bone: "", shape: "" };
      for (const evaluated of state.shapes[view]) {
        const extent = extentOf([evaluated]);
        if (extent !== undefined && sign * extent[pick] > best.value) best = { value: sign * extent[pick], bone: evaluated.shape.bone, shape: evaluated.shape.id };
      }
      return { ...best, value: sign * best.value };
    };
    const compare = (check: string, a: View, pa: "X" | "Y", b: View, pb: "X" | "Y"): void => {
      for (const side of ["min", "max"] as const) {
        const ea = edge(a, `${side}${pa}`);
        const eb = edge(b, `${side}${pb}`);
        const gap = Math.abs(ea.value - eb.value);
        if (!(gap > tolerance)) continue;
        const aExceeds = side === "min" ? ea.value < eb.value : ea.value > eb.value;
        const outer = aExceeds ? ea : eb;
        found.push({ check, gap, lacking: aExceeds ? b : a, exceeding: aExceeds ? a : b, bone: outer.bone, shape: outer.shape, name });
      }
    };
    compare("x", "elevation", "X", "plan", "X");
    compare("y", "elevation", "Y", "front", "Y");
    compare("z", "front", "X", "plan", "Y");
  }
  return found.sort((a, b) => b.gap - a.gap);
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  const option = (name: string): string | undefined => (args.indexOf(name) < 0 ? undefined : args[args.indexOf(name) + 1]);
  const file = option("--file")!;
  const document = JSON.parse(readFileSync(file, "utf8"));
  const restater = new Restater(document.asset);
  const asset = document.asset;
  const pairs = (text: string | undefined): [View, string, string?][] => (text ?? "").split(",").filter((entry) => entry.length > 0).map((entry) => entry.split(":") as [View, string, string?]);
  const skip = new Set(pairs(option("--skip")).map(([view, bone]) => `${view}:${bone}`));
  const only = pairs(option("--only"));
  const sideFirst = new Set(pairs(option("--side")).map(([view, bone]) => `${view}:${bone}`));
  const both = (scale: number, view: View, bone: string, mode: Mode = "through") => [bone, args.includes("--one-side") ? undefined : mirrorOf(bone)].filter((b): b is string => b !== undefined && restater.frames[b] !== undefined).map((b) => restater.restateBone(scale, view, b, mode));
  for (const scale of authoredScales(asset).filter((s) => option("--scale") === undefined || s === Number(option("--scale")))) {
    if (only.length > 0) {
      for (const [view, bone, mode] of only) for (const result of both(scale, view, bone, (mode as Mode | undefined) ?? "through")) console.log(`[DEBUG] 1:${scale} ${view} ${bone}: restated ${result.done.join(" ") || "nothing"}${result.left.length > 0 ? `; left ${result.left.join(", ")}` : ""}`);
      continue;
    }
    const refused = new Set<string>();
    for (let round = 0; round < 80; round++) {
      const count = sweep(asset, scale).length;
      const open = edges(asset, scale);
      if (count === 0) break;
      let moved = false;
      for (const target of open) {
        const order = (view: View): Mode[] => (sideFirst.has(`${view}:${target.bone}`) ? ["side", "through"] : ["through", "side"]);
        for (const [view, mode] of [...order(target.lacking).map((mode) => [target.lacking, mode] as const), ...order(target.exceeding).map((mode) => [target.exceeding, mode] as const)]) {
          const key = `${view}:${target.bone}:${mode}`;
          const plain = `${view}:${target.bone}`;
          if (refused.has(key) || skip.has(plain) || (mirrorOf(target.bone) !== undefined && skip.has(`${view}:${mirrorOf(target.bone)}`))) continue;
          const results = both(scale, view, target.bone, mode);
          const done = results.flatMap((result) => result.done);
          if (done.length === 0) { refused.add(key); console.log(`[DEBUG] 1:${scale} ${key}: nothing to restate (${results.flatMap((result) => result.left).join(", ")})`); continue; }
          const after = sweep(asset, scale).length;
          const gapAfter = edges(asset, scale).filter((edge) => edge.name === target.name && edge.check === target.check).reduce((worst, edge) => Math.max(worst, edge.gap), 0);
          if (after > count || (after === count && !(gapAfter < target.gap - 0.001))) {
            for (const result of results.reverse()) result.undo();
            refused.add(key);
            console.log(`[DEBUG] 1:${scale} ${key}: refused (${target.name}, ${target.check}: findings ${count} -> ${after}, gap ${target.gap.toFixed(3)} -> ${gapAfter.toFixed(3)})`);
            continue;
          }
          console.log(`[DEBUG] 1:${scale} ${key}: restated ${done.join(" ")} (${target.name}, ${target.check}: findings ${count} -> ${after}, gap ${target.gap.toFixed(3)} -> ${gapAfter.toFixed(3)})`);
          moved = true;
          break;
        }
        if (moved) break;
      }
      if (!moved) { console.log(`[DEBUG] 1:${scale}: stuck with ${count} findings; widest: ${open.slice(0, 4).map((edge) => `${edge.name} ${edge.check} ${edge.exceeding}>${edge.lacking} ${edge.bone}/${edge.shape} ${edge.gap.toFixed(3)}`).join(" | ")}`); break; }
    }
    console.log(`1:${scale}: sweep ${sweep(asset, scale).length}`);
  }
  writeFileSync(file, JSON.stringify(document, null, 2) + "\n");
  writeFileSync(option("--ops")!, `[\n${restater.ops().map((op) => JSON.stringify(op)).join(",\n")}\n]\n`);
  console.log(`${asset.id}: ${restater.ops().length} shapes restated`);
}
