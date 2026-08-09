/** 📸️ Present snapshot schema — persistent fields only. */
export interface PresentSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ source: FigureTileSource;
  /** @state persistent */ tiles: FigureTileDraft[];
}
export interface FigureTileFrame { x: number; y: number; width: number; height: number; }
export interface FigureTileSource { src: string; kind: string; frame: FigureTileFrame; sourceAspect?: number | null; pdfPage?: number | null; }
export interface FigureTileDraft { id: string; name: string; crop: FigureTileFrame; }
