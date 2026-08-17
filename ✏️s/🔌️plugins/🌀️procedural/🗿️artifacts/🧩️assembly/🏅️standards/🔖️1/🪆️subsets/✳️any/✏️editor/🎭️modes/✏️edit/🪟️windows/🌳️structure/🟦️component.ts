/** 🌳️ Assembly editor — `structure` window: typed twin of `🦀️component.rs`'s `TreeWindowKit`
 * view-model. Mirrors the Rust `render()` boundary's output shape: one branch per collection
 * (slots/edges/modules/weights/rules) on the WFC problem spec — never the solved assignment, which is
 * an inference, not persisted state. */

/** 🌳️ One tree leaf — mirrors the framework `TreeNodeView` shape (`framework.window.tree`). */
export interface AssemblyStructureNode {
  id: string;
  label: string;
  children: AssemblyStructureNode[];
}

/** ✏️ The `structure` window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * input (a bare `AssemblySnapshot`). */
export interface AssemblyStructureViewModel {
  windowKindId: "framework.window.tree";
  bodyKey: "framework.window.tree";
  roots: AssemblyStructureNode[];
}

/** ✏️ Typed command payload shapes — mirror `AssemblyEditorCommand`'s variants one-for-one. */
export interface AssemblyCreateSlot { index: number; id: string; x: number; y: number; z: number; pinnedModuleId: string | null }
export interface AssemblyDeleteSlot { id: string }
export interface AssemblyCreateRule { index: number; id: string; moduleAId: string; moduleBId: string; allowed: boolean }
export interface AssemblyDeleteRule { id: string }
export interface AssemblyConnectSlots { index: number; id: string; fromSlotId: string; toSlotId: string }
export interface AssemblyDisconnectSlots { id: string }
export interface AssemblyChangeWeight { moduleId: string; weight: number }
export interface AssemblyRemoveWeight { moduleId: string }
export interface AssemblyChangeSeed { seed: number }

export const ASSEMBLY_STRUCTURE_WINDOW_KIND_ID = "framework.window.tree" as const;
export const ASSEMBLY_STRUCTURE_BODY_KEY = "framework.window.tree" as const;
