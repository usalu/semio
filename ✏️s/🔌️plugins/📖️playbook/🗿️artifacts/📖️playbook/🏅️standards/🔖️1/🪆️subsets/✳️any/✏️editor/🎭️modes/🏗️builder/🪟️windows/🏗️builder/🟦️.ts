/** 🏗️ Playbook editor — Builder window: typed twin of `🦀️.rs`'s `render()` boundary (the
 * drag/drop Blockly-like form-authoring block-list surface). */

export interface PlaybookBuilderBlockOption {
  value: string;
  label: string;
}

export interface PlaybookBuilderVectorField {
  key: string;
  label?: string;
  value?: number;
}

/** 🧱️ One authored form field — mirrors the Rust `PlaybookBlock`'s full form-field vocabulary. */
export interface PlaybookBuilderBlock {
  id: string;
  label: string;
  kind: string;
  description?: string;
  required?: boolean;
  placeholder?: string;
  min?: number;
  max?: number;
  step?: number;
  unit?: string;
  text?: string;
  options?: PlaybookBuilderBlockOption[];
  fields?: PlaybookBuilderVectorField[];
  schema?: string;
  src?: string;
  accept?: string;
  fixtureSlug?: string;
}

export interface PlaybookBuilderStep {
  id: string;
  title: string;
  description?: string;
  blocks: PlaybookBuilderBlock[];
}

/** 🎨️ One palette entry (a `kind` a new block can be created as) — builtin or module-contributed via
 * the `"playbook.blockKind"` topic contribution. */
export interface PlaybookBuilderPaletteEntry {
  blockKind: string;
  label: string;
  iconId: string;
}

/** 🪟️ The Builder window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (the document's steps plus the current palette). */
export interface PlaybookBuilderViewModel {
  windowKindId: "playbook-builder";
  bodyKey: "playbook.play.builder";
  surfaceId: "playbook.play.builder";
  steps: PlaybookBuilderStep[];
  palette: PlaybookBuilderPaletteEntry[];
}

export const PLAYBOOK_PLAY_WINDOW_BUILDER = "playbook-builder" as const;
export const PLAYBOOK_PLAY_BODY_BUILDER = "playbook.play.builder" as const;
export const PLAYBOOK_PLAY_SURFACE_BUILDER = "playbook.play.builder" as const;
