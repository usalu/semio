/** 🔺️ generation2d create-widget diff — mirrors `diff()` (…/🌱create-widget/🔺️diff/🦀️.rs), a sparse insert into the fixture's widget collection. */
import type { CreateWidget, Widget } from "../🦠️mutation/🟦️.ts";

export interface CreateWidgetDiff {
  widgets: { removed: string[]; set: Array<[number, Widget]> };
}

export function diff(payload: CreateWidget): CreateWidgetDiff {
  return { widgets: { removed: [], set: [[payload.index, payload.widget]] } };
}
