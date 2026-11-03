/** 🧲️ Snapping of the drag parameter and the keyboard nudge step, TypeScript twin of `./🦀️.rs`: explains which rule moved `s` and derives the schema step of a nudge. The handle types are the generated schema twin (`🎬️structural-scene/🧬️schema/🕹️handles/🟦️.ts`).
 *
 * 🧫️ Replayed on `🧪️tests/🧫️fixtures/🎯️grip-keyboard/🔣️.json` (`nudges`, `statechart`) by both implementations. */
import type { Handle, Quantise } from "../../../../../🎬️structural-scene/🧬️schema/🟦️.ts";

/** ⌨️ Modifier keys of one pointer or key event; `ctrl` frees the drag from snapping, `shift` multiplies and `alt` divides a nudge. */
export type ModifiersV1 = Readonly<{ shift: boolean; alt: boolean; ctrl: boolean }>;

/** 🚫️ No modifier. */
export const NO_MODIFIERS_V1: ModifiersV1 = { shift: false, alt: false, ctrl: false };

/** ✖️ Nudge multiplier: Shift x10, Alt x0.1, both cancel. */
export const nudgeScaleV1 = (modifiers: ModifiersV1): number => (modifiers.shift && !modifiers.alt ? 10 : !modifiers.shift && modifiers.alt ? 0.1 : 1);

/** 🏷️ The rule that decided a snapped value. */
export type SnapSourceV1 = "free" | "target" | "catalogue" | "increment" | "grid" | "limit";

/** 🧲️ A drag parameter after limits and snapping, with the rule that produced it. */
export type SnappedV1 = Readonly<{ s: number; source: SnapSourceV1 }>;

/** 🔢️ Rounds half away from zero like Rust `f64::round` (JavaScript `Math.round` rounds half up). */
export const roundHalfAwayV1 = (value: number): number => (value < 0 ? -Math.round(-value) : Math.round(value));

const signumV1 = (value: number): number => (value < 0 ? -1 : 1);

const clampTo = (handle: Handle, value: number): number => (handle.limits === null ? value : Math.min(Math.max(value, handle.limits.min), handle.limits.max));
const outside = (handle: Handle, value: number): boolean => handle.limits !== null && !(value >= handle.limits.min && value <= handle.limits.max);

const nearest = (values: readonly number[], s: number, tolerance: number): number | undefined => {
  let best: number | undefined;
  for (const value of values) if (Math.abs(value - s) <= tolerance && (best === undefined || Math.abs(value - s) < Math.abs(best - s))) best = value;
  return best;
};

/** 🧲️ Limits first, then the nearest target or catalogue value, else increment or grid; `ctrl` keeps only the limits. */
export function snapV1(handle: Handle, raw: number, modifiers: ModifiersV1): SnappedV1 {
  const bounded = clampTo(handle, raw);
  const rule = handle.snap;
  let moved = bounded;
  let source: SnapSourceV1 = "free";
  if (!modifiers.ctrl) {
    const hit = nearest([...rule.targets, ...rule.catalogue], bounded, rule.tolerance);
    if (hit !== undefined) {
      moved = hit;
      source = rule.targets.includes(hit) ? "target" : "catalogue";
    } else if (rule.increment !== null) {
      moved = roundHalfAwayV1(bounded / rule.increment) * rule.increment;
      source = "increment";
    } else if (rule.grid !== null) {
      moved = roundHalfAwayV1(bounded / rule.grid) * rule.grid;
      source = "grid";
    }
  }
  return { s: clampTo(handle, moved), source: outside(handle, raw) || outside(handle, moved) ? "limit" : source };
}

const quantiseStep = (quantise: Quantise): number | undefined => {
  switch (quantise.kind) {
    case "round":
      return quantise.data.step;
    case "int":
      return 1;
    case "choice": {
      const sorted = [...quantise.data.values].sort((left, right) => left - right);
      const gaps = sorted.slice(1).map((value, index) => value - sorted[index]!).filter((gap) => gap > 0);
      return gaps.length === 0 ? undefined : Math.min(...gaps);
    }
    case "decades":
      return 1 / quantise.data.perDecade;
    case "token":
      return undefined;
  }
};

/** 📐️ The schema step of the first bound field in handle units: its quantisation step divided by the slope of the affine map; `undefined` for token choices and unquantised fields. */
export function schemaStepV1(handle: Handle): number | undefined {
  const binding = handle.writes[0];
  if (binding === undefined || binding.quantise === null) return undefined;
  const k = Math.abs(binding.map.k);
  const step = quantiseStep(binding.quantise);
  return step !== undefined && Number.isFinite(step) && step > 0 && k > 0 ? step / k : undefined;
}

/** 👣️ The base nudge step in handle units: the schema step of the bound field, else the increment, else the grid, else one percent of the limit span, else 1; Shift multiplies by ten, Alt divides by ten. */
export function nudgeStepV1(handle: Handle, modifiers: ModifiersV1): number {
  const span = handle.limits === null ? undefined : (handle.limits.max - handle.limits.min) / 100;
  const base = schemaStepV1(handle) ?? handle.snap.increment ?? handle.snap.grid ?? (span !== undefined && span > 0 ? span : undefined) ?? 1;
  return base * nudgeScaleV1(modifiers);
}

const euclid = (value: number, modulus: number): number => ((value % modulus) + modulus) % modulus;

/** ⌨️ One keyboard nudge of `s` by `direction` (+1 or -1) steps, clamped to the limits and never snapped away again; a choice stepper moves one token and cycles past either end. */
export function nudgeV1(handle: Handle, s: number, direction: number, modifiers: ModifiersV1): SnappedV1 {
  if (handle.kind.kind === "choiceStepper" && handle.limits !== null) {
    const min = roundHalfAwayV1(handle.limits.min);
    const count = roundHalfAwayV1(handle.limits.max) - min + 1;
    return { s: euclid(roundHalfAwayV1(s) - min + signumV1(direction), count) + min, source: "increment" };
  }
  const raw = s + signumV1(direction) * nudgeStepV1(handle, modifiers);
  return { s: clampTo(handle, raw), source: outside(handle, raw) ? "limit" : "free" };
}
