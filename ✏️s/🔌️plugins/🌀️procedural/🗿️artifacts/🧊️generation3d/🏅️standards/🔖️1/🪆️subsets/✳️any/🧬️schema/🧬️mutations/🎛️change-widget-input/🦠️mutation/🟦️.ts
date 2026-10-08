/** 🎛️ generation3d direct `change-widget-input` payload mirror of `ChangeWidgetInput`, with its closed-schema parser. */
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

const finite = (value: unknown): value is number => typeof value === "number" && Number.isFinite(value);

/** 🔣️ One scalar item of the element type `type`, or throws. */
function parseItem(type: string, value: unknown): unknown {
  switch (type) {
    case "number":
      if (!finite(value)) throw new TypeError("change-widget-input: a number value must be finite");
      return value;
    case "text":
      if (typeof value !== "string" || Array.from(value).length > 16777216) throw new TypeError("change-widget-input: a text value is a string of at most 16 MiB");
      return value;
    case "boolean":
      if (typeof value !== "boolean") throw new TypeError("change-widget-input: a boolean value is true or false");
      return value;
    case "point":
    case "vector":
      if (!Array.isArray(value) || value.length !== 3 || !value.every(finite)) throw new TypeError(`change-widget-input: a ${type} value is three finite numbers`);
      return [value[0], value[1], value[2]];
    case "plane": {
      if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("change-widget-input: a plane value is an origin and a normal");
      const row = value as Record<string, unknown>;
      if (Object.keys(row).some((key) => key !== "origin" && key !== "normal")) throw new TypeError("change-widget-input: a plane value holds only an origin and a normal");
      return { origin: parseItem("point", row.origin), normal: parseItem("vector", row.normal) };
    }
    default:
      throw new TypeError("change-widget-input: type must be number, text, boolean, point, vector or plane, or a list of one of them");
  }
}

/** 🔣️ The one typed value the discriminator `type` names — a scalar, or a list of at most 1024 scalars of one type —
 * or throws. */
function parseValue(type: unknown, value: unknown): WidgetInputValue {
  if (typeof type !== "string") throw new TypeError("change-widget-input: type must be a string");
  if (!type.endsWith("List")) return { type, value: parseItem(type, value) } as WidgetInputValue;
  if (!Array.isArray(value) || value.length > 1024) throw new TypeError("change-widget-input: a list value holds at most 1024 items");
  return { type, value: value.map((item) => parseItem(type.slice(0, -4), item)) } as WidgetInputValue;
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
