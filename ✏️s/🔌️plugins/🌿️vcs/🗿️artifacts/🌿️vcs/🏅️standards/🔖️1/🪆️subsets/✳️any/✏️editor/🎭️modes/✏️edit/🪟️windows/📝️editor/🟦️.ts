/** 📝️ VCS editor — Editor window: typed twin of `🦀️.rs`'s
 * `render(projection: &VcsSnapshot, labels: &VcsPlayLabels)` boundary — counter/commit/branch/undo/
 * redo actions plus a projection summary. */

/** ✏️ Mirrors the artifact-level `VcsSnapshot` fields this window reads. */
export interface VcsPlaySnapshotViewModel {
  schema: string;
  title: string;
  counter: number;
  notes: string;
  status: string;
  tags: string[];
}

/** ✏️ Mirrors the resolved (locale-picked) `VcsPlayLabels` fields this window reads. */
export interface VcsPlayLabelsViewModel {
  actions: string;
  counter: string;
  commit: string;
  branch: string;
  undo: string;
  redo: string;
}

/** ✏️ The Editor window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface VcsEditorViewModel {
  windowKindId: "vcs-editor";
  bodyKey: "vcs.play.editor";
  projection: VcsPlaySnapshotViewModel;
  labels: VcsPlayLabelsViewModel;
}

export const VCS_PLAY_WINDOW_EDITOR = "vcs-editor" as const;
export const VCS_PLAY_BODY_EDITOR = "vcs.play.editor" as const;
