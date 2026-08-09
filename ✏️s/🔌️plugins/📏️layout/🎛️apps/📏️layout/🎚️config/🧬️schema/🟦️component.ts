/** 🧬️ LayoutConfig */
export interface LayoutConfig {
  /** @state local-ui */
  activePageId: string;
  /** @state local-ui */
  selectedIds: string[];
  /** @state local-ui */
  hoveredId?: string;
  /** @state local-ui */
  dropPreview: LayoutDropPreviewState;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  camera: LayoutCamera;
  /** @state local-ui */
  previewCamera: LayoutCamera;
  /** @state local-ui */
  locale: string;
}
export interface LayoutDropPreviewState { kind: string; x: number; y: number; }
export interface LayoutCamera { x: number; y: number; zoom: number; }
