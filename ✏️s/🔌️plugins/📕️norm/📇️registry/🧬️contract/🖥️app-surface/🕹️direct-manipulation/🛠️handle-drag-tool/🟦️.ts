/** 🛠️ Handle drag statechart, TypeScript twin of `./🦀️.rs`: `Idle -> Hover -> Armed -> Dragging -> Commit | Abort`; pure, so pointer, keyboard and peers replay the same events. A keyboard nudge moves the parameter by the schema step of the bound field, `Enter` on a focused grip opens its value field (the typed entry starts with the shown value) and a second `Enter` applies it.
 *
 * 🧫️ Replayed on `🧪️tests/🧫️fixtures/🎯️grip-keyboard/🔣️.json` (`statechart`) by both implementations; handle types are the generated schema twin. */
import type { FieldBinding, Handle, Quantise } from "../../../../../🎬️structural-scene/🧬️schema/🟦️.ts";
import { nudgeV1, roundHalfAwayV1, snapV1, type ModifiersV1 } from "../🧲️snap/🟦️.ts";

/** 📍️ Where a handle gesture stands; `s` is the drag parameter in handle units. */
export type DragStateV1 =
  | Readonly<{ kind: "idle" }>
  | Readonly<{ kind: "hover"; handle: string; s: number }>
  | Readonly<{ kind: "armed"; handle: string; origin: number }>
  | Readonly<{ kind: "dragging"; handle: string; origin: number; s: number; entry: string }>;

/** 🛑️ Why a gesture ended without committing. */
export type AbortReasonV1 = "escape" | "captureLost" | "blur" | "retired";

/** 🎬️ One input of the statechart, from the pointer, the keyboard or the host phase vocabulary. */
export type DragEventV1 =
  | Readonly<{ kind: "hover"; handle: string | null; s: number }>
  | Readonly<{ kind: "press"; handle: string; s: number }>
  | Readonly<{ kind: "move"; s: number; modifiers: ModifiersV1 }>
  | Readonly<{ kind: "nudge"; direction: number; modifiers: ModifiersV1 }>
  | Readonly<{ kind: "entry"; char: string }>
  | Readonly<{ kind: "entryBackspace" }>
  | Readonly<{ kind: "commit" }>
  | Readonly<{ kind: "release" }>
  | Readonly<{ kind: "abort"; reason: AbortReasonV1 }>;

/** ✍️ One field value a handle writes: a number, or the enum token of a token binding. */
export type FieldWriteV1 = Readonly<{ path: string; value: number | string }>;

/** 📤️ What the host must do after one step. */
export type DragEffectV1 =
  | Readonly<{ kind: "none" }>
  | Readonly<{ kind: "preview"; handle: string; writes: readonly FieldWriteV1[]; origin: number; s: number; entry: string }>
  | Readonly<{ kind: "commit"; writes: readonly FieldWriteV1[] }>
  | Readonly<{ kind: "cleared" }>;

const NONE: DragEffectV1 = { kind: "none" };
const CLEARED: DragEffectV1 = { kind: "cleared" };
const IDLE: DragStateV1 = { kind: "idle" };

/** 🔢️ Applies a quantisation rule to a mapped field value. */
export function quantiseApplyV1(quantise: Quantise, value: number): number {
  switch (quantise.kind) {
    case "round":
      return roundHalfAwayV1(value / quantise.data.step) * quantise.data.step;
    case "choice": {
      let best: number | undefined;
      for (const candidate of quantise.data.values) if (best === undefined || Math.abs(candidate - value) < Math.abs(best - value)) best = candidate;
      return best ?? value;
    }
    case "int":
      return roundHalfAwayV1(value);
    case "decades":
      return 10 ** (roundHalfAwayV1(value * quantise.data.perDecade) / quantise.data.perDecade);
    case "token":
      return Math.min(Math.max(roundHalfAwayV1(value), 0), Math.max(quantise.data.tokens.length - 1, 0));
  }
}

/** 🎯️ The field value a binding produces for drag parameter `s`. */
export function valueAtV1(binding: FieldBinding, s: number): number {
  const value = binding.map.base + binding.map.k * s;
  return binding.quantise === null ? value : quantiseApplyV1(binding.quantise, value);
}

/** 🔤️ The enum token a token binding produces for `s`; `undefined` for a numeric binding. */
export function tokenAtV1(binding: FieldBinding, s: number): string | undefined {
  if (binding.quantise === null || binding.quantise.kind !== "token") return undefined;
  return binding.quantise.data.tokens[quantiseApplyV1(binding.quantise, binding.map.base + binding.map.k * s)]?.token;
}

/** 🔁️ The drag parameter whose affine image is the field value `field`. */
export const invertAffineV1 = (binding: FieldBinding, field: number): number => (field - binding.map.base) / binding.map.k;

const NUMBER_ENTRY = /^[+-]?(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?$/u;

/** 🔢️ Parses typed numeric entry with `.` or `,` as decimal separator. */
export function parseEntryV1(entry: string): number | undefined {
  const text = entry.trim().replaceAll(",", ".");
  if (!NUMBER_ENTRY.test(text)) return undefined;
  const value = Number(text);
  return Number.isFinite(value) ? value : undefined;
}

const plainNumberText = (value: number): string => {
  const text = String(value);
  return text.includes("e") ? value.toLocaleString("en-US", { useGrouping: false, maximumFractionDigits: 20 }) : text;
};

/** 🔢️ The text a freshly opened numeric field starts with: the bound field value at `s` to twelve significant digits; `undefined` for token fields, which are chosen by stepping and not typed. */
export function entryFieldTextV1(handle: Handle, s: number): string | undefined {
  const binding = handle.writes[0];
  if (binding === undefined || binding.quantise?.kind === "token") return undefined;
  const rounded = Number(valueAtV1(binding, s).toExponential(11));
  return Number.isFinite(rounded) ? plainNumberText(rounded) : undefined;
}

/** ✍️ Every bound field value for parameter `s` as a provisional write, quantised but not re-snapped. */
export const writesForV1 = (handle: Handle, s: number): readonly FieldWriteV1[] => handle.writes.map((binding) => ({ path: binding.path, value: tokenAtV1(binding, s) ?? valueAtV1(binding, s) }));

const sameWrites = (left: readonly FieldWriteV1[], right: readonly FieldWriteV1[]): boolean => left.length === right.length && left.every((write, index) => write.path === right[index]!.path && write.value === right[index]!.value);

const isEntryChar = (char: string): boolean => /^[0-9\-.,]$/u.test(char);

/** 🛠️ The statechart over the handles of the current selection. */
export class HandleDragToolV1 {
  constructor(readonly handles: readonly Handle[]) {}

  private find(id: string): Handle | undefined {
    return this.handles.find((handle) => handle.id === id);
  }

  private dragging(handle: Handle, origin: number, s: number, entry: string): readonly [DragStateV1, DragEffectV1] {
    return [{ kind: "dragging", handle: handle.id, origin, s, entry }, { kind: "preview", handle: handle.id, writes: writesForV1(handle, s), origin, s, entry }];
  }

  private settle(handle: Handle, origin: number, s: number): readonly [DragStateV1, DragEffectV1] {
    const writes = writesForV1(handle, s);
    return [{ kind: "hover", handle: handle.id, s }, sameWrites(writes, writesForV1(handle, origin)) ? CLEARED : { kind: "commit", writes }];
  }

  private typed(handle: Handle, origin: number, s: number, entry: string): readonly [DragStateV1, DragEffectV1] {
    const first = handle.writes[0];
    const value = parseEntryV1(entry);
    let exact: number | undefined;
    if (value !== undefined && first !== undefined) {
      const mapped = invertAffineV1(first, value);
      exact = handle.limits === null ? mapped : Math.min(Math.max(mapped, handle.limits.min), handle.limits.max);
    }
    return this.dragging(handle, origin, exact ?? s, entry);
  }

  /** 🎬️ One transition. Unknown handles and events that do not apply keep the state; a missing handle ends the gesture without trace. */
  step(state: DragStateV1, event: DragEventV1): readonly [DragStateV1, DragEffectV1] {
    if (event.kind === "abort") {
      if (state.kind === "dragging") return [event.reason === "escape" ? { kind: "hover", handle: state.handle, s: state.origin } : IDLE, CLEARED];
      if (state.kind === "armed" || (state.kind === "hover" && event.reason === "retired")) return [IDLE, NONE];
      return [state, NONE];
    }
    if (state.kind === "idle" || state.kind === "hover") {
      if (event.kind === "hover") {
        if (event.handle === null) return [IDLE, NONE];
        return this.find(event.handle) === undefined ? [state, NONE] : [{ kind: "hover", handle: event.handle, s: event.s }, NONE];
      }
      if (event.kind === "press" && this.find(event.handle) !== undefined) return [{ kind: "armed", handle: event.handle, origin: event.s }, NONE];
    }
    if (state.kind === "armed") {
      if (event.kind === "move") {
        const handle = this.find(state.handle);
        return handle === undefined ? [IDLE, NONE] : this.dragging(handle, state.origin, snapV1(handle, event.s, event.modifiers).s, "");
      }
      if (event.kind === "release") return [{ kind: "hover", handle: state.handle, s: state.origin }, NONE];
    }
    if (state.kind === "hover") {
      const handle = this.find(state.handle);
      if (event.kind === "nudge") return handle === undefined ? [IDLE, NONE] : this.dragging(handle, state.s, nudgeV1(handle, state.s, event.direction, event.modifiers).s, "");
      if (event.kind === "commit") {
        const text = handle === undefined ? undefined : entryFieldTextV1(handle, state.s);
        return handle !== undefined && text !== undefined ? this.typed(handle, state.s, state.s, text) : [state, NONE];
      }
      if (event.kind === "entry" && isEntryChar(event.char)) return handle === undefined ? [IDLE, NONE] : this.typed(handle, state.s, state.s, event.char);
    }
    if (state.kind === "dragging") {
      const handle = this.find(state.handle);
      if (event.kind === "move" && state.entry === "") return handle === undefined ? [IDLE, CLEARED] : this.dragging(handle, state.origin, snapV1(handle, event.s, event.modifiers).s, state.entry);
      if (event.kind === "nudge") return handle === undefined ? [IDLE, CLEARED] : this.dragging(handle, state.origin, nudgeV1(handle, state.s, event.direction, event.modifiers).s, "");
      if (event.kind === "entry" && isEntryChar(event.char)) return handle === undefined ? [IDLE, CLEARED] : this.typed(handle, state.origin, state.s, state.entry + event.char);
      if (event.kind === "entryBackspace") return handle === undefined ? [IDLE, CLEARED] : this.typed(handle, state.origin, state.s, state.entry.slice(0, -1));
      if (event.kind === "commit" || event.kind === "release") return handle === undefined ? [IDLE, CLEARED] : this.settle(handle, state.origin, state.s);
    }
    return [state, NONE];
  }
}
