/** 🔁️ How the deformation playback clock wraps when the phase leaves `0..=1`. */
export type FemLoopMode = "loop" | "pingPong" | "once";
/** 〰️ The curve the phase is read through before it scales the deformation. */
export type FemWaveform = "ramp" | "sine";
/** ⏯️ Deformation playback transport of one FEM results window (2D and 3D alike). */
export interface FemResultsAnimation {
  phase: number;
  playing: boolean;
  speed: number;
  loopMode: FemLoopMode;
  waveform: FemWaveform;
  reverse: boolean;
}
const KEYS = ["phase", "playing", "speed", "loopMode", "waveform", "reverse"] as const;
const flag = (value: unknown, at: string): boolean => { if (typeof value !== "boolean") throw new TypeError(`${at} must be a boolean`); return value; };
const finite = (value: unknown, at: string): number => { if (typeof value !== "number" || !Number.isFinite(value)) throw new TypeError(`${at} must be finite`); return value; };
const unitInterval = (value: unknown, at: string): number => { const result = finite(value, at); if (result < 0 || result > 1) throw new TypeError(`${at} must lie in 0..=1`); return result; };
const positive = (value: unknown, at: string): number => { const result = finite(value, at); if (result <= 0) throw new TypeError(`${at} must be positive`); return result; };
/** 🔁️ Admits one loop mode from an external value. */
export function parseFemLoopMode(value: unknown): FemLoopMode {
  if (value !== "loop" && value !== "pingPong" && value !== "once") throw new TypeError("Unknown FEM loop mode");
  return value;
}
/** 〰️ Admits one waveform from an external value. */
export function parseFemWaveform(value: unknown): FemWaveform {
  if (value !== "ramp" && value !== "sine") throw new TypeError("Unknown FEM waveform");
  return value;
}
/** ⏯️ Admits one playback transport from an external value; `at` names its place in the enclosing record. */
export function parseFemResultsAnimation(value: unknown, at = "$.animation"): FemResultsAnimation {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);
  const row = value as Record<string, unknown>;
  for (const key of Object.keys(row)) if (!(KEYS as readonly string[]).includes(key)) throw new TypeError(`${at}.${key} is unknown`);
  for (const key of KEYS) if (!(key in row)) throw new TypeError(`${at}.${key} is required`);
  return {
    phase: unitInterval(row.phase, `${at}.phase`),
    playing: flag(row.playing, `${at}.playing`),
    speed: positive(row.speed, `${at}.speed`),
    loopMode: parseFemLoopMode(row.loopMode),
    waveform: parseFemWaveform(row.waveform),
    reverse: flag(row.reverse, `${at}.reverse`),
  };
}
