/** 🧬️ WriterConfig */
export interface WriterConfig {
  /** @state config */
  selectedAstIds: string[];
  /** @state config */
  editorSelection?: WriterEditorSelection;
  /** @state config */
  formatSignal: number;
  /** @state config */
  lintSignal: number;
  /** @state config */
  revision: number;
  /** @state config */
  editorSettings: WriterEditorSettings;
  /** @state config */
  treeHoveredAstId?: string;
  /** @state config */
  editorHoverOffset?: number;
  /** @state config */
  engagementInput: string;
  /** @state config */
  camera: WriterCamera;
  /** @state config */
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

export interface WriterCamera {
  x: number;
  y: number;
  zoom: number;
}
