/** 🧬️ LayoutConfig */
export interface LayoutConfig {
  /** @state config */
  activePageId: string;
  /** @state config */
  selectedIds: string[];
  /** @state config */
  hoveredId?: string;
  /** @state config */
  dropPreview: LayoutDropPreviewState;
  /** @state config */
  engagementInput: string;
  /** @state config */
  camera: LayoutCamera;
  /** @state config */
  previewCamera: LayoutCamera;
  /** @state config */
  locale: string;
}
export interface LayoutDropPreviewState { kind: string; x: number; y: number; }
export interface LayoutCamera { x: number; y: number; zoom: number; }
