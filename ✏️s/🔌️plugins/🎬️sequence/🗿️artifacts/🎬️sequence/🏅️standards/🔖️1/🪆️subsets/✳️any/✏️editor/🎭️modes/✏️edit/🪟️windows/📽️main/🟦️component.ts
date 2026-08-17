/** 📽️ Sequence editor — Main window: typed twin of `🦀️component.rs`'s view-model. Mirrors the
 * window's `render(fixture: &SequenceSnapshot, config: &SequenceConfig)` boundary — the live
 * step/edge node-graph plus the config-owned viewport camera, editable (mutation-capable), absent
 * from the viewer's read-only twin (see `👁️viewer/…/📽️main/🟦️component.ts`). */

/** ✏️ One sequence step — mirrors Rust `SequenceStep` (the working-representation type, not the
 * composed-child persisted shape). */
export interface SequenceStep {
  id: string;
  kind: string;
  params: Record<string, unknown>;
  x: number;
  y: number;
  slot?: { owner: string; name: string };
  collapsed: boolean;
}

/** ✏️ One flow edge between two steps — mirrors Rust `SequenceEdge`. */
export interface SequenceEdge {
  id: string;
  from: string;
  to: string;
}

/** ✏️ Pan/zoom viewport — mirrors Rust `SequenceCamera` (config-owned, session-only). */
export interface SequenceViewport {
  x: number;
  y: number;
  zoom: number;
}

/** ✏️ The Main window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface SequenceMainViewModel {
  windowKindId: "sequence-main";
  bodyKey: "sequence.play.main";
  surfaceId: "sequence.play.main";
  steps: SequenceStep[];
  edges: SequenceEdge[];
  viewport: SequenceViewport;
  editable: true;
}

export const SEQUENCE_PLAY_WINDOW_MAIN = "sequence-main" as const;
export const SEQUENCE_PLAY_BODY_MAIN = "sequence.play.main" as const;
export const SEQUENCE_PLAY_SURFACE_MAIN = "sequence.play.main" as const;
