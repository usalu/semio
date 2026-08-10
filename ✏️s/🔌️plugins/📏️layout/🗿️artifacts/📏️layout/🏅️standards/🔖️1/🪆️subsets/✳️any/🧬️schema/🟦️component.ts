/** 🧬️ Layout LayoutArtifact schema. */
export interface LayoutArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  name: string;
  /** @state persistent */
  grid: GridSettings;
  /** @state persistent */
  paragraphStyles: ParagraphStyle[];
  /** @state persistent */
  characterStyles: CharacterStyle[];
  /** @state persistent */
  stories: TextStory[];
  /** @state persistent */
  links: ImageLink[];
  /** @state persistent */
  parentPages: ParentPage[];
  /** @state persistent */
  spreads: Spread[];
  /** @state persistent */
  pages: Page[];
  /** @state persistent */
  printTarget?: string;
  /** @state persistent */
  dataFieldsJson?: string;
  /** @state shared-ui */
  selectedIds: string[];
  /** @state local-ui */
  activePageId: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  cameraX: number;
  /** @state local-ui */
  cameraY: number;
  /** @state local-ui */
  cameraZoom: number;
  /** @state local-ui */
  previewCameraX: number;
  /** @state local-ui */
  previewCameraY: number;
  /** @state local-ui */
  previewCameraZoom: number;
  /** @state local-ui */
  dropPreview: LayoutDropPreviewState;
  /** @state local-ui */
  locale: string;
  /** @state preview */
  hoveredId?: string;
}

export interface GridSettings { baselineGrid: number; baselineOffset: number; snapToBaseline: boolean; }
export interface LayoutDropPreviewState { kind: string; x: number; y: number; }
export interface LayoutStringList { values: string[]; }
