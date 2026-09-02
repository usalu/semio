/** 📜️ Sequence editor — Script window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * window's `render(fixture: &SequenceSnapshot, config: &SequenceConfig)` boundary — the compiled
 * imperative path text, with the last `run` result appended when present. Mutation-capable surface;
 * the viewer has no Script window (see `👁️viewer` design notes). */

/** ✏️ The Script window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface SequenceScriptViewModel {
  windowKindId: "sequence-script";
  bodyKey: "sequence.play.script";
  surfaceId: "sequence.play.script";
  /** 📝️ `SequenceHost::compile_text()` — the compiled imperative path source. */
  compiledText: string;
  /** 🏃️ `SequenceConfig.lastRunJson` — appended under a `# run result` heading when non-empty. */
  lastRunJson: string;
  language: "imperative";
}

export const SEQUENCE_PLAY_WINDOW_SCRIPT = "sequence-script" as const;
export const SEQUENCE_PLAY_BODY_SCRIPT = "sequence.play.script" as const;
export const SEQUENCE_PLAY_SURFACE_SCRIPT = "sequence.play.script" as const;
