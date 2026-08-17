/** 📽️ Sequence viewer — Main window: typed twin of `🦀️component.rs`'s view-model. Mirrors the
 * window's `render(document: &SequenceSnapshot)` boundary — the read-only step/edge node-graph, no
 * config dependency (a viewer has no persisted per-session state, `Config = NoConfig`), and no
 * editability (unlike the editor's own Main window, see `✏️editor/…/📽️main/🟦️component.ts`). */

/** 👁️ One sequence step, read-only — mirrors Rust `SequenceStep` (the working-representation type). */
export interface SequenceViewStep {
  id: string;
  kind: string;
  x: number;
  y: number;
}

/** 👁️ One flow edge between two steps, read-only — mirrors Rust `SequenceEdge`. */
export interface SequenceViewEdge {
  id: string;
  from: string;
  to: string;
}

/** 👁️ The Main window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface SequenceViewMainViewModel {
  windowKindId: "sequence-view-main";
  bodyKey: "sequence.view.main";
  surfaceId: "sequence.view.main";
  steps: SequenceViewStep[];
  edges: SequenceViewEdge[];
  editable: false;
}

export const SEQUENCE_VIEW_WINDOW_MAIN = "sequence-view-main" as const;
export const SEQUENCE_VIEW_BODY_MAIN = "sequence.view.main" as const;
export const SEQUENCE_VIEW_SURFACE_MAIN = "sequence.view.main" as const;
