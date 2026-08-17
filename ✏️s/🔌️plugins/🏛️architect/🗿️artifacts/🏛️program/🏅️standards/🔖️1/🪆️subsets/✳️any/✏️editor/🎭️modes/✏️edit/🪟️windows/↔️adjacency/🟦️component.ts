/** ↔️ Architect editor — Adjacency window: typed twin of `🦀️component.rs`'s view boundary. Mirrors
 * `render(program: &ProgramSnapshot, cfg: &ArchitectConfig) -> UiNode`'s signature — the signature
 * triangle glyph strip plus lower-triangle pair rows, each pair cycling its `AdjacencyKind` on
 * activation. */

/** ↔️ Mirrors the Rust `AdjacencyKind` enum's `#[serde(rename_all = "camelCase")]` wire shape. */
export type ArchitectAdjacencyKind = "required" | "preferred" | "optional" | "prohibited";

/** ↔️ The Adjacency window's typed view-model — mirrors the Rust `render()` boundary's inputs: the
 * whole program document (read for its elements/adjacencies) plus the config's optional filter. */
export interface ArchitectAdjacencyViewModel {
  windowKindId: "architect-adjacency";
  bodyKey: "architect.adjacency";
  adjacencyKindFilter: ArchitectAdjacencyKind | null;
}

export const ARCHITECT_WINDOW_ADJACENCY = "architect-adjacency" as const;
export const ARCHITECT_BODY_ADJACENCY = "architect.adjacency" as const;
