/** 🧬️ FormsMutation union — closed semantic mutation vocabulary for the forms document.
 * Mirrors Rust `FormMutation` (sibling `🦀️.rs`'s `KINDS`,
 * `#[serde(tag = "mutation", rename_all = "camelCase")]`), same declaration order.
 * The enum-level `rename_all` renames the VARIANTS (the `mutation` tag), not the payload
 * fields: none of the payload structs carries its own `#[serde(rename_all)]`, so every
 * payload field stays snake_case — confirmed against the committed
 * committed per-verb `🧪️tests` mutation fixtures (e.g. `"new_description": null`).
 * Payloads are imported from each verb's own `🦠️mutation` leaf; `changeStepDescription`
 * is declared inline because that verb has no TS leaf on disk (Rust only). */

import type { ChangeFormTitle } from "./🏷️change-form-title/🦠️mutation/🟦️.ts";
import type { CreateBlock } from "./➕create-block/🦠️mutation/🟦️.ts";
import type { CreateStep } from "./🌱create-step/🦠️mutation/🟦️.ts";
import type { DeleteBlock } from "./➖delete-block/🦠️mutation/🟦️.ts";
import type { DeleteStep } from "./🗑️delete-step/🦠️mutation/🟦️.ts";
import type { MoveBlockToStep } from "./📦move-block-to-step/🦠️mutation/🟦️.ts";
import type { RenameStep } from "./✏️rename-step/🦠️mutation/🟦️.ts";
import type { ReorderStep } from "./🔀reorder-step/🦠️mutation/🟦️.ts";
import type { ReplaceBlock } from "./🔁replace-block/🦠️mutation/🟦️.ts";


export type DslValue = Record<string, unknown>;

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
  fixtureSlug?: string;
  params?: DslValue;
  condition?: FormExpr;
}

export interface FormStep {
  id: string;
  title: string;
  description?: string;
  blocks: FormQuestion[];
}

/** 📝️ `change-step-description` payload — mirrors Rust `ChangeStepDescription`
 * (`📝change-step-description/🦠️mutation/🦀️.rs`); that verb has no TS leaf on disk. */
export interface ChangeStepDescription {
  id: string;
  new_description: string | null;
}

export type FormsMutation =
  | ({ mutation: 'createStep' } & CreateStep)
  | ({ mutation: 'deleteStep' } & DeleteStep)
  | ({ mutation: 'reorderStep' } & ReorderStep)
  | ({ mutation: 'renameStep' } & RenameStep)
  | ({ mutation: 'changeStepDescription' } & ChangeStepDescription)
  | ({ mutation: 'createBlock' } & CreateBlock)
  | ({ mutation: 'deleteBlock' } & DeleteBlock)
  | ({ mutation: 'moveBlockToStep' } & MoveBlockToStep)
  | ({ mutation: 'replaceBlock' } & ReplaceBlock)
  | ({ mutation: 'changeFormTitle' } & ChangeFormTitle);
