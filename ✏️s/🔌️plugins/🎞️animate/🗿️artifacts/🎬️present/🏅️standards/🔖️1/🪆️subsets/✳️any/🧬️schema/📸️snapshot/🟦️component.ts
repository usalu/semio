/** 📸️ Present snapshot schema — persistent fields only. */
export interface PresentSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ source: FigureTileSource;
  /** @state artifact */ tiles: FigureTileDraft[];
}
export interface FigureTileFrame { x: number; y: number; width: number; height: number; }
export interface FigureTileSource { src: string; kind: string; frame: FigureTileFrame; sourceAspect?: number | null; pdfPage?: number | null; }
export interface FigureTileDraft { id: string; name: string; crop: FigureTileFrame; }
