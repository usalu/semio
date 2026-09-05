/** 🧬️ AssemblyMutation — one discriminated-union member per `🧬️mutations/<slug>/` triad's payload
 * shape. Mirrors the Rust `🦀️.rs` sibling's `AssemblyMutation` enum, which carries only
 * `#[derive(dsl::Mutations)]` — no `#[serde(tag = ...)]` — so it serializes with serde's default
 * EXTERNALLY TAGGED shape: `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, confirmed by
 * the committed `🧩️create-slot/🧪️tests/*​/🦠️mutation/🔣️.json` fixture
 * (`{"CreateSlot":{"index":2,"slot":{...}}}`) — NOT the `{ kind: "create-slot"; ... }` internally
 * tagged, kebab-case shape this previously declared. None of the 9 leaf structs carry
 * `#[serde(rename_all = ...)]`, so every leaf's own field names are the literal Rust snake_case
 * names verbatim. */
import type { AssemblySlot, AssemblySlotEdge, AssemblyRule } from "../📸️snapshot/🟦️";

export interface CreateSlot {
  index: number;
  slot: AssemblySlot;
}

export interface DeleteSlot {
  id: string;
}

export interface CreateRule {
  index: number;
  rule: AssemblyRule;
}

export interface DeleteRule {
  id: string;
}

export interface ChangeWeight {
  module_id: string;
  weight: number;
}

export interface RemoveWeight {
  module_id: string;
}

export interface ConnectSlots {
  index: number;
  edge: AssemblySlotEdge;
}

export interface DisconnectSlots {
  id: string;
}

export interface ChangeSeed {
  seed: number;
}

export type AssemblyMutation =
  | { CreateSlot: CreateSlot }
  | { DeleteSlot: DeleteSlot }
  | { CreateRule: CreateRule }
  | { DeleteRule: DeleteRule }
  | { ChangeWeight: ChangeWeight }
  | { RemoveWeight: RemoveWeight }
  | { ConnectSlots: ConnectSlots }
  | { DisconnectSlots: DisconnectSlots }
  | { ChangeSeed: ChangeSeed };
