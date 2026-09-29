/** 🎚️ Portable typed input edits preserve the operator's declared schema. */
export function editInputValue(types: string[], current: unknown, text: string, component?: string): Record<string, unknown> {
  const existing = current && typeof current === "object" ? current as Record<string, unknown> : undefined;
  const schema = typeof existing?.$schema === "string" ? existing.$schema : types[0];
  if (!types.includes(schema) || !["number", "text", "boolean", "point", "vector"].includes(schema)) throw new Error("Connect a compatible output to this input");
  const numeric = () => {
    if (!text.trim() || !/^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/.test(text.trim()) || !Number.isFinite(Number(text))) throw new Error("Input must be a finite number");
    return Number(text);
  };
  if (schema === "point" || schema === "vector") {
    if (!component || !["x", "y", "z"].includes(component)) throw new Error("Choose a coordinate to edit");
    return { $schema: schema, x: 0, y: 0, z: 0, ...existing, [component]: numeric() };
  }
  if (component) throw new Error("Scalar input has no coordinates");
  if (schema === "boolean" && text !== "true" && text !== "false") throw new Error("Boolean input must be true or false");
  return { $schema: schema, ...existing, value: schema === "number" ? numeric() : schema === "boolean" ? text === "true" : text };
}
