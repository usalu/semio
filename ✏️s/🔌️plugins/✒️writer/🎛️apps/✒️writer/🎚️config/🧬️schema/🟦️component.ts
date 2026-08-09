/** 🧬️ WriterConfig */
export interface WriterConfig {
  /** @state local-ui */
  selectedAstIds: string[];
  /** @state local-ui */
  editorSelection?: WriterEditorSelection;
  /** @state local-ui */
  formatSignal: number;
  /** @state local-ui */
  lintSignal: number;
  /** @state local-ui */
  revision: number;
  /** @state local-ui */
  editorSettings: WriterEditorSettings;
  /** @state local-ui */
  treeHoveredAstId?: string;
  /** @state local-ui */
  editorHoverOffset?: number;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  camera: WriterCamera;
  /** @state local-ui */
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
