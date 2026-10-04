import{binary64,type Binary64}from"../../../🟦️.ts";
/** 🎚️ generation2d direct `change-slider-value` payload mirror of `ChangeSliderValue`, with its closed-schema parser. */
export interface ChangeSliderValue {
  id: string;
  value: Binary64;
}

/** 🚪️ Parses one `change-slider-value` payload the way its JSON Schema admits it, or throws. */
export function parseChangeSliderValue(value: unknown): ChangeSliderValue {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("change-slider-value: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["id", "value"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`change-slider-value: unknown field ${unknownKey}`);
  if (typeof row.id !== "string" || row.id.length === 0) throw new TypeError("change-slider-value: id must be a nonempty string");
  if (typeof row.value !== "number" || !Number.isFinite(row.value)) throw new TypeError("change-slider-value: value must be a finite number");
  return { id: row.id as string, value: binary64(row.value) };
}
