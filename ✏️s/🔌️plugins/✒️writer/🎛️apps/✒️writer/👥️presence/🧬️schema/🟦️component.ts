/** 🧬️ WriterPresence */
export interface WriterPresence {
  /** @state presence */
  selectedAstIds: string[];
  /** @state presence */
  editorSelection?: WriterEditorSelection;
  /** @state presence */
  treeHoveredAstId?: string;
  /** @state presence */
  editorHoverOffset?: number;
  /** @state presence */
  camera: WriterCamera;
}

export interface WriterEditorSelection {
  start: number;
  end: number;
}

export interface WriterCamera {
  x: number;
  y: number;
  zoom: number;
}
