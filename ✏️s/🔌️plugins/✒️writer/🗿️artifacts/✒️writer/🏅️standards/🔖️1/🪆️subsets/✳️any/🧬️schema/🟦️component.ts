/** 🧬️ Writer artifact schema. */

export interface WriterArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  languageId: string;
  /** @state artifact */
  uri: string;
  /** @state artifact */
  text: string;
  /** @state presence */
  selectedAstIds: string[];
  /** @state presence */
  editorSelection?: WriterEditorSelection;
  /** @state presence */
  editorSettings: WriterEditorSettings;
  /** @state config */
  formatSignal: number;
  /** @state config */
  lintSignal: number;
  /** @state config */
  revision: number;
  /** @state config */
  engagementInput: string;
  /** @state config */
  cameraX: number;
  /** @state config */
  cameraY: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
  locale: string;
  /** @state artifact */
  treeHoveredAstId?: string;
  /** @state artifact */
  editorHoverOffset?: number;
}

export interface WriterEditorSelection {
  start: number;
  end: number;
}

export interface WriterEditorSettings {
  showLineNumbers: boolean;
  fontPx: number;
  lineHeight: number;
  tabSize: number;
}

export interface WriterStringList {
  values: string[];
}

export interface WriterTextRangeEdit {
  start: number;
  end: number;
  insert: string;
}

export interface WriterTextDelta {
  replacement?: string;
  edits: WriterTextRangeEdit[];
}

