/** 🎯️ A first parameter edit seeds authored siblings while explicit answers retain authority. */
import type { DslValue, FormQuestion } from "../../../🧬️schema/🧱️form-model/🟦️.ts";
import { parseFormsValue } from "../../../🧬️schema/🌱️value/🟦️.ts";
export function setTryParameterValue(question: FormQuestion, current: DslValue | undefined, key: string, incoming: DslValue, initialValue?: DslValue): DslValue {
  if (initialValue !== undefined && initialValue.kind !== "object") throw Error("initial parameter value must be an object");
  const base = current ?? initialValue ?? question.params;
  const members = base?.kind === "object" ? base.members.filter(member => member.name !== key) : [];
  return parseFormsValue({kind:"object",members:[...members,{name:key,value:incoming}]});
}

import { binary64 } from "@semio-tech/framework/value/ieee754";
export type TryPartialChange = { vectorIndex: number } | { optionValue: string };

/** 🧩️ Partial controls publish complete answers using the exact defaults their fields display. */
export function setTryPartialValue(question: FormQuestion, current: DslValue | undefined, change: TryPartialChange, incoming: DslValue): DslValue {
  const base = current ?? question.default;
  if ("vectorIndex" in change) {
    const fields = question.fields ?? [];
    const index = change.vectorIndex;
    if (!Number.isSafeInteger(index) || index < 0 || index >= fields.length) throw Error("vector component is outside its question");
    const items = fields.map((field, position) => base?.kind === "array" && base.items[position] !== undefined ? base.items[position]! : parseFormsValue({kind:"float",value:binary64(field.value ?? 0)}));
    items[index] = incoming;
    return parseFormsValue({kind:"array",items});
  }
  const items = base?.kind === "array" ? [...base.items] : [];
  const index = items.findIndex(value => value.kind === "text" && value.value === change.optionValue);
  if (index >= 0) items.splice(index, 1);
  else items.push(parseFormsValue({kind:"text",value:change.optionValue}));
  return parseFormsValue({kind:"array",items});
}

export type TryValueTarget = { readonly key:string; readonly windowId:string; readonly windowKindId:"forms-try"; readonly inputId?:string|null; readonly paramKey?:string|null; readonly optionValue?:string|null };

/** 🎟️ Target metadata has explicit UTF-8 bounds before command ingress. */
export function validateTryValueTarget(value:unknown): value is TryValueTarget {
  if (!value || typeof value !== "object" || Array.isArray(value)) return false;
  const target = value as Record<string,unknown>;
  if (target.windowKindId !== "forms-try") return false;
  for (const [key, maximum, minimum] of [["key",512,1],["windowId",512,1],["inputId",512,1],["paramKey",128,1],["optionValue",512,0]] as const) {
    const field = target[key];
    if ((field === undefined || field === null) && key !== "key" && key !== "windowId") continue;
    if (typeof field !== "string" || field.length < minimum || new TextEncoder().encode(field).length > maximum) return false;
  }
  return true;
}
