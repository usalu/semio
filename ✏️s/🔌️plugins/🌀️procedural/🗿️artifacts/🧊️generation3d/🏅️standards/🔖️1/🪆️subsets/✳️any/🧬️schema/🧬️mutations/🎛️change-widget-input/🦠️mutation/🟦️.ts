/** 🎛️ generation3d direct `change-widget-input` payload mirror of `ChangeWidgetInput`. */
type Triple = [number, number, number];

export type WidgetInputValue =
  | { type: "number"; value: number }
  | { type: "text"; value: string }
  | { type: "boolean"; value: boolean }
  | { type: "point"; value: Triple }
  | { type: "vector"; value: Triple }
  | { type: "plane"; value: { origin: Triple; normal: Triple } }
  | { type: "numberList"; value: number[] }
  | { type: "textList"; value: string[] }
  | { type: "booleanList"; value: boolean[] }
  | { type: "pointList"; value: Triple[] }
  | { type: "vectorList"; value: Triple[] };

export type ChangeWidgetInput = { id: string; channel: string } & WidgetInputValue;
