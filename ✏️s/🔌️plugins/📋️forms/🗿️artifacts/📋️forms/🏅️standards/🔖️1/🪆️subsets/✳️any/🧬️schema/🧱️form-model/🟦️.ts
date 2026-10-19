/** 🧱️ The form model the mutations edit and the validation, definition and response parsers read: expressions, questions, options, vector fields and steps. */

import type { IntrinsicValue as DslValue } from "@semio-tech/framework/value/schema";

export type{IntrinsicValue as DslValue}from"@semio-tech/framework/value/schema";

export interface FormQuestionOption {
  value: string;
  label: string;
}

export interface FormVectorField {
  key: string;
  label?: string;
  value?: number;
}

export type FormExpr =
  | { kind: 'const'; value: DslValue }
  | { kind: 'var'; name: string }
  | { kind: 'eq'; left: FormExpr; right: FormExpr }
  | { kind: 'and'; items: FormExpr[] }
  | { kind: 'or'; items: FormExpr[] }
  | { kind: 'truthy'; expr: FormExpr };

export interface FormQuestion {
  id: string;
  label: string;
  kind: string;
  description?: string;
  required?: boolean;
  placeholder?: string;
  default?: DslValue;
  min?: number;
  max?: number;
  step?: number;
  unit?: string;
  text?: string;
  options?: FormQuestionOption[];
  fields?: FormVectorField[];
  schema?: string;
  src?: string;
  accept?: string;
  exampleId?: string;
  params?: DslValue;
  condition?: FormExpr;
}

export interface FormStep {
  id: string;
  title: string;
  description?: string;
  blocks: FormQuestion[];
}
