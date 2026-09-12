import type { Viewport2d } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";

/** 🕸️ Architect editor — Graph window: typed twin of `🦀️.rs`'s view boundary. Mirrors
 * `render(program: &ProgramSnapshot, cfg: &ArchitectGraphWindowConfig) -> UiNode`'s signature — the program
 * elements and their adjacencies as an undirected node-graph surface, laid out on a circle. */

/** 🕸️ The Graph window's typed view-model — mirrors the Rust `render()` boundary's inputs: the whole
 * program document (read for its elements/adjacencies) plus the exact window's shared viewport. */
export interface ArchitectGraphViewModel {
  windowKindId: "architect-graph";
  bodyKey: "architect.graph";
  viewport: Viewport2d;
}

export const ARCHITECT_WINDOW_GRAPH = "architect-graph" as const;
export const ARCHITECT_BODY_GRAPH = "architect.graph" as const;
