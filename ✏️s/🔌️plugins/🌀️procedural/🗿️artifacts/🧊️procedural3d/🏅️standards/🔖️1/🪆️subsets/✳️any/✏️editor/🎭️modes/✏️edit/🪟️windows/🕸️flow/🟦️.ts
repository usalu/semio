/** 🕸️ Procedural3d editor — Flow window (edit mode): typed twin of `🦀️.rs`'s view-model.
 * Mirrors the pane's `render(document: &Procedural3dSnapshot, config: &Procedural3dConfig, session:
 * &FlowEvalSession)` boundary — the editable node-graph scene (operators, catalogue, live eval) a
 * mutation-capable surface carries. There is no viewer twin of this window: the read-only surface
 * ships a single mesh-preview window instead (see `👁️viewer/…/🟦️.ts`). */

/** ✏️ The Flow window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface Procedural3dFlowViewModel {
  windowKindId: "procedural-main";
  bodyKey: "procedural.play.main";
  surfaceId: "procedural.play";
  /** 📷️ The flow-graph node canvas camera (`x`/`y`/`zoom`). */
  camera: { x: number; y: number; zoom: number };
  /** 🎚️ Level-of-detail tessellation mode driving the LOD chrome measure. */
  lodMode: string;
  /** 🧬️ Whether the graph is editable (always true for the editor's own Flow window). */
  editable: true;
}

export const PROCEDURAL3D_PLAY_FLOW_WINDOW_KIND_ID = "procedural-main" as const;
export const PROCEDURAL3D_PLAY_FLOW_BODY_KEY = "procedural.play.main" as const;
export const PROCEDURAL3D_PLAY_FLOW_SURFACE_ID = "procedural.play" as const;
