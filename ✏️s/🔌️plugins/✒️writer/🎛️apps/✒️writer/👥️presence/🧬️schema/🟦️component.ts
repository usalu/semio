/** 🧬️ WriterPresence */
export interface WriterPresence {
  /** @state presence */
  editorSelection?: WriterEditorSelection;
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
