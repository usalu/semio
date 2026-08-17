/** 🪚️ Process 3D viewer — Workpiece window: typed twin of `🦀️component.rs`'s view-model. Mirrors
 * the window's read-only `render(fixture: &Process3dSnapshot)` boundary — no camera/sun/active-
 * utility fields here, unlike the editor's twin: the viewer has no config lane to read them from
 * and uses hardcoded defaults (see `🦀️component.rs`'s own doc comment). */

/** 👁️ The Workpiece window's typed view-model — the processed-mesh preview is derived
 * server-side from the snapshot, not carried here. */
export interface Process3dWorkpieceViewViewModel {
  windowKindId: "process-workpiece-view";
  bodyKey: "process.view.main";
  surfaceId: "process.view";
}

export const PROCESS3D_VIEW_WINDOW_MAIN = "process-workpiece-view" as const;
export const PROCESS3D_VIEW_BODY_MAIN = "process.view.main" as const;
