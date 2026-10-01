/** 🔺️ generation3d change-slider-value/🔺️diff — mirror of the one-slider replacement, including the range widening
 * `semio_framework_artifact_flow_flow::set_widget_slider_value` applies to a value outside the slider's range. */
import type { ChangeSliderValue } from "../🦠️mutation/🟦️.ts";

/** 🎚️ The numeric fields of one input slider. */
export interface SliderFields {
  value: number;
  min: number;
  max: number;
  step: number;
}

const decimalPlaces = (value: number): number => {
  if (Math.abs(value - Math.round(value)) < 1e-9) return 0;
  for (let places = 1; places <= 12; places += 1) {
    const step = 10 ** -places;
    if (Math.abs(Math.round(value / step) * step - value) < 1e-9) return places;
  }
  return 1;
};

const sensibleMax = (value: number): number => {
  const v = Math.abs(value);
  if (v <= 1) return 1;
  if (v <= 10) return 10;
  const magnitude = 10 ** Math.floor(Math.log10(v));
  const normalized = v / magnitude;
  const nice = normalized <= 1 ? 1 : normalized <= 2 ? 2 : normalized <= 5 ? 5 : 10;
  return Math.max(nice * magnitude, v);
};

/** 📐️ The range a slider widens to for a value outside it — `sensible_slider_range`. */
export function sensibleSliderRange(value: number): [number, number, number] {
  const places = decimalPlaces(value);
  const step = places === 0 ? 1 : 10 ** -places;
  const bound = sensibleMax(value);
  return value < 0 ? [-bound, bound, step] : [0, bound, step];
}

/** 🔺️ The slider's next fields, or `undefined` for a value or range the slider cannot hold (`target-mismatch`). */
export function diff(payload: ChangeSliderValue, base: SliderFields): SliderFields | undefined {
  if (!Number.isFinite(payload.value) || !Number.isFinite(base.min) || !Number.isFinite(base.max) || base.min > base.max) return undefined;
  const [min, max, step] = payload.value < base.min || payload.value > base.max ? sensibleSliderRange(payload.value) : [base.min, base.max, base.step];
  return { value: Math.min(Math.max(payload.value, min), max), min, max, step };
}
