/** 📍 procedural2d move-widget payload — mirrors `MoveWidget` (…/📍move-widget/🦠️mutation/🦀️component.rs:16-19). */
export interface WidgetLayout {
  x: number;
  y: number;
}

export interface MoveWidget {
  id: string;
  layout: WidgetLayout;
}
