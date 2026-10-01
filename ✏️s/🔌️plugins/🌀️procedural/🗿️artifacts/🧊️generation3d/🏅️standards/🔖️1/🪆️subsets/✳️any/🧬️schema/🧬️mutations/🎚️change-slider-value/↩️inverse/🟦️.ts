/** ↩️ generation3d change-slider-value/↩️inverse — mirror of the BASE slider widget restored whole. */
import type { ChangeSliderValue } from "../🦠️mutation/🟦️.ts";
import type { UpdateWidget } from "../../🩹update-widget/🦠️mutation/🟦️.ts";
import type { Widget } from "../../🌱️create-widget/🦠️mutation/🟦️.ts";

export function inverse(_payload: ChangeSliderValue, baseSlider: Widget | undefined): UpdateWidget[] {
  return baseSlider === undefined ? [] : [{ widget: baseSlider }];
}
