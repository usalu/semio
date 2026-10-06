import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import type { DslValue, FormExpr, FormQuestion, FormStep } from "../🧬️mutations/🟦️.ts";
import{parseIntrinsicValue}from"../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";

/** 📝️ Authoritative form content survives standalone document reloads. */
export interface FormsDefinition { steps: FormStep[] }

/** 🌳️ Admits only the closed condition language. */
export function parseCondition(value: unknown): FormExpr {
  let result:FormExpr|undefined;const active=new Set<object>(),pending:({value:unknown;put:(expr:FormExpr)=>void}|{close:object})[]=[{value,put:expr=>{result=expr}}];
  while(pending.length){const frame=pending.pop()!;if("close"in frame){active.delete(frame.close);continue;}const source=frame.value;if(!source||typeof source!=="object"||Array.isArray(source)||active.has(source)||!("kind"in source))throw Error("invalid-condition");active.add(source);pending.push({close:source});
    const put=frame.put;switch(source.kind){
      case"const":{const row=parseSchemaRecord(source,["kind","value"]);put({kind:"const",value:parseIntrinsicValue(row.value)});break;}
      case"var":{const row=parseSchemaRecord(source,["kind","name"]);if(typeof row.name!=="string")throw Error("invalid-condition");put({kind:"var",name:row.name});break;}
      case"eq":{const row=parseSchemaRecord(source,["kind","left","right"]),expr={kind:"eq"}as FormExpr&{kind:"eq"};put(expr);pending.push({value:row.right,put:value=>{expr.right=value}},{value:row.left,put:value=>{expr.left=value}});break;}
      case"and":case"or":{const row=parseSchemaRecord(source,["kind","items"]);if(!Array.isArray(row.items))throw Error("invalid-condition");const items:FormExpr[]=new Array(row.items.length);put({kind:source.kind,items});for(let i=row.items.length-1;i>=0;i--)pending.push({value:row.items[i],put:value=>{items[i]=value}});break;}
      case"truthy":{const row=parseSchemaRecord(source,["kind","expr"]),expr={kind:"truthy"}as FormExpr&{kind:"truthy"};put(expr);pending.push({value:row.expr,put:value=>{expr.expr=value}});break;}
      default:throw Error("invalid-condition");
    }
  }return result!;
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
      const question = {...parseSchemaRecord(value, ["id", "label", "kind", "description", "required", "placeholder", "default", "min", "max", "step", "unit", "text", "options", "fields", "schema", "src", "accept", "exampleId", "params", "condition"])};
      unique(question.id, questionIds);
      if (typeof question.kind !== "string" || !question.kind.trim() || typeof question.label !== "string") throw new Error("invalid question");
      for (const key of ["description", "placeholder", "unit", "text", "schema", "src", "accept", "exampleId"]) if (question[key] !== undefined && typeof question[key] !== "string") throw new Error("invalid question text");
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
      if(question.default!==undefined)question.default=parseIntrinsicValue(question.default);
      if(question.params!==undefined){question.params=parseIntrinsicValue(question.params);if((question.params as DslValue).kind!=="object")throw Error("invalid extension parameters");}
      if (question.condition !== undefined) question.condition = parseCondition(question.condition);
      return question as unknown as FormQuestion;
    });
    return { ...step, blocks } as unknown as FormStep;
  });
  return { steps };
}
/** 🌱️ A blank form starts with one editable page and no questions. */
export function blankFormsDefinition(): FormsDefinition {
  return { steps: [{ id: "s", title: "Inputs", blocks: [] }] };
}
