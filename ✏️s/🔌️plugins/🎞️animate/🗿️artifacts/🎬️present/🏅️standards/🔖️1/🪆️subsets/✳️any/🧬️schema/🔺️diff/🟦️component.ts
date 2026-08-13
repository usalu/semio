/** 🔺️ Present diff schema — sparse field delta. */
export interface PresentDiff {
  /** @state artifact */ artifact?: PresentArtifact | null;
  /** @state artifact */ schema?: string | null;
  /** @state artifact */ source?: FigureTileSource | null;
  /** @state artifact */ tiles?: PresentTilesDelta | null;
  /** @state presence */ selectedIds?: PresentStringList | null;
  /** @state config */ engagementInput?: string | null;
  /** @state config */ locale?: string | null;
}
export interface PresentStringList { values: string[]; }
export interface PresentTilesDelta { added: FigureTileDraft[]; removed: string[]; patched: PresentTilePatchEntry[]; reordered?: string[] | null; }
export interface PresentTilePatchEntry { id: string; patch: FigureTileDraftPatch; }
export interface FigureTileDraftPatch { name?: string | null; crop?: FigureTileFrame | null; }
export interface FigureTileFrame { x: number; y: number; width: number; height: number; }
export interface FigureTileSource { src: string; kind: string; frame: FigureTileFrame; sourceAspect?: number | null; pdfPage?: number | null; }
export interface FigureTileDraft { id: string; name: string; crop: FigureTileFrame; }
export interface PresentArtifact {
  schema: string;
  source: FigureTileSource;
  tiles: FigureTileDraft[];
  selectedIds: string[];
  engagementInput: string;
  locale: string;
}
