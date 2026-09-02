/** 📋️ Architect viewer — Register window: typed twin of `🦀️.rs`'s view boundary. Mirrors
 * `render(program: &ProgramSnapshot) -> UiNode`'s read-only, document-wide register overview — every
 * non-empty register's entity count plus its draft/approved split. No per-session config (absent
 * entirely from the sibling editor surface's own Register window's typed twin, which additionally
 * carries an `activeRegister` selector — see the sibling surface's own file for that shape). */

/** 📋️ Per-register entity count and dominant status — mirrors the Rust `RegisterStatusCount` struct. */
export interface ArchitectViewRegisterStatusCount {
  register: string;
  count: number;
  draftCount: number;
  approvedCount: number;
}

/** 📋️ The Register window's typed view-model — mirrors the Rust `render()` boundary's sole input: the
 * whole program document (read for its `status_summary()`-derived per-register counts). */
export interface ArchitectViewRegisterViewModel {
  windowKindId: "architect-view-register";
  bodyKey: "architect.view.register";
}

export const ARCHITECT_VIEW_WINDOW_REGISTER = "architect-view-register" as const;
export const ARCHITECT_VIEW_BODY_REGISTER = "architect.view.register" as const;
