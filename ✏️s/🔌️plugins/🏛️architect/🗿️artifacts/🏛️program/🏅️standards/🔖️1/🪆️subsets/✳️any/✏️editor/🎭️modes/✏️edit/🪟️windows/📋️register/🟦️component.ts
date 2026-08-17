/** 📋️ Architect editor — Register window: typed twin of `🦀️component.rs`'s view boundary. Mirrors
 * `render(program: &ProgramSnapshot, cfg: &ArchitectConfig) -> UiNode`'s signature — the active
 * register's rows rendered as a block-list surface. */

/** 📋️ One block-list item — mirrors the Rust `RegisterBlockItem` struct. */
export interface ArchitectRegisterBlockItem {
  id: string;
  label: string;
  kind: string;
}

/** 📋️ One block-list step per register row — mirrors the Rust `RegisterBlockStep` struct, the wire
 * shape the block-list surface consumes. */
export interface ArchitectRegisterBlockStep {
  id: string;
  title: string;
  blocks: ArchitectRegisterBlockItem[];
}

/** 📋️ The Register window's typed view-model — mirrors the Rust `render()` boundary's inputs: the
 * whole program document (read for the active register's rows) plus the config's active register id. */
export interface ArchitectRegisterViewModel {
  windowKindId: "architect-register";
  bodyKey: "architect.register";
  activeRegister: string;
}

export const ARCHITECT_WINDOW_REGISTER = "architect-register" as const;
export const ARCHITECT_BODY_REGISTER = "architect.register" as const;
