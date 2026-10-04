import type { Widget } from "../../../🟦️.ts";

export type { Widget };

/** 🔎️ The shared `id` every `Widget` variant carries — mirror of `generation3d::widget_id`. */
export function widgetId(widget: Widget): string {
  return widget.id;
}

/** ➕ generation3d direct `create-widget` payload mirror of `CreateWidget`. */
export interface CreateWidget {
  index: bigint;
  widget: Widget;
}
