/** 🧬️ Writer artifact schema. */

export interface WriterArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  id: string;
  /** @state persistent */
  languageId: string;
  /** @state persistent */
  uri: string;
  /** @state persistent */
  text: string;
  /** @state shared-ui */
  selectedAstIds: string[];
  /** @state shared-ui */
  editorSelection?: WriterEditorSelection;
  /** @state shared-ui */
  editorSettings: WriterEditorSettings;
  /** @state local-ui */
  formatSignal: number;
  /** @state local-ui */
  lintSignal: number;
  /** @state local-ui */
  revision: number;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  cameraX: number;
  /** @state local-ui */
  cameraY: number;
  /** @state local-ui */
  cameraZoom: number;
  /** @state local-ui */
  locale: string;
  /** @state preview */
  treeHoveredAstId?: string;
  /** @state preview */
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

