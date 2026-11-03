/** ⌨️ Keyboard equivalents of every pointer gesture, TypeScript twin of `./🦀️.rs`: Tab cycles handles, arrows nudge by the schema step, digits type a value, Enter opens the value field and applies it, Escape aborts.
 *
 * 🧫️ Replayed on `🧪️tests/🧫️fixtures/🎯️grip-keyboard/🔣️.json` (`keyIntents`) by both implementations. */
import type { DragEventV1 } from "../🛠️handle-drag-tool/🟦️.ts";
import type { ModifiersV1 } from "../🧲️snap/🟦️.ts";

/** ⌨️ A key press reduced to the vocabulary the editor cares about. */
export type HandleKeyV1 = "Tab" | "Enter" | "Escape" | "Backspace" | "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown" | Readonly<{ char: string }>;

/** 🧭️ What a key means for handle manipulation. */
export type KeyIntentV1 = Readonly<{ kind: "cycle"; step: -1 | 1 }> | Readonly<{ kind: "drag"; event: DragEventV1 }> | Readonly<{ kind: "ignored" }>;

/** ⌨️ Maps one key press to an intent; Shift+Tab cycles backwards, Shift multiplies and Alt divides a nudge, Up/Right increase and Down/Left decrease. */
export function keyIntentV1(key: HandleKeyV1, modifiers: ModifiersV1): KeyIntentV1 {
  const nudge = (direction: number): KeyIntentV1 => ({ kind: "drag", event: { kind: "nudge", direction, modifiers } });
  switch (key) {
    case "Tab":
      return { kind: "cycle", step: modifiers.shift ? -1 : 1 };
    case "ArrowRight":
    case "ArrowUp":
      return nudge(1);
    case "ArrowLeft":
    case "ArrowDown":
      return nudge(-1);
    case "Enter":
      return { kind: "drag", event: { kind: "commit" } };
    case "Escape":
      return { kind: "drag", event: { kind: "abort", reason: "escape" } };
    case "Backspace":
      return { kind: "drag", event: { kind: "entryBackspace" } };
    default:
      return /^[0-9\-.,]$/u.test(key.char) ? { kind: "drag", event: { kind: "entry", char: key.char } } : { kind: "ignored" };
  }
}

/** 🔄️ The next focus in the cyclic Tab order; no focus starts at the first (forward) or last (backward) handle. */
export function cycleFocusV1(ids: readonly string[], current: string | null, step: number): string | undefined {
  if (ids.length === 0) return undefined;
  const position = current === null ? -1 : ids.indexOf(current);
  if (position < 0) return ids[step < 0 ? ids.length - 1 : 0];
  return ids[(((position + step) % ids.length) + ids.length) % ids.length];
}

/** 📣️ The announcement for a focused handle: its accessible name plus the current displayed value, spoken by the live region. */
export const focusAnnouncementV1 = (name: string, value: string, unit: string): string => `${name}: ${value} ${unit}`;
