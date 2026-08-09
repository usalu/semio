/** 🧬️ Present artifact schema — every field with its state class. */
export interface PresentArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ source: FigureTileSource;
  /** @state persistent */ tiles: FigureTileDraft[];
  /** @state shared-ui */ selectedIds: string[];
  /** @state local-ui */ engagementInput: string;
  /** @state local-ui */ locale: string;
}
export interface FigureTileFrame { x: number; y: number; width: number; height: number; }
export interface FigureTileSource { src: string; kind: string; frame: FigureTileFrame; sourceAspect?: number | null; pdfPage?: number | null; }
export interface FigureTileDraft { id: string; name: string; crop: FigureTileFrame; }
