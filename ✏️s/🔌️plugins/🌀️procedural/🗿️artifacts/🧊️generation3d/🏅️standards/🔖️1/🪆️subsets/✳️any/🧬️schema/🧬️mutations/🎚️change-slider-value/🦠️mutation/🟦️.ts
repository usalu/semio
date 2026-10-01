/** 🎚️ generation3d direct `change-slider-value` payload mirror of `ChangeSliderValue`, with its closed-schema parser. */
export interface ChangeSliderValue {
  id: string;
  value: number;
}

/** 🚪️ Parses one `change-slider-value` payload the way its JSON Schema admits it, or throws. */
export function parseChangeSliderValue(value: unknown): ChangeSliderValue {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("change-slider-value: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["id", "value", "mutation"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`change-slider-value: unknown field ${unknownKey}`);
  if (row.mutation !== undefined && row.mutation !== "changeSliderValue") throw new TypeError("change-slider-value: wrong mutation tag");
  if (typeof row.id !== "string" || row.id.length === 0) throw new TypeError("change-slider-value: id must be a nonempty string");
  if (typeof row.value !== "number" || !Number.isFinite(row.value)) throw new TypeError("change-slider-value: value must be a finite number");
  return { id: row.id as string, value: row.value as number };
}
