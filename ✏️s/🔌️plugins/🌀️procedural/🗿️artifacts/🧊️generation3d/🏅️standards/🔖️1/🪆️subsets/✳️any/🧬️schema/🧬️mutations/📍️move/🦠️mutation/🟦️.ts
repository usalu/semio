/** 📍 generation3d direct `move-widget` payload mirror of `MoveWidget`. */
export interface WidgetLayout {
  x: number;
  y: number;
}

export interface MoveWidget {
  id: string;
  layout: WidgetLayout;
}
