/** 🔺️ generation3d change-widget-input/🔺️diff — mirror of the typed literal one operator input receives. */
import type { ChangeWidgetInput } from "../🦠️mutation/🟦️.ts";

/** 🧩️ One typed operator param literal: `{$schema, value}`, or `{$schema, x, y, z}` for a point or a vector. */
export type InputLiteral = { $schema: string; value?: unknown; x?: number; y?: number; z?: number };

/** 🧩️ The literal the payload sets. */
export function literal(payload: ChangeWidgetInput): InputLiteral {
  if (payload.type === "point" || payload.type === "vector") return { $schema: payload.type, x: payload.value[0], y: payload.value[1], z: payload.value[2] };
  return { $schema: payload.type, value: payload.value };
}

/** 🔺️ The input's next literal, or `undefined` when it holds a literal of another type (`target-mismatch`). */
export function diff(payload: ChangeWidgetInput, current: InputLiteral | undefined): InputLiteral | undefined {
  return current === undefined || current.$schema === payload.type ? literal(payload) : undefined;
}
