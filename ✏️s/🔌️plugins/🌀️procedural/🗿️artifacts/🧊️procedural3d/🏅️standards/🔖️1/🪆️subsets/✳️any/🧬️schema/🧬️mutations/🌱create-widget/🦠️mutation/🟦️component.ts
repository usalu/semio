/** ➕ procedural3d direct `create-widget` payload mirror of `CreateWidget`. */
/** @description Opaque `flow::Widget` — JSON text (tagged union serialized by `kind`). */
export type Widget = string;

/** 🔎️ Extracts the shared `id` field every `Widget` variant carries, by parsing its JSON text — mirror of `procedural3d::widget_id`. */
export function widgetId(widget: Widget): string {
  return (JSON.parse(widget) as { id: string }).id;
}

export interface CreateWidget {
  index: number;
  widget: Widget;
}
