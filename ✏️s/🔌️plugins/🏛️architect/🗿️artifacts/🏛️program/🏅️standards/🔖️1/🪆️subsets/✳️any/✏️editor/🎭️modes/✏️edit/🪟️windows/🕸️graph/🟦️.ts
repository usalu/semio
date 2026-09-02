/** 🕸️ Architect editor — Graph window: typed twin of `🦀️.rs`'s view boundary. Mirrors
 * `render(program: &ProgramSnapshot, cfg: &ArchitectConfig) -> UiNode`'s signature — the program
 * elements and their adjacencies as an undirected node-graph surface, laid out on a circle. */

/** 🎥️ Ephemeral node-graph camera — mirrors the Rust `GraphCamera` struct, parsed from
 * `nodeGraphViewport`'s JSON payload and, on render, reassembled from the config's flattened
 * `graph_camera_{x,y,zoom}` fields. */
export interface ArchitectGraphCamera {
  x: number;
  y: number;
  zoom: number;
}

/** 🕸️ The Graph window's typed view-model — mirrors the Rust `render()` boundary's inputs: the whole
 * program document (read for its elements/adjacencies) plus the config-derived camera. */
export interface ArchitectGraphViewModel {
  windowKindId: "architect-graph";
  bodyKey: "architect.graph";
  camera: ArchitectGraphCamera;
}

export const ARCHITECT_WINDOW_GRAPH = "architect-graph" as const;
export const ARCHITECT_BODY_GRAPH = "architect.graph" as const;
