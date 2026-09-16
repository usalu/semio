import { parseViewport3dOrbit, type Viewport3dOrbit } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🟦️.ts";
import { parseResultMode, type ResultMode } from "../../../../../../../../../../../../../⚙️engine/🖥️app-surface/🧬️schema/👁️result-mode/🟦️.ts";
/** 🔁️ How the deformation playback clock wraps when the phase leaves `0..=1`. */
export type Fem3dLoopMode = "loop" | "pingPong" | "once";
/** 〰️ The curve the phase is read through before it scales the deformation. */
export type Fem3dWaveform = "ramp" | "sine";
/** ⏯️ Deformation playback state of one exact FEM 3D results window. */
export interface Fem3dResultsAnimation {
  phase: number;
  playing: boolean;
  speed: number;
  loopMode: Fem3dLoopMode;
  waveform: Fem3dWaveform;
  reverse: boolean;
}
/** 🧬️ Exact FEM 3D results window configuration. */
export interface Fem3dResultsWindowConfig {
  camera: Viewport3dOrbit;
  resultSourceId?: string | null;
  resultMode: ResultMode;
  resultModeIndex: number;
  animation: Fem3dResultsAnimation;
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
export function parseFem3dLoopMode(value: unknown): Fem3dLoopMode {
  if (value !== "loop" && value !== "pingPong" && value !== "once") throw new TypeError("Unknown FEM 3D loop mode");
  return value;
}
/** 〰️ Admits one waveform from an external value. */
export function parseFem3dWaveform(value: unknown): Fem3dWaveform {
  if (value !== "ramp" && value !== "sine") throw new TypeError("Unknown FEM 3D waveform");
  return value;
}
/** ⏯️ Admits one playback state from an external value. */
export function parseFem3dResultsAnimation(value: unknown): Fem3dResultsAnimation {
  const row = exact(value, "$.animation", ["phase", "playing", "speed", "loopMode", "waveform", "reverse"]);
  return {
    phase: unitInterval(row.phase, "$.animation.phase"),
    playing: flag(row.playing, "$.animation.playing"),
    speed: positive(row.speed, "$.animation.speed"),
    loopMode: parseFem3dLoopMode(row.loopMode),
    waveform: parseFem3dWaveform(row.waveform),
    reverse: flag(row.reverse, "$.animation.reverse"),
  };
}
export function parseFem3dResultsWindowConfig(value: unknown): Fem3dResultsWindowConfig {
  const row = exact(value, "$", ["camera", "resultSourceId", "resultMode", "resultModeIndex", "animation"], ["resultSourceId"]);
  const parsed = { camera: parseViewport3dOrbit(row.camera) };
  const mode = parseResultMode(row.resultMode);
  const resultSourceId = row.resultSourceId === undefined || row.resultSourceId === null ? row.resultSourceId : text(row.resultSourceId, "$.resultSourceId");
  return {
    ...parsed,
    ...(resultSourceId === undefined ? {} : { resultSourceId }),
    resultMode: mode,
    resultModeIndex: natural(row.resultModeIndex, "$.resultModeIndex"),
    animation: parseFem3dResultsAnimation(row.animation),
  };
}
