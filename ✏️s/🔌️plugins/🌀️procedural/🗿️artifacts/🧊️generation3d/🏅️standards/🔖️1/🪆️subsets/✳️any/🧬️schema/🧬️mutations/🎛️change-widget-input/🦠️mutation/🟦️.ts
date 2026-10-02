/** 🎛️ generation3d direct `change-widget-input` payload mirror of `ChangeWidgetInput`, with its closed-schema parser. */
export type WidgetInputValue =
  | { type: "number"; value: number }
  | { type: "text"; value: string }
  | { type: "boolean"; value: boolean }
  | { type: "point"; value: [number, number, number] }
  | { type: "vector"; value: [number, number, number] };

export type ChangeWidgetInput = { id: string; channel: string } & WidgetInputValue;

const finite = (value: unknown): value is number => typeof value === "number" && Number.isFinite(value);

/** 🔣️ The one typed value the discriminator `type` names, or throws. */
function parseValue(type: unknown, value: unknown): WidgetInputValue {
  switch (type) {
    case "number":
      if (!finite(value)) throw new TypeError("change-widget-input: a number value must be finite");
      return { type, value };
    case "text":
      if (typeof value !== "string" || Array.from(value).length > 1048576) throw new TypeError("change-widget-input: a text value is a string of at most 1 MiB");
      return { type, value };
    case "boolean":
      if (typeof value !== "boolean") throw new TypeError("change-widget-input: a boolean value is true or false");
      return { type, value };
    case "point":
    case "vector":
      if (!Array.isArray(value) || value.length !== 3 || !value.every(finite)) throw new TypeError(`change-widget-input: a ${type} value is three finite numbers`);
      return { type, value: [value[0], value[1], value[2]] };
    default:
      throw new TypeError("change-widget-input: type must be number, text, boolean, point or vector");
  }
}

/** 🚪️ Parses one `change-widget-input` payload the way its JSON Schema admits it, or throws. */
export function parseChangeWidgetInput(value: unknown): ChangeWidgetInput {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("change-widget-input: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["id", "channel", "type", "value", "mutation"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`change-widget-input: unknown field ${unknownKey}`);
  if (row.mutation !== undefined && row.mutation !== "changeWidgetInput") throw new TypeError("change-widget-input: wrong mutation tag");
  if (typeof row.id !== "string" || row.id.length === 0) throw new TypeError("change-widget-input: id must be a nonempty string");
  if (typeof row.channel !== "string" || row.channel.length === 0 || Array.from(row.channel).length > 256) throw new TypeError("change-widget-input: channel must be a nonempty string of at most 256 characters");
  return { id: row.id, channel: row.channel, ...parseValue(row.type, row.value) };
}
