import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import type { DslValue, FormExpr, FormQuestion, FormStep } from "../🧬️mutations/🟦️.ts";

/** 📝️ Authoritative form content survives standalone document reloads. */
export interface FormsDefinition { steps: FormStep[] }

/** 🌳️ Admits only the closed condition language. */
export function parseCondition(value: unknown, depth = 0): FormExpr {
  if (depth > 32 || !value || typeof value !== "object" || Array.isArray(value)) throw new Error("invalid-condition");
  const row = value as Record<string, unknown>;
  switch (row.kind) {
    case "const": parseSchemaRecord(value, ["kind", "value"]); if ("value" in row) return { kind: "const", value: row.value as DslValue }; break;
    case "var": parseSchemaRecord(value, ["kind", "name"]); if (typeof row.name === "string") return { kind: "var", name: row.name }; break;
    case "eq": parseSchemaRecord(value, ["kind", "left", "right"]); return { kind: "eq", left: parseCondition(row.left, depth + 1), right: parseCondition(row.right, depth + 1) };
    case "and": case "or": parseSchemaRecord(value, ["kind", "items"]); if (Array.isArray(row.items)) return { kind: row.kind, items: row.items.map(item => parseCondition(item, depth + 1)) }; break;
    case "truthy": parseSchemaRecord(value, ["kind", "expr"]); return { kind: "truthy", expr: parseCondition(row.expr, depth + 1) };
  }
  throw new Error("invalid-condition");
}

/** 🪪️ Decodes the schema's complete field contract without external runtime dependencies. */
export function parseFormsDefinition(value: unknown): FormsDefinition {
  const root = parseSchemaRecord(value, ["steps"]);
  if (!Array.isArray(root.steps)) throw new Error("invalid form steps");
  const stepIds = new Set<string>(), questionIds = new Set<string>();
  const unique = (value: unknown, seen: Set<string>): string => {
    if (typeof value !== "string" || !value || seen.has(value)) throw new Error("invalid or duplicate identity");
    seen.add(value); return value;
  };
  const steps = root.steps.map(value => {
    const step = parseSchemaRecord(value, ["id", "title", "description", "blocks"]);
    unique(step.id, stepIds);
    if (typeof step.title !== "string" || step.description !== undefined && typeof step.description !== "string" || !Array.isArray(step.blocks)) throw new Error("invalid step");
    const blocks = step.blocks.map(value => {
      const question = parseSchemaRecord(value, ["id", "label", "kind", "description", "required", "placeholder", "default", "min", "max", "step", "unit", "text", "options", "fields", "schema", "src", "accept", "fixtureSlug", "params", "condition"]);
      unique(question.id, questionIds);
      if (typeof question.kind !== "string" || !question.kind.trim() || typeof question.label !== "string") throw new Error("invalid question");
      for (const key of ["description", "placeholder", "unit", "text", "schema", "src", "accept", "fixtureSlug"]) if (question[key] !== undefined && typeof question[key] !== "string") throw new Error("invalid question text");
      if (question.required !== undefined && typeof question.required !== "boolean") throw new Error("invalid required flag");
      for (const key of ["min", "max", "step"]) if (question[key] !== undefined && (typeof question[key] !== "number" || !Number.isFinite(question[key]))) throw new Error("invalid question range");
      if (typeof question.min === "number" && typeof question.max === "number" && question.min > question.max || typeof question.step === "number" && question.step <= 0) throw new Error("invalid question range");
      for (const [collection, identity, allowed] of [["options", "value", ["value", "label"]], ["fields", "key", ["key", "label", "value"]]] as const) {
        if (question[collection] === undefined) continue;
        if (!Array.isArray(question[collection])) throw new Error("invalid question collection");
        const seen = new Set<string>();
        for (const entry of question[collection] as unknown[]) {
          const item = parseSchemaRecord(entry, allowed);
          unique(item[identity], seen);
          if (collection === "options" && typeof item.label !== "string" || collection === "fields" && item.label !== undefined && typeof item.label !== "string") throw new Error("invalid collection label");
          if (collection === "fields" && item.value !== undefined && (typeof item.value !== "number" || !Number.isFinite(item.value))) throw new Error("invalid vector value");
        }
      }
      if (question.params !== undefined && (!question.params || typeof question.params !== "object" || Array.isArray(question.params))) throw new Error("invalid extension parameters");
      if (question.condition !== undefined) question.condition = parseCondition(question.condition);
      return structuredClone(question) as unknown as FormQuestion;
    });
    return { ...step, blocks } as unknown as FormStep;
  });
  return { steps };
}
/** 🌱️ A blank form starts with one editable page and no questions. */
export function blankFormsDefinition(): FormsDefinition {
  return { steps: [{ id: "s", title: "Inputs", blocks: [] }] };
}
