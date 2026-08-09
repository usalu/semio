/** 🧬️ LayoutPresence */
export interface LayoutPresence {
  /** @state shared-ui */
  activePageId: string;
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  hoveredId?: string;
  /** @state shared-ui */
  dropPreview: LayoutDropPreviewState;
  /** @state shared-ui */
  camera: LayoutCamera;
  /** @state shared-ui */
  previewCamera: LayoutCamera;
}
export interface LayoutDropPreviewState { kind: string; x: number; y: number; }
export interface LayoutCamera { x: number; y: number; zoom: number; }
