/** 🧬️ Sequence editor — Compiled window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * window's `render(fixture: &SequenceSnapshot)` boundary — the read-only compiled DAG wire literal,
 * independent of `SequenceConfig` (unlike the Main/Script windows). */

/** ✏️ The Compiled window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface SequenceCompiledViewModel {
  windowKindId: "sequence-compiled-dag";
  bodyKey: "sequence.play.compiled-dag";
  surfaceId: "sequence.play.compiled-dag";
  /** 📝️ `SequenceHost::compiled_wire_literal()` — the compiled DAG rendered as wire-literal text. */
  wireLiteral: string;
  language: "wire";
}

export const SEQUENCE_PLAY_WINDOW_COMPILED = "sequence-compiled-dag" as const;
export const SEQUENCE_PLAY_BODY_COMPILED = "sequence.play.compiled-dag" as const;
export const SEQUENCE_PLAY_SURFACE_COMPILED = "sequence.play.compiled-dag" as const;
