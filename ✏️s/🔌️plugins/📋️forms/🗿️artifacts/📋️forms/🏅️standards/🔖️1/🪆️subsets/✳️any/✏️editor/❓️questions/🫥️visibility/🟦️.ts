import type { DslValue, FormExpr } from "../../../🧬️schema/🧬️mutations/🟦️.ts";

/** 🌱️ A visual rule starts with editable operands without executing source text. */
function createCondition(kind: string): FormExpr {
  switch (kind) {
    case "const": return { kind, value: true };
    case "var": return { kind, name: "" };
    case "eq": return { kind, left: createCondition("var"), right: { kind: "const", value: "" } };
    case "truthy": return { kind, expr: createCondition("var") };
    case "and": case "or": return { kind, items: [createCondition("const")] };
    default: throw new Error("invalid-condition-edit");
  }
}

/** 🫥️ Edits a rule subtree by stable child indices while preserving its siblings. */
export function patchCondition(condition: FormExpr | null, path: string, field: string, value: DslValue): FormExpr | null {
  if (!/^(\d+(\/\d+)*)?$/.test(path)) throw new Error("invalid-path");
  const indices = path ? path.split("/").map(Number) : [];
  if (indices.length > 32) throw new Error("invalid-path");
  if (!indices.length && (field === "remove" || field === "kind" && value === "none")) return null;
  let root = structuredClone(condition ?? createCondition("const"));
  const edit = (node: FormExpr, remaining: number[]): FormExpr => {
    if (remaining.length) {
      const [index, ...rest] = remaining;
      if (node.kind === "and" || node.kind === "or") {
        if (!node.items[index]) throw new Error("invalid-path");
        if (!rest.length && field === "remove") node.items.splice(index, 1);
        else node.items[index] = edit(node.items[index], rest);
      } else if (node.kind === "eq" && index < 2) {
        const key = index === 0 ? "left" : "right"; node[key] = edit(node[key], rest);
      } else if (node.kind === "truthy" && index === 0) node.expr = edit(node.expr, rest);
      else throw new Error("invalid-path");
      return node;
    }
    if (field === "kind" && typeof value === "string") return node.kind === value ? node : createCondition(value);
    if (field === "remove") return { kind: "const", value: false };
    if (field === "name" && node.kind === "var" && typeof value === "string") node.name = value;
    else if (field === "value" && node.kind === "const") node.value = structuredClone(value);
    else if (field === "valueType" && node.kind === "const") {
      if (value === "boolean") node.value = false;
      else if (value === "number") node.value = 0;
      else if (value === "text") node.value = "";
      else if (value === "null") node.value = null;
      else throw new Error("invalid-condition-edit");
    } else if (field === "add" && (node.kind === "and" || node.kind === "or")) node.items.push(createCondition("const"));
    else throw new Error("invalid-condition-edit");
    return node;
  };
  return edit(root, indices);
}
