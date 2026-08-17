/** 🗂️ Flow editor — Generate-mode Generations window: typed twin of `🦀️component.rs`'s view-model.
 * Mirrors the pane's `render(config: &FlowConfig, locale: Locale, terminology: Terminology)` boundary —
 * the generation list tree with the "add generation" affordance. */

/** 🗂️ One row of the generation list — mirrors the Rust `playbook::GenerationPlayState` entry shape. */
export interface FlowGenerationListEntry {
  id: string;
  name: string;
}

/** 🗂️ The Generations window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface FlowGenerationsViewModel {
  windowKindId: "flow-generations";
  bodyKey: "flow.play.generations";
  generations: FlowGenerationListEntry[];
  selectedGenerationId: string | null;
  locale: string;
}

export const FLOW_PLAY_WINDOW_GENERATIONS = "flow-generations" as const;
export const FLOW_PLAY_BODY_GENERATIONS = "flow.play.generations" as const;
