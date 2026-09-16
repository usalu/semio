import { parseViewport2d, type Viewport2d } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";
import { parseResultMode, type ResultMode } from "../../../../../../../../../../../../../⚙️engine/🖥️app-surface/🧬️schema/👁️result-mode/🟦️.ts";
/** 🔁️ How the deformation playback clock wraps when the phase leaves `0..=1`. */
export type Fem2dLoopMode = "loop" | "pingPong" | "once";
/** 〰️ The curve the phase is read through before it scales the deformation. */
export type Fem2dWaveform = "ramp" | "sine";
/** ⏯️ Deformation playback state of one exact FEM 2D results window. */
export interface Fem2dResultsAnimation {
  phase: number;
  playing: boolean;
  speed: number;
  loopMode: Fem2dLoopMode;
  waveform: Fem2dWaveform;
  reverse: boolean;
}
/** 🧬️ Exact FEM 2D results window configuration. */
export interface Fem2dResultsWindowConfig {
  camera: Viewport2d;
  resultSourceId?: string | null;
  resultMode: ResultMode;
  resultModeIndex: number;
  animation: Fem2dResultsAnimation;
}
const exact = (value: unknown, at: string, keys: readonly string[], optional: readonly string[] = []): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);
  const row = value as Record<string, unknown>;
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new TypeError(`${at}.${key} is unknown`);
  for (const key of keys) if (!(key in row) && !optional.includes(key)) throw new TypeError(`${at}.${key} is required`);
  return row;
};
const text = (value: unknown, at: string): string => { if (typeof value !== "string") throw new TypeError(`${at} must be a string`); return value; };
const flag = (value: unknown, at: string): boolean => { if (typeof value !== "boolean") throw new TypeError(`${at} must be a boolean`); return value; };
const finite = (value: unknown, at: string): number => { if (typeof value !== "number" || !Number.isFinite(value)) throw new TypeError(`${at} must be finite`); return value; };
const natural = (value: unknown, at: string): number => { const result = finite(value, at); if (!Number.isInteger(result) || result < 0) throw new TypeError(`${at} must be a natural number`); return result; };
const unitInterval = (value: unknown, at: string): number => { const result = finite(value, at); if (result < 0 || result > 1) throw new TypeError(`${at} must lie in 0..=1`); return result; };
const positive = (value: unknown, at: string): number => { const result = finite(value, at); if (result <= 0) throw new TypeError(`${at} must be positive`); return result; };
/** 🔁️ Admits one loop mode from an external value. */
export function parseFem2dLoopMode(value: unknown): Fem2dLoopMode {
  if (value !== "loop" && value !== "pingPong" && value !== "once") throw new TypeError("Unknown FEM 2D loop mode");
  return value;
}
/** 〰️ Admits one waveform from an external value. */
export function parseFem2dWaveform(value: unknown): Fem2dWaveform {
  if (value !== "ramp" && value !== "sine") throw new TypeError("Unknown FEM 2D waveform");
  return value;
}
/** ⏯️ Admits one playback state from an external value. */
export function parseFem2dResultsAnimation(value: unknown): Fem2dResultsAnimation {
  const row = exact(value, "$.animation", ["phase", "playing", "speed", "loopMode", "waveform", "reverse"]);
  return {
    phase: unitInterval(row.phase, "$.animation.phase"),
    playing: flag(row.playing, "$.animation.playing"),
    speed: positive(row.speed, "$.animation.speed"),
    loopMode: parseFem2dLoopMode(row.loopMode),
    waveform: parseFem2dWaveform(row.waveform),
    reverse: flag(row.reverse, "$.animation.reverse"),
  };
}
export function parseFem2dResultsWindowConfig(value: unknown): Fem2dResultsWindowConfig {
  const row = exact(value, "$", ["camera", "resultSourceId", "resultMode", "resultModeIndex", "animation"], ["resultSourceId"]);
  const parsed = { camera: parseViewport2d(row.camera) };
  const mode = parseResultMode(row.resultMode);
  const resultSourceId = row.resultSourceId === undefined || row.resultSourceId === null ? row.resultSourceId : text(row.resultSourceId, "$.resultSourceId");
  return {
    ...parsed,
    ...(resultSourceId === undefined ? {} : { resultSourceId }),
    resultMode: mode,
    resultModeIndex: natural(row.resultModeIndex, "$.resultModeIndex"),
    animation: parseFem2dResultsAnimation(row.animation),
  };
}
