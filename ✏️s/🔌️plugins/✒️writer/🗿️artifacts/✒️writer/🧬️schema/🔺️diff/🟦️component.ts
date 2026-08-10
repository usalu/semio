/** 🧬️ Writer diff schema — sparse field delta. */

export interface WriterDiff {
  /** @state persistent */
  artifact?: WriterArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  id?: string;
  /** @state persistent */
  languageId?: string;
  /** @state persistent */
  uri?: string;
  /** @state persistent */
  text?: WriterTextDelta;
  /** @state shared-ui */
  selectedAstIds?: WriterStringList;
  /** @state shared-ui */
  editorSelection?: WriterEditorSelection | null;
  /** @state shared-ui */
  editorSettings?: WriterEditorSettings;
  /** @state local-ui */
  formatSignal?: number;
  /** @state local-ui */
  lintSignal?: number;
  /** @state local-ui */
  revision?: number;
  /** @state local-ui */
  engagementInput?: string;
  /** @state local-ui */
  cameraX?: number;
  /** @state local-ui */
  cameraY?: number;
  /** @state local-ui */
  cameraZoom?: number;
  /** @state local-ui */
  locale?: string;
  /** @state preview */
  treeHoveredAstId?: string | null;
  /** @state preview */
  editorHoverOffset?: number | null;
}

export interface WriterArtifact {
  schema: string;
  id: string;
  languageId: string;
  uri: string;
  text: string;
  selectedAstIds: string[];
  editorSelection?: WriterEditorSelection;
  editorSettings: WriterEditorSettings;
  formatSignal: number;
  lintSignal: number;
  revision: number;
  engagementInput: string;
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
  locale: string;
  treeHoveredAstId?: string;
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
