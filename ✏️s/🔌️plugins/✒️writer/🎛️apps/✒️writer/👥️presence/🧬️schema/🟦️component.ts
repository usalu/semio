/** 🧬️ WriterPresence */
export interface WriterPresence {
  /** @state shared-ui */
  selectedAstIds: string[];
  /** @state shared-ui */
  editorSelection?: WriterEditorSelection;
  /** @state shared-ui */
  treeHoveredAstId?: string;
  /** @state shared-ui */
  editorHoverOffset?: number;
  /** @state shared-ui */
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
