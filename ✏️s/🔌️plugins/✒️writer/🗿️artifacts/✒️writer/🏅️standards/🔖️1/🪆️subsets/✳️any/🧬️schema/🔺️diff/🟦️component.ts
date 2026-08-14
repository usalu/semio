/** 🧬️ Writer diff schema — sparse field delta. */

export interface WriterDiff {
  /** @state artifact */
  artifact?: WriterArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  languageId?: string;
  /** @state artifact */
  uri?: string;
  /** @state artifact */
  text?: WriterTextDelta;
  /** @state presence */
  editorSelection?: WriterEditorSelection | null;
  /** @state presence */
  editorSettings?: WriterEditorSettings;
  /** @state config */
  formatSignal?: number;
  /** @state config */
  lintSignal?: number;
  /** @state config */
  revision?: number;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  cameraX?: number;
  /** @state config */
  cameraY?: number;
  /** @state config */
  cameraZoom?: number;
  /** @state config */
  locale?: string;
}

export interface WriterArtifact {
  schema: string;
  id: string;
  languageId: string;
  uri: string;
  text: string;
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

export interface WriterTextRangeEdit {
  start: number;
  end: number;
  insert: string;
}

export interface WriterTextDelta {
  replacement?: string;
  edits: WriterTextRangeEdit[];
}
