/** 🧬️ Presentation artifact schema — every field with its state class. */
export interface PresentationArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ source: FigureTileSource;
  /** @state artifact */ tiles: FigureTileDraft[];
  /** @state presence */ selectedIds: string[];
  /** @state config */ engagementInput: string;
  /** @state config */ locale: string;
}
export interface FigureTileFrame { x: number; y: number; width: number; height: number; }
export interface FigureTileSource { src: string; kind: string; frame: FigureTileFrame; sourceAspect?: number | null; pdfPage?: number | null; }
export interface FigureTileDraft { id: string; name: string; crop: FigureTileFrame; }
