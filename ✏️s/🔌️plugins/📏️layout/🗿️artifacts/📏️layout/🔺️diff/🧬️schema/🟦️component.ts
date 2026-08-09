/** 🧬️ Layout LayoutDiff schema. */
export interface LayoutDiff {
  /** @state persistent */
  artifact?: LayoutArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  name?: string;
  /** @state persistent */
  grid?: GridSettings;
  /** @state persistent */
  paragraphStyles?: LayoutParagraphStylesDelta;
  /** @state persistent */
  characterStyles?: LayoutCharacterStylesDelta;
  /** @state persistent */
  stories?: LayoutStoriesDelta;
  /** @state persistent */
  links?: LayoutLinksDelta;
  /** @state persistent */
  parentPages?: LayoutParentPagesDelta;
  /** @state persistent */
  spreads?: LayoutSpreadsDelta;
  /** @state persistent */
  pages?: LayoutPagesDelta;
  /** @state persistent */
  printTarget?: string | null;
  /** @state persistent */
  dataFieldsJson?: string | null;
  /** @state shared-ui */
  selectedIds?: LayoutStringList;
  /** @state local-ui */
  activePageId?: string;
  /** @state local-ui */
  engagementInput?: string;
  /** @state local-ui */
  cameraX?: number;
  /** @state local-ui */
  cameraY?: number;
  /** @state local-ui */
  cameraZoom?: number;
  /** @state local-ui */
  previewCameraX?: number;
  /** @state local-ui */
  previewCameraY?: number;
  /** @state local-ui */
  previewCameraZoom?: number;
  /** @state local-ui */
  dropPreview?: LayoutDropPreviewState;
  /** @state local-ui */
  locale?: string;
  /** @state preview */
  hoveredId?: string | null;
}

export interface GridSettings { baselineGrid: number; baselineOffset: number; snapToBaseline: boolean; }
export interface LayoutDropPreviewState { kind: string; x: number; y: number; }
export interface LayoutStringList { values: string[]; }
