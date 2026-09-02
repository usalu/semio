/** 🌳️ Assembly viewer — `structure` window: typed twin of `🦀️.rs`'s `TreeWindowKit`
 * view-model. Read-only counterpart of `✏️editor/…/🌳️structure/🟦️.ts`: no command payload
 * types, since the viewer declares no actions. */

/** 🌳️ One tree leaf — mirrors the framework `TreeNodeView` shape (`framework.window.tree`). */
export interface AssemblyStructureNode {
  id: string;
  label: string;
  children: AssemblyStructureNode[];
}

/** 👁️ The `structure` window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * input (a bare `AssemblySnapshot`). */
export interface AssemblyStructureViewModel {
  windowKindId: "framework.window.tree";
  bodyKey: "framework.window.tree";
  roots: AssemblyStructureNode[];
}

export const ASSEMBLY_STRUCTURE_WINDOW_KIND_ID = "framework.window.tree" as const;
export const ASSEMBLY_STRUCTURE_BODY_KEY = "framework.window.tree" as const;
