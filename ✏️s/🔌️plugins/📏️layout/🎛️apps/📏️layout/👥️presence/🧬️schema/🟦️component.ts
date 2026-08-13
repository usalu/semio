/** 🧬️ LayoutPresence */
export interface LayoutPresence {
  /** @state presence */
  activePageId: string;
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  hoveredId?: string;
  /** @state presence */
  dropPreview: LayoutDropPreviewState;
  /** @state presence */
  camera: LayoutCamera;
  /** @state presence */
  previewCamera: LayoutCamera;
}
export interface LayoutDropPreviewState { kind: string; x: number; y: number; }
export interface LayoutCamera { x: number; y: number; zoom: number; }
