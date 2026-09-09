/** 🧬️ LowpolyMutation dispatch — real facet mirror of the Rust `🦀️.rs` sibling's
 * `LowpolyMutation` enum (`dsl::Mutations`-derived, seventeen variants: nine object-lane verbs, a
 * create/delete pair for the `mesh` CHILD slot, and six paint-layer verbs plus one pixel edit).
 * Untagged-by-variant-name on the wire (`serde`'s default externally-tagged enum representation —
 * `{ "MoveObject": { … } }`, confirmed against the committed `🧪️tests/…/🦠️mutation/🔣️.json`
 * fixtures across every mutation family), never a `{ mutation, payload }` envelope. */
import type { ArtifactRef } from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import type { LowpolyObject, LowpolyPaintLayer } from "../🟦️.ts";

/** One contiguous run of RGBA bytes written into a paint-layer pixel buffer at `offset`. */
export interface PixelRun {
  offset: number;
  /** base64-encoded RGBA bytes */
  bytes: string;
}

export type LowpolyMutation =
  | { CreateObject: { index: number; object: LowpolyObject } }
  | { DeleteObject: { id: string } }
  | { ReorderObjects: { id: string; toIndex: number } }
  | { RenameObject: { id: string; newName: string } }
  | { ChangeObjectSmoothShading: { id: string; newSmoothShading: boolean } }
  | { MoveObject: { id: string; newPosition: [number, number, number] } }
  | { RotateObject: { id: string; newRotation: [number, number, number] } }
  | { ScaleObject: { id: string; newScale: [number, number, number] } }
  | { CreateMesh: { id: string; childId: string; target: ArtifactRef; meshWorkspace: string } }
  | { DeleteMesh: { id: string } }
  | { InsertPaintLayer: { objectId: string; index: number; layer: LowpolyPaintLayer } }
  | { RemovePaintLayer: { objectId: string; index: number } }
  | { RenamePaintLayer: { objectId: string; index: number; newName: string } }
  | { ChangePaintLayerVisible: { objectId: string; index: number; newVisible: boolean } }
  | { ChangePaintLayerOpacity: { objectId: string; index: number; newOpacity: number } }
  | { ChangePaintLayerBlendMode: { objectId: string; index: number; newBlendMode: string } }
  | { EditPaintLayer: { objectId: string; layerIndex: number; runs: PixelRun[] } };

/** 🏷️ The exact wire tag (Rust enum variant name / `dsl::Mutations` `aggregateVariant`) of every
 * [`LowpolyMutation`] member, in declaration order — mirrors `🦀️.rs`'s `KINDS` intent one
 * level up (PascalCase tag, not the kebab-case `semanticKind`). */
export const LOWPOLY_MUTATION_TAGS = [
  "CreateObject",
  "DeleteObject",
  "ReorderObjects",
  "RenameObject",
  "ChangeObjectSmoothShading",
  "MoveObject",
  "RotateObject",
  "ScaleObject",
  "CreateMesh",
  "DeleteMesh",
  "InsertPaintLayer",
  "RemovePaintLayer",
  "RenamePaintLayer",
  "ChangePaintLayerVisible",
  "ChangePaintLayerOpacity",
  "ChangePaintLayerBlendMode",
  "EditPaintLayer",
] as const;
