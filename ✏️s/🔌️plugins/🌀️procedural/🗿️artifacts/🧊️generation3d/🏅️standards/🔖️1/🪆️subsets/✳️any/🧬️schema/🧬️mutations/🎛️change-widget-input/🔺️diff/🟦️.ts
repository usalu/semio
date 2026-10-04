/** 🔺️ generation3d change-widget-input/🔺️diff — mirror of the typed literal one operator input receives. */
import type { ChangeWidgetInput } from "../🦠️mutation/🟦️.ts";

/** 🧩️ One typed operator param literal: `{$schema, value}`, or `{$schema, x, y, z}` for a point or a vector. */
export type InputLiteral = { $schema: string; value?: unknown; x?: number; y?: number; z?: number; [key:string]:unknown };

/** 🧩️ The literal the payload sets. */
export function literal(payload: ChangeWidgetInput): InputLiteral {
  if (payload.type.endsWith("List")) return Object.fromEntries([["$schema","list"],...(payload.value as unknown[]).map((value,index) => [String(index),literal({...payload,type:payload.type.slice(0,-4),value} as ChangeWidgetInput)])]) as InputLiteral;
  if (payload.type === "point" || payload.type === "vector") return { $schema: payload.type, x: payload.value[0], y: payload.value[1], z: payload.value[2] };
  return { $schema: payload.type, value: payload.value };
}

/** 🔺️ The input's next literal, or `undefined` when it holds a literal of another type (`target-mismatch`). */
export function diff(payload: ChangeWidgetInput, current: InputLiteral | undefined): InputLiteral | undefined {
  const type = payload.type.endsWith("List") ? "list" : payload.type;
  if (current && current.$schema !== type) return undefined;
  if (current && type === "list") {
    const entries = Object.entries(current).filter(([key]) => key !== "$schema");
    if (entries.length > 1024 || entries.some(([key,item]) => !/^(0|[1-9]\d*)$/.test(key) || Number(key) >= entries.length || !item || typeof item !== "object" || (item as InputLiteral).$schema !== payload.type.slice(0,-4))) return undefined;
  }
  return literal(payload);
}
