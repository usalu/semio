/** 📜️ VCS viewer — History window: typed twin of `🦀️.rs`'s
 * `render(history: &HistoryView)` boundary — a read-only tree of checkpoints (each checkpoint nested
 * under its parent). Read-only counterpart of the editor's swimlane-graph History window: no
 * `alternativeIds`/navigation-action fields here, since a viewer declares no actions at all. */

/** ✏️ One node of the read-only checkpoint tree — mirrors framework `TreeNodeView`. */
export interface VcsHistoryTreeNode {
  id: string;
  label: string;
  children: VcsHistoryTreeNode[];
}

/** ✏️ The History window's typed view-model — mirrors the Rust `render()` boundary's input, already
 * reduced to the framework `TreeWindowKit`'s `TreeView` shape. */
export interface VcsViewHistoryViewModel {
  windowKindId: "framework.window.tree";
  bodyKey: "framework.window.tree";
  roots: VcsHistoryTreeNode[];
}

export const VCS_VIEW_WINDOW_HISTORY = "framework.window.tree" as const;
export const VCS_VIEW_BODY_HISTORY = "framework.window.tree" as const;
