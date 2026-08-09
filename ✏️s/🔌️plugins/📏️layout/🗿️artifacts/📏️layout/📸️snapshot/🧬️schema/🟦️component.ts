/** 🧬️ Layout LayoutSnapshot schema. */
export interface LayoutSnapshot {
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
}

export interface GridSettings { baselineGrid: number; baselineOffset: number; snapToBaseline: boolean; }
export interface LayoutDropPreviewState { kind: string; x: number; y: number; }
export interface LayoutStringList { values: string[]; }
