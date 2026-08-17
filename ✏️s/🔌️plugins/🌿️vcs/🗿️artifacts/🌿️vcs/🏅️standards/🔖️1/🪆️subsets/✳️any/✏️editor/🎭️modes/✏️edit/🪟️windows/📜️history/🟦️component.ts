/** 📜️ VCS editor — History window: typed twin of `🦀️component.rs`'s
 * `render(history: &HistoryView)` boundary — the checkpoint/alternative swimlane graph. Mirrors
 * the framework `HistoryColumn`/`Author` wire shapes (`🌿️vcs/🦀️component.rs`) field-for-field. */

/** ✏️ One committed checkpoint's author, mirrors framework `Author`. */
export interface VcsHistoryAuthor {
  id: string;
  name: string;
  avatar?: string;
}

/** ✏️ One row of the swimlane graph, mirrors framework `HistoryColumn`. */
export interface VcsHistoryColumnViewModel {
  checkpointId: string;
  timestamp: string;
  labels: string[];
  authors: VcsHistoryAuthor[];
  parentCheckpointId?: string;
  description?: string;
  lane: number;
  alternativeIds: string[];
}

/** ✏️ The History window's typed view-model — mirrors the Rust `render()` boundary's input. */
export interface VcsHistoryViewModel {
  windowKindId: "vcs-history";
  bodyKey: "vcs.play.history";
  columns: VcsHistoryColumnViewModel[];
}

export const VCS_PLAY_WINDOW_HISTORY = "vcs-history" as const;
export const VCS_PLAY_BODY_HISTORY = "vcs.play.history" as const;
