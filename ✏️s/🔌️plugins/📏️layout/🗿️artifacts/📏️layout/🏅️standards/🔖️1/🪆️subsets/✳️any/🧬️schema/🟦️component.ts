/** 🧬️ Layout LayoutArtifact schema. */
export interface LayoutArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  name: string;
  /** @state artifact */
  grid: GridSettings;
  /** @state artifact */
  paragraphStyles: ParagraphStyle[];
  /** @state artifact */
  characterStyles: CharacterStyle[];
  /** @state artifact */
  stories: TextStory[];
  /** @state artifact */
  links: ImageLink[];
  /** @state artifact */
  parentPages: ParentPage[];
  /** @state artifact */
  spreads: Spread[];
  /** @state artifact */
  pages: Page[];
  /** @state artifact */
  printTarget?: string;
  /** @state artifact */
  dataFieldsJson?: string;
  /** @state presence */
  selectedIds: string[];
  /** @state config */
  activePageId: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  cameraX: number;
  /** @state config */
  cameraY: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
  previewCameraX: number;
  /** @state config */
  previewCameraY: number;
  /** @state config */
  previewCameraZoom: number;
  /** @state config */
  dropPreview: LayoutDropPreviewState;
  /** @state config */
  locale: string;
  /** @state artifact */
  hoveredId?: string;
}

export interface GridSettings { baselineGrid: number; baselineOffset: number; snapToBaseline: boolean; }
export interface LayoutDropPreviewState { kind: string; x: number; y: number; }
export interface LayoutStringList { values: string[]; }
